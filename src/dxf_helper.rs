use dxf::entities::LwPolyline;

fn signed_bulge_radius(distance: f64, bulge: f64) -> f64 {
    distance * (1.0 + (bulge * bulge)) / 4.0 / bulge
}

fn angle(start_point: (f64, f64), end_point: (f64, f64)) -> f64 {
    (end_point.1 - start_point.1).atan2(end_point.0 - start_point.0)
}

fn polar(point: (f64, f64), angle: f64, radius: f64) -> (f64, f64) {
    (
        angle.cos() * radius + point.0,
        angle.sin() * radius + point.1,
    )
}

fn bulge_to_arc(
    start_point: (f64, f64),
    end_point: (f64, f64),
    bulge: f64,
) -> ((f64, f64), f64, f64, f64) {
    let distance =
        ((end_point.0 - start_point.0).powi(2) + (end_point.1 - start_point.1).powi(2)).sqrt();
    let r = signed_bulge_radius(distance, bulge);
    let a = angle(start_point, end_point) + (std::f64::consts::PI / 2.0 - bulge.atan() * 2.0);
    let c = polar(start_point, a, r);

    if bulge < 0.0 {
        (c, angle(c, end_point), angle(c, start_point), r.abs())
    } else {
        (c, angle(c, start_point), angle(c, end_point), r.abs())
    }
}

pub struct Point {
    pub x: f64,
    pub y: f64,
}
impl Point {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

pub struct Arc {
    pub to_point: Point,
    pub from_point: Point,
    pub center: Point,
    pub start_angle: f64,
    pub end_angle: f64,
    pub radius: f64,
    pub bulge: f64,
}

pub enum ArcMoveLineTo {
    Arc(Arc),
    Move(Point),
    LineTo(Point),
}

/// Used to help convert a dxf file to your own type
///
/// Convert an lwpolyline in a dxf file to a `Vec<ArcMoveLineTo>`
/// ArcMoveLineTo is simple an can either be a Line or a an arc
///
/// # Example
/// ```
/// use dxfexports::convert_lwpolyline_to_path;
/// use dxf::{Drawing, entities::EntityType};
///
/// let drawing = Drawing::load_file("test.dxf").expect("Not a good file");
/// let mut paths = Vec::new();
///
/// for e in drawing.entities() {
///     match e.specific {
///         EntityType::Circle(ref circle) => {
///             eprintln!("Circle: {:?}", circle);
///         }
///         EntityType::Line(ref line) => {
///             eprintln!("Line: {:?}", line);
///         }
///         EntityType::Spline(_) => {}
///         EntityType::LwPolyline(ref polyline) => {
///             let mut path = lyon::path::Path::svg_builder();
///             convert_lwpolyline_to_path(&polyline, &mut path);
///             path.close();
///             paths.push(path.build());
///         }
///         _ => {
///             eprintln!("Other: {:?}", e.specific);
///         }
///     }
/// }
/// ```
pub fn lwpolyline_to_arcs_and_lines(lwpolyline: &LwPolyline) -> Vec<ArcMoveLineTo> {
    let mut arcs = Vec::new();

    for i in 0..lwpolyline.vertices.len() {
        let vertex = lwpolyline.vertices[i];
        let last_vertex =
            lwpolyline.vertices[(i + lwpolyline.vertices.len() - 1) % lwpolyline.vertices.len()];
        if i == 0 {
            arcs.push(ArcMoveLineTo::Move(Point::new(
                last_vertex.x,
                last_vertex.y,
            )));
        }
        if last_vertex.bulge == 0.0 {
            arcs.push(ArcMoveLineTo::LineTo(Point::new(vertex.x, vertex.y)));
        } else {
            let (center, start_angle, end_angle, radius) = bulge_to_arc(
                (last_vertex.x, last_vertex.y),
                (vertex.x, vertex.y),
                last_vertex.bulge,
            );
            if !center.0.is_finite() {
                eprintln!(
                    "ERROR: CENTER: ({}, {}), start_angle: {}, end_angle: {}, radius: {}",
                    center.0, center.1, start_angle, end_angle, radius,
                );
                eprintln!("ERROR: NOT FINATE");
                continue;
            }
            if radius <= 0.0 {
                eprintln!("ERROR: RADIUS: {}", radius);
                continue;
            }

            arcs.push(ArcMoveLineTo::Arc(Arc {
                from_point: Point::new(last_vertex.x, last_vertex.y),
                to_point: Point::new(vertex.x, vertex.y),
                center: Point::new(center.0, center.1),
                start_angle,
                end_angle,
                radius,
                bulge: last_vertex.bulge,
            }));
        }
    }

    arcs
}
