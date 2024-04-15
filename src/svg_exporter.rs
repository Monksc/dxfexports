use dxf::entities::*;
use dxf::Drawing;

use dxf::entities::LwPolyline;

/// Used to add a method to a dxf::Drawing to convert it to a svg::node::element::path::Data
///
/// # Examples
///
/// ```
/// use dxfexports::ToSVG;
/// use dxf::Drawing;
///
/// let drawing = Drawing::load_file("test.dxf").expect("Not a good file");
/// let svg_nodes = drawing.to_svg();
/// ```
pub trait ToSVG {
    fn to_svg(&self) -> Vec<svg::node::element::path::Data>;
}

impl ToSVG for Drawing {
    fn to_svg(&self) -> Vec<svg::node::element::path::Data> {
        export_svg(self)
    }
}

/// Takes a dxf::Drawing and converts it to a svg::node::element::path::Data
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// use dxfexports::export_svg;
/// use dxf::Drawing;
///
/// let drawing = Drawing::load_file("test.dxf").expect("Not a good file");
/// let svg = export_svg(&drawing);
/// ```
pub fn export_svg(drawing: &Drawing) -> Vec<svg::node::element::path::Data> {
    let mut data = Vec::new();

    for e in drawing.entities() {
        match e.specific {
            EntityType::Circle(ref circle) => {
                eprintln!("Circle: {:?}", circle);
            }
            EntityType::Line(ref line) => {
                eprintln!("Line: {:?}", line);
            }
            EntityType::Spline(_) => {
                // data = data.move_to((10, 10));
            }
            EntityType::LwPolyline(ref polyline) => {
                data.push(convert_lwpolyline_to_svg(&polyline));
            }
            _ => {
                eprintln!("Other: {:?}", e.specific);
            }
        }
    }

    data
}

pub fn convert_lwpolyline_to_svg(lwpolyline: &LwPolyline) -> svg::node::element::path::Data {
    let mut data = svg::node::element::path::Data::new();

    use super::dxf_helper::ArcMoveLineTo;
    let mut last_point = super::dxf_helper::Point::new(0.0, 0.0);
    for arc in super::dxf_helper::lwpolyline_to_arcs_and_lines(lwpolyline) {
        match arc {
            ArcMoveLineTo::Arc(arc) => {
                // let large_arc_flag = arc.bulge.abs() > 1.0;
                // let sweep_flag = (arc.bulge >= 0.0) == (arc.bulge.abs() < 1.0);
                // let bezier_curve = svg::node::element::path::Data::parse(&format!(
                //     "A {} {} {} {} {} {} {}",
                //     arc.radius,
                //     arc.radius,
                //     0.0,
                //     large_arc_flag as i32,
                //     sweep_flag as i32,
                //     arc.to_point.x,
                //     arc.to_point.y
                // ));
                // data.append(bezier_curve);

                data = data.line_by((
                    // arc.radius,
                    // arc.radius,
                    // 0.0,
                    // if arc.bulge.abs() > 1.0 { 1.0 } else { 0.0 },
                    // if  { 1.0 } else {0.0},
                    // (
                    arc.to_point.x - arc.from_point.x,
                    arc.to_point.y - arc.from_point.y,
                    // ),
                ));

                last_point = arc.to_point;
            }
            ArcMoveLineTo::Move(point) => {
                data = data.move_to((point.x, point.y));
                last_point = point;
            }
            ArcMoveLineTo::LineTo(point) => {
                data = data.line_by((point.x - last_point.x, point.y - last_point.y));
                last_point = point;
            }
        }
    }

    data.close()
}
