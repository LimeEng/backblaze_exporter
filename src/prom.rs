use crate::Disk;
use prometheus_client::{
    encoding::{text, EncodeLabelSet},
    metrics::{family::Family, gauge::Gauge},
    registry::Registry,
};

#[must_use]
pub fn encode(disks: &[Disk]) -> String {
    let mut registry = <Registry>::default();
    let metrics = DiskMetrics::new(&mut registry);
    metrics.update(disks);

    let mut buffer = String::new();
    text::encode(&mut buffer, &registry).unwrap();

    buffer
}

#[derive(Clone, Hash, PartialEq, Eq, Debug, EncodeLabelSet)]
pub struct DiskLabels {
    mountpoint: String,
}

struct DiskMetrics {
    total_bytes: Family<DiskLabels, Gauge>,
    used_bytes: Family<DiskLabels, Gauge>,
    remaining_files: Family<DiskLabels, Gauge>,
    remaining_bytes: Family<DiskLabels, Gauge>,
    selected_files: Family<DiskLabels, Gauge>,
    selected_bytes: Family<DiskLabels, Gauge>,
}

impl DiskMetrics {
    fn new(registry: &mut Registry) -> Self {
        let metrics = Self {
            total_bytes: Family::default(),
            used_bytes: Family::default(),
            remaining_files: Family::default(),
            remaining_bytes: Family::default(),
            selected_files: Family::default(),
            selected_bytes: Family::default(),
        };
        metrics.register(registry);
        metrics
    }

    fn register(&self, registry: &mut Registry) {
        registry.register(
            "backblaze_disk_total_bytes",
            "Total bytes available on disk",
            self.total_bytes.clone(),
        );
        registry.register(
            "backblaze_disk_used_bytes",
            "Bytes used on the disk",
            self.used_bytes.clone(),
        );
        registry.register(
            "backblaze_disk_remaining_files",
            "Number of remaining files to upload on disk",
            self.remaining_files.clone(),
        );
        registry.register(
            "backblaze_disk_remaining_bytes",
            "Bytes remaining to upload on disk",
            self.remaining_bytes.clone(),
        );
        registry.register(
            "backblaze_disk_selected_files",
            "Number of files selected for backup",
            self.selected_files.clone(),
        );
        registry.register(
            "backblaze_disk_selected_bytes",
            "Bytes selected for backup",
            self.selected_bytes.clone(),
        );
    }

    fn update(&self, disks: &[Disk]) {
        for disk in disks {
            // : and / have special meaning in Prometheus, so these need to be encoded
            let labels = DiskLabels {
                mountpoint: url::form_urlencoded::byte_serialize(
                    disk.mountpoint.clone().as_bytes(),
                )
                .collect(),
            };

            self.total_bytes
                .get_or_create(&labels)
                .set(disk.total_bytes as i64);
            self.used_bytes
                .get_or_create(&labels)
                .set(disk.used_bytes as i64);
            self.remaining_files
                .get_or_create(&labels)
                .set(disk.remaining_files as i64);
            self.remaining_bytes
                .get_or_create(&labels)
                .set(disk.remaining_bytes as i64);
            self.selected_files
                .get_or_create(&labels)
                .set(disk.selected_files as i64);
            self.selected_bytes
                .get_or_create(&labels)
                .set(disk.selected_bytes as i64);
        }
    }
}
