use crate::Update;
use battery::units::time::minute;
use battery::{
    units::{ratio::percent, thermodynamic_temperature::degree_celsius},
    Battery, Manager,
};
use std::fmt;

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
#[derive(Default)]
pub struct BatPlugin {
    manager: Vec<Manager>,
    batteries: Vec<Battery>,
    format: BatOut,
    display: String,
}

enum BatOut {
    Celsius,
    Percent,
    Time,
}
impl Default for BatOut {
    fn default() -> Self {
        BatOut::Percent
    }
}
impl BatPlugin {
    fn new() -> Self {
        let manager = Manager::new().unwrap();
        let batteries = manager.batteries().unwrap();
        let batteries = {
            let mut vec = Vec::new();
            for i in batteries {
                vec.push(i.unwrap());
            }
            vec
        };
        BatPlugin {
            manager: vec![manager],
            batteries,
            ..BatPlugin::default()
        }
    }
    pub fn new_percent() -> Box<BatPlugin> {
        Box::new(BatPlugin::new())
    }
    /// may not work on all systems (or it's bugged)
    pub fn new_celsius() -> Box<BatPlugin> {
        let mut bat_plug = BatPlugin::new();
        bat_plug.format = BatOut::Celsius;
        Box::new(bat_plug)
    }

    pub fn new_time() -> Box<BatPlugin> {
        let mut bat_plug = BatPlugin::new();
        bat_plug.format = BatOut::Time;
        Box::new(bat_plug)
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
            self.manager[0].refresh(i).unwrap();
        }
        let format = match self.format {
            BatOut::Percent => self.update_percent(),
            BatOut::Celsius => self.update_celcius(),
            BatOut::Time => self.update_time(),
        };
        self.display = format;
    }
}

/// Extra functions for [BatPlugin].
trait BatExt {
    /// returns the remaining battery life as a percent
    fn update_percent(&self) -> String;
    /// returns the remaining time in hours as a floating point
    fn update_time(&self) -> String;
    /// returns the temperature of the battery
    fn update_celcius(&self) -> String;

    /// fomates time to full
    fn time_to_empty(batteries: &Vec<Battery>) -> String {
        let mut string = String::new();

        for i in batteries {
            if let Some(value) = i.time_to_empty() {
                let duration = value.get::<minute>();
                if duration > 60.0 {
                    let duration = (duration.round() / 60_f32).round();
                    string.push_str(format!("{}h", duration).as_str());
                } else {
                    let duration = duration.round();

                    string.push_str(format!("{}m", duration).as_str());
                }
            }
        }
        string
    }

    /// fomates time to empty
    fn time_to_full(batteries: &Vec<Battery>) -> String {
        let mut string = String::new();

        for i in batteries {
            if let Some(value) = i.time_to_full() {
                let duration = value.get::<minute>();

                if duration > 60.0 {
                    let duration = (duration.round() / 60_f32).round();
                    string.push_str(format!("{}h (Charging)", duration).as_str());
                } else {
                    let duration = duration.round();

                    string.push_str(format!("{}m (Charging)", duration).as_str());
                }
            }
        }

        string
    }
}


impl BatExt for BatPlugin {
    fn update_percent(&self) -> String {
        let mut string = String::new();
        for i in &self.batteries {
            string.push_str(format!("{}%", i.state_of_charge().get::<percent>().round()).as_str());
        }
        string
    }

    fn update_time(&self) -> String {
        match self.batteries[0].state() {
            battery::State::Discharging => BatPlugin::time_to_empty(&self.batteries),
            battery::State::Charging => BatPlugin::time_to_full(&self.batteries),
            battery::State::Empty => String::from("empty"),
            battery::State::Full => String::from("Full"),
            battery::State::__Nonexhaustive => String::from("nonexhastive"),
            battery::State::Unknown => String::from("Unknown"),
        }
    }

    fn update_celcius(&self) -> String {
        let mut string = String::new();

        for i in &self.batteries {
            if let Some(value) = i.temperature() {
                string.push_str(format!("{} °C", value.get::<degree_celsius>()).as_str());
            }
        }
        string
    }
}
