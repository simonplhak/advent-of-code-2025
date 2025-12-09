use anyhow::Result;
use geo::prelude::*;
use geo::{Polygon, Rect};
use std::path::Path;

use crate::utils::{Point2D, read_lines};

pub fn task_01(data_path: &Path) -> Result<String> {
    let points = load_points(data_path)?;
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let area = rectangle_area(&points[i], &points[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }
    Ok(format!("Max area: {}", max_area))
}

pub fn task_02(data_path: &Path) -> Result<String> {
    let points = load_points(data_path)?;
    let mask = create_mask(&points);
    let mut max_area = 0;
    for i in 0..points.len() {
        for j in (i + 1)..points.len() {
            let rect = create_geo_rectangle(&points[i], &points[j]);
            let x = mask.intersection(&rect);
            if x.unsigned_area() != rect.unsigned_area() {
                continue;
            }
            let area = rectangle_area(&points[i], &points[j]);
            if area > max_area {
                max_area = area;
            }
        }
    }
    Ok(format!("Answer: {}", max_area))
}

fn rectangle_area(p1: &Point2D, p2: &Point2D) -> usize {
    let width = p2.x.abs_diff(p1.x) + 1;
    let height = p2.y.abs_diff(p1.y) + 1;
    width * height
}

fn create_geo_rectangle(p1: &Point2D, p2: &Point2D) -> Polygon {
    let (min_x, max_x) = if p1.x < p2.x {
        (p1.x, p2.x)
    } else {
        (p2.x, p1.x)
    };
    let (min_y, max_y) = if p1.y < p2.y {
        (p1.y, p2.y)
    } else {
        (p2.y, p1.y)
    };
    Rect::new(
        geo::Coord {
            x: min_x as f64,
            y: min_y as f64,
        },
        geo::Coord {
            x: max_x as f64,
            y: max_y as f64,
        },
    )
    .to_polygon()
}

fn create_mask(points: &[Point2D]) -> Polygon {
    let points = points
        .iter()
        .map(|point| (point.x as f64, point.y as f64))
        .chain(std::iter::once((points[0].x as f64, points[0].y as f64)))
        .collect::<Vec<_>>();
    Polygon::new(geo::LineString::from(points), vec![])
}

fn load_points(data_path: &Path) -> Result<Vec<Point2D>> {
    let lines = read_lines(data_path)?;
    lines
        .iter()
        .map(|line| {
            let mut parts = line.split(',');
            let x: usize = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing x coordinate"))?
                .trim()
                .parse()?;
            let y: usize = parts
                .next()
                .ok_or_else(|| anyhow::anyhow!("Missing y coordinate"))?
                .trim()
                .parse()?;
            Ok(Point2D { x, y })
        })
        .collect()
}
