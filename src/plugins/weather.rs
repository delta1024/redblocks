#[cfg(feature = "weather")]
use openweathermap::Receiver;

#[cfg(feature = "weather")]
use crate::Update;

#[cfg(feature = "weather")]
use std::fmt;

#[cfg(feature = "weather")]
#[doc(hidden)]
pub use openweathermap::init;

#[cfg(feature = "weather")]
#[cfg_attr(docsrs, doc(cfg(feature = "weather")))]
/** Weather Plugin

For an explenation of the paramaters please see [this page](self::init). You don't need to worry about handeling the thread handling, the plugin will take care of all of that for you.
*/
pub struct WeatherPlugin {
    receiver: Receiver,
    output: String,
}

#[cfg(feature = "weather")]
impl WeatherPlugin {
    pub fn new(location: &str, units: &str, lang: &str, api_key: &str, poll_mins: u64) -> Box<WeatherPlugin> {
	Box::new(
	    WeatherPlugin {
		receiver: openweathermap::init(location, units, lang, api_key, poll_mins),
		output: String::new(),

	    }
	)
    }
}

#[cfg(feature = "weather")]
impl fmt::Display for WeatherPlugin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
	write!(f, "{}", self.output)
    }
}

#[cfg(feature = "weather")]
impl Update for WeatherPlugin {
    fn refresh(&mut self) {
	let n = if let Some(weather) = openweathermap::update(&self.receiver) {
	    weather
	}else {
	    return
	};
	todo!{}
    }
}
