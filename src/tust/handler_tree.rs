use crate::tust::request::Request;
use crate::tust::response::Response;

pub type PathHandler = fn(&mut Request, &mut Response) -> ();

pub struct HandlerTree {
    path: String,
    method: String,
    children: Vec<HandlerTree>,
    handlers: Vec<PathHandler>
}

#[allow(dead_code)]
impl HandlerTree {
    pub fn new() -> Self {
        HandlerTree {
            path: String::new(),
            method: String::new(),
            children: vec![],
            handlers: vec![]
        }
    }

    // TODO [7] Add path variables to resolver
    pub fn resolve(&self, path_vec: &[&str], req: &mut Request, res: &mut Response) {
        if res.complete { return; }

        for child in &self.children {
            let is_var = child.path.chars().nth(0) == Some(':');

            if child.path.eq(path_vec[0]) | child.path.eq("*") | is_var {
                if is_var {
                    let key = &child.path[1..];
                    let value = path_vec[0];

                    req.path_variables.insert(key.to_string(), value.to_string());
                }

                if path_vec.len() == 1 {
                    if child.method.eq(&req.method) | child.method.eq("*") {
                        for h in &child.handlers {
                            h(req, res);
                            if res.complete { return; }
                        }
                    }
                } else {
                    child.resolve(&path_vec[1..], req, res);
                }

                if is_var {
                    let key = &child.path[1..];
                    req.path_variables.remove(key);
                }
            }
        }
    }

    pub fn add(&mut self, method: &str, path: &[&str], handler: PathHandler) {
        for (i, child) in self.children.iter_mut().enumerate() {
            if child.path.eq(path[0]) & child.method.eq(method) {
                if path.len() == 1 {
                    self.children[i].handlers.push(handler.to_owned());
                } else {
                    child.add(method, &path[1..], handler);
                }
                return;
            }
        }

        self.children.push(HandlerTree {
            path: path[0].to_owned(),
            method: "".to_owned(),
            children: vec![],
            handlers: vec![]
        });

        let handler_tree = self.children.last_mut().unwrap();

        if path.len() == 1 {
            handler_tree.method = method.to_owned();
            handler_tree.handlers.push(handler)
        } else {
            handler_tree.add(method, &path[1..], handler);
        }
    }

    pub fn print_tree(&self, level: u8) {
        println!("handler: {} {} {}", self.method, self.path, level);
        for handler in &self.children {
            handler.print_tree(level + 1);
        }
    }

    pub fn shrink_to_fit(&mut self) {
        self.path.shrink_to_fit();
        self.method.shrink_to_fit();
        self.children.shrink_to_fit();
        self.handlers.shrink_to_fit();

        for child in &mut self.children {
            child.shrink_to_fit();
        }
    }
}