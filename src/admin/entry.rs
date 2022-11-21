use crate::admin::select_esc;
use rust_i18n::{i18n, t};
i18n!("locales");

use crate::admin::structs_types::{AdminCommand, CommandResult};
use promkit::{build::Builder, crossterm::style, register::Register, select, selectbox::SelectBox};

pub fn entry() -> CommandResult {
    let mut selectbox = Box::new(SelectBox::default());
    selectbox.register_all(vec![
        t!("select_options.set_auth"),
        t!("select_options.set_public_room"),
        t!("select_options.query"),
        t!("select_options.quit"),
    ]);
    let mut init_promt = select::Builder::default()
        .title(t!("select_options.title"))
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox)
        .handler(select_esc())
        .window(5)
        .build()?;
    let res = init_promt.run()?;

    if res == t!("select_options.set_auth") {
        Ok(AdminCommand::SetAuth)
    } else if res == t!("select_options.set_public_room") {
        Ok(AdminCommand::SetPublicRoom)
    } else if res == t!("select_options.query") {
        Ok(AdminCommand::Query)
    } else if res == t!("select_options.quit") {
        Ok(AdminCommand::Exit)
    } else {
        unreachable!();
    }
}
