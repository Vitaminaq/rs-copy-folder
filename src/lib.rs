use napi::{Result};
use napi_derive::napi;
use std::fs;
use std::path::{Path};
use tokio::fs as tokio_fs;

#[napi]
pub async fn copy_directory(src: String, dest: String) -> Result<()> {
    let src_path = Path::new(&src);
    let dest_path = Path::new(&dest);

    // 创建目标文件夹
    if !dest_path.exists() {
      fs::create_dir_all(dest_path)?;
    }

    // 异步遍历源文件夹下的所有文件
    let mut entries = tokio_fs::read_dir(src_path).await?;

    while let Some(entry) = entries.next_entry().await? {
      // println!("{:?}", entry.path());
      let new_src = src_path.join(entry.file_name());
      let new_dest = dest_path.join(entry.file_name());
      if new_src.is_file() {
        println!("src: {}, dest: {}", src, dest);
        let _ = fs::copy(&new_src, &new_dest);
      } else if new_src.is_dir() {
        // println!("copy dir: {:?}", path);
        let future = Box::pin(copy_directory(new_src.to_string_lossy().to_string(), new_dest.to_string_lossy().to_string()));
            future.await?;
      }
    }
    Ok(())
}
