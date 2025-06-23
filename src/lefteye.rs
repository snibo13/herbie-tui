use ratatui::{
    style::Stylize,
    text::{Line, Span},
    widgets::canvas::Circle,
};


use crate::rotated_rectangle::RotatedRectangle;

pub struct LeftEye {
    pub x: f64,
    pub y: f64,
    pub theta: f64, // in radians
}

impl LeftEye {
    pub fn new(x: f64, y: f64, theta: f64) -> Self {
        Self { x, y, theta }
    }

    pub fn draw(&self, ctx: &mut ratatui::widgets::canvas::Context) {
        let r = 20.0;
        let rotated_rect = RotatedRectangle::new(self.x + r * self.theta.cos(), self.y + r *  self.theta.sin(), 12.0, 12.0, self.theta);
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
                x: self.x + 27.0,
                y: self.y - 30.0,
                radius: 5.0,
                color: ratatui::style::Color::White,
            });
        rotated_rect.draw(ctx);
    }
}