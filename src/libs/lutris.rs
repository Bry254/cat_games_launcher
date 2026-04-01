use std::env::temp_dir;
use std::fs::File;
use std::io::copy;

use crate::libs::desktop::IconProcesor;
use serde_json::Value;

pub fn download_file(url: &str, file: &str) -> anyhow::Result<()> {
    let mut resp = ureq::get(url).call()?;
    let mut file = File::create(file)?;
    copy(&mut resp.body_mut().as_reader(), &mut file)?;
    return Ok(());
}

// pub fn get_cover(name: &str) {

// }

pub fn get_icon(name: &str) -> anyhow::Result<String> {
    let mut json = ureq::get(format!(
        "https://lutris.net/api/games/{}",
        name.replace(" ", "-")
    ))
    .call()?;
    let body = json.body_mut().read_to_string()?;
    let json: Value = serde_json::from_str(&body.as_str())?;
    if let Some(url) = json.get("icon_url").and_then(|v| v.as_str()) {
        // println!("{url}");
        let temp_file = temp_dir()
            .join("cat_game_temp.png")
            .to_string_lossy()
            .to_string();
        download_file(&url, temp_file.as_str())?;
        println!("{} {}", &temp_file, &name);
        let processor = IconProcesor::default();
        processor.image_resize(&temp_file, &name.to_string())?;
        return Ok(processor
            .gen_outpath(&name.to_string())
            .to_string_lossy()
            .to_string());
    }
    Err(anyhow::anyhow!(""))
}
