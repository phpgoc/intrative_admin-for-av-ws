use crate::admin::select_esc;
use crate::admin::structs_types::{AdminCommand, CommandResult};
use crate::admin::tcp::{send_tcp_request, TcpResponse};
use promkit::build::Builder;
use promkit::crossterm::style;
use promkit::register::Register;
use promkit::select;
use promkit::selectbox::SelectBox;
use std::process::exit;
use promkit::termutil::clear;

pub fn query() -> CommandResult {
    let TcpResponse::List(mut channels) = send_tcp_request(crate::admin::tcp::TcpRequest::ListChannels).unwrap() else {
        unreachable!();
    };
    channels.push(t!("select_options.quit"));
    let mut selectbox_select_channel = Box::new(SelectBox::default());
    selectbox_select_channel.register_all(channels);
    let mut p = select::Builder::default()
        .title(t!("titles.select_channel"))
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox_select_channel)
        .handler(select_esc())
        .build()?;
    let res = p.run()?;
    if res == t!("select_options.quit") {
        return Ok(AdminCommand::Entry);
    }
    let TcpResponse::Query(info) = send_tcp_request(crate::admin::tcp::TcpRequest::QueryRoom(res.clone())).unwrap() else {
        unreachable!();
    };
    println!("{}: {}", t!("noun.channel_name"), res.clone());
    println!(
        "{}: {}",
        t!("noun.is_public"),
        if info.is_public_room {
            t!("noun.yes")
        } else {
            t!("noun.no")
        }
    );
    println!("{}: {:?}", t!("noun.published_user"), info.published);
    println!("{}: {:?}", t!("noun.joined"), info.joined);
    std::io::stdin().read_line(&mut String::new()).unwrap();
    clear(&mut std::io::stdout()).unwrap();
    Ok(AdminCommand::Query)
}
