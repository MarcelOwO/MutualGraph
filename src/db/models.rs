#[derive(Default)]
pub(crate) struct Friend {
    pub(crate) id: u32,
    pub(crate) uuid: String,
    pub(crate) friends: Vec<String>,
}
