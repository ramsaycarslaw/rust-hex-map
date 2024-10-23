use ggez::graphics::{self, Color, Canvas, Mesh};
use ggez::{Context, GameResult};
use noise::{NoiseFn, Perlin};
use crate::hexagon;

pub struct Hexgrid {
    pub cells: Vec<hexagon::Hexagon>,
    pub selected_index: Option<usize>,
    mesh_batch: Vec<Mesh>, // Store individual hexagon meshes
}

impl Hexgrid {
    pub fn new(ctx: &mut Context, width: i32, height: i32, outer_radius: f32) -> GameResult<Hexgrid> {
        let mut cells = Vec::new();
        let inner_radius = outer_radius * 0.866025404;
        let perlin = Perlin::new();
        let scale = 0.1;

        let mut mesh_batch = Vec::new();

        for x in 0..width {
            for y in 0..height {
                let x_offset = if y % 2 == 0 {
                    x as f32 * inner_radius * 2.0
                } else {
                    (x as f32 + 0.5) * inner_radius * 2.0
                };
                let y_pos = y as f32 * 1.5 * outer_radius;

                let depth = perlin.get([x as f64 * scale, y as f64 * scale]) as f32;
                let color = match depth {
                    d if d < -0.5 => Color::new(0.0, 0.0, 0.5, 1.0), // Deep ocean blue
                    d if d < 0.0 => Color::new(0.0, 0.5, 1.0, 1.0), // Light blue
                    d if d < 0.1 => Color::new(0.859, 0.855, 0.573, 1.0),
                    d if d < 0.4 => Color::new(0.0, 0.6, 0.0, 1.0),
                    d if d < 0.6 => Color::new(0.6, 0.9, 0.6, 1.0),
                    d if d < 0.8 => Color::new(0.6, 0.6, 0.6, 1.0),
                    _ => Color::new(0.8, 0.8, 0.8, 1.0), // Light gray
                };

                // Create a hexagon mesh for each hexagon
                let hex_corners = hexagon::Hexagon::new(x_offset, y_pos, outer_radius, color).corners;
                let mesh = Mesh::new_polygon(ctx, graphics::DrawMode::fill(), &hex_corners, color)?;
                mesh_batch.push(mesh);
                
                // Add the hexagon data to the cell data
                cells.push(hexagon::Hexagon::new(x_offset, y_pos, outer_radius, color));
            }
        }

        Ok(Hexgrid {
            cells,
            selected_index: None,
            mesh_batch,
        })
    }

    pub fn select(&mut self, x: f32, y: f32) {
        for (i, cell) in self.cells.iter().enumerate() {
            if cell.contains(x, y) {
                self.selected_index = if self.selected_index == Some(i) { None } else { Some(i) };
                break;
            }
        }
    }

    pub fn draw(&mut self, ctx: &mut Context, canvas: &mut Canvas, draw_param: graphics::DrawParam) -> GameResult<()> {
        // Draw the individual hexagon meshes
        for mesh in &self.mesh_batch {
            canvas.draw(mesh, draw_param);
        }

        // Draw the selected hexagon's outline, if any
        if let Some(selected_index) = self.selected_index {
            let selected_cell = &self.cells[selected_index];
            let outline = Mesh::new_polygon(
                ctx,
                graphics::DrawMode::stroke(10.0),
                &selected_cell.corners,
                Color::new(1.0, 1.0, 0.0, 1.0),
            )?;
            canvas.draw(&outline, draw_param);
        }

        if let Some(image) = &self.cells[0].image {
            let image_draw_param = graphics::DrawParam::new()
                .dest([100.0, 100.0])
                .scale([0.5, 0.5]);
            canvas.draw(image, image_draw_param);
        }

        Ok(())
    }
}
