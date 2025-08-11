use clap::{ArgMatches, command};
use crafter::CRAFTERS;
use item::{BlueprintResource, ITEMS, ItemResult};

mod crafter;
mod item;

fn main() {
    let matches = matches();

    let calculate_iter = calculate(ITEMS.iter().filter_map(|item| {
        let count: usize = matches.get_one(item.item_type.into()).copied()?;

        Some(BlueprintResource {
            item_type: item.item_type,
            count,
        })
    }));

    for item_result in calculate_iter.clone() {
        println!("{item_result}");
    }
    println!("======================");
    for crafter in CRAFTERS {
        crafter.print_blueprint_info(calculate_iter.clone());
    }
}

pub fn matches() -> ArgMatches {
    command!().args(ITEMS).get_matches()
}

fn calculate<I>(blueprint_cost: I) -> impl IntoIterator<Item = ItemResult> + Clone
where
    I: IntoIterator<Item = BlueprintResource>,
    <I as IntoIterator>::IntoIter: Clone,
{
    blueprint_cost.into_iter().map(|resource| {
        let item = ITEMS
            .iter()
            .find(|item| item.item_type == resource.item_type)
            .expect("Item type should be in ITEMS");

        item.calculate(resource.count)
    })
}
