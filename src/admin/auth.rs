use crate::admin::select_esc;
use crate::admin::structs_types::{AdminCommand, CommandResult};
use crate::admin::tcp::{send_tcp_request, TcpRequest, TcpResponse};
use promkit::{build::Builder, crossterm::style, register::Register, select, selectbox::SelectBox};

pub fn set_auth() -> CommandResult {
    let TcpResponse::List(mut channels) = send_tcp_request(TcpRequest::ListChannels).unwrap() else{
        unreachable!();
    };
    channels.push(t!("select_options.quit"));
    let mut channel_selectbox = Box::new(SelectBox::default());
    channel_selectbox.register_all(channels);

    let mut channel_p = select::Builder::default()
        .title(t!("titles.select_channel"))
        .title_color(style::Color::DarkGreen)
        .selectbox(channel_selectbox)
        .handler(select_esc())
        .build()?;
    let channel = channel_p.run()?;
    if channel == t!("select_options.quit") {
        return Ok(AdminCommand::Entry);
    }
    let TcpResponse::List(mut users) = send_tcp_request(TcpRequest::ListChannelUsers(channel.clone())).unwrap() else{
        unreachable!();
    };
    users.push(t!("select_options.quit"));
    let mut user_selectbox = Box::new(SelectBox::default());
    user_selectbox.register_all(users);
    let mut user_p = select::Builder::default()
        .title(t!("titles.select_user"))
        .title_color(style::Color::DarkGreen)
        .selectbox(user_selectbox)
        .handler(select_esc())
        .build()?;

    let mut opertion_selectbox = Box::new(SelectBox::default());
    let operations = vec![
        t!("select_options.auth.add"),
        t!("select_options.auth.remove"),
        t!("select_options.auth.kick_out"),
        t!("select_options.quit"),
    ];
    opertion_selectbox.register_all(operations);
    let mut operation_p = select::Builder::default()
        .title(t!("titles.common_select"))
        .title_color(style::Color::DarkGreen)
        .selectbox(opertion_selectbox)
        .handler(select_esc())
        .build()?;

    let (user, opertion_res) = 'outer: loop {
        let user = user_p.run()?;
        if user == t!("select_options.quit") {
            return Ok(AdminCommand::SetAuth);
        }

        loop {
            let operation = operation_p.run()?;
            if operation == t!("select_options.quit") {
                continue 'outer;
            }
            break 'outer (user, operation);
        }
    };
    let _res = if opertion_res == t!("select_options.auth.add") {
        send_tcp_request(TcpRequest::SetAuth(channel.clone(), user.clone(), true)).unwrap()
    } else if opertion_res == t!("select_options.auth.remove") {
        send_tcp_request(TcpRequest::SetAuth(channel.clone(), user.clone(), false)).unwrap()
    } else if opertion_res == t!("select_options.auth.kick_out") {
        send_tcp_request(TcpRequest::KickUser(channel.clone(), user.clone())).unwrap()
    } else {
        unreachable!();
    };

    return Ok(AdminCommand::SetAuth);
}
