use crate::tust::request::Request;
use crate::tust::response::Response;

pub type PathHandler = fn(&mut Request, &mut Response) -> ();

pub struct HandlerTree {
    path: String,
    method: String,
    children: Vec<HandlerTree>,
    handlers: Vec<PathHandler>
}

// TODO [8] Shrink HandlerTree vectors to fit
impl HandlerTree {
    pub fn new() -> Self {
        HandlerTree {
            path: String::new(),
            method: String::new(),
            children: vec![],
            handlers: vec![]
        }
    }

    // TODO [6] Add wildcards to resolver
    // TODO [7] Add path variables to resolver
    pub fn resolve(&self, req: &mut Request, res: &mut Response) {
        let p = req.path.clone();
        let path: Vec<&str> = p.split("/").skip(1).collect();
        HandlerTree::resolve_recurse(self, &path, req, res);
    }
    fn resolve_recurse(node: &HandlerTree, path_vec: &[&str], req: &mut Request, res: &mut Response) {
        if res.complete { return; }

        for child in &node.children {
            if child.path.eq(path_vec[0]) {
                if path_vec.len() == 1 {
                    if req.method.eq(&child.method) {
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
        let mut path_found = false;

        for (i, child) in node.children.iter().enumerate() {
            if child.path.eq(path_vec[0]) & child.method.eq(method) {
                path_found = true;
                if path_vec.len() == 1 {
                    node.children[i].handlers.push(path_handler.to_owned());
                } else {
                    HandlerTree::add_recurse(&mut node.children[i], &path_vec[1..], method, path_handler);
                }
                break;
            }
        }

        if !path_found {
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
    }

    pub fn print_tree(&self, level: u8) {
        println!("handler: {} {} {}", self.method, self.path, level);
        for handler in &self.children {
            handler.print_tree(level + 1);
        }
    }
}