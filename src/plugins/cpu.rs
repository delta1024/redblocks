use crate::Update;
use sysinfo::{ProcessorExt, System, SystemExt};
use std::fmt;
/**# Cpu Monitor
## Example
```no_run
#[macro_use]
extern crate redblocks;

use redblocks::{Widget, plugins::CpuPlugin};

fn main() {
    let bar = vec![
        Widget::new_mili(CpuPlugin::new(), 750),
    ];

    start_bar!(bar);
}
```
*/
pub struct CpuPlugin {
    pub used: System,
}

impl CpuPlugin {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            used: System::new(),
        })
    }
}

impl fmt::Display for CpuPlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}%",
            (self.used.get_global_processor_info().get_cpu_usage() as f64 * 100_f64).round()
                / 100_f64
        )
    }
}

impl Update for CpuPlugin {
    fn refresh(&mut self) {
        self.used.refresh_cpu();
    }
}
