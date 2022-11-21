use promkit::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyModifiers},
    grapheme::Graphemes,
    keybind::KeyBind,
    readline::State as ReadlineState,
    select::State as SelectState,
    EventHandleFn,
};
use std::cell::Cell;
use structs_types::AdminCommand;

mod auth;
mod db;
mod entry;
mod query;
mod set_public_room;
mod structs_types;
pub(crate) mod tcp;

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
