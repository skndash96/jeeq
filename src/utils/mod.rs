use rand::{thread_rng, Rng};
use csv::StringRecord;

mod constants;

pub fn random_q (
    records: &Vec<StringRecord>,
    subject: Option<String>
) -> &StringRecord {
    let mut rng = thread_rng();
    let mut n : usize = rng.gen_range(0..=constants::QI_SIZE);

    if let Some(value) = subject {
        while records[n].get(1).unwrap() != value {
            n += 1;
        }
    }

    &records[n]
}

pub fn random_qp (
    records : &Vec<StringRecord>
) -> Vec<&StringRecord> {
    let mut ques : Vec<&StringRecord> = vec![];

    for i in 0..75 {
        ques.push(random_q(
            &records,
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

    ques
}