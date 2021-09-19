use tust::Server;

mod tust;

fn main() {
    Server::init(|app| {
        app.get("/test", |_, res| {
            println!("test1");
            // if !req.body.is_empty() {
            //     println!("--{}--", req.body);
            // } else {
                res.status_code = 201;
                res.header.set("Con", "22");
                res.body = "asdf".to_owned();
                // res.complete = true;
            // }
        });
        app.get("/test", |_, res| {
            println!("test2");
            // if !req.body.is_empty() {
            //     println!("--{}--", req.body);
            // } else {
            // res.status_code = 201;
            // res.header.set("Con", "22");
            // res.body = "asdf".to_owned();
            res.complete = true;
            // }
        });

        app.post("/test", |_, res| {
            println!("test2");
            // if !req.body.is_empty() {
            //     println!("--{}--", req.body);
            // } else {
            res.status_code = 500;
            res.status_text = String::from("ERROR");
            // res.header.set("Con", "22");
            // res.body = "asdf".to_owned();
            res.complete = true;
            // }
        });

        app.get("/test/test", |_, res| {
            println!("BINGO");
            // res.success("");
            // res.header.set("Con", "22");
            // res.body = "asdf".to_owned();
            res.complete = true;
        });

        app.get("/test/*", |_, res| {
            println!("BINGO");
            // res.success("");
            res.status_code = 404;
            res.status_text = String::from("ERROR");
            // res.header.set("Con", "22");
            // res.body = "asdf".to_owned();
            res.complete = true;
        });
        // app.all("/*", |_, res| {
        //     res.status_code = 500;
        //     res.status_text = String::from("ERROR");
        //     // res.header.set("Con", "22");
        //     // res.body = "asdf".to_owned();
        //     res.complete = true;
        // });
    }).listen(8080);
}