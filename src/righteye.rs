use ratatui::{
    widgets::canvas::Circle,
};

use crate::rotated_rectangle::RotatedRectangle;

pub struct RightEye {
    pub x: f64,
    pub y: f64,
    pub theta: f64, // in radians
}

impl RightEye {
    pub fn new(x: f64, y: f64, theta: f64) -> Self {
        Self { x, y, theta }
    }

    pub fn draw(&self, ctx: &mut ratatui::widgets::canvas::Context) {
        let r = 17.0;
        let t2 = self.theta + std::f64::consts::PI / 2.0;
        let t3 = self.theta + 2.0 * std::f64::consts::PI / 2.0;
        let t4 = self.theta + 3.0 * std::f64::consts::PI / 2.0;
        let c1 = RotatedRectangle::new(self.x + r * self.theta.cos(),   self.y + r *  self.theta.sin(),   14.0, 8.0, self.theta);
        let c2 = RotatedRectangle::new(self.x + r * t2.cos(),           self.y + r *  t2.sin(),           14.0, 8.0, t2);
        let c3 = RotatedRectangle::new(self.x + r * t3.cos(),           self.y + r *  t3.sin(),           14.0, 8.0, t3);
        let c4 = RotatedRectangle::new(self.x + r * t4.cos(),           self.y + r *  t4.sin(),           14.0, 8.0, t4);
        ctx.draw(&Circle {
                x: self.x,
                y: self.y,
                radius: 30.0,
                color: ratatui::style::Color::White,
            });
            ctx.draw(&Circle {
                x: self.x,
                y: self.y,
                radius: 10.0,
                color: ratatui::style::Color::White,
            });
             ctx.draw(&Circle {
                x: self.x ,
                y: self.y - 45.0,
                radius: 10.0,
                color: ratatui::style::Color::White,
            });
             ctx.draw(&Circle {
                x: self.x - 27.0,
                y: self.y - 30.0,
                radius: 5.0,
                color: ratatui::style::Color::White,
            });
            c1.draw(ctx);
            c2.draw(ctx);
            c3.draw(ctx);
            c4.draw(ctx);
    }
}
