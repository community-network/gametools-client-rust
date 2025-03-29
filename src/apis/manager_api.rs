/*
 * Stats API for the Battlefield series
 *
 * This project also has a <a href=\"https://top.gg/bot/714524944783900794\" target=\"_blank\">Discord bot</a>, allowing anyone in your server to check their Battlefield stats via simple commands.<br><br>Language tags (lang) for both Battlefield 1 and Battlefield 5 are based on <a href=\"https://www.oracle.com/java/technologies/javase/jdk8-jre8-suported-locales.html\" target=\"_blank\">Java 8 language tags.</a> (response keys do not change based on the language tags, only the values do).<br><br>Data for Battlefield 2 is retrieved from the two revive projects: <a href=\"https://www.bf2hub.com\" target=\"_blank\">BF2Hub</a> and <a href=\"https://playbf2.tilda.ws/en\" target=\"_blank\">PlayBF2</a>.<br><br>Data for Battlefield 2142 is retrieved from the <a href=\"https://battlefield2142.co/\" target=\"_blank\">BF2142 Reclamation</a> project.<br><br>All other titles are still managed by EA/Dice.<br><br>If you discover any issues or have suggestions for new features, post them in the Community Network Discord: <a href=\"https://discord.gg/zMuxW6c\" target=\"_blank\">discord.gg/zMuxW6c</a>.<br><br>If you want to support the project, consider <a href=\"https://github.com/sponsors/community-network\" target=\"_blank\">becoming a sponsor on GitHub</a>.
 *
 * The version of the OpenAPI document: 1.3.0
 * 
 * Generated by: https://openapi-generator.tech
 */


use reqwest;
use serde::{Deserialize, Serialize, de::Error as _};
use crate::{apis::ResponseContent, models};
use super::{Error, configuration, ContentType};


/// struct for typed errors of method [`checkban_manager_checkban_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckbanManagerCheckbanGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`checkbans_manager_checkbans_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum CheckbansManagerCheckbansGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`info_manager_info_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InfoManagerInfoGetError {
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`leaderboard_manager_leaderboard_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaderboardManagerLeaderboardGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`leaderboard_v2_manager_leaderboard_v2_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LeaderboardV2ManagerLeaderboardV2GetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`live_manager_live_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LiveManagerLiveGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`players_manager_players_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlayersManagerPlayersGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`server_sessions_manager_server_sessions_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ServerSessionsManagerServerSessionsGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`sessions_manager_sessions_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SessionsManagerSessionsGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


pub async fn checkban_manager_checkban_get(configuration: &configuration::Configuration, name: Option<&str>, playerid: Option<i32>, oid: Option<i32>, platform: Option<models::FrostbitePlatforms>, skip_battlelog: Option<bool>) -> Result<serde_json::Value, Error<CheckbanManagerCheckbanGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_playerid = playerid;
    let p_oid = oid;
    let p_platform = platform;
    let p_skip_battlelog = skip_battlelog;

    let uri_str = format!("{}/manager/checkban/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_playerid {
        req_builder = req_builder.query(&[("playerid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_oid {
        req_builder = req_builder.query(&[("oid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_skip_battlelog {
        req_builder = req_builder.query(&[("skip_battlelog", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CheckbanManagerCheckbanGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn checkbans_manager_checkbans_get(configuration: &configuration::Configuration, personaids: Option<&str>) -> Result<serde_json::Value, Error<CheckbansManagerCheckbansGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_personaids = personaids;

    let uri_str = format!("{}/manager/checkbans/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_personaids {
        req_builder = req_builder.query(&[("personaids", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<CheckbansManagerCheckbansGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// This is only the amount of usage for privacy
pub async fn info_manager_info_get(configuration: &configuration::Configuration, ) -> Result<models::ManagerInfo, Error<InfoManagerInfoGetError>> {

    let uri_str = format!("{}/manager/info/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ManagerInfo`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ManagerInfo`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<InfoManagerInfoGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// It only works on servers using our manager: manager.gametools.network
pub async fn leaderboard_manager_leaderboard_get(configuration: &configuration::Configuration, sort: Option<models::ManagerLeaderboardSortTypes>, amount: Option<i32>, gameid: Option<&str>, serverid: Option<&str>) -> Result<models::ManagerServerLeaderBoard, Error<LeaderboardManagerLeaderboardGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_sort = sort;
    let p_amount = amount;
    let p_gameid = gameid;
    let p_serverid = serverid;

    let uri_str = format!("{}/manager/leaderboard/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_amount {
        req_builder = req_builder.query(&[("amount", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_gameid {
        req_builder = req_builder.query(&[("gameid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_serverid {
        req_builder = req_builder.query(&[("serverid", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ManagerServerLeaderBoard`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ManagerServerLeaderBoard`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LeaderboardManagerLeaderboardGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// It only works on servers using our manager: manager.gametools.network
pub async fn leaderboard_v2_manager_leaderboard_v2_get(configuration: &configuration::Configuration, serverid: Option<&str>, gameid: Option<i32>, player_name_filter: Option<&str>, amount: Option<i32>, sort: Option<models::V2ManagerLeaderboardSortTypes>, days: Option<i32>) -> Result<models::ManagerServerLeaderBoardV2, Error<LeaderboardV2ManagerLeaderboardV2GetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_serverid = serverid;
    let p_gameid = gameid;
    let p_player_name_filter = player_name_filter;
    let p_amount = amount;
    let p_sort = sort;
    let p_days = days;

    let uri_str = format!("{}/manager/leaderboard/v2/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_serverid {
        req_builder = req_builder.query(&[("serverid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_gameid {
        req_builder = req_builder.query(&[("gameid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_player_name_filter {
        req_builder = req_builder.query(&[("player_name_filter", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_amount {
        req_builder = req_builder.query(&[("amount", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_sort {
        req_builder = req_builder.query(&[("sort", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_days {
        req_builder = req_builder.query(&[("days", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ManagerServerLeaderBoardV2`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ManagerServerLeaderBoardV2`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LeaderboardV2ManagerLeaderboardV2GetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// It only works on servers using our manager: manager.gametools.network
pub async fn live_manager_live_get(configuration: &configuration::Configuration, name: Option<&str>, playerid: Option<i32>, oid: Option<i32>, platform: Option<models::FrostbitePlatforms>, skip_battlelog: Option<bool>) -> Result<models::ManagerLivePlayer, Error<LiveManagerLiveGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_playerid = playerid;
    let p_oid = oid;
    let p_platform = platform;
    let p_skip_battlelog = skip_battlelog;

    let uri_str = format!("{}/manager/live/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_playerid {
        req_builder = req_builder.query(&[("playerid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_oid {
        req_builder = req_builder.query(&[("oid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_skip_battlelog {
        req_builder = req_builder.query(&[("skip_battlelog", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ManagerLivePlayer`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ManagerLivePlayer`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LiveManagerLiveGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// It only works on servers using our manager: manager.gametools.network
pub async fn players_manager_players_get(configuration: &configuration::Configuration, gameid: Option<&str>, serverid: Option<&str>) -> Result<models::ManagerServerPlayers, Error<PlayersManagerPlayersGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_gameid = gameid;
    let p_serverid = serverid;

    let uri_str = format!("{}/manager/players/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_gameid {
        req_builder = req_builder.query(&[("gameid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_serverid {
        req_builder = req_builder.query(&[("serverid", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ManagerServerPlayers`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ManagerServerPlayers`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PlayersManagerPlayersGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// It only tracks the session within this week and only works on servers using our manager: manager.gametools.network
pub async fn server_sessions_manager_server_sessions_get(configuration: &configuration::Configuration, serverid: Option<&str>, start_datetime: Option<String>, end_datetime: Option<String>) -> Result<models::ManagerPlayerSession, Error<ServerSessionsManagerServerSessionsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_serverid = serverid;
    let p_start_datetime = start_datetime;
    let p_end_datetime = end_datetime;

    let uri_str = format!("{}/manager/server_sessions/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_serverid {
        req_builder = req_builder.query(&[("serverid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_start_datetime {
        req_builder = req_builder.query(&[("start_datetime", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_end_datetime {
        req_builder = req_builder.query(&[("end_datetime", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ManagerPlayerSession`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ManagerPlayerSession`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<ServerSessionsManagerServerSessionsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// It only tracks the session within this week and only works on servers using our manager: manager.gametools.network
pub async fn sessions_manager_sessions_get(configuration: &configuration::Configuration, name: Option<&str>, playerid: Option<i32>, oid: Option<i32>, platform: Option<models::FrostbitePlatforms>, skip_battlelog: Option<bool>) -> Result<models::ManagerPlayerSession, Error<SessionsManagerSessionsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_playerid = playerid;
    let p_oid = oid;
    let p_platform = platform;
    let p_skip_battlelog = skip_battlelog;

    let uri_str = format!("{}/manager/sessions/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_playerid {
        req_builder = req_builder.query(&[("playerid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_oid {
        req_builder = req_builder.query(&[("oid", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_skip_battlelog {
        req_builder = req_builder.query(&[("skip_battlelog", &param_value.to_string())]);
    }
    if let Some(ref user_agent) = configuration.user_agent {
        req_builder = req_builder.header(reqwest::header::USER_AGENT, user_agent.clone());
    }

    let req = req_builder.build()?;
    let resp = configuration.client.execute(req).await?;

    let status = resp.status();
    let content_type = resp
        .headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .unwrap_or("application/octet-stream");
    let content_type = super::ContentType::from(content_type);

    if !status.is_client_error() && !status.is_server_error() {
        let content = resp.text().await?;
        match content_type {
            ContentType::Json => serde_json::from_str(&content).map_err(Error::from),
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::ManagerPlayerSession`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::ManagerPlayerSession`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SessionsManagerSessionsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

