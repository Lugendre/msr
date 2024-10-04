use msr::kernel::Kernel;
use msr::usecase::add_new_song::fetch_and_create_song;

#[tokio::main]
pub async fn main() -> anyhow::Result<()> {
    let default_header = reqwest::header::HeaderMap::new();
    let kernel = Kernel::try_new(default_header)?;
    // let song = kernel
    //     .provide_msr_repository()
    //     .fetch_song("953967".to_string())
    //     .await?;
    // dbg!(song);
    // let album = kernel
    //     .provide_msr_repository()
    //     .fetch_album("0249".to_string())
    //     .await?;
    // dbg!(album);
    let song = fetch_and_create_song(&kernel, 953967.into()).await?;
    dbg!(song);
    Ok(())
}
