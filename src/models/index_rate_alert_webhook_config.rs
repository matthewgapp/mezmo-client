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
pub struct IndexRateAlertWebhookConfig {
    /// Webhook URL.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::models::Method>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Box<crate::models::Headers>>,
    #[serde(rename = "bodyTemplate", skip_serializing_if = "Option::is_none")]
    pub body_template: Option<Box<crate::models::BodyTemplate>>,
}

impl IndexRateAlertWebhookConfig {
    pub fn new() -> IndexRateAlertWebhookConfig {
        IndexRateAlertWebhookConfig {
            url: None,
            method: None,
            headers: None,
            body_template: None,
        }
    }
}

