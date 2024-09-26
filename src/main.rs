use std::thread::sleep;
use std::time::Duration;
use serde_json::Value::Number;
use serde_json::Value;
use std::time::SystemTime;
use std::fs::File;
use std::io::prelude::*;
use chrono::offset::Utc;
use chrono::DateTime;

struct BerlinsWeatherAPI {

}

impl BerlinsWeatherAPI {
    fn new() -> BerlinsWeatherAPI {
        BerlinsWeatherAPI {

        }
    }
}

struct BerlinsWeatherScraper {

}

impl BerlinsWeatherScraper {
    fn new() -> BerlinsWeatherScraper {
        BerlinsWeatherScraper {

        }
    }

    async fn request_weather_datapoint(&mut self) {
        let r = reqwest::get("https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m,wind_speed_10m").await.unwrap().text().await.unwrap();

        let server_response_string = r.as_str();
        let res: Value = serde_json::from_str(
            r.as_str()
        ).unwrap();

        let system_time = SystemTime::now();
        let datetime: DateTime<Utc> = system_time.into();

        /*if let Number(celsius) = &res["current"]["temperature_2m"] {
            println!("{}", celsius.as_f64().unwrap());
        }
        else {
            println!("Error with parsed response!");
        }*/
        let mut file = File::create(
            format!("berlin-{}.json", datetime.format("%d_%m_%Y-%H_%M_%S").to_string())
        ).unwrap();
        file.write_all(server_response_string.as_bytes()).unwrap();
    }
}

#[tokio::main]
async fn main() {
    let mut bw = BerlinsWeatherScraper::new();
    loop {
        bw.request_weather_datapoint().await;
        sleep(Duration::from_secs(10));
    }
}
