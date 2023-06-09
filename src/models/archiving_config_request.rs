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
pub struct ArchivingConfigRequest {
    /// Defines the type of notification channel. Valid values are: `email`, `webhook`, `slack`, and `pagerduty`
    #[serde(rename = "integration", skip_serializing_if = "Option::is_none")]
    pub integration: Option<String>,
    /// Name of the bucket.
    #[serde(rename = "bucket", skip_serializing_if = "Option::is_none")]
    pub bucket: Option<String>,
    /// Private endpoint associated with the bucket.
    #[serde(rename = "endpoint", skip_serializing_if = "Option::is_none")]
    pub endpoint: Option<String>,
    /// API key that is needed to authenticate with the Cloud Object Storage service.
    #[serde(rename = "apikey", skip_serializing_if = "Option::is_none")]
    pub apikey: Option<String>,
    /// CRN of the Cloud Object Storage instance ID where the bucket is available.
    #[serde(rename = "resourceinstanceid", skip_serializing_if = "Option::is_none")]
    pub resourceinstanceid: Option<String>,
    /// Account name
    #[serde(rename = "accountname", skip_serializing_if = "Option::is_none")]
    pub accountname: Option<String>,
    /// Account key
    #[serde(rename = "accountkey", skip_serializing_if = "Option::is_none")]
    pub accountkey: Option<String>,
    /// Project ID
    #[serde(rename = "projectid", skip_serializing_if = "Option::is_none")]
    pub projectid: Option<String>,
    /// Space
    #[serde(rename = "space", skip_serializing_if = "Option::is_none")]
    pub space: Option<String>,
    /// Access key
    #[serde(rename = "accesskey", skip_serializing_if = "Option::is_none")]
    pub accesskey: Option<String>,
    /// Secret key
    #[serde(rename = "secretkey", skip_serializing_if = "Option::is_none")]
    pub secretkey: Option<String>,
    /// Auth URL
    #[serde(rename = "authurl", skip_serializing_if = "Option::is_none")]
    pub authurl: Option<String>,
    /// Expires
    #[serde(rename = "expires", skip_serializing_if = "Option::is_none")]
    pub expires: Option<String>,
    /// Username
    #[serde(rename = "username", skip_serializing_if = "Option::is_none")]
    pub username: Option<String>,
    /// Password provided in credentials for authorization
    #[serde(rename = "password", skip_serializing_if = "Option::is_none")]
    pub password: Option<String>,
    /// Tenant name
    #[serde(rename = "tenantname", skip_serializing_if = "Option::is_none")]
    pub tenantname: Option<String>,
}

impl ArchivingConfigRequest {
    pub fn new() -> ArchivingConfigRequest {
        ArchivingConfigRequest {
            integration: None,
            bucket: None,
            endpoint: None,
            apikey: None,
            resourceinstanceid: None,
            accountname: None,
            accountkey: None,
            projectid: None,
            space: None,
            accesskey: None,
            secretkey: None,
            authurl: None,
            expires: None,
            username: None,
            password: None,
            tenantname: None,
        }
    }
}


