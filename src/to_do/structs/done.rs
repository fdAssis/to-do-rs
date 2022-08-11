use super::base::Base;
use super::traits::{delete::Delete, edit::Edit, get::Get};

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(title: &str, last_update: &str) -> Self {
        let base = Base::new(title, "done", last_update);

        Self { super_struct: base }
    }
}

impl Get for Done {}
impl Edit for Done {}
impl Delete for Done {}
