use super::base::Base;

pub struct Done {
    pub super_struct: Base,
}

impl Done {
    pub fn new(title: &str) -> Self {
        let base = Base::new(title, "done");

        Self { super_struct: base }
    }
}
