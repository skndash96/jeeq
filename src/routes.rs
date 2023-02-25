use crate::db;
use crate::types;
use db::{ rand_q, rand_qp };
use types::{ Db, Q, Params };
use std::result::Result;
use std::fs;
use std::error::Error;

type Res = Result<String, Box<dyn Error>>;

pub fn get_home() -> Res {
    let html = fs::read_to_string("views/index.html")?;
    
    Ok(format!(
        "HTTP/1.1 200 Ok\r\nContent-Length: {}\r\n\r\n{}",
        html.len(),
        html
    ))
}

pub fn get_q (records: Db, params: Params) -> Res {
    let sub : Option<String> = match params.iter().find(|x| x[0] == "sub") {
        Some(value) => match *value.get(1).unwrap_or(&"a") {
            "p" => Some("Physics".to_string()),
            "c" => Some("Chemistry".to_string()),
            "b" => Some("Biology".to_string()),
            "m" => Some("Maths".to_string()),
            _ => None
        },
        None => None
    };

    let q : &Q = rand_q(
        &records,
        sub
    );
    let response : String = serde_json::to_string(&q)?;

    Ok(format!(
        "HTTP/1.1 200 Ok\r\nContent-Length: {}\r\n\r\n{}",
        response.len(),
        response
    ))
}

pub fn get_qp (records: Db, _params: Params) -> Res {
    let qp : Vec<&Q> = rand_qp(&records);
    let response : String = serde_json::to_string(&qp)?;

    Ok(format!(
        "HTTP/1.1 200 Ok\r\nContent-Length: {}\r\n\r\n{}",
        response.len(),
        response
    ))
}

pub fn get_404() -> Res {
    let response : String = serde_json::to_string("Page Not Found")?;

    Ok(format!(
        "HTTP/1.1 404 Not Found\r\nContent-Length: {}\r\n\r\n{}",
        response.len(),
        response
    ))
}