struct _Question {
    id: String,
    ques: String,
    topic: Option<String>,
    sub: String
}

fn main() {
    let mut reader = csv::Reader::from_path("split/2jab")
        .expect("Failed to parse questions.");
    let mut records = reader
        .records()
        .map(|rec| rec.unwrap());

    let mut n = 0;
    for rec in records {
        n += 1;
        println!("{} {:?}", n, rec.get(1).unwrap());
    }
    //from 1,65,892
}
//3,00,000 questions