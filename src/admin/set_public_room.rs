use crate::admin::{AdminCommand, readline_esc, Result, select_esc};
use promkit::termutil::clear;
use promkit::{
    build::Builder, crossterm::style, readline, register::Register, select, selectbox::SelectBox,
};
use std::io::stdout;

pub fn set_public_room() -> Result {
    let mut selectbox = Box::new(SelectBox::default());
    #[cfg(not(feature = "no_db"))]
    let vec = vec!["选择当前在线的", "输入channel_id", "退出"];
    #[cfg(feature = "no_db")]
    let vec = vec!["选择当前在线的", "退出"];
    selectbox.register_all(vec.iter().map(|v| v.to_string()).collect::<Vec<String>>());
    let mut p = select::Builder::default()
        .title("请选择")
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox)
        .handler(select_esc())
        .build()?;
    let res = p.run()?;
    match res.as_str() {
        "选择当前在线的" => Ok(AdminCommand::SetPublicRoomSelectOnline),
        "输入channel_id" => Ok(AdminCommand::SetPublicRoomInputChannelId),
        "退出" => Ok(AdminCommand::Entry),
        _ => {
            unreachable!()
        }
    }
}

pub fn select_online() -> Result {
    //todo select from db
    let mut vec = vec![];
    vec.push("退出");
    let mut selectbox_select_channel = Box::new(SelectBox::default());
    selectbox_select_channel
        .register_all(vec.iter().map(|v| v.to_string()).collect::<Vec<String>>());
    let mut p = select::Builder::default()
        .title("请选择channel_id")
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox_select_channel)
        .handler(select_esc())
        .build()?;
    let mut selectbox_is_public = Box::new(SelectBox::default());
    selectbox_is_public.register_all(
        ["公开的", "私密的"]
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
    );
    let mut set_public_or_private = select::Builder::default()
        .title("请选择")
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox_is_public)
        .handler(select_esc())
        .build()?;
    loop {
        let res = p.run()?;
        match res.as_str() {
            "退出" => break,
            str => {
                let is_public_string = set_public_or_private.run()?;
                let is_public = match is_public_string.as_str() {
                    "公开的" => true,
                    "私密的" => false,
                    "退出" => return Ok(AdminCommand::SetPublicRoomSelectOnline),
                    _ => unreachable!(),
                };
                //todo update db
            }
        }
    }
    Ok(AdminCommand::SetPublicRoom)
}

// #[cfg(not(feature = "no_db"))]
pub fn input_channel_id() -> Result {
    let mut p = readline::Builder::default()
        .title("<输入q退出>选择channel_id：")
        .title_color(style::Color::DarkBlue)
        .handler(readline_esc())
        .build()?;
    let mut selectbox = Box::new(SelectBox::default());
    selectbox.register_all(
        ["公开的", "私密的","退出"]
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
    );
    let mut set_public_or_private = select::Builder::default()
        .title("请选择")
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox)
        .handler(select_esc())
        .build()?;
    loop {
        let res = p.run()?;
        match res.as_str() {
            "q" => {
                clear(&mut stdout())?;
                break;
            }
            str => {
                //todo select channel_id from db
                let mut is_public = true;
                loop{

                    clear(&mut stdout())?;

                    let is_public_string = set_public_or_private.run()?;
                    is_public = match is_public_string.as_str() {
                        "公开的" => true,
                        "私密的" => false,
                        "退出" => return Ok(AdminCommand::SetPublicRoomInputChannelId),
                        _ => unreachable!(),
                    }
                }
                //todo update db
                println!("输入的channel_id({})为：,{}", str, is_public);
            }
        }
    }
    Ok(AdminCommand::SetPublicRoom)
}
