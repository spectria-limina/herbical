use crate::desc_win::WinEvent;
use crate::rs::*;
use bevy::prelude::*;
use bevy_egui::{egui, EguiContexts};
use regex::RegexBuilder;

pub trait Searchable {
    fn hay(&self) -> &str;
    fn type_name(&self) -> &str;
}

impl Searchable for Item {
    fn hay(&self) -> &str {
        &*self.name
    }
    fn type_name(&self) -> &str {
        "Item"
    }
}

impl Searchable for Recipe {
    fn hay(&self) -> &str {
        &*self.name
    }
    fn type_name(&self) -> &str {
        "Recipe"
    }
}

#[derive(Default, Resource, Debug)]
pub struct State {
    needle: String,
    results: Vec<(Entity, Indexed)>,
}

impl State {
    pub fn new() -> Self {
        default()
    }
}

pub fn search(
    mut contexts: EguiContexts,
    mut state: ResMut<State>,
    mut win_events: EventWriter<WinEvent>,
) {
    let ctx = contexts.ctx_mut();
    egui::TopBottomPanel::top("search").show(ctx, |ui| {
        ui.horizontal(|ui| {
            ui.label("Search:");
            ui.text_edit_singleline(&mut state.needle);
        });

        for (id, idx) in &state.results {
            ui.separator();
            if ui.link(format!("{}: {}", idx.type_name, idx.hay)).clicked() {
                win_events.send(WinEvent::Open(*id))
            }
        }
    });
}

// TODO: Replace with a better form of indexing?
// TODO: Don't duplicate strings.
#[derive(Component, Clone, Debug)]
pub struct Indexed {
    hay: String,
    type_name: String,
}

pub fn index_new<S: Component + Searchable>(
    mut q: Query<(Entity, &S), Without<Indexed>>,
    mut commands: Commands,
) {
    for (id, s) in &mut q {
        commands.entity(id).insert(Indexed {
            hay: s.hay().into(),
            type_name: s.type_name().into(),
        });
    }
}

pub fn update_index<S: Component + Searchable>(mut q: Query<(&S, &mut Indexed), Changed<S>>) {
    for (s, mut idx) in &mut q {
        idx.hay = s.hay().into();
        idx.type_name = s.type_name().into();
    }
}

pub fn update_results(q: Query<(Entity, &Indexed)>, mut state: ResMut<State>) {
    state.results.clear();
    if state.needle == "" {
        return;
    }
    let re = RegexBuilder::new(&*regex::escape(&*state.needle))
        .case_insensitive(true)
        .build()
        .unwrap();
    for (id, idx) in &q {
        if re.is_match(&*idx.hay) {
            state.results.push((id, idx.clone()));
        }
    }
}
