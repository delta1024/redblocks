/*! Library provided plugins
# Building your own plugins

First you will need to create a struct to hold the information you wish displayed in the status blocks. When implementing the plugin's new() function it is importatn that it return itself in a [`Box`]. Once you have created your status plugin you will need to implement both the [`std::fmt::Display`] and [Update](crate::Update) traits; the implementation of which can be found below.


## Example Plugin
For the following example we are going to be creating a simple widget that couts how many seconds the status blocks have been running.
```no_run

use redblocks::Update;

use std::fmt::{self, Display};

struct Counter(u64);

impl Counter {

    fn new() -> Box<Self> {
        Box::new(Self(0))
    }

}

impl Display for Counter {

    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Update for Counter {

    fn refresh(&mut self) {
        self.0 = self.0 + 1;
    }
}

```
*/
use crate::Update;
use battery::{
    units::{ratio::percent, thermodynamic_temperature::degree_celsius},
    Battery, Manager,
};
use chrono::{DateTime, Local};
use std::fmt;
use sysinfo::{ProcessorExt, System, SystemExt};

#[doc(inline)]
pub use chrono::format::strftime;

/** # Battery Monitor
displays in percent
```no_run
#[macro_use]
extern crate redblocks;

use redblocks::{Widget, plugins::BatPlugin};

fn main() {
    let widgets = vec![
        Widget::new(BatPlugin::new_percent(), 1),
    ];

    start_bar!(widgets);
}

```
*/
pub struct BatPlugin {
    manager: Manager,
    batteries: Vec<Battery>,
    format: BatOut,
    display: String,
}

enum BatOut {
    Celsius,
    Percent,
}
impl BatPlugin {
    pub fn new_percent() -> Box<BatPlugin> {
        let manager = Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        let batteries = {
            let mut vec = Vec::new();
            for i in batteries {
                vec.push(i.unwrap());
            }
            vec
        };
        Box::new(BatPlugin {
            manager,
            batteries,
            format: BatOut::Percent,
            display: String::new(),
        })
    }
    pub fn new_celsius() -> Box<BatPlugin> {
        let manager = Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        let batteries = {
            let mut vec = Vec::new();
            for i in batteries {
                vec.push(i.unwrap());
            }
            vec
        };
        Box::new(BatPlugin {
            manager,
            batteries,
            format: BatOut::Celsius,
            display: String::new(),
        })
    }
}

impl fmt::Display for BatPlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.display)
    }
}

impl Update for BatPlugin {
    fn refresh(&mut self) {
        for i in &mut self.batteries {
            self.manager.refresh(i).unwrap();
        }
        let format = match self.format {
            BatOut::Percent => {
                let mut string = String::new();
                for i in &self.batteries {
                    string.push_str(format!("{} %", i.state_of_charge().get::<percent>()).as_str());
                }
                string
            }
            BatOut::Celsius => {
                let mut string = String::new();

                for i in &self.batteries {
                    if let Some(value) = i.temperature() {
                        string.push_str(format!("{} °C", value.get::<degree_celsius>()).as_str());
                    }
                }
                string
            }
        };
        self.display = format;
    }
}
/** # Cpu Monitor
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
            "Cpu: {}%",
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

/**# Prints Ram usage
## Example
```no_run
#[macro_use]
extern crate redblocks;

use redblocks::{Widget, plugins::MemPlugin};

fn main() {
    let bar = vec![Widget::new(
        MemPlugin::new(), 1
    )];

    start_bar!(bar);
}
```
*/
pub struct MemPlugin {
    mem: System,
}

impl MemPlugin {
    pub fn new() -> Box<Self> {
        Box::new(MemPlugin { mem: System::new() })
    }
}

impl fmt::Display for MemPlugin {
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

impl Update for MemPlugin {
    fn refresh(&mut self) {
        self.mem.refresh_memory()
    }
}

/** Displays the current time.



# Using the default configuration
formats to "Saturday 06/26/2021 3:41:48 pm"
```no_run
#[macro_use]
extern crate redblocks;

use redblocks::{plugins::TimePlugin, Widget};

fn main() {
    let time = Box::new(TimePlugin::default());
    let time = vec![Widget::new(time, 1)];

    start_bar!(time);
}
```

# Using a custom format
for formating options see the [strftime](chrono::format::strftime) module
```no_run
#[macro_use]
extern crate redblocks;

use redblocks::{plugins::TimePlugin, Widget};

fn main() {
    let time = TimePlugin::new("%H%M%S"); // formats to "14:23:08"
    let time = vec![Widget::new(time, 1)];

    start_bar!(time);
}
```
*/
pub struct TimePlugin {
    /// holds the time formated as a string
    pub time: String,
    /// holds the formating string
    pub format: String,
}

impl TimePlugin {
    pub fn new(format: &str) -> Box<Self> {
        let time: DateTime<Local> = Local::now();
        let time = format!("{}", time.format(&format));
        Box::new(Self {
            time,
            format: format.to_string(),
        })
    }
}

impl fmt::Display for TimePlugin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.time)
    }
}

impl Update for TimePlugin {
    fn refresh(&mut self) {
        let dt: DateTime<Local> = Local::now();
        self.time = format!("{}", dt.format(&self.format));
    }
}

impl Default for TimePlugin {
    fn default() -> TimePlugin {
        let dt: DateTime<Local> = Local::now();
        let format = "%A %D %_I:%M:%S %P".to_string();
        TimePlugin {
            time: format!("{}", dt.format(&format)),
            format,
        }
    }
}
