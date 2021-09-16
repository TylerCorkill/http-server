use tust::Server;

mod tust;

fn main() {
    Server::init(|app| {
        app.get("/test", |req, res| {
            println!("\n\n{}\n--{}--\n", req.body.len(), req.body);
            // TODO [3] body handling
            if req.body.trim().is_empty() {
                println!("--{}--", req.body);
            } else {
                res.status_code = 201;
                res.headers.set("Con", "22");
                res.body = "asdf".to_owned();
                res.complete = true;
            }
        });
        app.get("/bingo", |_, res| {
            res.success("");
        });
        app.all("*", |_, res| {
            res.error("");
        });
    }).listen(8080);
}