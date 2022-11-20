use crate::admin::{Result, select_esc};
use promkit::{build::Builder, crossterm::style, register::Register, select, selectbox::SelectBox};
pub fn entry() -> Result {
    let mut selectbox = Box::new(SelectBox::default());
    selectbox.register_all(
        vec!["设置发言权限", "设置房间公开", "查询", "退出"]
            .iter()
            .map(|v| v.to_string())
            .collect::<Vec<String>>(),
    );
    let mut init_promt = select::Builder::default()
        .title("请选择: ")
        .title_color(style::Color::DarkGreen)
        .selectbox(selectbox)
        .handler(select_esc())
        .window(5)
        .build()?;
    let res = init_promt.run()?;
    match res.as_str() {
        "设置发言权限" => Ok(crate::admin::AdminCommand::SetAuth),
        "设置房间公开" => Ok(crate::admin::AdminCommand::SetPublicRoom),
        "查询" => Ok(crate::admin::AdminCommand::Query),
        "退出" => Ok(crate::admin::AdminCommand::Exit),
        _ => {
            unreachable!()
        }
    }
}
