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
pub struct StoreOffer {
    #[serde(rename = "Identifier")]
    pub identifier: Box<models::Identifier>,
    #[serde(rename = "title")]
    pub title: String,
    #[serde(rename = "readableTitle")]
    pub readable_title: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "readableDescription")]
    pub readable_description: String,
    #[serde(rename = "owned")]
    pub owned: bool,
    #[serde(rename = "art")]
    pub art: Box<models::OfferArt>,
    #[serde(rename = "order")]
    pub order: Box<models::OfferPrice>,
    #[serde(rename = "isNew")]
    pub is_new: bool,
}

impl StoreOffer {
    pub fn new(identifier: models::Identifier, title: String, readable_title: String, description: String, readable_description: String, owned: bool, art: models::OfferArt, order: models::OfferPrice, is_new: bool) -> StoreOffer {
        StoreOffer {
            identifier: Box::new(identifier),
            title,
            readable_title,
            description,
            readable_description,
            owned,
            art: Box::new(art),
            order: Box::new(order),
            is_new,
        }
    }
}

