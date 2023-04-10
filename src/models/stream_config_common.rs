/*
 * Log Analysis API
 *
 * *REST API to send log data, export data and manage configuration*  To utilize the inline testing functionality here, please go into your Mezmo account and add the following address to your CORS Origins page: `https://docs.mezmo.com`. 
 *
 * The version of the OpenAPI document: 2.1
 * 
 * Generated by: https://openapi-generator.tech
 */




#[derive(Clone, Debug, PartialEq, Default, Serialize, Deserialize)]
pub struct StreamConfigCommon {
    /// List of URLs referencing Kafka broker connections via SASL_SSL
    #[serde(rename = "brokers")]
    pub brokers: Vec<String>,
    /// Name of the relevant Kafka topic category storing log records
    #[serde(rename = "topic")]
    pub topic: String,
    /// Username in provided credentials to authorize access to brokers
    #[serde(rename = "user")]
    pub user: String,
    /// Enumerated (binary) string representing whether or not the stream config is active
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<Status>,
}

impl StreamConfigCommon {
    pub fn new(brokers: Vec<String>, topic: String, user: String) -> StreamConfigCommon {
        StreamConfigCommon {
            brokers,
            topic,
            user,
            status: None,
        }
    }
}

/// Enumerated (binary) string representing whether or not the stream config is active
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash, Serialize, Deserialize)]
pub enum Status {
    #[serde(rename = "active")]
    Active,
    #[serde(rename = "inactive")]
    Inactive,
}

impl Default for Status {
    fn default() -> Status {
        Self::Active
    }
}

