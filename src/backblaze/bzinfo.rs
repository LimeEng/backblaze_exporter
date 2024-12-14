use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BzInfo {
    #[serde(rename = "hard_drives_to_backup")]
    pub hard_drives_to_backup: HardDrivesToBackup,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct HardDrivesToBackup {
    #[serde(rename = "bzvolume")]
    pub bz_volumes: Vec<BzVolume>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BzVolume {
    #[serde(rename = "@bzVolumeGuid")]
    pub bz_volume_guid: String,
    #[serde(rename = "@mountPointPath")]
    pub mount_point_path: String,
}
