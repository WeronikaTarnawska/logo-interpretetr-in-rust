use crate::parser::{Command, Expr};
use std::collections::VecDeque;
use std::fs::File;
use std::io::Write;

pub fn eval(cmd: Command, image: &mut Image) {
    match cmd {
        Command::Forward(expr) => image.forward(eval_expr(expr)),
        Command::Backward(expr) => image.backward(eval_expr(expr)),
        Command::Right(expr) => image.right(eval_expr(expr)),
        Command::Left(expr) => image.left(eval_expr(expr)),
        Command::Show(expr) => println!("{}", eval_expr(expr)),
        Command::Repeat(iters, body ) => eval_loop(eval_expr(iters), body, image)
    }
}

fn eval_loop(iters:f32, commands: VecDeque<Command>, image: &mut Image){
    let n = iters as i32;
    for _i in 0..n{
        for cmd in &commands{
            eval(cmd.clone(), image);
        }
    }
}

fn eval_expr(expr: Expr) -> f32 {
    match expr {
        Expr::Number(n) => n,
        Expr::Add(e1, e2) => eval_expr(*e1) + eval_expr(*e2),
        Expr::Mul(e1, e2) => eval_expr(*e1) * eval_expr(*e2),
    }
}

pub struct Image {
    x: f32,
    y: f32,
    angle: f32,
    svg: String,
    // width: f32, height: f32,
    pen_width: f32, pen_color: String
}
impl Image {
    pub fn new(w:f32, h:f32) -> Self {
        Image {
            x: w/2.0,
            y: h/2.0,
            angle: 0.0,
            // width: w,
            // height: h,
            pen_color: "red".to_string(),
            pen_width: 2.0,
            svg: format!("<svg width=\"{}\" height=\"{}\">", w, h).to_string(),
        }
    }

    fn calculate_new_position(&self, dist: f32) -> (f32, f32) {
        let angle_rad = self.angle.to_radians();
        let new_x = self.x + dist * angle_rad.cos();
        let new_y = self.y + dist * angle_rad.sin();
        (new_x, new_y)
    }
    fn add_line_to_svg(&mut self, x1: f32, y1: f32, x2: f32, y2: f32) {
        let line = format!(
            "<line x1=\"{}\" y1=\"{}\" x2=\"{}\" y2=\"{}\" stroke=\"{}\" stroke-width=\"{}\" />\n",
            x1, y1, x2, y2, self.pen_color, self.pen_width
        );
        self.svg.push_str(&line);
    }
    fn forward(&mut self, dist: f32) {
        let (new_x, new_y) = self.calculate_new_position(dist);
        self.add_line_to_svg(self.x, self.y, new_x, new_y);
        self.x = new_x;
        self.y = new_y;
    }

    fn backward(&mut self, dist: f32) {
        self.forward(-dist);
    }

    fn right(&mut self, angle: f32) {
        self.angle -= angle;
    }

    fn left(&mut self, angle: f32) {
        self.angle += angle;
    }
    pub fn save_svg(&mut self, filename: &str) {
        self.svg.push_str("</svg>");
        let mut file = File::create(filename).expect("Unable to create SVG file");
        file.write_all(self.svg.as_bytes())
            .expect("Unable to write SVG content to file");
    }
}
