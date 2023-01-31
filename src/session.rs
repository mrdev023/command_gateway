pub struct Session {
    pub id: String,
    pub pid: u32,
}

impl From<u32> for Session {
    fn from(pid: u32) -> Self {
        Self {
            id: format!("ID{pid}"),
            pid
        }
    }
}