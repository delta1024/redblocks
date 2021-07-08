use std::fmt;
use crate::Update;
use chrono::{DateTime, Local};
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
