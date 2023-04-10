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
pub struct IndexRateAlertConfigResponseChannels {
    #[serde(rename = "email", skip_serializing_if = "Option::is_none")]
    pub email: Option<Vec<String>>,
    #[serde(rename = "pagerduty", skip_serializing_if = "Option::is_none")]
    pub pagerduty: Option<Vec<String>>,
    #[serde(rename = "slack", skip_serializing_if = "Option::is_none")]
    pub slack: Option<Vec<String>>,
    #[serde(rename = "webhook", skip_serializing_if = "Option::is_none")]
    pub webhook: Option<Vec<crate::models::IndexRateAlertWebhookConfigResponse>>,
}

impl IndexRateAlertConfigResponseChannels {
    pub fn new() -> IndexRateAlertConfigResponseChannels {
        IndexRateAlertConfigResponseChannels {
            email: None,
            pagerduty: None,
            slack: None,
            webhook: None,
        }
    }
}


