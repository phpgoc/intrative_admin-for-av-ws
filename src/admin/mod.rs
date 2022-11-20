use promkit::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    grapheme::Graphemes,
    keybind::KeyBind,
    readline::State as ReadlineState,
    select::State as SelectState,
    EventHandleFn,
};
use std::cell::Cell;

mod auth;
mod entry;
mod query;
mod set_public_room;
pub(crate) mod tcp;

type Result = std::result::Result<AdminCommand, std::io::Error>;

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
    pub fn run(self) -> Result {
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

pub async fn admin() -> promkit::Result<()> {
    let mut job = AdminCommand::Entry;
    loop {
        job = job.run()?;
        if let AdminCommand::Exit = job {
            break;
        }
    }
    Ok(())
}

pub fn readline_esc() -> KeyBind<ReadlineState> {
    let mut b = KeyBind::default();
    b.assign(vec![(
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
        }),
        Box::new(|_, _, _: &mut std::io::Stdout, state: &mut ReadlineState| {
            state.0.editor.replace(&Graphemes::from("q"));
            Ok(true)
        }) as Box<EventHandleFn<ReadlineState>>,
    )]);
    b
}
pub fn select_esc() -> KeyBind<SelectState> {
    let mut b = KeyBind::default();
    b.assign(vec![(
        Event::Key(KeyEvent {
            code: KeyCode::Char('q'),
            modifiers: KeyModifiers::CONTROL,
        }),
        Box::new(|_, _, _: &mut std::io::Stdout, state: &mut SelectState| {
            state.0.editor.position = Cell::from(state.0.editor.data.len() - 1);
            Ok(true)
        }) as Box<EventHandleFn<SelectState>>,
    )]);
    b
}
