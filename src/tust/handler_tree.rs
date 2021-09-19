use crate::tust::request::Request;
use crate::tust::response::Response;

pub type PathHandler = fn(&mut Request, &mut Response) -> ();

pub struct HandlerTree {
    path: String,
    method: String,
    children: Vec<HandlerTree>,
    handlers: Vec<PathHandler>
}

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
    pub fn resolve(&self, req: &mut Request, res: &mut Response) {
        let p = req.path.clone();
        let path: Vec<&str> = p.split("/").skip(1).collect();
        HandlerTree::resolve_recurse(self, &path, req, res);
    }
    fn resolve_recurse(node: &HandlerTree, path_vec: &[&str], req: &mut Request, res: &mut Response) {
        if res.complete { return; }

        for child in &node.children {
            if child.path.eq(path_vec[0]) | child.path.eq("*") {
                if path_vec.len() == 1 {
                    if child.method.eq(&req.method) | child.method.eq("*") {
                        for h in &child.handlers {
                            h(req, res);
                            if res.complete { return; }
                        }
                    }
                } else {
                    HandlerTree::resolve_recurse(child, &path_vec[1..], req, res);
                }
            }
        }
    }

    pub fn add(&mut self, method: &str, path: &str, handler: PathHandler) {
        let path: Vec<&str> = path.split("/").skip(1).collect();
        HandlerTree::add_recurse(self, &path, method, handler);
    }
    fn add_recurse(node: &mut HandlerTree, path_vec: &[&str], method: &str, path_handler: PathHandler) {
        for (i, child) in node.children.iter().enumerate() {
            if child.path.eq(path_vec[0]) & child.method.eq(method) {
                if path_vec.len() == 1 {
                    node.children[i].handlers.push(path_handler.to_owned());
                } else {
                    HandlerTree::add_recurse(&mut node.children[i], &path_vec[1..], method, path_handler);
                }
                return;
            }
        }

        node.children.push(HandlerTree {
            path: path_vec[0].to_owned(),
            method: "".to_owned(),
            children: vec![],
            handlers: vec![]
        });

        let handler_tree = node.children.last_mut().unwrap();

        if path_vec.len() == 1 {
            handler_tree.method = method.to_owned();
            handler_tree.handlers.push(path_handler)
        } else {
            HandlerTree::add_recurse(handler_tree, &path_vec[1..], method, path_handler);
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