use std::{
    collections::HashMap,
    fs::read_to_string,
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Status {
    Creating,
    Created,
    Running,
    Stopped,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct State {
    pub oci_version: String,
    pub id: String,
    pub status: Status,
    pub pid: u64,
    pub bundle: PathBuf,
    pub annotations: Option<HashMap<String, String>>,
}

impl State {
    pub fn new(id: &String, pid: u64, bundle: &String) -> Self {
        Self {
            oci_version: "1.0.2".to_string(),
            id: id.clone(),
            status: Status::Creating,
            pid,
            bundle: PathBuf::from(bundle),
            annotations: Some(HashMap::new()),
        }
    }

    pub fn save(&self, root_path: &Path) -> crate::Result<()> {
        let mut state_file = std::fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(root_path.join("state.json"))
            .map_err(|e| {
                crate::Error::StringError(format!(
                    "can not open {root_path:?} when trying to save state: {e:?}"
                ))
            })?;
        state_file
            .write_all(
                &serde_json::to_string(self)
                    .map_err(|e| {
                        crate::Error::StringError(format!(
                            "deseralize from struct State failed: {e:?}"
                        ))
                    })?
                    .as_bytes(),
            )
            .map_err(|e| {
                crate::Error::StringError(format!("can not write to state.json: {e:?}"))
            })?;
        Ok(())
    }
}

impl TryFrom<&Path> for State {
    type Error = crate::Error;

    fn try_from(value: &Path) -> crate::Result<Self, Self::Error> {
        let state_file_path = value.join("state.json");
        let state_json_str = read_to_string(state_file_path).map_err(|e| {
            crate::Error::StringError(format!("state file not found: {state_file_path:?}"))
        })?;
        let state_json = serde_json::from_str(&state_json_str).map_err(|e| {
            crate::Error::StringError(format!(
                "deseralize from state.json failed: {state_json_str}"
            ))
        })?;
        Ok(state_json)
    }
}
