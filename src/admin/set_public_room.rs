use crate::admin::{AdminCommand, Result};
use promkit::build::Builder;
use promkit::crossterm::style;
use promkit::register::Register;
use promkit::select;
use promkit::selectbox::SelectBox;

pub fn set_public_room() -> Result {
    let mut selectbox = Box::new(SelectBox::default());
    selectbox.register_all(
        vec!["选择当前在线的", "输入channel_id", "退出"]
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
    );
    let mut prompt = select::Builder::default()
        .title("请选择")
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox)
        .build()?;
    let res = prompt.run()?;
    match res.as_str() {
        "选择当前在线的" => Ok(crate::admin::AdminCommand::SetPublicRoomSelectOnline),
        "输入channel_id" => Ok(crate::admin::AdminCommand::SetPublicRoomInputChannelId),
        "退出" => Ok(crate::admin::AdminCommand::Entry),
        _ => {
            unreachable!()
        }
    }
}

pub fn select_online() -> Result {
    Ok(AdminCommand::Entry)
}

pub fn input_channel_id() -> Result {
    Ok(AdminCommand::Entry)
}
