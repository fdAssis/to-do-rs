mod to_do;
use to_do::structs::traits::create::Create;
use to_do::structs::{done::Done, pending::Pending};
use to_do::to_do_factory;
use to_do::ItemType;

fn main() {
    let to_do_item = to_do_factory("pending", "washing");

    match to_do_item.unwrap() {
        ItemType::Peding(item) => item.create(&item.super_struct.title),
        ItemType::Done(item) => println!(
            "it's a done item with the title: {}",
            item.super_struct.title
        ),
    }
}
