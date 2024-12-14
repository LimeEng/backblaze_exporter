use serde::{Deserialize, Serialize};

pub mod backblaze;
pub mod prom;

#[derive(Debug, Serialize, Deserialize)]
pub struct Disk {
    pub mountpoint: String,
    pub total_bytes: u64,
    pub used_bytes: u64,
    pub remaining_files: u64,
    pub remaining_bytes: u64,
    pub selected_files: u64,
    pub selected_bytes: u64,
}
