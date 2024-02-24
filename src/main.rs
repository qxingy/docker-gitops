use anyhow::Result;

#[tokio::main]
async fn main() -> Result<()> {
    docker_gitops::run().await
}
