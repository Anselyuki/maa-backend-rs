use serde::Deserialize;
use validator::Validate;

#[derive(Deserialize, Validate, Debug)]
pub struct LoginRequest {
    #[validate(email(message = "邮箱格式错误"))]
    pub email: String,
    #[validate(length(min = 1, message = "请输入用户密码"))]
    pub password: String,
}

#[derive(Deserialize, Validate, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RegisterRequest {
    #[validate(email(message = "邮箱格式错误"))]
    pub email: String,
    #[validate(length(
        min = 4,
        max = 24,
        message = "用户名长度必须在4-20之间"
    ))]
    pub user_name: String,
    #[validate(length(min = 8, max = 32, message = "密码长度必须在8-32之间"))]
    pub password: String,
    #[validate(length(min = 1, message = "请输入验证码"))]
    pub registration_token: String,
}
