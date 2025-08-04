use clap::Parser;
use color_eyre::eyre::eyre;
use crate::item::{BlueprintResource, ItemType, ITEMS, ITEMS_LIST};

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
pub struct Cli {
    #[arg(short, long)]
    pub list_items: bool,

    #[arg(short, long, value_parser = parse_item)]
    pub item: Vec<BlueprintResource>,
}

pub fn parse_item(s: &str) -> color_eyre::Result<BlueprintResource> {
    let Some((t,c)) = s.split_once(':') else {
        return Err(eyre!("Invalid item format"));
    };

    let Some(item_type) = ITEMS_LIST.iter().find_map(|i| {
        if i.aliases.contains(&t.trim()) {
            Some(i.item_type)
        } else {
            None
        }
    }) else {
        return Err(eyre!("Invalid item type {t}"));
    };
    let count = c.trim().parse::<usize>()?;

    Ok(BlueprintResource { item_type, count })
}