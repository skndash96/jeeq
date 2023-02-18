use std::time::Instant;
use csv::StringRecord;

mod utils;

fn main() {
    let now = Instant::now();

    let mut reader = csv::Reader::from_path("jeeq.csv")
        .expect("Failed to parse questions.");
    let records : Vec<StringRecord> = reader
        .records()
        .map(|rec| rec.unwrap())
        .collect();

    println!("Completed loading records within {}ms.", now.elapsed().as_millis());

    // let rand_q = utils::random_q(
    //     &records,
    //     Some("Physics".to_string())
    // );
    // println!("{:?}", rand_q);

    // let rand_qp = utils::random_qp(
    //     &records
    // );
    // println!("{:?}", rand_qp);
}