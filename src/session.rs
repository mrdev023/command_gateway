pub struct Session {
    pub id: String,
    pub pid: usize,
}

impl From<usize> for Session {
    fn from(pid: usize) -> Self {
        Self {
            id: format!("ID{pid}"),
            pid
        }
    }
}