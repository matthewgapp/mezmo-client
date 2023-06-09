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
pub struct Channel {
    /// Defines the type of notification channel. Valid values are: `email`, `webhook`, `slack`, and `pagerduty`
    #[serde(rename = "integration", skip_serializing_if = "Option::is_none")]
    pub integration: Option<String>,
    #[serde(rename = "emails", skip_serializing_if = "Option::is_none")]
    pub emails: Option<Vec<String>>,
    /// Webhook URL.
    #[serde(rename = "url", skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
    /// PagerDuty key.
    #[serde(rename = "key", skip_serializing_if = "Option::is_none")]
    pub key: Option<String>,
    /// Specify the number of log lines that match the view's filtering and search criteria. When the number of log lines is reached, an alert is triggered.
    #[serde(rename = "triggerlimit", skip_serializing_if = "Option::is_none")]
    pub triggerlimit: Option<String>,
    /// Specify how often to trigger an alert. Valid values are: 30 seconds, 1 minute, 5 minutes, 15 minutes, 30 minutes, 1 hour, 6 hours, 12 hours, 24 hours
    #[serde(rename = "triggerinterval", skip_serializing_if = "Option::is_none")]
    pub triggerinterval: Option<String>,
    /// Set to true if you want the trigger condition to be evaluated as soon as the triggerlimit is reached.
    #[serde(rename = "immediate", skip_serializing_if = "Option::is_none")]
    pub immediate: Option<bool>,
    /// Set to true if you want the trigger condition to be evaluated after the time that you specify in the triggerinterval field is reached.
    #[serde(rename = "terminal", skip_serializing_if = "Option::is_none")]
    pub terminal: Option<bool>,
    /// Type of alert. Valid values are: `presence`and `absence`.
    #[serde(rename = "operator", skip_serializing_if = "Option::is_none")]
    pub operator: Option<String>,
    /// Timezone used to report timestamps. This can be any value defined in the tz database https://en.wikipedia.org/wiki/List_of_tz_database_time_zones.
    #[serde(rename = "timezone", skip_serializing_if = "Option::is_none")]
    pub timezone: Option<String>,
    #[serde(rename = "method", skip_serializing_if = "Option::is_none")]
    pub method: Option<crate::models::Method>,
    #[serde(rename = "headers", skip_serializing_if = "Option::is_none")]
    pub headers: Option<Box<crate::models::Headers>>,
    #[serde(rename = "bodyTemplate", skip_serializing_if = "Option::is_none")]
    pub body_template: Option<Box<crate::models::BodyTemplate>>,
}

impl Channel {
    pub fn new() -> Channel {
        Channel {
            integration: None,
            emails: None,
            url: None,
            key: None,
            triggerlimit: None,
            triggerinterval: None,
            immediate: None,
            terminal: None,
            operator: None,
            timezone: None,
            method: None,
            headers: None,
            body_template: None,
        }
    }
}


