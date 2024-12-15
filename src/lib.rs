use serde::Deserialize;



#[derive(Deserialize,Debug)]
pub struct WeatherResponse {
  pub name: String, // City name
  pub main: Main,
  pub weather: Vec<Weather>, // Array of weather conditions
}

#[derive(Deserialize,Debug)]
pub struct Main {
  pub temp: f64,  // Temperature
  pub humidity: u8, // Humidity percentage
}

#[derive(Deserialize,Debug)]
pub struct Weather {
  pub description: String, // Weather description (e.g., clear sky)
}