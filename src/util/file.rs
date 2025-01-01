use crate::core::error::AppResult;
use std::path::Path;
use tokio::{fs, io::AsyncWriteExt};

pub async fn store_file<P: AsRef<Path>>(file_path: &P, content: &[u8]) -> AppResult<()> {
    if let Some(parent_dir) = file_path.as_ref().parent() {
        fs::create_dir_all(&parent_dir).await?;
    }
    let mut file = fs::File::create(&file_path).await?;
    file.write_all(content).await?;
    Ok(())
}
