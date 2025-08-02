use owo_colors::{OwoColorize, Style};

use crate::item::{ItemResult, ItemType};

pub const CRAFTERS: &[Crafter<'static>] = &[
    Crafter {
        name: "Smithy",
        modifier: None,
        slots: 75,
    },
    Crafter {
        name: "Fabricator",
        modifier: None,
        slots: 70,
    },
    Crafter {
        name: "Tek Replicator",
        modifier: None,
        slots: 600,
    },
    Crafter {
        name: "Argentavis",
        modifier: Some(argy_modifier),
        slots: 300,
    },
];

pub fn argy_modifier(mut item_result: ItemResult) -> ItemResult {
    if item_result.item_type == ItemType::MetalIngot {
        item_result.weight /= 2.0;
    }

    item_result
}

pub struct Crafter<'n> {
    name: &'n str,
    modifier: Option<fn(ItemResult) -> ItemResult>,
    slots: u16,
}

impl Crafter<'_> {
    pub fn print_blueprint_info(&self, items: impl IntoIterator<Item = ItemResult>) {
        let Crafter { name, slots, .. } = self;

        println!("{name}:");
        let (blueprint_slots, blueprint_weight) = items
            .into_iter()
            .map(|i| if let Some(m) = self.modifier { m(i) } else { i })
            .fold((0, 0.0), |(s, w), r| {
                let slots = s + r.slots;
                let weight = w + r.weight;

                (slots, weight)
            });

        let possible = blueprint_slots < *slots as usize;
        let style = if possible {
            Style::new().green()
        } else {
            Style::new().red()
        };

        let slots_msg = format!("\tSlots:\t{blueprint_slots}/{slots}");

        println!("{}", slots_msg.style(style));
        println!("\tWeight:\t{blueprint_weight:.2}");
    }
}
