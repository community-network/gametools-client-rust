/*
 * Stats API for the Battlefield series
 *
 * This project also has a <a href=\"https://top.gg/bot/714524944783900794\" target=\"_blank\">Discord bot</a>, allowing anyone in your server to check their Battlefield stats via simple commands.<br><br>Language tags (lang) for both Battlefield 1 and Battlefield 5 are based on <a href=\"https://www.oracle.com/java/technologies/javase/jdk8-jre8-suported-locales.html\" target=\"_blank\">Java 8 language tags.</a> (response keys do not change based on the language tags, only the values do).<br><br>Data for Battlefield 2 is retrieved from the two revive projects: <a href=\"https://www.bf2hub.com\" target=\"_blank\">BF2Hub</a> and <a href=\"https://playbf2.tilda.ws/en\" target=\"_blank\">PlayBF2</a>.<br><br>Data for Battlefield 2142 is retrieved from the <a href=\"https://battlefield2142.co/\" target=\"_blank\">BF2142 Reclamation</a> project.<br><br>All other titles are still managed by EA/Dice.<br><br>If you discover any issues or have suggestions for new features, post them in the Community Network Discord: <a href=\"https://discord.gg/zMuxW6c\" target=\"_blank\">discord.gg/zMuxW6c</a>.<br><br>If you want to support the project, consider <a href=\"https://github.com/sponsors/community-network\" target=\"_blank\">becoming a sponsor on GitHub</a>.
 *
 * The version of the OpenAPI document: 1.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */

use crate::models;
use serde::{Deserialize, Serialize};

#[derive(Clone, Default, Debug, PartialEq, Serialize, Deserialize)]
pub struct Bf5DetailedServerInfo {
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "currentMap")]
    pub current_map: String,
    #[serde(rename = "currentMapImage")]
    pub current_map_image: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "playerAmount")]
    pub player_amount: i32,
    #[serde(rename = "prefix")]
    pub prefix: String,
    #[serde(rename = "owner")]
    pub owner: Box<models::ServerOwner>,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "favorites")]
    pub favorites: i32,
    #[serde(rename = "mode")]
    pub mode: String,
    #[serde(rename = "platform")]
    pub platform: String,
    #[serde(rename = "maxPlayerAmount")]
    pub max_player_amount: i32,
    #[serde(rename = "rotation")]
    pub rotation: Vec<models::ServerRotation>,
    #[serde(rename = "settings")]
    pub settings: Box<models::ServerSettings>,
    #[serde(rename = "gameId")]
    pub game_id: String,
    #[serde(rename = "serverId")]
    pub server_id: String,
    #[serde(rename = "inQueue")]
    pub in_queue: i32,
    #[serde(rename = "smallmode")]
    pub smallmode: String,
    #[serde(rename = "teams")]
    pub teams: Box<models::ServerTeams>,
    #[serde(rename = "official")]
    pub official: bool,
}

impl Bf5DetailedServerInfo {
    pub fn new(region: String, current_map: String, current_map_image: String, description: String, player_amount: i32, prefix: String, owner: models::ServerOwner, country: String, favorites: i32, mode: String, platform: String, max_player_amount: i32, rotation: Vec<models::ServerRotation>, settings: models::ServerSettings, game_id: String, server_id: String, in_queue: i32, smallmode: String, teams: models::ServerTeams, official: bool) -> Bf5DetailedServerInfo {
        Bf5DetailedServerInfo {
            region,
            current_map,
            current_map_image,
            description,
            player_amount,
            prefix,
            owner: Box::new(owner),
            country,
            favorites,
            mode,
            platform,
            max_player_amount,
            rotation,
            settings: Box::new(settings),
            game_id,
            server_id,
            in_queue,
            smallmode,
            teams: Box::new(teams),
            official,
        }
    }
}

