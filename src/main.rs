use std::net::{ TcpListener, TcpStream };
use std::io::{ Write, BufRead, BufReader };
use urlencoding::decode;
use std::rc::Rc;

mod routes;
mod db;
mod types;
use types::{ Db, Param, Params };

fn main() {
    let data : Db = db::new_db();
    
    let end_point = "0.0.0.0:8080";
    let listener = TcpListener::bind(end_point).unwrap();

    println!("Jeeqing at end point: {}", end_point);

    for stream in listener.incoming() {
        if stream.is_ok() {
            let stream = stream.unwrap();

            handle_stream(
                stream,
                Rc::clone(&data)
            );
        } else {
            println!("Failed to handle a Stream.");
        }
    }
}

fn handle_stream(
    mut stream : TcpStream,
    records: Db
) {
    let info = BufReader::new(&stream)
        .lines()
        .next()
        .unwrap()
        .unwrap();//Potential PANIC

    let info = &info
        .split(" ")
        .collect::<Vec<&str>>()[..];
    let method = info[0];
    let path = decode(info[1])
        .unwrap_or(
            std::borrow::Cow::Borrowed(info[1])
        );//Potential DUMBNESS

    println!("{} {}", method, path);

    let (way, params) = get_params(&path);

    let response = match way {
            "/" => routes::get_home(),
            "/api/q" => routes::get_q(records, params),
            "/api/qp" => routes::get_qp(records, params),
            _ => routes::get_404()
        };

    let response = (match response {
        Ok(res) => res,
        Err(err) => {
            println!("{:?}", err);
            "HTTP/1.1 500 Internal Server Error\r\n\r\n".to_string()
        }
    }).as_bytes().to_owned();
    
    stream
        .write_all(&response[..])
        .expect("Failed to write to TcpStream.");
}

fn get_params (
    path: &str
) -> (&str, Params) {
    let mut split = path.split("?");

    let way = split.next().unwrap_or("/");
    let params = split.next().unwrap_or("");

    let params = params
        .split("&")
        .map(|x| x.split("=").collect::<Param>())
        .collect::<Params>();

    (way, params)
}