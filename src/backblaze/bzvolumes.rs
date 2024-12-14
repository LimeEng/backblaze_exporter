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
    #[serde(rename = "@numBytesUsedOnVolume")]
    pub num_bytes_used_on_volume: u64,
    #[serde(rename = "@numBytesTotalOnVolume")]
    pub num_bytes_total_on_volume: u64,
}
