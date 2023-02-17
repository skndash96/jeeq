fn put (path: &str) {
    let mut reader = csv::Reader::from_path(&path)
        .expect("Failed to read file path.");
    let records = reader
        .records()
        .map(|rec| rec.unwrap());

    let mut writer = fs::File::options()
        .create(true)
        .append(true)
        .open("jeeq.csv")
        .expect("Failed to open writer.");

    let mut n : usize = 0;
    for rec in records {
        writeln!(
            writer,
            "{:?},{:?}",
            rec.get(1).unwrap(),
            rec.get(3).unwrap()
        ).expect("Failed to write record.");

        n += 1;
        println!("{}", n);
    }

    println!("######### Completed {} #########", path);
}