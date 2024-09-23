use serde_json::Value::Number;
use serde_json::Value;

struct BerlinsWeather {

}

impl BerlinsWeather {
    fn new() -> BerlinsWeather {
        BerlinsWeather {

        }
    }

    async fn request_weather_datapoint(&mut self) {
        let r = reqwest::get("https://api.open-meteo.com/v1/forecast?latitude=52.52&longitude=13.41&current=temperature_2m,wind_speed_10m");
        let res: Value = serde_json::from_str(r.await.unwrap().text().await.unwrap().as_str()).unwrap();
        if let Number(celsius) = &res["current"]["temperature_2m"] {
            println!("{}", celsius.as_f64().unwrap());
        }
        else {
            println!("Error with parsed response!");
        }
    }
}

#[tokio::main]
async fn main() {
    let mut bw = BerlinsWeather::new();
    bw.request_weather_datapoint().await;
}
