use crate::admin::{AdminCommand, Result};
pub fn set_auth() -> Result {
    Ok(AdminCommand::Entry)
}

pub fn select_online() -> Result {
    Ok(AdminCommand::Entry)
}

pub fn input_channel_id() -> Result {
    Ok(AdminCommand::Entry)
}
