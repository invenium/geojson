// Copyright 2015 The GeoRust Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::collections::BTreeMap;
use rustc_serialize::json::{Json, Object, ToJson};
use Geometry;

/// Feature
///
/// [GeoJSON Format Specification § 2.2](http://geojson.org/geojson-spec.html#feature-objects)
pub struct Feature {
    pub geometry: Geometry,
    pub properties: Json,
}

impl ToJson for Feature {
    fn to_json(&self) -> Json {
        let mut d = BTreeMap::new();
        d.insert(format!("type"), "Feature".to_json());
        d.insert(format!("geometry"), self.geometry.to_json());
        d.insert(format!("properties"), self.properties.to_json());
        d.to_json()
    }
}

impl Feature {
    pub fn from_json(json_feature: &Object) -> Feature {
        let geometry_json = json_feature.get("geometry").unwrap();
        return Feature{
            geometry: Geometry::from_json(geometry_json.as_object().unwrap()),
            properties: json_feature.get("properties").unwrap().clone(),
        };
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use rustc_serialize::json::ToJson;
    use {Geometry, Feature, Poly, MultiPolygon, Pos, Ring};

    #[test]
    fn test_feature_to_json() {
        let mut map = BTreeMap::new();
        map.insert(format!("hi"), "there".to_json());
        let point = Feature {
            geometry: Geometry::MultiPolygon(MultiPolygon {
                    coordinates: vec![
                        Poly(vec![
                            Ring(vec![
                                Pos(vec![1., 2., 3.]),
                                Pos(vec![2., 4., 3.])
                            ]),
                            Ring(vec![
                                Pos(vec![3., 2., 3.]),
                                Pos(vec![2., 4., 3.])
                            ])
                        ])
                    ]
                }),
        properties: map.to_json()

        };
        let json_string = format!("{}",point.to_json());
        assert_eq!("{\"geometry\":{\"coordinates\":[[[[1.0,2.0,3.0],[2.0,4.0,3.0]],[[3.0,2.0,3.0],[2.0,4.0,3.0]]]],\"type\":\"MultiPolygon\"},\"properties\":{\"hi\":\"there\"},\"type\":\"Feature\"}", json_string);
    }
}
