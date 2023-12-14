use crate::parser::{Command, Expr};
use rand::Rng;
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Write;

#[derive(Debug, Clone)]
pub enum Value {
    _String(String),
    Color(String),
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

pub enum LogoErr {
    Stop,
}

pub fn eval_all(
    ast: VecDeque<Command>,
    functions: &mut HashMap<String, (Vec<String>, VecDeque<Command>)>,
    variables: &HashMap<String, Value>,
    image: &mut Image,
) -> Result<(), LogoErr> {
    for cmd in ast {
        // println!(" Parsed to:\n{:?}", cmd);
        match eval(cmd, functions, variables, image) {
            Err(e) => {
                return Err(e);
            }
            Ok(()) => {}
        };
    }
    Ok(())
}

fn eval(
    cmd: Command,
    functions: &mut HashMap<String, (Vec<String>, VecDeque<Command>)>,
    variables: &HashMap<String, Value>,
    image: &mut Image,
) -> Result<(), LogoErr> {
    match cmd {
        Command::Forward(expr) => {
            image.forward(eval_expr(expr, variables).get_number());
            Ok(())
        }
        Command::Backward(expr) => {
            image.backward(eval_expr(expr, variables).get_number());
            Ok(())
        }
        Command::Right(expr) => {
            image.right(eval_expr(expr, variables).get_number());
            Ok(())
        }
        Command::Left(expr) => {
            image.left(eval_expr(expr, variables).get_number());
            Ok(())
        }
        Command::PenDown => {
            image.pendown();
            Ok(())
        }
        Command::PenUp => {
            image.penup();
            Ok(())
        }
        Command::Show(expr) => {
            println!("{:?}", eval_expr(expr, variables));
            Ok(())
        }
        Command::Repeat(iters, body) => eval_loop(
            eval_expr(iters, variables).get_number(),
            body,
            functions,
            variables,
            image,
        ),
        Command::If(pred, ifcommands) => eval_ifelse(
            eval_expr(pred, variables).get_number(),
            ifcommands,
            VecDeque::new(),
            functions,
            variables,
            image,
        ),
        Command::IfElse(pred, ifcommands, elsecommands) => eval_ifelse(
            eval_expr(pred, variables).get_number(),
            ifcommands,
            elsecommands,
            functions,
            variables,
            image,
        ),
        Command::FunctionCall(name, args) => call_function(name, args, functions, variables, image),
        Command::FunctionDeclaration(name, args, cmds) => {
            functions.insert(name, (args, cmds));
            Ok(())
        }
        Command::Clearscreen => {
            image.clear();
            Ok(())
        }
        Command::Stop => Err(LogoErr::Stop),
        Command::Setcolor(cmd) => {
            let col = eval_expr(cmd, variables);
            match col {
                Value::Color(c) => image.setcolor(c),
                _ => unimplemented!("setcolor: unimplemented"),
            }
            Ok(())
        }
        _ => panic!("invalid command"),
    }
}

fn eval_ifelse(
    pred: f32,
    ifcommands: VecDeque<Command>,
    elsecommands: VecDeque<Command>,
    functions: &mut HashMap<String, (Vec<String>, VecDeque<Command>)>,
    variables: &HashMap<String, Value>,
    image: &mut Image,
) -> Result<(), LogoErr> {
    let n = pred != 0.0;
    if n {
        eval_all(ifcommands, functions, variables, image)
    } else {
        eval_all(elsecommands, functions, variables, image)
    }
}

fn call_function(
    name: String,
    arg_values: Vec<Expr>,
    functions: &mut HashMap<String, (Vec<String>, VecDeque<Command>)>,
    variables: &HashMap<String, Value>,
    image: &mut Image,
) -> Result<(), LogoErr> {
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

        _ = eval_all(func_body, functions, &local_vars, image);
        Ok(())
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
) -> Result<(), LogoErr> {
    let n = iters as i32;
    for _i in 0..(n as i32) {
        if let Err(e) = eval_all(commands.clone(), functions, variables, image) {
            return Err(e);
        };
    }
    Ok(())
}

fn eval_list(exprs: VecDeque<Expr>, variables: &HashMap<String, Value>) -> Vec<Value> {
    let mut result = vec![];
    for e in exprs {
        result.push(eval_expr(e, variables))
    }
    result
}

fn eval_expr(expr: Expr, variables: &HashMap<String, Value>) -> Value {
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
            Some(value) => value.clone(),
            _ => panic!("variable {} was not declared", name),
        },
        Expr::Sub(e1, e2) => match (eval_expr(*e1, variables), eval_expr(*e2, variables)) {
            (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 - n2),
            _ => panic!("sub: wrong types"),
        },
        Expr::Div(e1, e2) => match (eval_expr(*e1, variables), eval_expr(*e2, variables)) {
            (Value::Number(n1), Value::Number(n2)) => {
                if n2 == 0.0 {
                    panic!("Attempt to divide by 0")
                } else {
                    Value::Number(n1 / n2)
                }
            }
            _ => panic!("div: wrong types"),
        },
        Expr::Minus(e) => match eval_expr(*e, variables) {
            Value::Number(n) => Value::Number(-n),
            _ => panic!("add: wrong types"),
        },
        Expr::Lt(e1, e2) => match (eval_expr(*e1, variables), eval_expr(*e2, variables)) {
            (Value::Number(n1), Value::Number(n2)) => {
                Value::Number(if n1 < n2 { 1.0 } else { 0.0 })
            }
            _ => panic!("(<): wrong types"),
        },
        Expr::Rand(e) => match eval_expr(*e, variables) {
            Value::Number(n) => {
                let mut rng = rand::thread_rng();
                Value::Number(rng.gen_range(0..(n as i32)) as f32)
            }
            _ => panic!("rand: wrong types"),
        },
        Expr::Color(c) => Value::Color(c),
        Expr::Pick(exprs) => {
            let lst = eval_list(exprs, variables);
            let mut rng = rand::thread_rng();
            if let Some(v) = lst.get(rng.gen_range(0..lst.len())) {
                v.clone()
            } else {
                panic!("pick needs at least one option, bu vector is empty")
            }
        }
    }
}

pub struct Image {
    x: f32,
    y: f32,
    angle: f32,
    svg: String,
    width: f32,
    height: f32,
    pen_width: f32,
    pen_color: String,
    pen_active: bool,
}
impl Image {
    pub fn new(w: f32, h: f32) -> Self {
        Image {
            x: w / 2.0,
            y: h / 2.0,
            angle: -90.0,
            width: w,
            height: h,
            pen_color: "black".to_string(),
            pen_width: 1.0,
            pen_active: true,
            svg: format!("<svg width=\"{}\" height=\"{}\">", w, h).to_string(),
        }
    }

    fn clear(&mut self) {
        self.svg = format!("<svg width=\"{}\" height=\"{}\">", self.width, self.height).to_string()
    }

    fn setcolor(&mut self, color: String) {
        self.pen_color = color;
    }

    fn penup(&mut self) {
        self.pen_active = false;
    }
    fn pendown(&mut self) {
        self.pen_active = true;
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
        if self.pen_active {
            self.add_line_to_svg(self.x, self.y, new_x, new_y);
        }
        self.x = new_x;
        self.y = new_y;
        // println!("image-forward {}, {}", self.x, self.y);
    }

    fn backward(&mut self, dist: f32) {
        self.forward(-dist);
        // println!("image-backward {}, {}", self.x, self.y);
    }

    fn right(&mut self, angle: f32) {
        self.angle += angle;
        // println!("image-right {}", self.angle);
    }

    fn left(&mut self, angle: f32) {
        self.angle -= angle;
        // println!("image-left {}", self.angle);
    }
    pub fn save_svg(&mut self, filename: &str) {
        self.svg.push_str("</svg>");
        let mut file = File::create(filename).expect("Unable to create SVG file");
        file.write_all(self.svg.as_bytes())
            .expect("Unable to write SVG content to file");
    }
}
