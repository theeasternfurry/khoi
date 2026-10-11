use std::path::Path;

use sysinfo::{Disks, System};

const GIB: f64 = 1_073_741_824.0;

#[derive(Debug, Clone, Copy)]
pub struct Usage {
    pub used_gib: f64,
    pub total_gib: f64,
}

impl Usage {
    fn from_bytes(used: u64, total: u64) -> Self {
        Self {
            used_gib: used as f64 / GIB,
            total_gib: total as f64 / GIB,
        }
    }

    pub fn percent(&self) -> f64 {
        if self.total_gib > 0.0 {
            self.used_gib / self.total_gib * 100.0
        } else {
            0.0
        }
    }
}

pub struct Model {
    system: System,
    disks: Disks,
}

impl Model {
    pub fn new() -> Self {
        let mut system = System::new();
        system.refresh_cpu_usage();
        system.refresh_memory();

        Self {
            system,
            disks: Disks::new_with_refreshed_list(),
        }
    }

    pub fn refresh(&mut self) {
        self.system.refresh_cpu_usage();
        self.system.refresh_memory();
        self.disks.refresh(true);
    }

    pub fn cpu_percent(&self) -> f32 {
        self.system.global_cpu_usage()
    }

    pub fn memory(&self) -> Usage {
        Usage::from_bytes(self.system.used_memory(), self.system.total_memory())
    }

    pub fn swap(&self) -> Usage {
        Usage::from_bytes(self.system.used_swap(), self.system.total_swap())
    }

    pub fn disk(&self) -> Option<Usage> {
        let list = self.disks.list();
        let disk = list
            .iter()
            .find(|disk| disk.mount_point() == Path::new("/"))
            .or_else(|| list.first())?;

        let total = disk.total_space();
        let used = total.saturating_sub(disk.available_space());

        Some(Usage::from_bytes(used, total))
    }
}
