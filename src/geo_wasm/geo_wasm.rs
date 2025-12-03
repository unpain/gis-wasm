use wasm_bindgen::prelude::*;
use web_sys::js_sys::{ Array, JsString, Number };
use serde::{ Serialize, Deserialize };
#[derive(Debug,Clone, Serialize, Deserialize)]
struct PolygonGeoJSON {
    #[serde(rename = "type")]
    r#type: String,
    features: Vec<Feature>,
}
#[derive(Debug,Clone, Serialize, Deserialize)]
struct Feature{
  #[serde(rename = "type")]
  r#type: String,
  geometry: Geometry,
}
#[derive(Debug,Clone, Serialize, Deserialize)]
struct Geometry {
  #[serde(rename = "type")]
  r#type: String,
  coordinates: Vec<Vec<Vec<f64>>>,
}
#[wasm_bindgen]
pub fn get_bbox(geojson_jsstring: JsString) -> Array {
    let geojson_str = geojson_jsstring.as_string().unwrap();
    let geojson: PolygonGeoJSON = serde_json::from_str(&geojson_str).unwrap();
    let array = vec![1.0, 2.0, 3.0];
    let array = Array::from_iter(array.iter().map(|s| { JsValue::from_f64(*s) }));
    array
}
#[wasm_bindgen]
pub fn get_feature_length(geojson_jsstring: JsString) -> Number {
    let geojson_str = geojson_jsstring.as_string().unwrap();
    let geojson: PolygonGeoJSON = serde_json::from_str(&geojson_str).unwrap();
    let length = geojson.features.len() as f64;
    Number::from(length)
}
