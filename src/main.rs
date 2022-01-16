use std::collections::HashMap;
use std::hash::Hash;
use std::net::TcpListener;
use std::thread;
use std::thread::spawn;
use std::time::{Duration, SystemTime};
use crossbeam_channel::{Receiver, RecvTimeoutError, Sender};
use tokio_tungstenite::tungstenite::{accept, accept_hdr, Message};
use tokio_tungstenite::tungstenite::handshake::server::{Request, Response};

use crate::ddragon::get_version;
use crate::ddragon::item::Datum;
use crate::error::PlayerPurchasesError;
use crate::lcu::allgamedata::{Allgamedata, Item, Team};

pub mod lcu;
pub mod error;
pub mod ddragon;

type Gold = i64;
type ItemInfo = HashMap<String,Vec<ExternalItem>>;
type GoldInfo = HashMap<String, (Gold, Team)>;

#[derive(Serialize,Deserialize,Debug,Clone)]
enum Topics{
    ItemInformation(ItemInfo),
    GoldInformation(GoldInfo),
    PlayerInfo(Vec<PlayerInfo>),
    GameStart,
}

fn run_ws(r: Receiver<Topics>, sender: Sender<Topics>){
    let server = TcpListener::bind("127.0.0.1:3012").unwrap();

    for stream in server.incoming(){
        let rx = r.clone();
        let sx = sender.clone();
        spawn(move || {
            println!("Opened WS conn");
            let mut websocket = accept(stream.unwrap()).unwrap();

            loop {
                if !websocket.can_read() {
                    println!("Closed WS conn");
                    websocket.close(None);
                    break
                }
                let msg = match rx.recv_timeout(Duration::from_millis(10)) {
                    Ok(v) => {
                        v
                    }
                    Err(e) => {
                        if e.is_timeout(){
                            continue
                        }else{
                            println!("E: {:?}", e);
                            websocket.close(None);
                            break
                        }
                    }
                };
                let encoded = match &msg {
                    Topics::ItemInformation(v) => {
                        let f = (SystemTime::now(), "ItemInformation",v);
                        serde_json::to_string(&f).unwrap()
                    }
                    Topics::GoldInformation(v) => {
                        let f = (SystemTime::now(), "GoldInformation",v);
                        serde_json::to_string(&f).unwrap()

                    }
                    Topics::GameStart => {
                        let f = (SystemTime::now(), "GameStart");
                        serde_json::to_string(&f).unwrap()

                    }
                    Topics::PlayerInfo(v) => {
                        let f = (SystemTime::now(), "PlayerInfo",v);
                        serde_json::to_string(&f).unwrap()

                    }
                };
                let e = websocket.write_message(Message::Text(encoded));
                let ep = websocket.write_pending();
                if e.is_err() || ep.is_err(){
                    sx.send(msg);
                    websocket.close(None);
                    break
                }
            };
        });
    }

}

#[tokio::main]
async fn main() {
    let (s,r) = crossbeam_channel::unbounded();
    let sx = s.clone();
    thread::spawn(move || {
        run_ws(r, sx)
    });

    let mut last_items: HashMap<String, Vec<Item>> = HashMap::new();
    let mut game_running = false;
    loop {
        let agd = lcu::allgamedata::GetAllGameData().await;
        match agd {
            Ok(gamedata) => {
                if !game_running{
                    game_running = true;
                    s.send(Topics::GameStart);
                }
                let mut new_items: HashMap<String,Vec<Item>> = HashMap::new();
                for player in &gamedata.all_players {
                    let mut items = player.items.clone();
                    items.sort_unstable();
                    new_items.insert(player.summoner_name.clone(), items);
                }
                if last_items.is_empty() {
                    last_items = new_items;
                    continue
                }

                let mut actual_new_items: HashMap<String, Vec<Item>> = HashMap::new();

                for player in &gamedata.all_players {
                    let name = player.summoner_name.clone();
                    let player_last_items = last_items.get(&name);
                    let player_current_items = new_items.get(&name);

                    match player_last_items {
                        None => {
                            match player_current_items {
                                None => {
                                    //Skip
                                }
                                Some(items) => {
                                    actual_new_items.insert(name.clone(), items.clone());
                                }
                            }
                        }
                        Some(l_items) => {
                            match player_current_items {
                                None => {
                                    //Skip
                                }
                                Some(c_items) => {
                                    if c_items != l_items {
                                        let mut new_itemsx = vec![];
                                        for c_item in c_items {
                                            if l_items.iter().filter(|l| l.item_id == c_item.item_id && l.slot == c_item.slot).count() == 0{
                                                new_itemsx.push(c_item.clone());
                                            }
                                        }
                                        actual_new_items.insert(name,new_itemsx);
                                    }
                                }
                            }
                        }
                    }
                }

                if !actual_new_items.is_empty() {
                    let mut playerinfos: Vec<PlayerInfo>= vec![];
                    for all_player in &gamedata.all_players {
                        playerinfos.push(PlayerInfo{
                            champion_name: all_player.champion_name.clone(),
                            summoner_name: all_player.summoner_name.clone(),
                            skin_id: all_player.skin_id
                        });
                    }
                    s.send(Topics::PlayerInfo(playerinfos));

                    let readable_items = match generate_readable(actual_new_items).await {
                        Ok(v) => {v}
                        Err(e) => {
                            println!("Failed to generate readable item list: {}", e);
                            thread::sleep(Duration::from_millis(1000));
                            continue
                        }
                    };
                    s.send(Topics::ItemInformation(readable_items));

                    let mut goldInfoHM: GoldInfo = HashMap::new();
                    for (player, item) in &new_items {
                        let playerdata = gamedata.all_players.iter().filter(|p|&p.summoner_name == player).next().unwrap();
                        let gold: i64 = playerdata.items.iter().map(|i|i.price*i.count).sum();
                        let team = &playerdata.team;
                        goldInfoHM.insert(player.clone(), (gold, team.clone()));
                    }
                    s.send(Topics::GoldInformation(goldInfoHM));
                }
                last_items = new_items;
                thread::sleep(Duration::from_millis(100));
            }
            Err(err) => {
                println!("{}",err);
                last_items = HashMap::new();
                game_running = false;
                thread::sleep(Duration::from_secs(1));
            }
        }
    }
}

use serde::{Serialize,Deserialize};
use tokio_tungstenite::tungstenite::protocol::CloseFrame;

#[derive(Debug, Serialize, Deserialize,Clone)]
struct ExternalItem{
    name: String,
    image_url: String
}


#[derive(Debug, Serialize, Deserialize,Clone)]
struct PlayerInfo{
    champion_name: String,
    summoner_name: String,
    skin_id: i64,
}

async fn generate_readable(items: HashMap<String, Vec<Item>>) -> Result<HashMap<String, Vec<ExternalItem>>, PlayerPurchasesError>{

    let version = get_version().await?;
    let ddragon_items = ddragon::get_items().await.unwrap();
    let mut rethash = HashMap::new();
    for (summoner, items) in items.into_iter() {
        let mut itemsx = vec![];
        for item in items {
            let actual_item: &Datum = ddragon_items.data.get(&format!("{}",item.item_id)).unwrap();
             itemsx.push(ExternalItem{
                 name: actual_item.name.clone(),
                 image_url: format!("https://ddragon.leagueoflegends.com/cdn/{}/img/item/{}",version,actual_item.image.full)
             })
        }
        rethash.insert(summoner, itemsx);
    }
    Ok(rethash)
}