use crate::api::ApiClient;
use crate::commands::find_task_id;
use crate::config::Config;
use anyhow::Result;
use std::path::PathBuf;

pub fn run(
    task_index: usize,
    attachment_index_or_gid: String,
    output: Option<String>,
) -> Result<()> {
    let config = Config::load()?;
    let client = ApiClient::new(&config)?;

    // Check if attachment_index_or_gid is a GID or an index
    let attachment = if attachment_index_or_gid
        .chars()
        .all(|c| c.is_numeric() && attachment_index_or_gid.len() < 5)
    {
        // It's an index
        let task_id = find_task_id(Some(task_index))?;
        let attachments = client.get_attachments(&task_id)?;

        let index: usize = attachment_index_or_gid
            .parse()
            .map_err(|_| anyhow::anyhow!("Invalid attachment index"))?;

        if index >= attachments.len() {
            anyhow::bail!(
                "Attachment index {} out of range (0-{})",
                index,
                attachments.len() - 1
            );
        }

        attachments[index].clone()
    } else {
        // It's a GID
        client.get_attachment(&attachment_index_or_gid)?
    };

    // Determine output path
    let output_path = if let Some(path) = output {
        PathBuf::from(path)
    } else {
        PathBuf::from(&attachment.name)
    };

    // Get download URL
    let download_url = attachment
        .download_url
        .ok_or_else(|| anyhow::anyhow!("Attachment has no download URL"))?;

    println!("Downloading: {}", attachment.name);
    println!("To: {}", output_path.display());

    client.download_attachment(&download_url, &output_path)?;

    println!("Download complete!");

    Ok(())
}
