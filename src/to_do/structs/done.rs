use super::base::Base;
use super::traits::{delete::Delete, edit::Edit, get::Get};

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(title: &str) -> Self {
        let base = Base::new(title, "done");

        Self { super_struct: base }
    }
}

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}
