use crate::rs::*;
use bevy::prelude::*;
use bevy::utils::HashMap;
use chrono::{DateTime, TimeZone, Utc};
use pecs::core::PromiseState;
use pecs::prelude::*;

#[derive(Resource)]
pub struct GEMW {
    pub last_update: chrono::DateTime<Utc>,
    pub data: HashMap<ItemId, Quantity>,
}

#[derive(Component)]
pub struct Price(pub Quantity);

static URL: &str = "https://runescape.wiki/w/Module:GEPricesByIDs/data.json?action=raw";

pub fn load_gemw(mut commands: Commands) {
    commands
        .promise(|| ())
        .then(asyn!(state => {
            info!("GEMW data loading from {}...", URL);
            state.asyn().http().get(URL)
        }))
        .then(asyn!(state, result, mut commands: Commands=> {
            let mut raw: HashMap<String, serde_json::Value> = serde_json::from_slice(&result.unwrap().bytes).unwrap();
            let mut gemw = GEMW{
                data: HashMap::with_capacity(raw.len()-2),
                last_update: Utc.timestamp_opt(raw.remove("%LAST_UPDATE%").unwrap().as_i64().unwrap(), 0).unwrap(),
            };
            raw.remove("%LAST_UPDATE_F%");
            for (id, price) in raw.drain() {
                gemw.data.insert(id.parse().unwrap(), price.as_i64().unwrap());
            }
            commands.insert_resource(gemw);
            info!("GEMW data loaded");
        }));
}

pub fn populate_gemw(q: Query<(Entity, &Item)>, gemw: Option<Res<GEMW>>, mut commands: Commands) {
    let Some(gemw) = gemw else { return; };
    if !gemw.is_changed() {
        return;
    }
    for (id, item) in &q {
        if let Some(&price) = gemw.data.get(&item.id) {
            commands.entity(id).insert(Price(price));
        }
    }
}

pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_gemw);
        app.add_systems(Update, populate_gemw);
    }
}
