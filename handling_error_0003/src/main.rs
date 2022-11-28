/// This program is to learn how to use the crate thiserror
use thiserror::Error;
fn main() {
    let download_status = DownloadStatus::Stalling;
    println!("Download status: {:?}", download_status);
    println!("Download status: {}", download_status);
}

#[derive(Debug, Error)]
#[non_exhaustive]
enum DownloadStatus {
    #[error("??? downloading")]
    Downloading,
    #[error("??? stalling")]
    Stalling,
    #[error("??? success")]
    Success,
    #[error("??? failed")]
    Failed,
}
