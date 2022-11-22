use crate::admin::{auth, entry, query, set_public_room};

pub type CommandResult = Result<AdminCommand, std::io::Error>;

pub enum AdminCommand {
    Entry,
    SetPublicRoom,
    SetPublicRoomSelectOnline,
    SetPublicRoomInputChannelId,
    SetAuth,
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
            AdminCommand::Query => query::query(),
            AdminCommand::Exit => Ok(AdminCommand::Exit),
        }
    }
}
