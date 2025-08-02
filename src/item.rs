use std::{collections::HashMap, sync::LazyLock};

use color_eyre::eyre::eyre;

pub static ITEMS: LazyLock<HashMap<ItemType, Item>> = LazyLock::new(|| {
    ITEMS_LIST.iter().fold(HashMap::new(), |mut acc, item| {
        let old = acc.insert(item.item_type, *item);
        assert!(old.is_none(), "Duplicate item found in item list!");

        acc
    })
});

const ITEMS_LIST: &[Item] = &[FIBER, HIDE, METALINGOT, WOOD];

pub const FIBER: Item = Item {
    item_type: ItemType::Fiber,
    weight: 0.01,
    stack_size: 300,
};

pub const HIDE: Item = Item {
    item_type: ItemType::Hide,
    weight: 0.01,
    stack_size: 200,
};

pub const METALINGOT: Item = Item {
    item_type: ItemType::MetalIngot,
    weight: 1.0,
    stack_size: 300,
};

pub const WOOD: Item = Item {
    item_type: ItemType::Wood,
    weight: 0.5,
    stack_size: 100,
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum ItemType {
    Fiber,
    Hide,
    MetalIngot,
    Wood,
}

impl ItemType {
    const FIBER: &str = "Fiber";
    const HIDE: &str = "Hide";
    const METALINGOT: &str = "Metal Ingot";
    const WOOD: &str = "Wood";
}

impl TryFrom<&str> for ItemType {
    type Error = color_eyre::Report;

    fn try_from(mut value: &str) -> Result<Self, Self::Error> {
        match value.to_lowercase().as_str() {
            "fiber" => Ok(ItemType::Fiber),
            "hide" => Ok(ItemType::Hide),
            "metal ingot" | "mi" => Ok(ItemType::MetalIngot),
            "wood" => Ok(ItemType::Wood),
            _ => Err(eyre!(format!("Unkown item type {value}"))),
        }
    }
}

impl From<ItemType> for &'static str {
    fn from(val: ItemType) -> Self {
        match val {
            ItemType::Fiber => ItemType::FIBER,
            ItemType::Hide => ItemType::HIDE,
            ItemType::MetalIngot => ItemType::METALINGOT,
            ItemType::Wood => ItemType::WOOD,
        }
    }
}

impl std::fmt::Display for ItemType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let name: &str = (*self).into();
        write!(f, "{name}")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Item {
    pub item_type: ItemType,
    pub weight: f64,
    pub stack_size: u16,
}

impl Item {
    pub fn calculate(self, count: usize) -> ItemResult {
        let weight = self.weight * count as f64;
        let slots = count.div_ceil(self.stack_size as usize);

        ItemResult {
            item_type: self.item_type,
            slots,
            weight,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ItemResult {
    pub item_type: ItemType,
    pub slots: usize,
    pub weight: f64,
}

impl std::fmt::Display for ItemResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let ItemResult {
            item_type,
            slots,
            weight,
        } = self;

        write!(f, "{item_type}: \n\tslots: {slots} \n\tweight: {weight:.2}")
    }
}

#[derive(Debug, Clone, Copy)]
pub struct BlueprintResource {
    pub item_type: ItemType,
    pub count: usize,
}

impl std::fmt::Display for BlueprintResource {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let BlueprintResource { item_type, count } = self;
        write!(f, "{item_type}: {count}")
    }
}

impl PartialEq for BlueprintResource {
    fn eq(&self, other: &Self) -> bool {
        self.item_type == other.item_type
    }
}

impl Eq for BlueprintResource {}

impl PartialOrd for BlueprintResource {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.item_type.partial_cmp(&other.item_type)
    }
}

impl Ord for BlueprintResource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.item_type.cmp(&other.item_type)
    }
}
