use std::fs::File;
use std::io::Read;

use clap::{arg, ArgAction, Command, value_parser};
use geo::{coord, Geometry, GeometryCollection, Line, LineString, Point, Polygon, Rect};
use geo::bounding_rect::BoundingRect;
use geojson::{GeoJson, quick_collection};
use image::{Rgba, RgbaImage};
use imageproc::drawing::{draw_antialiased_line_segment_mut, draw_filled_circle_mut};
use imageproc::pixelops::interpolate;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = Command::new("GeoMage")
        .version("0.1.0")
        .author("Florian B. <florian@flob.fr>")
        .about("Convert geojson to image")
        .arg(
            arg!(
                -b --bbox "enable bboxing"
            ).action(ArgAction::SetTrue))
        .arg(
            arg!(
                -i --input <INPUT_FILE> "Sets the input geojson file to use"
            ).required(true))
        .arg(
            arg!(
                -o --output <OUTPUT_FILE> "Sets the output image file"
            ).required(true))
        .arg(
            arg!(
                -w --width <WIDTH> "Specifies the desired width in pixel of output image. Default is 1024px"
            ).default_value("1024").required(false)
                .value_parser(value_parser!(u32))
        ).get_matches();


    let input = matches.get_one::<String>("input").expect("input parameter is mandatory");
    let output = matches.get_one::<String>("output").expect("output parameter is mandatory");
    let image_width: u32 = *matches.get_one::<u32>("width").expect("Width must be an integer");

    let mut geojson_file = File::open(input)?;
    let mut geojson_file_content = String::new();

    //parse geojson input
    geojson_file.read_to_string(&mut geojson_file_content)?;
    let geojson = geojson_file_content.parse::<GeoJson>()?;
    let geometries: GeometryCollection<f64> = quick_collection(&geojson).expect("Error while reading geojson");


    let bbox: Rect;
    let mut image_height = image_width;
    if matches.get_flag("bbox") {
        bbox = geometries.bounding_rect().expect("Unable to compute bounding rect");
        let ratio = bbox.width() / bbox.height();
        image_height = (image_width as f64 / ratio) as u32;
    } else {
        bbox = Rect::new(
            coord! { x: -180.0, y: -90.0},
            coord! { x: 180.0, y: 90.0 },
        );
    }

    let paint_color = Rgba([0, 0, 0, 255]);

    let mut image = RgbaImage::new(image_width, image_height);
    image.fill(255);

    for geo in &geometries {
        draw_geometry(geo, &mut image, bbox, paint_color)
    }

    image.save(output).expect("Error while saving image");
    Ok(())
}


fn draw_geometry(geometry: &Geometry, image: &mut RgbaImage, bbox: Rect, color: Rgba<u8>) {
    match geometry {
        Geometry::Point(ref point) => {
            draw_point(point, image, bbox, color);
        }
        Geometry::Line(ref line) => {
            draw_line(line, image, bbox, color)
        }
        Geometry::LineString(ref ls) => {
            draw_line_string(ls, image, bbox, color);
        }
        Geometry::Polygon(ref polygon) => {
            draw_polygon(polygon, image, bbox, color);
        }
        Geometry::MultiPoint(ref points) => {
            for point in points {
                draw_point(point, image, bbox, color);
            }
        }
        Geometry::MultiLineString(ref mls) => {
            for ls in mls {
                draw_line_string(ls, image, bbox, color);
            }
        }
        Geometry::MultiPolygon(ref multi_poly) => {
            for poly in multi_poly {
                draw_polygon(poly, image, bbox, color)
            }
        }
        Geometry::GeometryCollection(geos) => {
            for geo in geos {
                draw_geometry(geo, image, bbox, color)
            }
        }
        Geometry::Rect(ref rec) => {
            draw_polygon(&rec.to_polygon(), image, bbox, color)
        }
        Geometry::Triangle(ref tri) => {
            draw_polygon(&tri.to_polygon(), image, bbox, color)
        }
    }
}

fn draw_point(point: &Point<f64>, image: &mut RgbaImage, bbox: Rect, color: Rgba<u8>) {
    let center = project_coordinate(point.x_y(), image, bbox);
    draw_filled_circle_mut(image, (center.0, center.1), 2, color);
}

fn draw_line_string(ls: &LineString<f64>, image: &mut RgbaImage, bbox: Rect, color: Rgba<u8>) {
    let mut line_start: Option<(i32, i32)> = None;
    for point in ls.points() {
        let coor = project_coordinate(point.x_y(), image, bbox);
        match line_start {
            Some(ref start) => {
                draw_antialiased_line_segment_mut(image, (start.0, start.1), (coor.0, coor.1), color, &interpolate);
            }
            _ => {}
        }
        line_start = Some((coor.0 as i32, coor.1 as i32));
    }
}

fn draw_line(line: &Line<f64>, image: &mut RgbaImage, bbox: Rect, color: Rgba<u8>) {
    let start = project_coordinate(line.start.x_y(), image, bbox);
    let end = project_coordinate(line.end.x_y(), image, bbox);
    draw_antialiased_line_segment_mut(image, (start.0, start.1), (end.0, end.1), color, &interpolate);
}

fn draw_polygon(polygon: &Polygon<f64>, image: &mut RgbaImage, bbox: Rect, color: Rgba<u8>) {
    for line in polygon.exterior().lines() {
        draw_line(&line, image, bbox, color);
    }
}

fn project_coordinate(coordinate: (f64, f64), image: &RgbaImage, bbox: Rect) -> (i32, i32) {
    let height = image.height() as f64;
    let width = image.width() as f64;

    let x = (coordinate.0 + (bbox.min().x * -1.0)) * (width / bbox.width());
    let mut y = height - ((coordinate.1 + (bbox.min().y * -1.0)) * (height / bbox.height()));
    if y.is_infinite() {
        y = height;
    } else if y.is_nan() {
        y = 0.0;
    }
    (x as i32, y as i32)
}