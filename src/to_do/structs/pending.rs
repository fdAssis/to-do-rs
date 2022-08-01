use super::base::Base;

pub struct Pending {
    pub super_struct: Base,
}

impl Pending {
    pub fn new(title: &str) -> Self {
        let base = Base::new(title, "pending");

        Self { super_struct: base }
    }
}
