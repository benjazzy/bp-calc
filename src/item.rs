use clap::{Arg, ArgAction};

pub const ITEMS: &[Item] = &[
    FIBER,
    HIDE,
    METALINGOT,
    WOOD,
    POLYMER,
    CRYSTAL,
    CEMENTING_PASTE,
    STONE,
    THATCH,
    SLICA_PEARL,
];

pub const CEMENTING_PASTE: Item = Item {
    item_type: ItemType::CementingPaste,
    weight: 0.01,
    stack_size: 100,
    aliases: &["cementing-paste", "cementing paste", "cp"],
};

pub const CRYSTAL: Item = Item {
    item_type: ItemType::Crystal,
    weight: 1.0,
    stack_size: 100,
    aliases: &["crystal"],
};

pub const FIBER: Item = Item {
    item_type: ItemType::Fiber,
    weight: 0.01,
    stack_size: 300,
    aliases: &["fiber"],
};

pub const HIDE: Item = Item {
    item_type: ItemType::Hide,
    weight: 0.01,
    stack_size: 200,
    aliases: &["hide"],
};

pub const METALINGOT: Item = Item {
    item_type: ItemType::MetalIngot,
    weight: 1.0,
    stack_size: 300,
    aliases: &["metal-ingot", "metal ingot", "metalingot", "mi"],
};

pub const POLYMER: Item = Item {
    item_type: ItemType::Polymer,
    weight: 0.25,
    stack_size: 100,
    aliases: &["polymer", "poly"],
};

pub const SLICA_PEARL: Item = Item {
    item_type: ItemType::SilicaPearl,
    weight: 0.02,
    stack_size: 100,
    aliases: &[
        "silica-pearl",
        "silica pearl",
        "silica",
        "pearl",
        "silicapearl",
        "sp",
    ],
};

pub const STONE: Item = Item {
    item_type: ItemType::Stone,
    weight: 0.5,
    stack_size: 100,
    aliases: &["stone"],
};
pub const THATCH: Item = Item {
    item_type: ItemType::Thatch,
    weight: 0.02,
    stack_size: 200,
    aliases: &["thatch"],
};

pub const WOOD: Item = Item {
    item_type: ItemType::Wood,
    weight: 0.5,
    stack_size: 100,
    aliases: &["wood"],
};

#[derive(Debug, Clone, Copy, Eq, PartialEq, PartialOrd, Ord, Hash)]
pub enum ItemType {
    CementingPaste,
    Crystal,
    Fiber,
    Hide,
    MetalIngot,
    Polymer,
    SilicaPearl,
    Stone,
    Thatch,
    Wood,
}

impl ItemType {
    const CEMENTING_PASTE: &str = "Cementing Paste";
    const CRYSTAL: &str = "Crystal";
    const FIBER: &str = "Fiber";
    const HIDE: &str = "Hide";
    const METAL_INGOT: &str = "Metal Ingot";
    const POLYMER: &str = "Polymer";
    const SILICA_PEARL: &str = "Silica Pearl";
    const STONE: &str = "Stone";
    const THATCH: &str = "Thatch";
    const WOOD: &str = "Wood";
}

impl From<ItemType> for &'static str {
    fn from(val: ItemType) -> Self {
        match val {
            ItemType::CementingPaste => ItemType::CEMENTING_PASTE,
            ItemType::Crystal => ItemType::CRYSTAL,
            ItemType::Fiber => ItemType::FIBER,
            ItemType::Hide => ItemType::HIDE,
            ItemType::MetalIngot => ItemType::METAL_INGOT,
            ItemType::Polymer => ItemType::POLYMER,
            ItemType::SilicaPearl => ItemType::SILICA_PEARL,
            ItemType::Stone => ItemType::STONE,
            ItemType::Thatch => ItemType::THATCH,
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
pub struct Item<'a> {
    pub item_type: ItemType,
    pub weight: f64,
    pub stack_size: u16,
    pub aliases: &'a [&'a str],
}

impl Item<'_> {
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

impl From<&Item<'static>> for Arg {
    fn from(val: &Item<'static>) -> Self {
        let id: &str = val.item_type.into();
        Arg::new(id)
            .long(val.aliases[0])
            .aliases(&val.aliases[1..])
            .action(ArgAction::Set)
            .value_parser(clap::value_parser!(usize))
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
        Some(self.cmp(other))
    }
}

impl Ord for BlueprintResource {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.item_type.cmp(&other.item_type)
    }
}
