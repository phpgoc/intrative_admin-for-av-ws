#[derive(Debug)]
pub struct LogInfo {
    pub action: &'static str,
    pub user: String,
    pub message: String,
}
