#[derive(Clone, Debug)]
pub struct Ctx {
    user_id: u64,
}

// Contructor.
impl Ctx {
    pub fn new(user_id: u64) -> Self {
        Self { user_id }
    }
}

// Property Accessors.
impl Ctx {
    pub fn user_id(&self) -> u64 {
        self.user_id
    }
}
