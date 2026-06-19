pub struct Task {
    pub name: String,
    pub body: String,
}
impl Task {
    pub fn new() -> Self {
        Self {
            name: String::new(),
            body: String::new(),
        }
    }
}
