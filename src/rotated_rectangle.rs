use ratatui::widgets::canvas::Line;

pub struct RotatedRectangle {
    pub x: f64,
    pub y: f64,
    pub width: f64,
    pub height: f64,
    pub angle: f64, // in radians
}

impl RotatedRectangle {
    pub fn new(x: f64, y: f64, width: f64, height: f64, angle: f64) -> Self {
        Self { x, y, width, height, angle }
    }

    pub fn draw(&self, ctx: &mut ratatui::widgets::canvas::Context) {
        // Calculate the corners of the rectangle
        let half_width = self.width / 2.0;
        let half_height = self.height / 2.0;
        let corners = [
            (-half_width, -half_height),
            ( half_width, -half_height),
            ( half_width,  half_height),
            (-half_width,  half_height),
        ];
        let corners: Vec<(f64, f64)> = corners.iter()
            .map(|&(dx, dy)| {
                let rotated_x = self.x + dx * self.angle.cos() - dy * self.angle.sin();
                let rotated_y = self.y + dx * self.angle.sin() + dy * self.angle.cos();
                (rotated_x, rotated_y)
            })
            .collect();
        
        // Draw the rectangle by connecting the corners
        for i in 0..corners.len() {
            let start_pt = corners[i];
            let end_pt = corners[(i + 1) % corners.len()];
            let line = Line::new(start_pt.0, start_pt.1, end_pt.0, end_pt.1, ratatui::style::Color::White);
            ctx.draw(&line);
        }
    }
}
