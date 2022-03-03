use serde::{Deserialize, Serialize};

// 设备属性
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Device {
    pub path: String,
    #[serde(rename = "type")]
    pub device_type: String,
    pub major: u64,
    pub minor: u64,
    pub file_mode: Option<u32>,
    pub uid: Option<u32>,
    pub gid: Option<u32>,
}

// 除了root外的其他挂载点
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Mount {
    pub destination: String,
    #[serde(rename = "type")]
    pub mount_type: Option<String>,
    pub source: Option<String>,
    #[serde(default)]
    pub options: Option<Vec<String>>,
}
