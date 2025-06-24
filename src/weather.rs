use reqwest::{Client, Response};
use serde_json::Value;

/// API to retrieve the minimum temperature for a given postcode.
///
/// Function takes a postode string refference and an API key for weatherapi.com, then parses and returns the
/// minimum temperatrure as a float.
///
/// ## Errors
///
/// - Returns [`Result<String, Box<dyn std::error::Error>>`](std::error::Error) - if the GET request is unsuccessful.
/// - Returns [`Result<T>`](core::result::Result<Error>) - if the data is unable to be parsed.
///
/// ## Examples
///
/// Get temperature
/// ```rust
/// let min: f64 = get_min_temp();
/// println!("The minimum temperature is {min}");
/// ```
///
pub async fn get_min_temp(postcode: &str, key: &str) -> f64 {
    let data: String = call_api(postcode, key)
        .await
        .expect("unable to retrieve data");
    let json: Value = serde_json::from_str(&data).expect("unable to parse data");

    json["forecast"]["forecastday"][0]["day"]["mintemp_c"]
        .as_f64()
        .expect("mintemp_c is not a float")
}

async fn call_api(postcode: &str, key: &str) -> Result<String, Box<dyn std::error::Error>> {
    let client: Client = Client::new();
    let url = format!(
        "http://api.weatherapi.com/v1/forecast.json?key={}&q={}&days=1&aqi=no&alerts=no",
        key, postcode
    );
   
    let response: Response = client.get(url).send().await?;

    if response.status().is_success() {
        Ok(response.text().await?)
    } else {
        Err(format!("HTTP response: {}", response.status()).into())
    }
}
