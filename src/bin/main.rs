use dotenv::dotenv;
use reqwest;
use real_time_global_weather_tracker::WeatherResponse;
use std::{env, io};
use dialoguer::{theme::ColorfulTheme, Select};
use crossterm::style::Stylize;




async fn get_weather(city: &str,api_key: &str,) -> WeatherResponse {
    let weather_url = format!( "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",city,api_key);
    let response  = reqwest::get(&weather_url).await.unwrap().json().await.unwrap();
    response
}

fn get_city_name() -> String{
    let mut  city = String::from("");
    println!("{}","Enter the city name: ".yellow().bold().rapid_blink());
    io::stdin().read_line(&mut city).expect("Wrong message entered");
    city.trim().to_string()
}


#[tokio::main]
async fn main() {

    dotenv().ok();
    loop {
        let api_key = env::var("WEATHER_API").expect("Please load a api key in the project");

        println!("{}","This is a weather forecasting app".yellow().bold().rapid_blink());
        let city_name = get_city_name();

        let weather_data = get_weather(&city_name, &api_key).await;
        println!();

        println!("{}{}{}","Temperature: ".green().bold(), weather_data.main.temp.to_string().green().bold(),"°C".green().bold());
        println!("{}{}{}","Humidity: ".blue(), weather_data.main.humidity.to_string().blue(),"%".blue());
        println!("{}{}","Description: ".cyan(), weather_data.weather[0].description.to_string().cyan());
        println!();

        let options = vec!["Yes","No"];
        let selection = Select::with_theme(&ColorfulTheme::default())
        .with_prompt("Do you want to continue: ")
        .items(&options)
        .default(0)
        .interact();

        match selection {
            Ok(index) => {
                if index == 1 {
                    break
                }
            },
            Err(_) => println!("Some Error occured!!"),
        }

    }


}
