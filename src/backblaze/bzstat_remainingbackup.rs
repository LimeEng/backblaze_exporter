use serde::Deserialize;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Contents {
    #[serde(rename = "bzvolume")]
    pub bz_volumes: Vec<BzVolume>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BzVolume {
    #[serde(rename = "@bzVolumeGuid")]
    pub bz_volume_guid: String,
    #[serde(rename = "@pervol_remaining_files_numfiles")]
    pub pervol_remaining_files_numfiles: u64,
    #[serde(rename = "@pervol_remaining_files_numbytes")]
    pub pervol_remaining_files_numbytes: u64,
}
