use std::sync::Arc;

use mail_send::{mail_builder::MessageBuilder, SmtpClient, SmtpClientBuilder};
use rand::{distributions::Alphanumeric, Rng};
use tokio::{net::TcpStream, sync::Mutex};
use tokio_rustls::client::TlsStream;

use crate::{
    envs::vcode_expire_time,
    util::{handlebars_util::render_vcode_email, redis_cache::RedisCache},
    MaaError, MaaResult,
};

pub struct MailService {
    mail_client: Mutex<SmtpClient<TlsStream<TcpStream>>>,
    redis_cache: Arc<RedisCache>,
    no_send: bool,
    vcode_expire: u64,
}

impl MailService {
    pub async fn new(
        redis_cache: Arc<RedisCache>,
        no_send: bool,
    ) -> MaaResult<Self> {
        let mail_client = SmtpClientBuilder::new("sss", 111).connect().await?;
        let vcode_expire = vcode_expire_time().unwrap_or(300);
        Ok(Self {
            mail_client: Mutex::new(mail_client),
            redis_cache,
            no_send,
            vcode_expire,
        })
    }

    pub async fn send_vcode(&self, email: &str) -> MaaResult<()> {
        // 一个过期周期最多重发十条，记录已发送的邮箱以及间隔时间
        let timeout = self.vcode_expire / 10;

        let exist = !self
            .redis_cache
            .set_if_not_exists_ex(
                &format!("HasBeenSentVCode:{}", email),
                timeout,
                timeout,
            )
            .await?;

        if exist {
            return Err(MaaError::VCodeSentTooFrequently);
        }

        // generate random string of 6 digits
        let vcode = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(6)
            .map(char::from)
            .collect::<String>();

        if self.no_send {
            tracing::warn!(
                "Email not sent, no_send enabled, vcode is {}",
                vcode
            );
        } else {
            let mail_content = render_vcode_email(&vcode)?;
            let mail = MessageBuilder::new()
                .to(email)
                .subject("Maa Backend Center 验证码")
                .html_body(&mail_content);

            let mut mail_client = self.mail_client.lock().await;
            mail_client.send(mail).await?;
        }

        self.redis_cache
            .set_ex(
                &format!("vCodeEmail:{}", email),
                vcode.to_ascii_uppercase(),
                self.vcode_expire,
            )
            .await?;

        Ok(())
    }

    pub async fn verify_vcode(
        &self,
        email: &str,
        vcode: &str,
    ) -> MaaResult<()> {
        let result = self
            .redis_cache
            .delete_if_equals(
                &format!("vCodeEmail:{}", email),
                vcode.to_ascii_uppercase().to_string(),
            )
            .await?;

        if !result {
            return Err(MaaError::VCodeNotMatch);
        }

        Ok(())
    }
}
