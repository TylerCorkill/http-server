use tust::Server;

mod tust;

fn main() {
    Server::init(|app| {
        app.get("/test", |req, res| {
            println!("--{}--", req.body);
            // TODO: body handle
            if req.body.trim().is_empty() {
                println!("--{}--", req.body);
            } else {
                res.status_code = 201;
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