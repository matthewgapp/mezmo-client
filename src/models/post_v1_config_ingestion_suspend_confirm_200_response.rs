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
pub struct PostV1ConfigIngestionSuspendConfirm200Response {
    #[serde(rename = "status", skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

impl PostV1ConfigIngestionSuspendConfirm200Response {
    pub fn new() -> PostV1ConfigIngestionSuspendConfirm200Response {
        PostV1ConfigIngestionSuspendConfirm200Response {
            status: None,
        }
    }
}

