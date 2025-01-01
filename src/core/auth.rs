pub struct Auth {
    user: String,
    pass: String,
}

impl Auth {
    pub fn new(user: &str, pass: &str) -> Self {
        Self {
            user: user.to_string(),
            pass: pass.to_string(),
        }
    }

    pub fn auth(&self) {
        return
    }
}
