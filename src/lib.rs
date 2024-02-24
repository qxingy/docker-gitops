use anyhow::Result;
use bollard::{
    image::{BuildImageOptions, BuilderVersion},
    Docker,
};
use rocket::futures::StreamExt;

pub async fn run() -> Result<()> {
    let docker = Docker::connect_with_local_defaults()?;

    docker
        .build_image(
            BuildImageOptions {
                dockerfile: "https://github.com/qxingy/qqbot.git",
                t: "abc:latest",
                version: BuilderVersion::BuilderBuildKit,
                q: true,
                session: Some("hello".to_string()),
                ..Default::default()
            },
            None,
            None,
        )
        .for_each(|build_info| async {
            match build_info {
                Ok(build_info) => {
                    println!("Build status: {:?}", build_info.aux.unwrap());
                }
                Err(e) => {
                    println!("Error: {}", e);
                }
            }
        })
        .await;

    Ok(())
}
