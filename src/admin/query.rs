use crate::admin::structs_types::{AdminCommand, CommandResult};

pub fn query() -> CommandResult {
    Ok(AdminCommand::Entry)
}
