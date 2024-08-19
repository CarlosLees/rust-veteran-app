use bson::oid::ObjectId;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Company {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    pub name: String,

    pub address: String,

    pub server_config_id: String,

    #[serde(with = "bson::serde_helpers::chrono_datetime_as_bson_datetime")]
    pub add_time: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompanyListResponse {
    pub id: String,

    pub name: String,

    pub address: String,

    pub server_config_id: String,

    pub add_time: String,
}

impl From<Company> for CompanyListResponse {
    fn from(company: Company) -> Self {
        Self {
            id: match company.id {
                Some(id) => id.to_hex(),
                None => String::from(""),
            },
            name: company.name,
            address: company.address,
            server_config_id: company.server_config_id,
            add_time: company.add_time.format("%Y-%m-%d %H:%M:%S").to_string(),
        }
    }
}
