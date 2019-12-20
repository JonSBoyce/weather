extern crate reqwest;
extern crate select;
use select::document::Document;
use select::predicate::{Class};
use std::env;

fn main() {
  let args: Vec<String> = env::args().collect();
  let zip_bytes = include_bytes!("2019_Gaz_zcta_national.txt");
  let zip_string = String::from_utf8_lossy(zip_bytes);
  let zip_data = zip_string.split("\n");
  for line in zip_data {
    if line.starts_with(&args[1]) {
      let chunks: Vec<&str> = line.split_whitespace().collect();
      let body: &str = &reqwest::get(&format!("https://forecast.weather.gov/MapClick.php?lat={}&lon={}", chunks[5], chunks[6])[..]).unwrap().text().unwrap();
      let document = Document::from(body);
      println!("{} {} | {}",
        document.find(Class("myforecast-current")).next().unwrap().text(),
        document.find(Class("myforecast-current-lrg")).next().unwrap().text(),
        document.find(Class("forecast-text")).next().unwrap().text());
      break;
    }
  }
}
