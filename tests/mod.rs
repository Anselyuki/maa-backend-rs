use maa_backend::repository::github_api::GithubApi;

#[tokio::test]
async fn test_git_commit() {
    let maa_repo_api = GithubApi::default();
    let commits = maa_repo_api.get_github_commits().await;
    println!("{:#?}", commits);
}

#[tokio::test]
async fn test_git_tree() {
    let maa_repo_api = GithubApi::default();
    let trees = maa_repo_api
        .get_github_trees("d989739981db071e80df1c66e473c729b50e8073")
        .await;
    println!("{:#?}", trees);
}
