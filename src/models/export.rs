#[derive(Debug, Serialize, Deserialize)]
pub struct LogLine {
    pub __key: String,
    pub _account: Option<String>,
    pub _app: Option<String>,
    pub _bid: Option<String>,
    pub _cluster: Option<String>,
    pub _file: Option<String>,
    pub _host: Option<String>,
    pub _id: String,
    pub _ingester: Option<String>,
    pub _ip: Option<String>,
    pub _ipremote: Option<String>,
    pub _line: Option<String>,
    pub _logtype: Option<String>,
    pub _mac: Option<String>,
    pub _mezmo_line_size: Option<u32>,
    pub _rawline: Option<String>,
    pub _ts: u64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LogExport {
    pub lines: Vec<LogLine>,
    pub pagination_id: Option<String>,
}
