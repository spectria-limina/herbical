use bevy::prelude::*;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
pub enum Skill {
    Attack,
    Constitution,
    Defence,
    Magic,
    Prayer,
    Ranged,
    Strength,
    Summoning,

    Archaeology,
    Divination,
    Farming,
    Fishing,
    Hunter,
    Mining,
    Smithing,
    Woodcutting,

    Construction,
    Cooking,
    Crafting,
    Firemaking,
    Fletching,
    Herblore,
    Invention,
    Runecrafting,

    Agility,
    Dungeoneering,
    Slayer,
    Thieving,
}

#[derive(Debug)]
pub struct ItemWithQuantity {
    pub item: Entity, // Expected entity type: Item
    pub quantity: i64,
}

#[derive(Component, Debug)]
pub struct Item {
    pub id: i32,
    pub name: String,
    pub desc: String,
}

#[derive(Component, Debug)]
pub struct Recipe {
    pub name: String,
    pub skill: Skill,
    pub level: i32,
    pub xp: i32,                   // Currently measured in tenths of an experience point.
    pub product: ItemWithQuantity, // Expected entity type: Item.
    pub materials: Vec<ItemWithQuantity>, // Expected entity type: Item.
    pub ticks: i32,
}
