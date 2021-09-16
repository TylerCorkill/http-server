use tust::Server;

mod tust;

fn main() {
    Server::init(|app| {
        app.get("/test", |req, res| {
            if !req.body.is_empty() {
                println!("--{}--", req.body);
            } else {
                res.status_code = 201;
                res.header.set("Con", "22");
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