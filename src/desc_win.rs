use crate::rs::*;
use bevy::{prelude::*, ui};
use bevy_egui::{egui, EguiContexts};
use itertools::Itertools;

#[derive(Component, Debug, Default)]
pub struct DescWin {
    pub open: bool,
}

impl DescWin {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn new_open() -> Self {
        Self {
            open: true,
            ..default()
        }
    }
}

#[derive(Debug, Event)]
pub enum WinEvent {
    Open(Entity),
}

pub fn item_windows(
    mut q: Query<(Entity, &Item, &mut DescWin)>,
    mut recipes: Query<(Entity, &Recipe)>,
    mut win_events: EventWriter<WinEvent>,
    mut contexts: EguiContexts,
) {
    for (id, item, mut win) in &mut q {
        egui::Window::new(format!("Item: {name}", name = item.name))
            .open(&mut win.open)
            .show(contexts.ctx_mut(), |ui| {
                ui.label(item.desc.clone());

                let mut created_in = recipes
                    .iter_mut()
                    .filter(|(_, r)| r.product.item == id)
                    .peekable();
                if created_in.peek().is_some() {
                    ui.separator();
                    ui.heading("Recipes:");
                    ui.indent("Recipes", |ui| {
                        for (id, r) in created_in {
                            if ui.link(r.name.clone()).clicked() {
                                win_events.send(WinEvent::Open(id))
                            }
                        }
                    });
                }

                let mut used_in = recipes
                    .iter()
                    .filter_map(|(_, r)| {
                        r.materials
                            .iter()
                            .filter_map(|&ItemWithQuantity { item, quantity }| {
                                (item == id).then_some((r, quantity))
                            })
                            .at_most_one()
                            .unwrap()
                    })
                    .peekable();
                if used_in.peek().is_some() {
                    ui.separator();
                    ui.heading("Used in:");
                    ui.indent("Used in", |ui| {
                        for (recipe, quantity) in used_in {
                            if ui
                                .link(format!("{name} [×{quantity}]", name = recipe.name))
                                .clicked()
                            {
                                win_events.send(WinEvent::Open(id))
                            }
                        }
                    });
                }
            });
    }
}

pub fn recipe_windows(
    mut q: Query<(&Recipe, &mut DescWin)>,
    items: Query<&Item>,
    mut contexts: EguiContexts,
    mut win_events: EventWriter<WinEvent>,
) {
    for (recipe, mut win) in &mut q {
        egui::Window::new(format!("Recipe: {name}", name = recipe.name))
            .open(&mut win.open)
            .show(contexts.ctx_mut(), |ui| {
                ui.heading("Materials:");
                ui.indent("Materials", |ui| {
                    for &ItemWithQuantity { item: id, quantity } in &recipe.materials {
                        let item = items.get(id).unwrap();
                        if ui
                            .link(format!("{quantity}× {name}", name = item.name))
                            .clicked()
                        {
                            win_events.send(WinEvent::Open(id))
                        }
                    }
                });
            });
    }
}

pub fn process_events(mut q: Query<&mut DescWin>, mut events: EventReader<WinEvent>) {
    for e in &mut events {
        match e {
            WinEvent::Open(e) => {
                q.get_mut(*e).unwrap().open = true;
            }
        }
    }
}
