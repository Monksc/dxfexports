use dxf::entities::*;
use dxf::Drawing;

use dxf::entities::LwPolyline;
use lyon::path::{math::Point, Path};

/// Used to add a method to a dxf::Drawing to convert it to a Vec<lyon::path::Path>
///
/// # Examples
///
/// ```
/// use dxfexports::ToLyon;
/// use dxf::Drawing;
///
/// let drawing = Drawing::load_file("test.dxf").expect("Not a good file");
/// let svg_nodes = drawing.to_lyon();
/// ```
pub trait ToLyon {
    fn to_lyon(&self) -> Vec<Path>;
}

impl ToLyon for Drawing {
    fn to_lyon(&self) -> Vec<Path> {
        export_lyon(&self)
    }
}

/// Takes a dxf::Drawing and converts it to a Vec<lyon::path::Path>
///
/// # Examples
///
/// Basic Usage:
///
/// ```
/// use dxfexports::export_lyon;
/// use dxf::Drawing;
///
/// let drawing = Drawing::load_file("test.dxf").expect("Not a good file");
/// let lyon_path = export_lyon(&drawing);
/// ```
pub fn export_lyon(drawing: &Drawing) -> Vec<Path> {
    let mut paths = Vec::new();

    for e in drawing.entities() {
        match e.specific {
            EntityType::Circle(ref circle) => {
                eprintln!("Circle: {:?}", circle);
            }
            EntityType::Line(ref line) => {
                eprintln!("Line: {:?}", line);
            }
            EntityType::Spline(_) => {}
            EntityType::LwPolyline(ref polyline) => {
                let mut path = lyon::path::Path::svg_builder();
                convert_lwpolyline_to_path(&polyline, &mut path);
                path.close();
                paths.push(path.build());
            }
            _ => {
                eprintln!("Other: {:?}", e.specific);
            }
        }
    }

    paths
}

pub fn convert_lwpolyline_to_path(
    lwpolyline: &LwPolyline,
    path_builder: &mut lyon::path::builder::WithSvg<lyon::path::path::BuilderImpl>,
) {
    use super::dxf_helper::ArcMoveLineTo;
    use lyon::path::builder::SvgPathBuilder;
    for arc in super::dxf_helper::lwpolyline_to_arcs_and_lines(lwpolyline) {
        match arc {
            ArcMoveLineTo::Arc(arc) => {
                path_builder.arc_to(
                    lyon::math::Vector::new(arc.radius.abs() as f32, arc.radius.abs() as f32),
                    lyon::math::Angle::radians(0.0),
                    lyon::path::ArcFlags {
                        large_arc: arc.bulge.abs() > 1.0,
                        sweep: arc.bulge >= 0.0,
                    },
                    Point::new(arc.to_point.x as f32, arc.to_point.y as f32),
                );
            }
            ArcMoveLineTo::Move(point) => {
                path_builder.move_to(Point::new(point.x as f32, point.y as f32));
            }
            ArcMoveLineTo::LineTo(point) => {
                path_builder.line_to(Point::new(point.x as f32, point.y as f32));
            }
        }
    }
}
