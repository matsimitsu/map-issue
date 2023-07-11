use d3_geo_rs::path::builder::Builder as PathBuilder;
use d3_geo_rs::projection::mercator::Mercator;
use d3_geo_rs::projection::Build;
use d3_geo_rs::projection::CenterSet;




use d3_geo_rs::projection::PrecisionBypass;
use d3_geo_rs::projection::RawBase;
use d3_geo_rs::projection::ScaleSet;
use d3_geo_rs::projection::TranslateSet;
use geo::Geometry;
use geo_types::Coord;
use geojson::GeoJson;

fn main() {
    let json =
        std::fs::read_to_string("./world.json").expect("Could not read 'world.json' fixture");

    let geojson = json.parse::<GeoJson>().unwrap();
    let width = 960_f64;
    let height = 500_f64;

    let mut mercator = Mercator::builder();
    let mercator = mercator.translate_set(&Coord {
        x: width / 2_f64,
        y: height / 2_f64,
    });
    let mercator = mercator
        .scale_set(110f64)
        .center_set(&Coord {
            x: 10_f64,
            y: 40_f64,
        })
        .translate_set(&Coord {
            x: width / 2_f64,
            y: height / 2_f64,
        })
        .precision_bypass()
        .build();

    let path_builder = PathBuilder::pathstring();

    let mut builder = path_builder.build(mercator);
    let mut out = String::new();

    out.push_str("<svg xmlns=\"http://www.w3.org/2000/svg\" viewBox=\"0 0 960 500\" width=\"960\" height=\"500\">");

    let mut features = Vec::new();
    if let geojson::GeoJson::FeatureCollection(feature_collection) = geojson {
        for feature in feature_collection.features {
            if let Some(geometry) = feature.geometry {
                let geo_geometry: Geometry = Geometry::try_from(geometry).unwrap();
                features.push(geo_geometry);
            }
        }
    }

    for feature in features {
        match &feature {
            Geometry::MultiPolygon(mp) => {
                for p in &mp.0 {
                    let s = builder.object(&Geometry::Polygon(p.clone()));

                    out.push_str(&format!("<path d=\"{}\" stroke-width=\"0.5\" fill=\"none\" stroke=\"black\"></path>", s));
                }
            }
            Geometry::Polygon(p) => {
                let s = builder.object(&Geometry::Polygon(p.clone()));
                out.push_str(&format!(
                    "<path d=\"{}\" stroke-width=\"0.5\" fill=\"none\" stroke=\"black\"></path>",
                    s
                ));
            }

            _ => {
                println!("Not polygon, Not Multipolygon.");
            }
        }
    }
    out.push_str("</svg>");

    std::fs::write("./map.svg", out).expect("Unable to write file");
}
