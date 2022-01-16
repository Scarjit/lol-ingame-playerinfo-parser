use std::cmp::Ordering;
use std::fmt::{Display, Formatter};
use serde::{Serialize, Deserialize};
use crate::error::PlayerPurchasesError;

pub(crate) async fn GetAllGameData() -> Result<Allgamedata, PlayerPurchasesError>{
    let client = reqwest::Client::builder().danger_accept_invalid_certs(true).build()?;
    let rq = client.get("https://127.0.0.1:2999/liveclientdata/allgamedata").send().await?;
    let rq_text = rq.text().await?;
    let parsed: Allgamedata = serde_json::from_str(&rq_text)?;
    Ok(parsed)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Allgamedata {
    #[serde(rename = "activePlayer")]
    pub active_player: ActivePlayer,
    #[serde(rename = "allPlayers")]
    pub all_players: Vec<AllPlayer>,
    pub events: Events,
    #[serde(rename = "gameData")]
    pub game_data: GameData,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ActivePlayer {
    pub abilities: Abilities,
    #[serde(rename = "championStats")]
    pub champion_stats: ChampionStats,
    #[serde(rename = "currentGold")]
    pub current_gold: f64,
    #[serde(rename = "fullRunes")]
    pub full_runes: FullRunes,
    pub level: i64,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "teamRelativeColors")]
    pub team_relative_colors: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Abilities {
    #[serde(rename = "E")]
    pub e: E,
    #[serde(rename = "Passive")]
    pub passive: E,
    #[serde(rename = "Q")]
    pub q: E,
    #[serde(rename = "R")]
    pub r: E,
    #[serde(rename = "W")]
    pub w: E,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct E {
    #[serde(rename = "abilityLevel")]
    pub ability_level: Option<i64>,
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub id: Option<String>,
    #[serde(rename = "rawDescription")]
    pub raw_description: String,
    #[serde(rename = "rawDisplayName")]
    pub raw_display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChampionStats {
    #[serde(rename = "abilityHaste")]
    pub ability_haste: f64,
    #[serde(rename = "abilityPower")]
    pub ability_power: f64,
    pub armor: f64,
    #[serde(rename = "armorPenetrationFlat")]
    pub armor_penetration_flat: f64,
    #[serde(rename = "armorPenetrationPercent")]
    pub armor_penetration_percent: f64,
    #[serde(rename = "attackDamage")]
    pub attack_damage: f64,
    #[serde(rename = "attackRange")]
    pub attack_range: f64,
    #[serde(rename = "attackSpeed")]
    pub attack_speed: f64,
    #[serde(rename = "bonusArmorPenetrationPercent")]
    pub bonus_armor_penetration_percent: f64,
    #[serde(rename = "bonusMagicPenetrationPercent")]
    pub bonus_magic_penetration_percent: f64,
    #[serde(rename = "critChance")]
    pub crit_chance: f64,
    #[serde(rename = "critDamage")]
    pub crit_damage: f64,
    #[serde(rename = "currentHealth")]
    pub current_health: f64,
    #[serde(rename = "healShieldPower")]
    pub heal_shield_power: f64,
    #[serde(rename = "healthRegenRate")]
    pub health_regen_rate: f64,
    #[serde(rename = "lifeSteal")]
    pub life_steal: f64,
    #[serde(rename = "magicLethality")]
    pub magic_lethality: f64,
    #[serde(rename = "magicPenetrationFlat")]
    pub magic_penetration_flat: f64,
    #[serde(rename = "magicPenetrationPercent")]
    pub magic_penetration_percent: f64,
    #[serde(rename = "magicResist")]
    pub magic_resist: f64,
    #[serde(rename = "maxHealth")]
    pub max_health: f64,
    #[serde(rename = "moveSpeed")]
    pub move_speed: f64,
    pub omnivamp: f64,
    #[serde(rename = "physicalLethality")]
    pub physical_lethality: f64,
    #[serde(rename = "physicalVamp")]
    pub physical_vamp: f64,
    #[serde(rename = "resourceMax")]
    pub resource_max: f64,
    #[serde(rename = "resourceRegenRate")]
    pub resource_regen_rate: f64,
    #[serde(rename = "resourceType")]
    pub resource_type: String,
    #[serde(rename = "resourceValue")]
    pub resource_value: f64,
    #[serde(rename = "spellVamp")]
    pub spell_vamp: f64,
    pub tenacity: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FullRunes {
    #[serde(rename = "generalRunes")]
    pub general_runes: Vec<Keystone>,
    pub keystone: Keystone,
    #[serde(rename = "primaryRuneTree")]
    pub primary_rune_tree: Keystone,
    #[serde(rename = "secondaryRuneTree")]
    pub secondary_rune_tree: Keystone,
    #[serde(rename = "statRunes")]
    pub stat_runes: Vec<StatRune>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Keystone {
    #[serde(rename = "displayName")]
    pub display_name: String,
    pub id: i64,
    #[serde(rename = "rawDescription")]
    pub raw_description: String,
    #[serde(rename = "rawDisplayName")]
    pub raw_display_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct StatRune {
    pub id: i64,
    #[serde(rename = "rawDescription")]
    pub raw_description: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AllPlayer {
    #[serde(rename = "championName")]
    pub champion_name: String,
    #[serde(rename = "isBot")]
    pub is_bot: bool,
    #[serde(rename = "isDead")]
    pub is_dead: bool,
    pub items: Vec<Item>,
    pub level: i64,
    pub position: String,
    #[serde(rename = "rawChampionName")]
    pub raw_champion_name: String,
    #[serde(rename = "rawSkinName")]
    pub raw_skin_name: Option<String>,
    #[serde(rename = "respawnTimer")]
    pub respawn_timer: f64,
    pub runes: Runes,
    pub scores: Scores,
    #[serde(rename = "skinID")]
    pub skin_id: i64,
    #[serde(rename = "skinName")]
    pub skin_name: Option<String>,
    #[serde(rename = "summonerName")]
    pub summoner_name: String,
    #[serde(rename = "summonerSpells")]
    pub summoner_spells: SummonerSpells,
    pub team: Team,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub struct Item {
    #[serde(rename = "canUse")]
    pub can_use: bool,
    pub consumable: bool,
    pub count: i64,
    #[serde(rename = "displayName")]
    pub display_name: String,
    #[serde(rename = "itemID")]
    pub item_id: i64,
    pub price: i64,
    #[serde(rename = "rawDescription")]
    pub raw_description: String,
    #[serde(rename = "rawDisplayName")]
    pub raw_display_name: String,
    pub slot: i64,
}

impl Display for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}x{}", self.item_id,self.count)
    }
}

impl PartialEq<Self> for Item {
    fn eq(&self, other: &Self) -> bool {
        (self.item_id, self.count) == (other.item_id, other.count)
    }
}

impl Eq for Item {}

impl PartialOrd<Self> for Item {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Item {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.item_id).cmp(&other.item_id)
    }
}


#[derive(Debug, Serialize, Deserialize)]
pub struct Runes {
    pub keystone: Keystone,
    #[serde(rename = "primaryRuneTree")]
    pub primary_rune_tree: Keystone,
    #[serde(rename = "secondaryRuneTree")]
    pub secondary_rune_tree: Keystone,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Scores {
    pub assists: i64,
    #[serde(rename = "creepScore")]
    pub creep_score: i64,
    pub deaths: i64,
    pub kills: i64,
    #[serde(rename = "wardScore")]
    pub ward_score: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SummonerSpells {
    #[serde(rename = "summonerSpellOne")]
    pub summoner_spell_one: E,
    #[serde(rename = "summonerSpellTwo")]
    pub summoner_spell_two: E,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Events {
    #[serde(rename = "Events")]
    pub events: Vec<Event>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Event {
    #[serde(rename = "EventID")]
    pub event_id: i64,
    #[serde(rename = "EventName")]
    pub event_name: String,
    #[serde(rename = "EventTime")]
    pub event_time: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GameData {
    #[serde(rename = "gameMode")]
    pub game_mode: String,
    #[serde(rename = "gameTime")]
    pub game_time: f64,
    #[serde(rename = "mapName")]
    pub map_name: String,
    #[serde(rename = "mapNumber")]
    pub map_number: i64,
    #[serde(rename = "mapTerrain")]
    pub map_terrain: String,
}

#[derive(Debug, Serialize, Deserialize,Clone)]
pub enum Team {
    #[serde(rename = "CHAOS")]
    Chaos,
    #[serde(rename = "ORDER")]
    Order,
}
