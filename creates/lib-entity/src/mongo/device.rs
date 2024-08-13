use bson::oid::ObjectId;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Device {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    pub id: Option<ObjectId>,

    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,

    pub imei: String,

    pub server_config_id: String,

    pub company_id: String,

    pub company_name: Option<String>,
}
