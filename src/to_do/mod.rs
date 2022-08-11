pub mod structs;
use structs::{done::Done, pending::Pending};

pub enum ItemType {
    Peding(Pending),
    Done(Done),
}

pub fn to_do_factory(item_type: &str, title: &str) -> Result<ItemType, &'static str> {
    match item_type {
        "pending" => Ok(ItemType::Peding(Pending::new(title))),
        "done" => Ok(ItemType::Done(Done::new(title))),
        _ => Err("This is not accepted"),
    }
}
