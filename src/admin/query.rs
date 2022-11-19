use crate::admin::{AdminCommand, Result};
pub fn query() -> Result {
    Ok(AdminCommand::Entry)
}
