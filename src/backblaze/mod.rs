use crate::Disk;
use bzinfo::BzInfo;
use serde::de::DeserializeOwned;
use std::{fs, io, path::Path};

mod bzinfo;
mod bzstat_remainingbackup;
mod bzstat_totalbackup;
mod bzvolumes;

pub fn parse_xml_file<T, P>(file_path: P) -> io::Result<T>
where
    T: DeserializeOwned + std::fmt::Debug,
    P: AsRef<Path>,
{
    let xml = fs::read_to_string(file_path)?;
    let result: T = quick_xml::de::from_str(&xml).map_err(|e| {
        io::Error::new(
            io::ErrorKind::InvalidData,
            format!("Failed to parse XML: {e}"),
        )
    })?;
    Ok(result)
}

pub fn collect_metrics() -> io::Result<Vec<Disk>> {
    let prefix = "C:/ProgramData/Backblaze/bzdata";

    let bzinfo: BzInfo = parse_xml_file(Path::new(prefix).join("bzinfo.xml"))?;
    let bzstat_remaining: bzstat_remainingbackup::Contents = parse_xml_file(
        Path::new(prefix)
            .join("bzreports")
            .join("bzstat_remainingbackup.xml"),
    )?;
    let bzstat_total: bzstat_totalbackup::Contents = parse_xml_file(
        Path::new(prefix)
            .join("bzreports")
            .join("bzstat_totalbackup.xml"),
    )?;
    let bzvolumes_contents: bzvolumes::Contents =
        parse_xml_file(Path::new(prefix).join("bzvolumes.xml"))?;

    let disks = bzinfo
        .hard_drives_to_backup
        .bz_volumes
        .iter()
        .filter_map(|volume| {
            construct_disk(
                volume,
                &bzstat_remaining,
                &bzstat_total,
                &bzvolumes_contents,
            )
        })
        .collect();
    Ok(disks)
}

fn construct_disk(
    volume: &bzinfo::BzVolume,
    bzstat_contents: &bzstat_remainingbackup::Contents,
    bzstat_total: &bzstat_totalbackup::Contents,
    bzvolumes_contents: &bzvolumes::Contents,
) -> Option<Disk> {
    let (used_bytes, total_bytes) = {
        let volume = bzvolumes_contents
            .bz_volumes
            .iter()
            .find(|v| v.bz_volume_guid == volume.bz_volume_guid)?;

        (
            volume.num_bytes_used_on_volume,
            volume.num_bytes_total_on_volume,
        )
    };

    let (remaining_files, remaining_bytes) = {
        let volume = bzstat_contents
            .bz_volumes
            .iter()
            .find(|v| v.bz_volume_guid == volume.bz_volume_guid)?;

        (
            volume.pervol_remaining_files_numfiles,
            volume.pervol_remaining_files_numbytes,
        )
    };

    let (selected_files, selected_bytes) = {
        let volume = bzstat_total
            .bz_volumes
            .iter()
            .find(|v| v.bz_volume_guid == volume.bz_volume_guid)?;

        (
            volume.pervol_sel_for_backup_numfiles,
            volume.pervol_sel_for_backup_numbytes,
        )
    };

    Some(Disk {
        mountpoint: volume.mount_point_path.clone(),
        total_bytes,
        used_bytes,
        remaining_files,
        remaining_bytes,
        selected_files,
        selected_bytes,
    })
}
