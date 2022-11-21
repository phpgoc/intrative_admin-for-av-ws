use crate::admin::{auth, entry, query, set_public_room};
use std::fmt;
#[derive(Debug)]
pub enum AdminError {
    DbError(String),
    UnknownChannel(String),
    UnknownUser(String),
    UserNotInChannel(String, String),
}

impl fmt::Display for AdminError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AdminError::DbError(e) => write!(f, "Database error: {}", e),
            AdminError::UnknownChannel(c) => write!(f, "Unknown channel: {}", c),
            AdminError::UnknownUser(u) => write!(f, "Unknown user: {}", u),
            AdminError::UserNotInChannel(u, c) => write!(f, "User {} is not in channel {}", u, c),
        }
        // write!(f, "Error: {:?}", self)
    }
}

pub type AdminResult<T> = Result<T, AdminError>;
pub type CommandResult = Result<AdminCommand, std::io::Error>;

trait SelfDefinedUnwrap<T> {
    fn self_defined_unwrap(self) -> T;
}

impl<T> SelfDefinedUnwrap<T> for AdminResult<T> {
    fn self_defined_unwrap(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => panic!("Error: {}", e),
        }
    }
}

pub enum AdminCommand {
    Entry,
    SetPublicRoom,
    SetPublicRoomSelectOnline,
    SetPublicRoomInputChannelId,
    SetAuth,
    SetAuthSelectOnline,
    SetAuthInputChannelId,
    Query,
    Exit,
}

impl AdminCommand {
    pub fn run(self) -> CommandResult {
        match self {
            AdminCommand::Entry => entry::entry(),
            AdminCommand::SetPublicRoom => set_public_room::set_public_room(),
            AdminCommand::SetPublicRoomSelectOnline => set_public_room::select_online(),
            AdminCommand::SetPublicRoomInputChannelId => set_public_room::input_channel_id(),
            AdminCommand::SetAuth => auth::set_auth(),
            AdminCommand::SetAuthSelectOnline => auth::select_online(),
            AdminCommand::SetAuthInputChannelId => auth::input_channel_id(),
            AdminCommand::Query => query::query(),
            AdminCommand::Exit => Ok(AdminCommand::Exit),
        }
    }
}
