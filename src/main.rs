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

    let port = std::env::var("PORT").unwrap_or("8080".to_string());

    let listener = TcpListener::bind(
        format!("0.0.0.0:{}", &port)
    ).unwrap();

    println!("Jeeqing at end point: 0.0.0.0:{}", port);

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
    let info = match BufReader::new(&stream)
        .lines()
        .next() {
        Some(line) => line.unwrap_or("".to_string()),
        None => "".to_string()
    };

    if info.as_str() == "" {
        println!("Failed to handle empty stream.");
        return ();
    }

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