use crate::admin::structs_types::{AdminCommand, CommandResult};
use crate::admin::{readline_esc, select_esc};
use promkit::termutil::clear;
use promkit::{
    build::Builder, crossterm::style, readline, register::Register, select, selectbox::SelectBox,
};
use std::io::stdout;
use crate::admin::tcp::{send_tcp_request, tcp_server, TcpRequest, TcpResponse};

pub fn set_public_room() -> CommandResult {
    let mut selectbox = Box::new(SelectBox::default());
    let vec = vec![
        t!("select_options.set_public_room_options.select_online"),
        t!("select_options.set_public_room_options.input_channel"),
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
        Ok(AdminCommand::SetPublicRoomSelectOnline)
    } else if res == t!("select_options.set_public_room_options.input_channel") {
        Ok(AdminCommand::SetPublicRoomInputChannelId)
    } else if res == t!("select_options.quit") {
        Ok(AdminCommand::Entry)
    } else {
        unreachable!();
    }
}

pub fn select_online() -> CommandResult {
    let TcpResponse::List(mut vec) = send_tcp_request(TcpRequest::ListChannels).unwrap() else{
        unreachable!();
    };

    vec.push(t!("select_options.quit"));
    let mut selectbox_select_channel = Box::new(SelectBox::default());
    selectbox_select_channel
        .register_all(vec);
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
            send_tcp_request(TcpRequest::SetRoomPublic(res, is_public)).unwrap();
            return Ok(AdminCommand::SetPublicRoomSelectOnline);
        }
    }
    Ok(AdminCommand::SetPublicRoom)
}

// #[cfg(not(feature = "no_db"))]
pub fn input_channel_id() -> CommandResult {
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

        if res == t!("select_options.quit") {
            break;
        }
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
        match send_tcp_request(TcpRequest::SetRoomPublic(res, is_public)).unwrap() {
            TcpResponse::Ok => {
                return Ok(AdminCommand::SetPublicRoomInputChannelId);
            }
            TcpResponse::UnknownSelected => {
                println!("{}", t!("errors.unknown_selected"));
                std::io::stdin().read_line(&mut String::new()).unwrap();
                clear(&mut stdout()).unwrap();
                return Ok(AdminCommand::SetPublicRoomInputChannelId);
            }
            _ => {
                unreachable!();
            }
        }
        //todo update db
        let (_, _) = (res, is_public);
        break;
    }
    Ok(AdminCommand::SetPublicRoom)
}
