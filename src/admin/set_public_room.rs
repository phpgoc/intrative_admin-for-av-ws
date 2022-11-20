use crate::admin::{readline_esc, select_esc, AdminCommand, Result};
use promkit::termutil::clear;
use promkit::{
    build::Builder, crossterm::style, readline, register::Register, select, selectbox::SelectBox,
};
use std::io::stdout;

pub fn set_public_room() -> Result {
    let mut selectbox = Box::new(SelectBox::default());
    #[cfg(not(feature = "no_db"))]
    let vec = vec![
        t!("select_options.set_public_room_options.select_online"),
        t!("select_options.set_public_room_options.input_channel"),
        t!("select_options.quit"),
    ];
    #[cfg(feature = "no_db")]
    let vec = vec![
        t!("select_options.set_public_room_options.select_online"),
        t!("select_options.quit"),
    ];
    selectbox.register_all(vec);
    let mut p = select::Builder::default()
        .title(t!("titles.common_select"))
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox)
        .handler(select_esc())
        .build()?;
    let res = p.run()?;
    if res == t!("select_options.set_public_room_options.select_online") {
        return Ok(AdminCommand::SetPublicRoomSelectOnline);
    } else if res == t!("select_options.set_public_room_options.input_channel") {
        return Ok(AdminCommand::SetPublicRoomInputChannelId);
    } else if res == t!("select_options.quit") {
        return Ok(AdminCommand::Entry);
    } else {
        unreachable!();
    }
}

pub fn select_online() -> Result {
    //todo select from db
    let mut vec = vec![];
    vec.push(t!("select_options.quit"));
    let mut selectbox_select_channel = Box::new(SelectBox::default());
    selectbox_select_channel
        .register_all(vec.iter().map(|v| v.to_string()).collect::<Vec<String>>());
    let mut p = select::Builder::default()
        .title(t!("titles.select_channel"))
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox_select_channel)
        .handler(select_esc())
        .build()?;
    let mut selectbox_is_public = Box::new(SelectBox::default());
    selectbox_is_public.register_all(vec![
        t!("select_options.public"),
        t!("select_options.private"),
        t!("select_options.quit"),
    ]);
    let mut set_public_or_private = select::Builder::default()
        .title(t!("titles.common_select"))
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox_is_public)
        .handler(select_esc())
        .build()?;
    loop {
        let res = p.run()?;
        if res == t!("select_options.quit") {
            break;
        } else {
            let is_public_string = set_public_or_private.run()?;
            let is_public = if is_public_string == t!("select_options.public") {
                true
            } else if is_public_string == t!("select_options.private") {
                false
            } else if is_public_string == t!("select_options.quit") {
                return Ok(AdminCommand::SetPublicRoomSelectOnline);
            } else {
                unreachable!();
            };
            let _ = is_public;
            //todo update db
        }
    }
    Ok(AdminCommand::SetPublicRoom)
}

// #[cfg(not(feature = "no_db"))]
pub fn input_channel_id() -> Result {
    let mut p = readline::Builder::default()
        .title(t!("titles.select_channel"))
        .title_color(style::Color::DarkBlue)
        .handler(readline_esc())
        .build()?;
    let mut selectbox = Box::new(SelectBox::default());
    selectbox.register_all(vec![
        t!("select_options.public"),
        t!("select_options.private"),
        t!("select_options.quit"),
    ]);
    let mut set_public_or_private = select::Builder::default()
        .title(t!("titles.common_select"))
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox)
        .handler(select_esc())
        .build()?;
    loop {
        let res = p.run()?;

        //todo select channel_id from db

        clear(&mut stdout())?;

        let is_public_string = set_public_or_private.run()?;
        let is_public = if is_public_string == t!("select_options.public") {
            true
        } else if is_public_string == t!("select_options.private") {
            false
        } else if is_public_string == t!("select_options.quit") {
            return Ok(AdminCommand::SetPublicRoomSelectOnline);
        } else {
            unreachable!();
        };

        //todo update db
        let (_, _) = (res, is_public);
        break;
    }
    Ok(AdminCommand::SetPublicRoom)
}
