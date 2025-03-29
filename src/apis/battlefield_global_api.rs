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


/// struct for typed errors of method [`bfgames_bfglobal_games_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum BfgamesBfglobalGamesGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`platoonstat_bfglobal_detailedplatoon_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PlatoonstatBfglobalDetailedplatoonGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`print_logged_data_bfglobal_statusarray_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum PrintLoggedDataBfglobalStatusarrayGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_platoon_bfglobal_platoons_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchPlatoonBfglobalPlatoonsGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`search_player_bfglobal_search_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum SearchPlayerBfglobalSearchGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`total_status_array_bfglobal_totalstatusarray_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TotalStatusArrayBfglobalTotalstatusarrayGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


/// you only have to fill in one of them: oid (used in bfban: 'https://bfban.com/#/cheaters/1008084550936'), name (normal playername), or playerid (same as id in what it returns - fastest method).  if you dont know what you need to use, just use 'name', just fill the playername in there: **_/stats/?name=iiTzArcur
pub async fn bfgames_bfglobal_games_get(configuration: &configuration::Configuration, include_emblem: Option<bool>, name: Option<&str>, playerid: Option<i32>, oid: Option<i32>, platform: Option<models::FrostbitePlatforms>, skip_battlelog: Option<bool>) -> Result<models::PlayerGames, Error<BfgamesBfglobalGamesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_include_emblem = include_emblem;
    let p_name = name;
    let p_playerid = playerid;
    let p_oid = oid;
    let p_platform = platform;
    let p_skip_battlelog = skip_battlelog;

    let uri_str = format!("{}/bfglobal/games/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_include_emblem {
        req_builder = req_builder.query(&[("include_emblem", &param_value.to_string())]);
    }
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PlayerGames`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PlayerGames`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<BfgamesBfglobalGamesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn platoonstat_bfglobal_detailedplatoon_get(configuration: &configuration::Configuration, id: &str, platform: Option<&str>, lang: Option<&str>) -> Result<models::DetailedPlatoonInfo, Error<PlatoonstatBfglobalDetailedplatoonGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_id = id;
    let p_platform = platform;
    let p_lang = lang;

    let uri_str = format!("{}/bfglobal/detailedplatoon/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("id", &p_id.to_string())]);
    if let Some(ref param_value) = p_platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_lang {
        req_builder = req_builder.query(&[("lang", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::DetailedPlatoonInfo`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::DetailedPlatoonInfo`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PlatoonstatBfglobalDetailedplatoonGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn print_logged_data_bfglobal_statusarray_get(configuration: &configuration::Configuration, days: Option<i32>, platform: Option<&str>) -> Result<models::GlobalStatusArray, Error<PrintLoggedDataBfglobalStatusarrayGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_days = days;
    let p_platform = platform;

    let uri_str = format!("{}/bfglobal/statusarray/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_days {
        req_builder = req_builder.query(&[("days", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::GlobalStatusArray`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::GlobalStatusArray`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<PrintLoggedDataBfglobalStatusarrayGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

/// It needs a minimum of 3 characters to start the search                          Available options for platform: pc, ps4, xboxone.
pub async fn search_platoon_bfglobal_platoons_get(configuration: &configuration::Configuration, name: &str, platform: Option<&str>, lang: Option<&str>) -> Result<models::PlatoonStats, Error<SearchPlatoonBfglobalPlatoonsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_platform = platform;
    let p_lang = lang;

    let uri_str = format!("{}/bfglobal/platoons/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("name", &p_name.to_string())]);
    if let Some(ref param_value) = p_platform {
        req_builder = req_builder.query(&[("platform", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_lang {
        req_builder = req_builder.query(&[("lang", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::PlatoonStats`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::PlatoonStats`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SearchPlatoonBfglobalPlatoonsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn search_player_bfglobal_search_get(configuration: &configuration::Configuration, name: &str) -> Result<Vec<models::Player>, Error<SearchPlayerBfglobalSearchGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;

    let uri_str = format!("{}/bfglobal/search/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("name", &p_name.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `Vec&lt;models::Player&gt;`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `Vec&lt;models::Player&gt;`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<SearchPlayerBfglobalSearchGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn total_status_array_bfglobal_totalstatusarray_get(configuration: &configuration::Configuration, days: Option<i32>) -> Result<models::TotalStatusArray, Error<TotalStatusArrayBfglobalTotalstatusarrayGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_days = days;

    let uri_str = format!("{}/bfglobal/totalstatusarray/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::TotalStatusArray`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::TotalStatusArray`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<TotalStatusArrayBfglobalTotalstatusarrayGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

