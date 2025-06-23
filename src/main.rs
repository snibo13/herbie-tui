use color_eyre::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind, KeyModifiers};
use ratatui::{
    DefaultTerminal, Frame,
    layout::{ Constraint, Flex,  Layout},
    style::Stylize,
    text::Line,
    widgets::{
        canvas::{Canvas, Circle},
        Paragraph,
        Block},
};
use ratatui::text::Text;
use std::{
    time::{Duration, Instant},
};
use local_ip_address::local_ip;
use gethostname::gethostname;

// Include the herbie.rs file to use the H.E.R.B.I.E logo
mod herbie;
mod righteye;
mod lefteye;
mod rotated_rectangle;
use righteye::RightEye;
use lefteye::LeftEye;


fn main() -> color_eyre::Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let result = App::new().run(terminal);
    ratatui::restore();
    result
}




/// The main application which holds the state and logic of the application.
#[derive(Debug, Default)]
pub struct App {
    /// Is the application running?
    running: bool,
    theta_left: f64, // angle in radians
    theta_right: f64, // angle in radians
    mouth_height: f64, // Height of the mouth, can be adjusted for animation
    min_mouth_height: f64, // Minimum height of the mouth
    max_mouth_height: f64, // Maximum height of the mouth
    ip: String, // Local IP address
    hostname: String, // Hostname of the machine
    left_eye_channel: String,
    right_eye_channel: String,
    mouth_channel: String,
}

impl App {
    /// Construct a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
        
        // theta: 0.0, // Initialize the angle to 0 radians
    }

    pub fn default() -> Self {
        Self {
            running: false,
            theta_left: 0.0, // Initialize the left eye angle to 0 radians
            theta_right: 0.0, // Initialize the right eye angle to 0 radians
            mouth_height: 18.0, // Initialize the mouth height
            min_mouth_height: 18.0, // Minimum height of the mouth
            max_mouth_height: 27.0, // Maximum height of the mouth
            ip: String::new(), // Initialize the local IP address
            hostname: String::new(), // Initialize the hostname
            left_eye_channel: "left_eye".to_string(), // Channel for the left eye
            right_eye_channel: "right_eye".to_string(), // Channel for the right eye
            mouth_channel: "mouth".to_string(), // Channel for the mouth
        }
    }

    /// Run the application's main loop.
    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        self.running = true;
        let tick_rate = Duration::from_millis(16); // ~60 FPS (1000ms / 60 ≈ 16.67ms)
        let mut last_tick = Instant::now();
        
        match local_ip() {
            Ok(ip) => {
                self.ip = ip.to_string();
            }
            Err(e) => println!("Error getting local IP: {}", e),
        }

        
        self.hostname = gethostname().to_string_lossy().to_string();
            
        

        
        while self.running {
            terminal.draw(|frame| self.render(frame))?;
            
            let timeout = tick_rate.saturating_sub(last_tick.elapsed());
            if event::poll(timeout)? {
                self.handle_crossterm_events()?;
            }
            
            if last_tick.elapsed() >= tick_rate {
                self.on_tick();
                last_tick = Instant::now();
            }
        }
        Ok(())
    }

    fn on_tick(&mut self) {
        // This method can be used to update the state of the application on each tick.
        // For example, you can update animations, timers, etc.
        
    }

    fn render(&mut self, frame: &mut Frame) {
        let _title = Line::from("Ratatui Simple Template")
            .bold()
            .blue();
        let _text = Text::from_iter([
            "H.E.R.B.I.E. TUI".bold(),
            "<q> Quit | <esc> Exit | <Ctrl-C> Exit".into(),
        ]);
        let vertical = Layout::vertical([
            Constraint::Length(_text.height() as u16),
            Constraint::Length(40), // Adjusted to fit the new layout
            Constraint::Min(5),
        ]);
        let [hdr, up, down] = vertical.areas(frame.area());
        let horizontal = Layout::horizontal([
            Constraint::Length(125), // Adjusted to fit the new layout
            Constraint::Min(5),
        ]);
        let [face, right_column] = horizontal.areas(up);
        let le = LeftEye::new(40.0, 60.0, self.theta_left);
        let re = RightEye::new(150.0, 60.0, self.theta_right);
        
       
        frame.render_widget(
            Canvas::default()
                // .background_color(ratatui::style::Color::White)
                .block(Block::bordered())
                .paint(|ctx| {
                    // Left Eye
                    le.draw(ctx);
                    // Right Eye
                    re.draw(ctx);
                    // mouth
                    ctx.draw(&Circle {
                        x: 87.0,
                        y: self.mouth_height,
                        radius: 6.0,
                        color: ratatui::style::Color::White,
                    });
                    ctx.draw(&Circle {
                        x: 103.0,
                        y: self.mouth_height,
                        radius: 6.0,
                        color: ratatui::style::Color::White,
                    });
                    


                })
                .x_bounds([0.0, 200.0])
                .y_bounds([0.0, 100.0]),
            face
        );

        let right_column_areas =  Layout::vertical([
            Constraint::Length(7),
            Constraint::Length(1),
            Constraint::Length(1),
            Constraint::Length(6),
            Constraint::Length(6),
        ]);

        let [logo, tagline, version, instructions, status] = right_column_areas.areas(right_column);

        let center_logo = Layout::horizontal([Constraint::Min(1), Constraint::Min(50), Constraint::Min(1)]).flex(Flex::Center);
        let [_,logo,_] = center_logo.areas(logo);
        let sub_center = Layout::horizontal([
            Constraint::Length(18),
            Constraint::Length(40),
            Constraint::Min(1),
        ]);
        let [_, tagline, _] = sub_center.areas(tagline);
        let [_, version, _] = sub_center.areas(version);
        let [_, instructions, _] = sub_center.areas(instructions);
        let [_, status, _] = sub_center.areas(status);

        frame.render_widget(
            herbie::HerbieLogo::tiny(),
            logo,
        );

        frame.render_widget(
            Text::raw("Reed Richards").bold().white(),
            tagline,
        );
        frame.render_widget(
            Text::raw("v0.1.0").bold().white(),
            version,
        );
        frame.render_widget(
            Text::raw("<w> and <s> to rotate the left eye \n<r> and <f> to rotate the right eye \n<e>/<d> to raise and lower the mouth  \n<q> or <esc> to quit.").bold().white(),
            instructions,
        );

        // Get IP address
        

        frame.render_widget(
            Paragraph::new(
                vec![
                    Line::from(format!("Local IP: {}\n", self.ip)).bold().white(),
                    Line::from(format!("Hostname: {}\n", self.hostname)).bold().white(),
                    Line::from(format!("Left Eye Channel: {}\n", self.left_eye_channel)).bold().white(),
                    Line::from(format!("Right Eye Channel: {}\n", self.right_eye_channel)).bold().white(),
                    Line::from(format!("Mouth Channel: {}\n", self.mouth_channel)).bold().white(),
                    ]
            ),
            status,
        );
    }

    /// Reads the crossterm events and updates the state of [`App`].
    ///
    /// If your application needs to perform work in between handling events, you can use the
    /// [`event::poll`] function to check if there are any events available with a timeout.
    fn handle_crossterm_events(&mut self) -> Result<()> {
        if let Ok(event) = event::read() {
            match event {
                // it's important to check KeyEventKind::Press to avoid handling key release events
                Event::Key(key) if key.kind == KeyEventKind::Press => self.on_key_event(key),
                Event::Mouse(_) => {}
                Event::Resize(_, _) => {}
                _ => {}
            }
        }
        Ok(())
    }

    /// Handles the key events and updates the state of [`App`].
    fn on_key_event(&mut self, key: KeyEvent) {
        match (key.modifiers, key.code) {
            (_, KeyCode::Esc | KeyCode::Char('q'))
            | (KeyModifiers::CONTROL, KeyCode::Char('c') | KeyCode::Char('C')) => self.quit(),
            (_,KeyCode::Char('w')) => self.theta_left += 0.1,
            (_,KeyCode::Char('s')) => self.theta_left -= 0.1,
            (_,KeyCode::Char('r')) => self.theta_right += 0.1,
            (_,KeyCode::Char('f')) => self.theta_right -= 0.1,
            (_,KeyCode::Char('e')) => {
                self.mouth_height += 1.0;
                if self.mouth_height > self.max_mouth_height {
                self.mouth_height = self.max_mouth_height;
                }
            }, // Raise the mouth height
            (_,KeyCode::Char('d')) => {
                self.mouth_height -= 1.0;
                if self.mouth_height < self.min_mouth_height {
                    self.mouth_height = self.min_mouth_height; // Prevent negative height
                }
            }
            _ => {}
        }
    }

    /// Set running to false to quit the application.
    fn quit(&mut self) {
        self.running = false;
    }
}
