/*!# Prints Ram usage
## Example
```no_run
#[macro_use]
extern crate redblocks;

use redblocks::{Widget, mem::MemoryUsage};

fn main() {
    let bar = vec![Widget::new(
        MemoryUsage::new(), 1
    )];

    start_bar!(bar);
}
```
*/

use crate::Update;
use std::fmt;
use sysinfo::{System, SystemExt};

pub struct MemoryUsage {
    mem: System,
}

impl MemoryUsage {
    pub fn new() -> Box<Self> {
        Box::new(MemoryUsage { mem: System::new() })
    }
}
impl fmt::Display for MemoryUsage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let usage = self.mem.get_used_memory() as f64;
        let count = usage.to_string().chars().count();
        if count < 4 {
            write!(f, "{} Kb", usage)
        } else if count < 7 {
            let usage = ((usage / 1000_f64) * 100_f64).round() / 100_f64;
            write!(f, "{} Mb", usage)
        } else {
            let usage = ((usage / 1000000_f64) * 100_f64).round() / 100_f64;
            write!(f, "{} Gb", usage)
        }
    }
}

impl Update for MemoryUsage {
    fn refresh(&mut self) {
        self.mem.refresh_memory()
    }
}
