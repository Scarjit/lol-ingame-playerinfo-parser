use crate::ddragon::item::Item;
use crate::PlayerPurchasesError;
pub mod item;
use cached::proc_macro::cached;

pub async fn get_version() -> Result<String,PlayerPurchasesError>{
    let resp = reqwest::get("https://ddragon.leagueoflegends.com/api/versions.json").await?;
    let text = resp.text().await?;
    let mut parsed:Vec<String> = serde_json::from_str(&text)?;
    Ok(parsed.get(0).unwrap().to_string())
}

#[cached(result = true)]
pub async fn get_items() -> Result<Item, PlayerPurchasesError>{
    let version = get_version().await?;
    let url =format!("https://ddragon.leagueoflegends.com/cdn/{}/data/en_US/item.json", version);
    let resp = reqwest::get(url).await?;
    let text = resp.text().await?;
    let parsed: Item = serde_json::from_str(&text)?;
    return Ok(parsed)
}