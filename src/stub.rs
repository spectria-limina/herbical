use crate::desc_win::DescWin;
use crate::rs::{Skill::*, *};
use bevy::prelude::*;

pub struct Plugin();

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn);
    }
}

fn spawn(mut commands: Commands) {
    let atk_pot = commands
        .spawn((
            Item {
                name: "Attack potion (3)".into(),
                desc: "3 doses of attack potion.".into(),
                id: 121,
            },
            DescWin::new_open(),
        ))
        .id();
    let guam_potion_unf = commands
        .spawn((
            Item {
                name: "Guam potion (unf)".into(),
                desc: "I need another ingredient to finish this guam potion.".into(),
                id: 91,
            },
            DescWin::new(),
        ))
        .id();
    let eye_of_newt = commands
        .spawn((
            Item {
                name: "Eye of newt".into(),
                desc: "It seems to be looking at me. Used in Herblore (3).".into(),
                id: 221,
            },
            DescWin::new(),
        ))
        .id();
    let vial_of_water = commands
        .spawn((
            Item {
                name: "Vial of water".into(),
                desc: "A glass vial containing water.".into(),
                id: 227,
            },
            DescWin::new(),
        ))
        .id();
    let clean_guam = commands
        .spawn((
            Item {
                name: "Clean guam".into(),
                desc: "A fresh herb.".into(),
                id: 249,
            },
            DescWin::new_open(),
        ))
        .id();
    let grimy_guam = commands
        .spawn((
            Item {
                name: "Grimy guam".into(),
                desc: "I need to clean this herb before I can use it.".into(),
                id: 199,
            },
            DescWin::new(),
        ))
        .id();

    type IWQ = ItemWithQuantity;
    commands.spawn((
        Recipe {
            name: "Attack potion".into(),
            skill: Herblore,
            level: 1,
            xp: 250,
            product: IWQ {
                item: atk_pot,
                quantity: 1,
            },
            materials: vec![
                IWQ {
                    item: guam_potion_unf,
                    quantity: 1,
                },
                IWQ {
                    item: eye_of_newt,
                    quantity: 1,
                },
            ],
            ticks: 2,
        },
        DescWin::new_open(),
    ));
    commands.spawn((
        Recipe {
            name: "Guam potion (unf)".into(),
            skill: Herblore,
            level: 1,
            xp: 10,
            product: IWQ {
                item: guam_potion_unf,
                quantity: 1,
            },
            materials: vec![
                IWQ {
                    item: clean_guam,
                    quantity: 1,
                },
                IWQ {
                    item: vial_of_water,
                    quantity: 1,
                },
            ],
            ticks: 2,
        },
        DescWin::new(),
    ));
    commands.spawn((
        Recipe {
            name: "Clean guam".into(),
            skill: Herblore,
            level: 1,
            xp: 25,
            product: IWQ {
                item: clean_guam,
                quantity: 1,
            },
            materials: vec![IWQ {
                item: grimy_guam,
                quantity: 1,
            }],
            ticks: 1,
        },
        DescWin::new(),
    ));
}
