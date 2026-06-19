pub struct Task {
    pub name: String,
    pub body: String,
}
impl Task {
    pub fn new(p_name: String, p_body: String) -> Self {
        Self {
            name: p_name,
            body: p_body,
        }
    }
}
