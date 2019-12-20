extern crate reqwest;
extern crate select;
use select::document::Document;
use select::predicate::{Class};

fn main() {
  let body: &str = &reqwest::get("https://forecast.weather.gov/MapClick.php?site=IND&textField1=39.9693&textField2=-86.1095&e=0").unwrap().text().unwrap();
  let document = Document::from(body);
  println!("{} {} | {}",
    document.find(Class("myforecast-current")).next().unwrap().text(),
    document.find(Class("myforecast-current-lrg")).next().unwrap().text(),
    document.find(Class("forecast-text")).next().unwrap().text());
}
