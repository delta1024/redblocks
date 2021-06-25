/*!# About
Redblocks is a library inspired by xmobar for creating your own status blocks that writes to XROOTNAME?. Primaraly intended for along side the [Penrose] library.

# Requirments
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
redblocks = 0.1.2
``
# Building your own widgets
Currently doing anything at all with [redblocks] requires you to creat your own custom plugins.

First you will need to create a struct to hold the information you wish displayed in the status blocks. When implementing the plugin's new() function it is importatn that it return itself in a [`Box`]. Once you have created your status plugin you will need to implement both the [`std::fmt::Display`] and [Update] traits; the implementation of which can be found below.
[Update]: crate::Update

 # Example Widget
For the following example we are going to be creating a simple widget that couts how many seconds the status blocks have been runing.
```
// for creating the plugin
use redblocks::Update;
// for drawing the statusblocks
use redblocks::{Widget, start_bar};
use std::fmt::{self,Display};

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

fn main() {
    // set the update intervel in seconds
    let update_intervel = 1;

    // create create the plugin 
    let counter_plugin = Counter::new();

    // create the widget
    let counter_widget = Widget::new(counter_plugin, update_intervel);

    // start_bar is a macro so it can accept more than one argument
    start_bar!(counter_widget);
    
}

```
*/

use std::fmt::Display;
use std::time::{Duration, Instant};

/// constructed by the [start_bar] macro 
pub type StatusBar = Vec<Widget>;

/// Holds [StatusBar]
pub struct Bar(pub StatusBar);

/// Handles timing and updating
pub struct Widget {
    pub content: Box<dyn Update>,
    pub intervel: Duration,
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
    pub fn update(&mut self) {
        if self.elapsed.elapsed() >= self.intervel {
            self.content.refresh();

            self.elapsed = Instant::now();
        }
    }
}

pub trait Update: Display {
    fn refresh(&mut self);
}

#[macro_export]
macro_rules! start_bar  {
    ($($v:ident), +) => {

        use std::process::Command;
	use redblocks::Bar;
        let mut bar = Bar(Vec::new());

	$(
	    let var = $v;
	    bar.0.push(var);
        )+

        let mut comm = Command::new("xsetroot");
        loop {
            let mut output = String::new();

            for i in &mut bar.0 {
                i.update();
                let pushing = format!("| {} ", i.content);
                output.push_str(&pushing);
            }
            comm.args(&["-name", &output])
                .spawn()
                .expect("xset root not installed");
    }
    }
}
