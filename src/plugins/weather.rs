#[cfg(feature = "weather")]
use openweathermap::Receiver;
/// Weather Plugin

#[cfg(feature = "weather")]
pub struct WeatherPlugin<T, U, V> {
    city: u64,
    receiver: Receiver,
    future: std::future::Future<Output = String>,
    output: String,
}
