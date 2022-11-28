/// This program is to learn how to implemnt custome type error.

use std::fmt::{Display, Formatter};
use std::error::Error;

fn main() {
    let download_status = DownloadStatus::Stalling;
    println!("Download status: {:?}", download_status);
    println!("Download status: {}", download_status);
}

#[derive(Debug)]
#[non_exhaustive]
enum DownloadStatus {
    Downloading,
    Stalling,
    Success,
    Failed,
}

impl Display for DownloadStatus {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        use DownloadStatus::*;
        match self {
            Downloading => write!(f, "~~~ downloading"),
            Stalling => write!(f, "~~~ stalling"),
            Success => write!(f, "~~~ success"),
            _ => write!(f, "~~~ failed"),
        }
    }
}

impl Error for DownloadStatus {}
