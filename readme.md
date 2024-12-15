# Weather Forecasting App 🌦️

A simple and interactive **weather forecasting app** built using **Rust**. This app allows you to check the weather forecast for any city worldwide by providing real-time data using the **OpenWeather API**.

### Features 🚀

- Interactive terminal interface powered by **dialoguer** and **crossterm**.
- Fetch real-time weather data (temperature, humidity, and weather description) for any city.
- Displays weather information with stylish terminal output using colors and animations.
- Option to continue checking weather forecasts for different cities or exit the app.

---

## Prerequisites 🧰

- **Rust**: You must have Rust installed. You can install it from [here](https://www.rust-lang.org/).
- **OpenWeather API Key**: You need to create an account at [OpenWeather](https://openweathermap.org/api) and obtain an API key.

---

## Setup 🔧

### 1. Clone this repository:

```bash
git clone https://github.com/Firespiko/weather-forecast-app
cd weather_forecasting_app
```

### 2. Add your **OpenWeather API Key** to an `.env` file in the root directory:

Create a `.env` file and add the following line:

```makefile
WEATHER_API=your_openweather_api_key
```

### 3. Add dependencies:

This project uses `dotenv`, `reqwest`, `forecast`, `dialoguer`, and `crossterm`. Cargo will automatically handle the dependencies for you when you build the project.

## How to Run 🚀

### 1. Build and run the project:

```bash
`cargo run`
```

### 2. Enter the city name when prompted.

- The app will display the **temperature** (°C), **humidity**, and a **description** of the weather.
- You can continue to check the weather for more cities or exit the app.

---

## Project Structure 🗂️

- **`src/bin/main.rs`**: The main binary code, where the app logic resides.
- **`src/lib.rs`**: Contains the data structures and function definitions used in the app.
- **`.env`**: Stores your **OpenWeather API key**.

---

## Technologies Used 🛠️

- **Rust**: Systems programming language.
- **Reqwest**: HTTP client for fetching data from the OpenWeather API.
- **Forecast**: Weather data structure for deserializing the API response.
- **Crossterm**: For terminal output with colors and animation.
- **Dialoguer**: For a user-friendly, interactive prompt to select options in the terminal.

---

## Screenshots 📸

---

## Contributions 🤝

Feel free to fork the repository and submit issues or pull requests. Contributions are welcome!

---

## Contact 📬

For any questions or feedback, feel free to reach out at [adikarks@gmail.com].

---

## License 🔑

This project is licensed under the MIT License.
