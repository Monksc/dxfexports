//! A DXF exporter
//!
//! Currently can only read lwpolyline but working on future improvements
//!
//! ## Example: Exporting to Lyon Path
//!
//! ```
//! # extern crate dxfexports;
//! # extern crate dxf;
//! # extern crate lyon;
//! use dxfexports::*;
//! use dxf::Drawing;
//!
//! let drawing = Drawing::load_file("test.dxf").expect("Not a good file");
//! let lyon_path = export_lyon(&drawing);
//!
//! ```
//!
//! ## Example: Exporting to geo_types::LineString
//!
//! ```
//! # extern crate dxfexports;
//! # extern crate dxf;
//! # extern crate geo_types;
//! use dxfexports::*;
//! use dxf::Drawing;
//!
//! let drawing = Drawing::load_file("test.dxf").expect("Not a good file");
//! let linestrings = export_geo_linestring(&drawing, 0.0001);
//!
//! ```

pub mod dxf_helper;
#[cfg(feature = "geo_types")]
pub mod geo_exporter;
#[cfg(feature = "lyon_path")]
pub mod lyon_exporter;
#[cfg(feature = "svg")]
pub mod svg_exporter;
pub use dxf_helper::{lwpolyline_to_arcs_and_lines, Arc, ArcMoveLineTo, Point};
#[cfg(feature = "geo_types")]
pub use geo_exporter::{export_geo_linestring, export_geo_linestring_from_lyon, ToGeoLineString};
#[cfg(feature = "lyon_path")]
pub use lyon_exporter::{convert_lwpolyline_to_path, export_lyon, ToLyon};
#[cfg(feature = "svg")]
pub use svg_exporter::{convert_lwpolyline_to_svg, export_svg, ToSVG};

#[cfg(test)]
mod tests {
    use std::fs::File;

    #[test]
    fn dxf_to_svg() {
        use super::*;
        use dxf::Drawing;
        use svg::save;

        let drawing = Drawing::load_file("test.dxf").expect("Not a good file");

        let datas = export_svg(&drawing);

        let mut document = svg::Document::new().set("viewBox", (0, 0, 40, 40));

        for data in datas {
            let path = svg::node::element::Path::new()
                .set("fill", "none")
                .set("stroke", "black")
                .set("stroke-width", 0.05)
                .set("d", data);

            document = document.add(path);
        }

        save("image.svg", &document).unwrap();
    }

    #[test]
    fn dxf_to_gcode() {
        use super::*;
        use dxf::Drawing;

        let drawing =
            Drawing::load_file("/home/cameron/Downloads/faraway.dxf").expect("Not a good file");

        let polygons = export_geo_linestring_from_lyon(&drawing, 0.001);
        // let polygons = export_geo_linestring(&drawing, 0.01);

        let mut test_file = File::create("test.nc").unwrap();

        for lines in polygons {
            let mut first = true;
            for line in lines {
                use std::io::Write;
                write!(
                    test_file,
                    "{} X{:3} Y{:3}\n",
                    if first { "G00" } else { "G01" },
                    line.x,
                    line.y
                )
                .unwrap();
                first = false;
            }
        }
    }
}
