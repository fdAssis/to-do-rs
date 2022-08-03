use super::base::Base;
use super::traits::{create::Create, delete::Delete, edit::Edit, get::Get};

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let base = Base::new(title, "pending");

        Self { super_struct: base }
    }
}

impl Get for Pending {}
impl Delete for Pending {}
impl Edit for Pending {}
impl Create for Pending {}
