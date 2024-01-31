
fn main() {

    //get first input argument
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: guid_to_base64 <inputfile> <column_number> >> output.csv");
        return;
    }

    //get input args    
    let inputfile: &String = &args[1];
    //let column_number: usize = args[2].parse().expect("Error parsing column number");
    let column_number: usize = args[2].parse().unwrap_or(0);


    //get file into reader
    let contents = std::fs::File::open(inputfile).expect("Something went wrong reading the file");
    let mut rdr = csv::Reader::from_reader(contents);
    for result in rdr.records() {
        // The iterator yields Result<StringRecord, Error>, so we check the
        // error here..
       // let record = result?;
        println!("{:}",guid_to_base64(&result.unwrap_or_default().get(column_number).unwrap_or_default()).unwrap_or_default());
    }

}

pub fn guid_to_base64(guid: &str) -> Option<String> {
    let uuid = uuid::Uuid::parse_str(&convert_kid(&guid)).ok()?;
    let bytes = uuid.as_bytes();
    Some(base64::encode(bytes))
}

fn convert_kid(kidorig: &str) -> String {
    let mut prkid = String::with_capacity(20);
    prkid.push_str(&kidorig[6..8]);
    prkid.push_str(&kidorig[4..6]);
    prkid.push_str(&kidorig[2..4]);
    prkid.push_str(&kidorig[0..2]);
    prkid.push(kidorig.chars().nth(8).unwrap());
    prkid.push_str(&kidorig[11..13]);
    prkid.push_str(&kidorig[9..11]);
    prkid.push(kidorig.chars().nth(13).unwrap());
    prkid.push_str(&kidorig[16..18]);
    prkid.push_str(&kidorig[14..16]);
    prkid.push_str(&kidorig[18..]);
    prkid
}