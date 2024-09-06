use reqwest::header::{HeaderMap, HeaderValue};

#[derive(serde::Deserialize, Debug)]
pub struct GithubTrees {
    pub sha: String,
    pub url: String,
    pub tree: Vec<GithubTree>,
}

#[derive(serde::Deserialize, Debug)]
pub struct GithubTree {
    pub path: String,
    pub mode: String,
    #[serde(rename = "type")]
    pub tree_type: String,
    pub sha: String,
    pub url: Option<String>,
}
#[derive(serde::Deserialize, Debug)]
pub struct GithubCommits {
    pub sha: String,
}

pub struct GithubApi {
    /// GitHub API 的身份验证令牌
    token: Option<String>,
    /// GitHub API 的 URL
    api_url: String,
    /// 仓库所有者
    owner: String,
    /// 仓库名称
    repo: String,
    /// 默认请求头, 包含接受的数据类型和用户代理
    default_headers: HeaderMap,
    /// 用于发送 HTTP 请求的 `reqwest::Client` 实例
    client: reqwest::Client,
}

impl GithubApi {
    /// 这个方法返回一个默认的 `GithubApi` 实例,其中不包含身份验证令牌
    ///
    /// 如果未设置,则生产的请求将不包含身份验证令牌,会有限流的风险,建议配置使用
    pub fn default() -> GithubApi {
        GithubApi::new(
            None,
            "MaaAssistantArknights".to_string(),
            "MaaAssistantArknights".to_string(),
        )
    }
    pub fn new(token: Option<String>, owner: String, repo: String) -> GithubApi {
        let mut header_map = HeaderMap::new();
        // 默认的两个请求头, 用于指定接受的数据类型和用户代理
        header_map.append(
            "Accept",
            HeaderValue::from_str("application/vnd.github.v3+json").unwrap(),
        );
        header_map.append("User-Agent", HeaderValue::from_str("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/128.0.0.0 Safari/537.36 Edg/128.0.0.0").unwrap());
        GithubApi {
            token,
            api_url: "https://api.github.com".to_string(),
            owner,
            repo,
            default_headers: header_map,
            client: reqwest::Client::new(),
        }
    }

    /// 设置 GitHub API 请求头中的身份验证令牌
    pub fn set_token(&mut self, token: String) {
        self.token = Some(token);
    }

    /// 获取指定 GitHub 仓库的提交列表。
    ///
    /// 返回一个包含 `GithubCommits` 结构体的向量,
    /// 代表仓库中的提交。如果请求或反序列化过程中发生错误,则返回一个空向量
    pub async fn get_github_commits(&self) -> Vec<GithubCommits> {
        // 构建用于获取提交的 GitHub API 端点 URL
        let url = format!(
            "{}/repos/{}/{}/commits",
            self.api_url, self.owner, self.repo
        );

        // 发送带有必要身份验证和头信息的 GET 请求
        let mut request_builder = self.client.get(url).headers(self.default_headers.clone());
        if let Some(token) = &self.token {
            request_builder = request_builder.bearer_auth(token);
        }

        // 处理响应
        match request_builder.send().await {
            Ok(res) => res
                .json::<Vec<GithubCommits>>()
                .await
                .unwrap_or_else(|_| Vec::new()),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                Vec::new()
            }
        }
    }

    /// 获取指定 GitHub 仓库的树列表
    pub async fn get_github_trees(&self, sha: &str) -> Option<GithubTrees> {
        let url = format!(
            "{}/repos/{}/{}/git/trees/{}",
            self.api_url, self.owner, self.repo, sha
        );
        dbg!(url.clone());

        let mut request_builder = self.client.get(url).headers(self.default_headers.clone());
        if let Some(token) = &self.token {
            request_builder = request_builder.bearer_auth(token);
        }

        match request_builder.send().await {
            Ok(res) => res.json::<GithubTrees>().await.ok(),
            Err(e) => {
                eprintln!("Error: {:?}", e);
                None
            }
        }
    }
}
