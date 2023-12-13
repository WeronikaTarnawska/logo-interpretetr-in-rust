use crate::parser::{Command, Expr};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum Value {
    String(String),
    Number(f32),
}
impl Value {
    fn get_number(&self) -> f32 {
        match self {
            Value::Number(x) => *x,
            _ => panic!("get_number: value not a number"),
        }
    }
}

pub fn eval(
    cmd: Command,
    functions: &mut HashMap<String, (Vec<String>, VecDeque<Command>)>,
    variables: &HashMap<String, Value>,
    image: &mut Image,
) {
    match cmd {
        Command::Forward(expr) => image.forward(eval_expr(expr, variables).get_number()),
        Command::Backward(expr) => image.backward(eval_expr(expr, variables).get_number()),
        Command::Right(expr) => image.right(eval_expr(expr, variables).get_number()),
        Command::Left(expr) => image.left(eval_expr(expr, variables).get_number()),
        Command::Show(expr) => println!("{:?}", eval_expr(expr, variables)),
        Command::Repeat(iters, body) => eval_loop(eval_expr(iters, variables).get_number(), body,functions, variables, image),
        Command::FunctionCall(name, args) => call_function(name, args, functions,variables, image),
        Command::FunctionDeclaration(name, args, cmds) => {
            functions.insert(name, (args, cmds));
        }
        _ => unreachable!(),
    }
}

fn call_function(
    name: String,
    arg_values: Vec<Expr>,
    functions: &mut HashMap<String, (Vec<String>, VecDeque<Command>)>,
    variables: &HashMap<String, Value>,
    image: &mut Image,
) {
    if let Some((arg_names, func_body)) = functions.get(&name) {
        let func_body = func_body.clone();
        if arg_names.len() != arg_values.len() {
            panic!("Incorrect number of arguments for function call");
        }
        let local_vars = arg_names
            .iter()
            .zip(arg_values)
            .map(|(arg, val)| (arg.clone(), eval_expr(val, variables)))
            .collect::<HashMap<String, Value>>();

        for cmd in func_body {
            eval(cmd.clone(), functions, &local_vars, image);
        }
    } else {
        panic!("Undefined function: {}", name);
    }
}

fn eval_loop(
    iters: f32,
    commands: VecDeque<Command>,
    functions: &mut HashMap<String, (Vec<String>, VecDeque<Command>)>,
    variables: &HashMap<String, Value>,
    image: &mut Image,
) {
    let n = iters as i32;
    for _i in 0..(n as i32) {
        for cmd in &commands {
            eval(cmd.clone(), functions, variables, image);
        }
    }
}

fn eval_expr(
    expr: Expr,
    variables: &HashMap<String, Value>
) -> Value {
    match expr {
        Expr::Number(n) => Value::Number(n),
        Expr::Add(e1, e2) => match (eval_expr(*e1, variables), eval_expr(*e2, variables)) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
            // (Value::String(n1), Value::String(n2)) => Value::String(n1+&n2),
            _ => panic!("add: wrong types"),
        },
        Expr::Mul(e1, e2) => match (eval_expr(*e1, variables), eval_expr(*e2, variables)) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 * n2),
            _ => panic!("add: wrong types"),
        },
        Expr::Variable(name) => match variables.get(&name) {
            Some(Value) => Value.clone(),
            _ => panic!("variable {} was not declared", name),
        }, // TODO env lookup
        Expr::Sub(e1, e2) => match (eval_expr(*e1, variables), eval_expr(*e2, variables)) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 - n2),
            // (Value::String(n1), Value::String(n2)) => Value::String(n1+&n2),
            _ => panic!("add: wrong types"),
        },
        Expr::Div(e1, e2) => match (eval_expr(*e1, variables), eval_expr(*e2, variables)) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 / n2),
            // (Value::String(n1), Value::String(n2)) => Value::String(n1+&n2),
            _ => panic!("add: wrong types"),
        },
        _ => unimplemented!()
    }
}

pub struct Image {
    x: f32,
    y: f32,
    angle: f32,
    svg: String,
    // width: f32, height: f32,
    pen_width: f32,
    pen_color: String,
}
impl Image {
    pub fn new(w: f32, h: f32) -> Self {
        Image {
            x: w / 2.0,
            y: h / 2.0,
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
