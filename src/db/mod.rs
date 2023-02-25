use rand::{ Rng, thread_rng };
use std::time::Instant;
use std::rc::Rc;
use crate::types;
use types::{ Db, Q };

mod constants;

pub fn new_db() -> Db {
    let now = Instant::now();

    let mut reader = csv::ReaderBuilder::new()
        .escape(Some(b'\\'))
        .from_path("jeeq.csv")
        .expect("Failed to parse questions.");
    let records : Vec<Q> = reader
        .records()
        .map(|rec| rec.unwrap().iter().map(|x| x.to_string()).collect::<Q>())
        .collect();
    let records = Rc::new(records);

    println!("Completed loading records within {}ms.", now.elapsed().as_millis());

    records
}

pub fn rand_q  (
    db: &Db,
    subject: Option<String>
) -> &Q {
    let mut rng = thread_rng();
    let mut n : usize = rng.gen_range(0..=constants::Q_SIZE);

    if let Some(value) = subject {
        while db[n][1] != value {
            n += 1;
        }
    }

    &db[n]
}

pub fn rand_qp  (
    db : &Db
) -> Vec<&Q> {
    let mut qp : Vec<&Q> = vec![];

    for i in 0..75 {
        qp.push(rand_q(
            &db,
            Some(sub(i))
        ));
    }

    fn sub(i : usize) -> String {
        if i < 25 {
            "Physics".to_string()
        } else if i < 50 {
            "Chemistry".to_string()
        } else {
            "Maths".to_string()
        }
    }

    qp
}
