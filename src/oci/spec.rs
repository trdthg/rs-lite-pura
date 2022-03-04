use super::Linux;
use crate::error::{Error, Result};
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::Path};
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub oci_version: String,
    pub root: Root,
    pub mounts: Option<Vec<Mount>>,
    pub process: Option<Process>,
    pub hostname: Option<String>,
    pub hooks: Option<Hooks>,
    pub linux: Option<Linux>,
    pub annotations: Option<HashMap<String, String>>,
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

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Process {
    pub terminal: Option<bool>,
    pub user: Option<User>,
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub cwd: String,
    pub capabilities: Option<Capabilities>,
    pub rlimits: Option<Vec<Rlimit>>,
    pub apparmor_profile: Option<String>,
    pub oom_score_adj: Option<i64>,
    pub selinux_label: Option<String>,
    pub no_new_privileges: Option<bool>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub uid: i64,
    pub gid: i64,
    pub additional_gids: Option<Vec<i64>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Capabilities {
    pub bounding: Option<Vec<String>>,
    pub permitted: Option<Vec<String>>,
    pub inheritable: Option<Vec<String>>,
    pub effective: Option<Vec<String>>,
    pub ambient: Option<Vec<String>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rlimit {
    #[serde(rename = "type")]
    pub type_field: String,
    pub hard: i64,
    pub soft: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub path: String,
    pub readonly: Option<bool>,
}

impl TryFrom<&Path> for Spec {
    type Error = Error;

    fn try_from(path: &Path) -> Result<Self, Self::Error> {
        let spec_json = std::fs::read_to_string(path)?;
        let state: Spec = serde_json::from_str(&spec_json)?;
        Ok(state)
    }
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hooks {
    pub prestart: Option<Vec<Hook>>,
    pub create_runtime: Option<Vec<Hook>>,
    pub create_container: Option<Vec<Hook>>,
    pub start_container: Option<Vec<Hook>>,
    pub poststart: Option<Vec<Hook>>,
    pub poststop: Option<Vec<Hook>>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hook {
    pub path: String,
    pub args: Option<Vec<String>>,
    pub env: Option<Vec<String>>,
    pub timeout: Option<i64>,
}
