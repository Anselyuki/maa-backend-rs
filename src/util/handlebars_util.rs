use serde_json::json;

use crate::MaaResult;

pub fn render_vcode_email(vcode: &str) -> MaaResult<String> {
    let mut reg = handlebars::Handlebars::new();

    reg.register_template_file("root", "templates/mail-includeHtml.hbs")?;

    reg.register_template_file("logo", "templates/logo.hbs")?;

    reg.register_template_file("vcode", "templates/mail-vcode.hbs")?;

    let data = json!({
        "content": "vcode",
        "vcode": vcode,
    });

    let rendered = reg.render("root", &data)?;

    Ok(rendered)
}

#[test]
fn t_render_vcode_email() {
    let vcode = "123456";
    let result = render_vcode_email(vcode).unwrap();
    println!("{}", result);
}
