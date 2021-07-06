/*!# About
Redblocks is a library inspired by dwmblocks for creating your own status blocks that writes to XROOTNAME?. Primaraly intended for along side the [Penrose] library.

# Dependencies 
* xsetroot
# Usage Requirments
Using redblock is intended to be simple, baring creating custom modules; if this is not the case I would consider that a bug and would engorage you to raise the issue as such.

The one caviate to the aformentioned principle is a basic understanding of rust is required to setup and configure your statusbar. You can paruse the [reference] for any concepts you don't understand (baring anyghing specific to [redblocks]). For a more compleate introduction to the language I would encorage you to check out [The Book]. a great place to start learing is [here]; if you need help installing Rust please see the [installation guide].

[Penrose]: [https://github.com/sminez/penrose]
[reference]: [https://doc.rust-lang.org/reference/introduction.html]
[The Book]: [https://www.rust-lang.org/learn]
[installation guide]: [https://www.rust-lang.org/tools/install]
[here]: [https://www.rust-lang.org/learn]
[redblocks]: crate

# Setup
To use redblocks add the following to your Cargo.toml.
```Cargo
[dependencies]
redblocks = 0.2.1
```
# Using Redblocks
Redblocks works on the principle of Widgets and Plugins. Widgets handles displaying the Plugin and timeing information. Plugins handle the actual data you watnt to display as how that information should be updated.
Currently the following plugins are avalible, please see their respecive module for more information:
* cpu
* memory usage
* time display

## Example
```no_run
#[macro_use]
extern crate redblocks;

use redblocks::{Widget, plugins::{TimePlugin, MemPlugin, CpuPlugin}};

fn main() {
    let time = Widget::new(TimePlugin::new("%A %D %_I:%M:%S %P"), 1);
   
    let cpu = Widget::new_mili(CpuPlugin::new(), 750);
    
    let mem = Widget::new(MemPlugin::new(), 2);

    let plugins = vec![mem, cpu, time];

    start_bar!(plugins);
}
```

# Wishlist
* internel xset root function
*/


use std::fmt::Display;
use std::time::{Duration, Instant};

pub mod plugins;

/// Vec\<Widget\>
pub type StatusBar = Vec<Widget>;

/// Holds [StatusBar]
pub struct Bar(pub StatusBar);

/// Handles timing and calling updates for plugins 
pub struct Widget {
    /// holds the plugin
    pub content: Box<dyn Update>,
    /// the update interval
    pub intervel: Duration,
    /// when the last update was preformed
    pub elapsed: Instant,
}

impl Widget {
    pub fn new(content: Box<dyn Update>, intervel: u64) -> Widget {
        Widget {
            content,
            intervel: Duration::from_secs(intervel),
            elapsed: Instant::now(),
        }
    }

        /// keep the interval to above 500ms as this is the sleep duration for the main event loop
    pub fn new_mili(content: Box<dyn Update>, intervel: u64) -> Widget {
	Widget {
	    content,
	    intervel: Duration::from_millis(intervel),
	    elapsed: Instant::now(),
	}
    }
    pub fn update(&mut self) {
        if self.elapsed.elapsed() >= self.intervel {
            self.content.refresh();

            self.elapsed = Instant::now();
        }
    }
}

/// Refreshes the widget plugin.
pub trait Update: Display {
    fn refresh(&mut self);
}

#[macro_export]
/// Constructs the [StatusBar] type as well as setting up the main event loop
macro_rules! start_bar  {
    {$v:tt, $x:tt} => {

        use std::process::Command;
	use redblocks::Bar;
        let mut bar = Bar($v);


        let mut comm = Command::new("xsetroot");
        loop {
            let mut output = String::new();
	    let mut num = 0;
	    let count = bar.0.iter().count();

            for i in &mut bar.0 {
		if num == 0 {

                    i.update();
                   let pushing = format!("{}", i.content);
                    output.push_str(&pushing);

		    num += 1;
		}else {

                    i.update();
                    let pushing = format!(" {} {}", $x, i.content);
                    output.push_str(&pushing);
		}
            }
            let mut child = comm.args(&["-name", &output])
                .spawn()
                .expect("xset root not installed");
	    std::thread::sleep(std::time::Duration::from_millis(500));
	    child.kill().expect("No process to kill");
        }
    };

    ($v:ident) => {

        use std::process::Command;
	use redblocks::Bar;
        let mut bar = Bar($v);

        let mut comm = Command::new("xsetroot");
        loop {
            let mut output = String::new();
	    let mut num = 0;
	    let count = bar.0.iter().count();

            for i in &mut bar.0 {
		if num == 0 {

                    i.update();
                    let pushing = format!("{}", i.content);
                    output.push_str(&pushing);

		    num += 1;
		}else {

                    i.update();
                    let pushing = format!(" | {}", i.content);
                    output.push_str(&pushing);
		}
            }
            let mut child = comm.args(&["-name", &output])
                .spawn()
                .expect("xset root not installed");
	    std::thread::sleep(std::time::Duration::from_millis(500));
	    child.kill().expect("No process to kill");
        }
    };
}
