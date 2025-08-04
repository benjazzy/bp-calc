use std::collections::BTreeSet;
use clap::Parser;
use color_eyre::eyre::eyre;
use crafter::CRAFTERS;
use item::{BlueprintResource, ITEMS, ItemResult, ItemType};
use text_io::read;
use crate::cli::Cli;
use crate::item::{Item, ITEMS_LIST};

mod cli;
mod crafter;
mod item;

fn main() {
    let cli = Cli::parse();
    if cli.list_items {
        for Item { item_type, aliases, ..} in ITEMS_LIST {
            print!("{item_type}: ");
            for alias in aliases.into_iter() {
                print!("'{alias}' ");
            }
            println!();
        }

        return;
    }

    for item_result in calculate(cli.items.iter()) {
        println!("{item_result}");
    }
    println!("======================");
    for crafter in CRAFTERS {
        crafter.print_blueprint_info(calculate(cli.items.iter()));
    }
}

fn calculate<'b, I>(blueprint_cost: I) -> impl IntoIterator<Item = ItemResult>
where
    I: IntoIterator<Item = &'b BlueprintResource>,
{
    blueprint_cost.into_iter().map(|resource| {
        let item = ITEMS
            .get(&resource.item_type)
            .expect("Item type should be in ITEMS");

        item.calculate(resource.count)
    })
}
