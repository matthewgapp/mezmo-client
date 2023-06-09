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
pub struct ListUsageTagV2200ResponseResultsInner {
    /// The name of the tag.
    #[serde(rename = "name", skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    /// The number of bytes of disk the tag consumes over the time range.
    #[serde(rename = "total_bytes", skip_serializing_if = "Option::is_none")]
    pub total_bytes: Option<i32>,
}

impl ListUsageTagV2200ResponseResultsInner {
    pub fn new() -> ListUsageTagV2200ResponseResultsInner {
        ListUsageTagV2200ResponseResultsInner {
            name: None,
            total_bytes: None,
        }
    }
}


