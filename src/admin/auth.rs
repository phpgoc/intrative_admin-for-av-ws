use crate::admin::structs_types::{AdminCommand, CommandResult};

pub fn set_auth() -> CommandResult {
    Ok(AdminCommand::Entry)
}

pub fn select_online() -> CommandResult {
    Ok(AdminCommand::Entry)
}

pub fn input_channel_id() -> CommandResult {
    Ok(AdminCommand::Entry)
}
