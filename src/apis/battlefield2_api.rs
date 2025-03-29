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


/// struct for typed errors of method [`bf2_status_array_bf2_statusarray_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bf2StatusArrayBf2StatusarrayGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bf2_status_bf2_status_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bf2StatusBf2StatusGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bf2classes_bf2_classes_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bf2classesBf2ClassesGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bf2leaderboard_bf2_leaderboard_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bf2leaderboardBf2LeaderboardGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bf2servers_bf2_servers_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bf2serversBf2ServersGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bf2stats_bf2_stats_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bf2statsBf2StatsGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bf2vehicles_bf2_vehicles_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bf2vehiclesBf2VehiclesGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`bf2weapons_bf2_weapons_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum Bf2weaponsBf2WeaponsGetError {
    Status404(),
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}

/// struct for typed errors of method [`logged_server_data_bf2_serverarray_get`]
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum LoggedServerDataBf2ServerarrayGetError {
    Status422(models::HttpValidationError),
    UnknownValue(serde_json::Value),
}


pub async fn bf2_status_array_bf2_statusarray_get(configuration: &configuration::Configuration, service: Option<models::Bf2Platform>, days: Option<i32>) -> Result<models::StatusArray, Error<Bf2StatusArrayBf2StatusarrayGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_service = service;
    let p_days = days;

    let uri_str = format!("{}/bf2/statusarray/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_service {
        req_builder = req_builder.query(&[("service", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::StatusArray`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::StatusArray`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<Bf2StatusArrayBf2StatusarrayGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bf2_status_bf2_status_get(configuration: &configuration::Configuration, service: Option<models::Bf2PlatformAll>) -> Result<models::OlderGameStatus, Error<Bf2StatusBf2StatusGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_service = service;

    let uri_str = format!("{}/bf2/status/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_service {
        req_builder = req_builder.query(&[("service", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OlderGameStatus`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OlderGameStatus`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<Bf2StatusBf2StatusGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bf2classes_bf2_classes_get(configuration: &configuration::Configuration, name: Option<&str>, playerid: Option<i32>, platform: Option<models::Bf2Platform>) -> Result<models::Bf2ClassStats, Error<Bf2classesBf2ClassesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_playerid = playerid;
    let p_platform = platform;

    let uri_str = format!("{}/bf2/classes/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_playerid {
        req_builder = req_builder.query(&[("playerid", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Bf2ClassStats`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Bf2ClassStats`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<Bf2classesBf2ClassesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bf2leaderboard_bf2_leaderboard_get(configuration: &configuration::Configuration, platform: Option<models::Bf2Platform>) -> Result<models::Bf2Leaderboard, Error<Bf2leaderboardBf2LeaderboardGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_platform = platform;

    let uri_str = format!("{}/bf2/leaderboard/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Bf2Leaderboard`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Bf2Leaderboard`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<Bf2leaderboardBf2LeaderboardGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bf2servers_bf2_servers_get(configuration: &configuration::Configuration, name: &str, r#type: Option<models::LegacyBfQueryType>, service: Option<models::Bf2Platform>) -> Result<models::OlderTitleSearch, Error<Bf2serversBf2ServersGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_type = r#type;
    let p_service = service;

    let uri_str = format!("{}/bf2/servers/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("name", &p_name.to_string())]);
    if let Some(ref param_value) = p_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_service {
        req_builder = req_builder.query(&[("service", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::OlderTitleSearch`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::OlderTitleSearch`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<Bf2serversBf2ServersGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bf2stats_bf2_stats_get(configuration: &configuration::Configuration, name: Option<&str>, playerid: Option<i32>, platform: Option<models::Bf2Platform>) -> Result<models::Bf2MainStats, Error<Bf2statsBf2StatsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_playerid = playerid;
    let p_platform = platform;

    let uri_str = format!("{}/bf2/stats/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_playerid {
        req_builder = req_builder.query(&[("playerid", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Bf2MainStats`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Bf2MainStats`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<Bf2statsBf2StatsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bf2vehicles_bf2_vehicles_get(configuration: &configuration::Configuration, name: Option<&str>, playerid: Option<i32>, platform: Option<models::Bf2Platform>) -> Result<models::Bf2VehicleStats, Error<Bf2vehiclesBf2VehiclesGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_playerid = playerid;
    let p_platform = platform;

    let uri_str = format!("{}/bf2/vehicles/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_playerid {
        req_builder = req_builder.query(&[("playerid", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Bf2VehicleStats`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Bf2VehicleStats`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<Bf2vehiclesBf2VehiclesGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn bf2weapons_bf2_weapons_get(configuration: &configuration::Configuration, name: Option<&str>, playerid: Option<i32>, platform: Option<models::Bf2Platform>) -> Result<models::Bf2WeaponStats, Error<Bf2weaponsBf2WeaponsGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_playerid = playerid;
    let p_platform = platform;

    let uri_str = format!("{}/bf2/weapons/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    if let Some(ref param_value) = p_name {
        req_builder = req_builder.query(&[("name", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_playerid {
        req_builder = req_builder.query(&[("playerid", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `models::Bf2WeaponStats`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `models::Bf2WeaponStats`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<Bf2weaponsBf2WeaponsGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

pub async fn logged_server_data_bf2_serverarray_get(configuration: &configuration::Configuration, name: &str, r#type: Option<models::LegacyBfQueryType>, service: Option<models::Bf2Platform>, days: Option<i32>) -> Result<serde_json::Value, Error<LoggedServerDataBf2ServerarrayGetError>> {
    // add a prefix to parameters to efficiently prevent name collisions
    let p_name = name;
    let p_type = r#type;
    let p_service = service;
    let p_days = days;

    let uri_str = format!("{}/bf2/serverarray/", configuration.base_path);
    let mut req_builder = configuration.client.request(reqwest::Method::GET, &uri_str);

    req_builder = req_builder.query(&[("name", &p_name.to_string())]);
    if let Some(ref param_value) = p_type {
        req_builder = req_builder.query(&[("type", &param_value.to_string())]);
    }
    if let Some(ref param_value) = p_service {
        req_builder = req_builder.query(&[("service", &param_value.to_string())]);
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
            ContentType::Text => return Err(Error::from(serde_json::Error::custom("Received `text/plain` content type response that cannot be converted to `serde_json::Value`"))),
            ContentType::Unsupported(unknown_type) => return Err(Error::from(serde_json::Error::custom(format!("Received `{unknown_type}` content type response that cannot be converted to `serde_json::Value`")))),
        }
    } else {
        let content = resp.text().await?;
        let entity: Option<LoggedServerDataBf2ServerarrayGetError> = serde_json::from_str(&content).ok();
        Err(Error::ResponseError(ResponseContent { status, content, entity }))
    }
}

