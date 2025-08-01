use crafter::{CRAFTERS, argy_modifier};
use item::{BlueprintResource, ITEMS, Item, ItemResult, ItemType};
use text_io::read;

mod crafter;
mod item;

fn main() {
    prompt();
}

fn prompt() {
    let mut items = Vec::new();
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
                    items.push(item);
                }
            }
            "c" => {
                for item_result in calculate(&items) {
                    println!("{item_result}");
                }
                println!("=====");
                for crafter in CRAFTERS {
                    crafter.print_blueprint_info(calculate(&items));
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

fn calculate(blueprint_cost: &[BlueprintResource]) -> impl IntoIterator<Item = ItemResult> {
    blueprint_cost.iter().map(|resource| {
        let item = ITEMS
            .get(Into::<&'static str>::into(resource.item_type))
            .expect("Item type should be in ITEMS");

        item.calculate(resource.count)
    })
}
