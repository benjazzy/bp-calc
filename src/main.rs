use std::collections::BTreeSet;

use crafter::CRAFTERS;
use item::{BlueprintResource, ITEMS, ItemResult, ItemType};
use text_io::read;

mod crafter;
mod item;

fn main() {
    prompt();
}

fn prompt() {
    let mut items = BTreeSet::new();
    let mut buf = String::new();
    loop {
        buf.clear();

        println!("=====");
        for resource in items.iter() {
            println!("{resource}");
        }

        print!(">> ");
        buf = read!("{}\n");
        match buf.trim() {
            "q" => return,
            "a" => {
                if let Some(item) = item_prompt() {
                    items.replace(item);
                }
            }
            "r" => {
                print!("Type: ");
                let buf: String = read!("{}\n");
                let Ok(item_type) = ItemType::try_from(buf.trim()) else {
                    println!("Unknown item name {buf}");
                    continue;
                };

                if let Some(to_remove) = items.iter().copied().find(|i| i.item_type == item_type) {
                    items.remove(&to_remove);
                }
            }
            "c" => {
                for item_result in calculate(items.iter()) {
                    println!("{item_result}");
                }
                println!("=====");
                for crafter in CRAFTERS {
                    crafter.print_blueprint_info(calculate(items.iter()));
                }
            }
            _ => println!("Unknown command {buf}"),
        }
    }
}

fn item_prompt() -> Option<BlueprintResource> {
    print!("Type: ");
    let mut buf: String = read!("{}\n");
    buf = buf.to_lowercase();
    let Ok(item_type) = ItemType::try_from(buf.trim()) else {
        println!("Unknown item name {buf}");
        return None;
    };

    print!("Count: ");
    buf = read!("{}\n");
    let Ok(count) = buf.trim().parse::<usize>() else {
        println!("Invalid int {buf}");
        return None;
    };

    Some(BlueprintResource { item_type, count })
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
