use std::default;

use ggez::graphics::Canvas;
use ggez::{graphics, mint::Point2, Context, GameResult, graphics::Color};

fn cross_product(a: Point2<f32>, b: Point2<f32>) -> f32 {
    a.x * b.y - a.y * b.x
}

pub struct Hexagon {
    pub corners: Vec<Point2<f32>>,
    pub color: graphics::Color,
    pub image: Option<graphics::Image>,
}

impl Hexagon {
    pub fn new(x: f32, y: f32, outer_radius: f32, color: graphics::Color) -> Hexagon {
        let inner_radius = outer_radius * 0.866025404;

        let corners = vec![
            Point2 { x: x, y: y - outer_radius },
            Point2 { x: x + inner_radius, y: y - outer_radius / 2.0 },
            Point2 { x: x + inner_radius, y: y + outer_radius / 2.0 },
            Point2 { x: x, y: y + outer_radius },
            Point2 { x: x - inner_radius, y: y + outer_radius / 2.0 },
            Point2 { x: x - inner_radius, y: y - outer_radius / 2.0 },
        ];

        let default_image = None;


        Hexagon {
            corners: corners,
            color: color,
            image: default_image,
        }
    }

    pub fn contains(&self, x: f32, y: f32) -> bool {
        let mut inside = false;

        let n = self.corners.len();

        for i in 0..n {
            let j = (i + 1) % n;
            let vertex1 = self.corners[i];
            let vertex2 = self.corners[j];
            
            if (vertex1.y > y) != (vertex2.y > y) {
                let intersect_x = (vertex2.x - vertex1.x) * (y - vertex1.y) / (vertex2.y - vertex1.y) + vertex1.x;
                if x < intersect_x {
                    inside = !inside;
                }
            }
        }
        inside
    }

}