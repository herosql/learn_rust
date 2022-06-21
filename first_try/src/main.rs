use std::fs;
fn main() {
    let mut args = std::env::args();
    let fileName = args.next().unwrap();
    let url = args.next().unwrap();
    let output = args.next().unwrap();

    println!("Now running file name:{}", fileName);

    println!("Fetching url:{}", url);

    let body = reqwest::blocking::get(url).unwrap().text().unwrap();

    println!("Conerting html to markdown...");
    let md = html2md::parse_html(&body);

    fs::write(output.clone(), md.as_bytes()).unwrap();

    println!("Coverted markdown has been saved in {}", output);
}
