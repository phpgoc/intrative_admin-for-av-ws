use crate::admin::{select_esc, AdminCommand, Result};
use rust_i18n::{i18n, t};
i18n!("locales");

use promkit::{build::Builder, crossterm::style, register::Register, select, selectbox::SelectBox};
pub fn entry() -> Result {
    let mut selectbox = Box::new(SelectBox::default());
    selectbox.register_all(vec![
        t!("select_options.set_auth"),
        t!("select_options.set_public_room"),
        t!("select_options.query"),
        t!("select_options.quit"),
    ]);
    let mut init_promt = select::Builder::default()
        .title("请选择: ")
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox)
        .handler(select_esc())
        .window(5)
        .build()?;
    let res = init_promt.run()?;

    if res == t!("select_options.set_auth") {
        return Ok(AdminCommand::SetAuth);
    } else if res == t!("select_options.set_public_room") {
        return Ok(AdminCommand::SetPublicRoom);
    } else if res == t!("select_options.query") {
        return Ok(AdminCommand::Query);
    } else if res == t!("select_options.quit") {
        return Ok(AdminCommand::Exit);
    } else {
        unreachable!();
    }
}
