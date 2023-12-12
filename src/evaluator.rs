use crate::parser::{Command, Expr};
use std::collections::{HashMap, VecDeque};
use std::fs::File;
use std::io::Write;

#[derive(Debug)]
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

pub struct EvalEnv {
    pub image: Image,
    functions: HashMap<String, (Vec<String>, VecDeque<Command>)>,
    variables: HashMap<String, Value>,
}
impl EvalEnv {
    pub fn new(w: f32, h: f32) -> Self {
        EvalEnv {
            image: Image::new(w, h),
            functions: HashMap::new(),
            variables: HashMap::new(),
        }
    }

    pub fn eval(&mut self, cmd: Command, env: &HashMap<String, Value>) {
        match cmd {
            Command::Forward(expr) => self.image.forward(self.eval_expr(expr, env).get_number()),
            Command::Backward(expr) => self.image.backward(self.eval_expr(expr, env).get_number()),
            Command::Right(expr) => self.image.right(self.eval_expr(expr, env).get_number()),
            Command::Left(expr) => self.image.left(self.eval_expr(expr, env).get_number()),
            Command::Show(expr) => println!("{:?}", self.eval_expr(expr, env)),
            Command::Repeat(iters, body) => {
                self.eval_loop(self.eval_expr(iters, env).get_number(), body, env)
            }
            Command::FunctionCall(name, args) => self.call_function(name, args, env),
            Command::FunctionDeclaration(name, args, cmds) => {
                self.functions.insert(name, (args, cmds));
            }
            _ => unreachable!(),
        }
    }

    fn call_function(&mut self, name: String, arg_values: Vec<Expr>, env: &HashMap<String, Value>) {
        if let Some((arg_names, func_body)) = self.functions.get(&name) {
            if arg_names.len() != arg_values.len() {
                panic!("Incorrect number of arguments for function call");
            }
            let mut local_vars = arg_names
                .iter()
                .zip(arg_values)
                .map(|(arg, val)| (arg.clone(), self.eval_expr(val, env)))
                .collect::<HashMap<String, Value>>();
            {
                self.eval_block(func_body, &local_vars);
            }
        } else {
            panic!("Undefined function: {}", name);
        }
    }

    fn eval_block(&mut self, commands: &VecDeque<Command>, env: &HashMap<String, Value>) {
        for cmd in commands {
            self.eval(cmd.clone(), &env);
        }
    }

    fn eval_loop(&mut self, iters: f32, commands: VecDeque<Command>, env: &HashMap<String, Value>) {
        let n = iters as i32;
        for _i in 0..(n as i32) {
            for cmd in &commands {
                self.eval(cmd.clone(), env);
            }
        }
    }

    fn eval_expr(&self, expr: Expr, env: &HashMap<String, Value>) -> Value {
        match expr {
            Expr::Number(n) => Value::Number(n),
            Expr::Add(e1, e2) => match (self.eval_expr(*e1, env), self.eval_expr(*e2, env)) {
                (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 + n2),
                // (Value::String(n1), Value::String(n2)) => Value::String(n1+&n2),
                _ => panic!("add: wrong types"),
            },
            Expr::Mul(e1, e2) => match (self.eval_expr(*e1, env), self.eval_expr(*e2, env)) {
                (Value::Number(n1), Value::Number(n2)) => Value::Number(n1 * n2),
                _ => panic!("add: wrong types"),
            },
            Expr::Variable(name) => match env.get(&name) {
                Some(Value) => *Value,
                _ => panic!("variable {} was not declared", name),
            }, // TODO env lookup
        }
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
    fn new(w: f32, h: f32) -> Self {
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
