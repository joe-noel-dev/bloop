use anyhow::anyhow;
use async_zip::base::read::seek::ZipFileReader;
use async_zip::{base::write::ZipFileWriter, Compression, ZipEntryBuilder};
use log::debug;
use std::{
    path::{Path, PathBuf},
    vec,
};
use tokio::fs::{create_dir_all, OpenOptions};
use tokio::{
    fs::File,
    io::{AsyncReadExt, BufReader},
};
use tokio_util::compat::{TokioAsyncReadCompatExt, TokioAsyncWriteCompatExt};

pub async fn unzip_file(archive: &Path, out_dir: &Path) -> anyhow::Result<()> {
    let archive = File::open(archive).await?;
    let archive = BufReader::new(archive).compat();
    let mut reader = ZipFileReader::new(archive).await?;

    for index in 0..reader.file().entries().len() {
        let entry = reader.file().entries().get(index).unwrap();
        let path = out_dir.join(entry.filename().as_str()?);

        let entry_is_dir = entry.dir()?;

        let mut entry_reader = reader.reader_without_entry(index).await?;

        if entry_is_dir {
            // The directory may have been created if iteration is out of order.
            if !path.exists() {
                create_dir_all(&path).await?;
            }
        } else {
            // Creates parent directories. They may not exist if iteration is out of order
            // or the archive does not contain directory entries.
            let parent = path.parent().unwrap();
            if !parent.is_dir() {
                create_dir_all(parent).await?;
            }
            let writer = OpenOptions::new()
                .write(true)
                .create_new(true)
                .open(&path)
                .await
                .expect("Failed to create extracted file");
            futures_lite::io::copy(&mut entry_reader, &mut writer.compat_write())
                .await
                .expect("Failed to copy to extracted file");

            // Closes the file and manipulates its metadata here if you wish to preserve its metadata from the archive.
        }
    }

    Ok(())
}

pub async fn zip_directory(directory: &Path, to_file: &Path) -> anyhow::Result<()> {
    let file = File::create(to_file).await?;
    let mut zip_writer = ZipFileWriter::with_tokio(file);
    let files_to_add = walk_directory(directory).await?;

    for entry in files_to_add {
        let relative_path = entry.strip_prefix(directory)?;

        debug!("Adding entry to export: {}", relative_path.display());

        let entry_path = relative_path
            .as_os_str()
            .to_str()
            .ok_or(anyhow!("Directory file path not valid UTF-8."))?;

        let mut input_file = File::open(&entry).await?;
        let input_file_size = input_file.metadata().await?.len() as usize;

        let mut buffer = Vec::with_capacity(input_file_size);
        input_file.read_to_end(&mut buffer).await?;

        let builder = ZipEntryBuilder::new(entry_path.into(), Compression::Deflate);

        zip_writer.write_entry_whole(builder, &buffer).await?;
    }

    zip_writer.close().await?;

    Ok(())
}

async fn walk_directory(directory: &Path) -> anyhow::Result<Vec<PathBuf>> {
    let mut directories = vec![directory.to_path_buf()];
    let mut files = vec![];

    while let Some(path) = directories.pop() {
        let mut directory_iterator = tokio::fs::read_dir(path).await?;

        while let Some(entry) = directory_iterator.next_entry().await? {
            let entry_path = entry.path();

            if entry_path.is_dir() {
                directories.push(entry_path);
            } else {
                files.push(entry_path);
            }
        }
    }

    Ok(files)
}
