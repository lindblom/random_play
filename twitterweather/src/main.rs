use egg_mode::account::UserProfile;
use serde;
use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct WeatherResponse {
    pub location: Location,
    pub current: Current,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "tz_id")]
    pub tz_id: String,
    #[serde(rename = "localtime_epoch")]
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    #[serde(rename = "last_updated_epoch")]
    pub last_updated_epoch: i64,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "temp_c")]
    pub temp_c: f64,
    #[serde(rename = "temp_f")]
    pub temp_f: f64,
    #[serde(rename = "is_day")]
    pub is_day: i64,
    pub condition: Condition,
    #[serde(rename = "wind_mph")]
    pub wind_mph: f64,
    #[serde(rename = "wind_kph")]
    pub wind_kph: f64,
    #[serde(rename = "wind_degree")]
    pub wind_degree: i64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mb")]
    pub pressure_mb: f64,
    #[serde(rename = "pressure_in")]
    pub pressure_in: f64,
    #[serde(rename = "precip_mm")]
    pub precip_mm: f64,
    #[serde(rename = "precip_in")]
    pub precip_in: f64,
    pub humidity: i64,
    pub cloud: i64,
    #[serde(rename = "feelslike_c")]
    pub feelslike_c: f64,
    #[serde(rename = "feelslike_f")]
    pub feelslike_f: f64,
    #[serde(rename = "vis_km")]
    pub vis_km: f64,
    #[serde(rename = "vis_miles")]
    pub vis_miles: f64,
    pub uv: f64,
    #[serde(rename = "gust_mph")]
    pub gust_mph: f64,
    #[serde(rename = "gust_kph")]
    pub gust_kph: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

fn get_wather_emoji(condition: &str) -> String {
    match condition {
        "Sunny" => "🌞",
        "Clear" => "🌝",
        "Partly Cloudy" => "⛅",
        "Cloudy" => "☁️",
        "Overcast" => "☁️",
        "Mist" => "🌫️",
        "Patchy rain nearby" => "🌧️",
        "Patchy snow nearby" => "❄️",
        "Patchy sleet nearby" => "❄️",
        "Patchy freezing drizzle nearby" => "🌧️",
        "Thundery outbreaks in nearby" => "⛈️",
        "Blowing snow" => "❄️",
        "Blizzard" => "⛈️",
        "Fog" => "🌫️",
        "Freezing fog" => "🌫️",
        "Patchy light drizzle" => "🌧️",
        "Light drizzle" =>  "🌧️",
        "Freezing drizzle" => "🌧️",
        "Heavy freezing drizzle" => "🌧️",
        "Patchy light rain" => "🌧️",
        "Light rain" =>  "🌧️",
        "Moderate rain at times" => "🌧️",
        "Moderate rain" => "🌧️",
        "Heavy rain at times" => "🌧️",
        "Heavy rain" => "🌧️",
        "Light freezing rain" => "🌧️",
        "Moderate or heavy freezing rain" => "🌧️",
        "Light sleet" => "❄️",
        "Moderate or heavy sleet" => "❄️",
        "Patchy light snow" => "❄️",
        "Light snow" => "❄️",
        "Patchy moderate snow" => "❄️",
        "Moderate snow" => "❄️",
        "Patchy heavy snow" => "❄️",
        "Heavy snow" => "❄️",
        "Ice pellets" => "❄️",
        "Light rain shower" => "🌧️",
        "Moderate or heavy rain shower" => "🌧️",
        "Torrential rain shower" => "🌧️",
        "Light sleet showers" => "🌧️",
        "Moderate or heavy sleet showers" => "❄️",
        "Light snow showers" => "❄️",
        "Moderate or heavy snow showers" => "❄️",
        "Light showers of ice pellets" => "❄️",
        "Moderate or heavy showers of ice pellets" => "❄️",
        "Patchy light rain in area with thunder" => "⛈️",
        "Moderate or heavy rain in area with thunder" => "⛈️",
        "Patchy light snow in area with thunder" => "⛈️",
        "Moderate or heavy snow in area with thunder" => "⛈️",
        _ => "",
    }.into()
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let weather_api_key = "weatherapi.com key";
    let city = "your city";
    let resp: WeatherResponse = reqwest::get(format!("https://api.weatherapi.com/v1/current.json?key={}&q={}&aqi=no", weather_api_key, city))
        .await?
        .json()
        .await?;
    let new_profile = format!("Your bio.. \n{} {} °C",
        get_wather_emoji(resp.current.condition.text.as_str()),
        resp.current.temp_c);
    
    let con_token = egg_mode::KeyPair::new("twitter app key", "twitter app secret");
    let access_token = egg_mode::KeyPair::new("user access key", "user access secret");
    let token = egg_mode::Token::Access {
        consumer: con_token,
        access: access_token,
    };

    let respo = egg_mode::account::update_profile(UserProfile {
        description: Some(new_profile.into()),
        ..UserProfile::default()
    }, &token).await?;
    
    println!("{:?}", respo.rate_limit_status);
    Ok(())
}
