// use core::f64;
// use std::str::FromStr;

// use geo::BoundingRect;
// use wasm_bindgen::prelude::*;
// use web_sys::js_sys::{ Array, JsString };

// #[wasm_bindgen]
// pub fn get_bbox(geojson_str_js_string: JsString) -> Array {
//     let geojson_string: String = geojson_str_js_string.into();
//     let geojson_str = geojson_string.as_str();
//     let geojson: geojson::GeoJson = geojson::GeoJson
//         ::from_str(geojson_str)
//         .expect("parse geojson failed");
//     let array = Array::new();
//     if let Some(bbox) = calculate_bbox(&geojson) {
//         for b in bbox.iter() {
//             array.push(&JsValue::from_f64(*b));
//         }
//     } else {
//         for _ in 0..4 {
//             array.push(&JsValue::from_f64(f64::NAN));
//         }
//     }
//     array
// }

// fn calculate_bbox(geojson: &geojson::GeoJson) -> Option<Vec<f64>> {
//     match geojson {
//         geojson::GeoJson::FeatureCollection(fc) => {
//             if let Some(bbox) = &fc.bbox {
//                 Some(bbox.clone())
//             } else {
//                 calculate_feature_collection_bbox(fc)
//             }
//         }
//         _ => { None }
//     }
// }

// fn calculate_feature_collection_bbox(fc: &geojson::FeatureCollection) -> Option<Vec<f64>> {
//     let mut min_x = f64::INFINITY;
//     let mut min_y = f64::INFINITY;
//     let mut max_x = f64::NEG_INFINITY;
//     let mut max_y = f64::NEG_INFINITY;
//     let mut has_geometry = false;
//     for feature in &fc.features {
//         if let Some(ref geom) = feature.geometry {
//             let geo_geometry: Result<geo_types::Geometry, _> = geom.value.clone().try_into();
//             if let Ok(geom) = geo_geometry {
//                 if let Some(rect) = geom.bounding_rect() {
//                     has_geometry = true;
//                     let rect_min = rect.min();
//                     let rect_max = rect.max();
//                     min_x = min_x.min(rect_min.x);
//                     min_y = min_y.min(rect_min.y);
//                     max_x = max_x.max(rect_max.x);
//                     max_y = max_y.max(rect_max.y);
//                 }
//             }
//         }
//     }
//     if has_geometry {
//         Some(vec![min_x, min_y, max_x, max_y])
//     } else {
//         None
//     }
// }

use serde::{ Deserialize, Serialize };
use wasm_bindgen::{ JsValue, prelude::wasm_bindgen };
use web_sys::js_sys::{ Array, JsString };

#[derive(Debug, Clone, Serialize, Deserialize)]
struct PolygonGeoJson<T = Geometry> {
    #[serde(rename = "type")]
    r#type: String,
    features: Vec<PolygonFeature<T>>,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
struct PolygonFeature<T> {
    #[serde(rename = "type")]
    r#type: String,
    geometry: T,
}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
enum Geometry {
    Polygon {
        coordinates: Vec<Vec<[f64; 2]>>,
    },
    MultiPolygon {
        coordinates: Vec<Vec<Vec<[f64; 2]>>>,
    },
}
#[wasm_bindgen]
pub fn get_bbox(geojson_str: JsString) -> Array {
    let array = Array::new();
    let geojson_string: String = geojson_str.into();
    let mut min_x = f64::INFINITY;
    let mut min_y = f64::INFINITY;
    let mut max_x = f64::NEG_INFINITY;
    let mut max_y = f64::NEG_INFINITY;
    macro_rules! get_max_min_xy {
        ($f:expr) => {
            $f.iter().for_each(|cor| {
              let x = cor[0];
              let y = cor[1];
              min_x = min_x.min(x);
              min_y = min_y.min(y);
              max_x = max_x.max(x);
              max_y = max_y.max(y);
          });
        };
    }
    let geojson: PolygonGeoJson = serde_json
        ::from_str(&geojson_string)
        .expect("serde parse failed");
    if geojson.features.is_empty() {
        for _ in 0..4 {
            array.push(&JsValue::from_f64(f64::NAN));
        }
    } else {
        for feature in &geojson.features {
            match &feature.geometry {
                Geometry::MultiPolygon { coordinates } => {
                    let _ = coordinates.iter().for_each(|f| {
                        f.iter().for_each(|c| {
                            get_max_min_xy!(c);
                        });
                    });
                }
                Geometry::Polygon { coordinates } => {
                    let _ = coordinates.iter().for_each(|f| {
                        get_max_min_xy!(f);
                    });
                }
            }
        }
        array.push(&JsValue::from_f64(min_x));
        array.push(&JsValue::from_f64(min_y));
        array.push(&JsValue::from_f64(max_x));
        array.push(&JsValue::from_f64(max_y));
    }
    array
}
