/*!# Cpu Monitor
## Example
```no_run
#[macro_use]
extern crate redblocks;

use redblocks::{Widget, cpu::Cpu};

fn main() {
    let bar = vec![
        Widget::new_mili(Cpu::new(), 750),
    ];

    start_bar!(bar);
}
```
*/

use crate::Update;
use std::fmt;
use sysinfo::{ProcessorExt, System, SystemExt};

pub struct Cpu {
    pub used: System,
}

impl Cpu {
    pub fn new() -> Box<Self> {
        Box::new(Self {
            used: System::new(),
        })
    }
}

impl fmt::Display for Cpu {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Cpu: {}%",
            (self.used.get_global_processor_info().get_cpu_usage() as f64 * 100_f64).round()
                / 100_f64
        )
    }
}

impl Update for Cpu {
    fn refresh(&mut self) {
        self.used.refresh_cpu();
    }
}
