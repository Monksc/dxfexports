use dxf::Drawing;

use geo_types::{coord, LineString};

/// Used to add a method to a dxf::Drawing to convert it to a Vec<geo::LineString>
///
/// # Examples
///
/// ```
/// use dxfexports::ToGeoLineString;
/// use dxf::Drawing;
///
/// let drawing = Drawing::load_file("test.dxf").expect("Not a good file");
/// let linestrings = drawing.to_geo_linestring(0.0001);
/// ```
pub trait ToGeoLineString {
    fn to_geo_linestring(&self, flatten_value: f32) -> Vec<LineString>;
}

impl ToGeoLineString for Drawing {
    fn to_geo_linestring(&self, flatten_value: f32) -> Vec<LineString> {
        export_geo_linestring(&self, flatten_value)
    }
}

/// Takes a dxf::Drawing and converts it to a Vec<geo_types::LineString>
///
/// First it converts it to a lyon_path then to Vec<geo_types::LineString>.
/// flatten_value is used in lyon_exporter to flatten the curves to lines.
/// A lower value will mean a more precise line_string when it comes to curves
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// use dxfexports::export_geo_linestring;
/// use dxf::Drawing;
///
/// let drawing = Drawing::load_file("test.dxf").expect("Not a good file");
/// let linestrings = export_geo_linestring(&drawing, 0.0001);
/// ```
pub fn export_geo_linestring(drawing: &Drawing, flatten_value: f32) -> Vec<LineString> {
    let paths = super::lyon_exporter::export_lyon(drawing);

    let mut line_strings: Vec<LineString> = Vec::new();
    let mut points = Vec::new();
    for path in paths {
        use lyon::path::iterator::PathIterator;
        let flattened_iter = path.iter().flattened(flatten_value);
        for evt in flattened_iter {
            match evt {
                lyon::path::PathEvent::Begin { at } => {
                    points.push(coord! { x: at.x as f64, y: at.y as f64});
                }
                lyon::path::PathEvent::Line { from: _, to } => {
                    points.push(coord! { x: to.x as f64, y: to.y as f64});
                }
                lyon::path::PathEvent::End {
                    last: _,
                    first,
                    close: _,
                } => {
                    points.push(coord! { x: first.x as f64, y: first.y as f64});
                    line_strings.push(LineString::new(points.clone()));
                    points.clear();
                }
                _ => {
                    panic!()
                }
            }
        }
    }

    line_strings
}
