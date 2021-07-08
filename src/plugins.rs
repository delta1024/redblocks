/*! Library provided plugins
# Building your own plugins

First you will need to create a struct to hold the information you wish displayed in the status blocks. When implementing the plugin's new() function it is importatn that it return itself in a [`Box`]. Once you have created your status plugin you will need to implement both the [Display](std::fmt::Display) and [Update](crate::Update) traits; the implementation of which can be found below.


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



#[doc(inline)]
pub use chrono::format::strftime;

#[doc(hidden)]
pub mod battery;
#[doc(inline)]
pub use self::battery::*;

#[doc(hidden)]
pub mod  cpu;
#[doc(inline)]
pub use self::cpu::*;

#[doc(hidden)]
pub mod mem;
#[doc(inline)]
pub use self::mem::*;

#[doc(hidden)]
pub mod time;
#[doc(inline)] 
pub use self::time::*;




#[cfg(feature = "weather")]
#[doc(hidden)]
pub mod weather;
#[cfg(feature = "weather")]
#[doc(inline)]
pub use self::weather::*;
