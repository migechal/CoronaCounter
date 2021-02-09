#![allow(non_snake_case)]

use error_chain::error_chain;
use std::path::Path;
use std::fs::File;
use std::io::prelude::*;
use std::io::stdin;
error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

struct FileDo;
impl FileDo{
    fn write_to_file(file: String, text: String){
        let mut file = std::fs::File::create(file).expect("create failed");
        file.write_all(text.as_bytes()).expect("write failed");
    }
    fn read_from_file(file: String) -> String{
        let mut file = std::fs::File::open(file).unwrap(); 
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        return contents;
    }
    fn delete(file: String){
        use std::fs;
        fs::remove_file(file).expect("could not remove file");
    }
    fn create_if_not_exist(file: String) -> u8{
        use std::fs;
        if !std::path::Path::new(&file).exists(){
            fs::File::create(file).expect("Failed to create file");
            return 1;
        }
        else{
            return 0;
        }
    }
}




#[tokio::main]
async fn main() -> Result<()> {
    let _comma: i8 = 21;

    if FileDo::create_if_not_exist("state.txt".to_owned()) == 1{
        println!("Enter your state name:  ");
        let mut state_name = String::new();
        stdin().read_line(&mut state_name).expect("Not a string");
        FileDo::write_to_file("state.txt".to_owned(), state_name.to_lowercase());
    }

    
    let state = FileDo::read_from_file("state.txt".to_owned()).to_lowercase();

    let target = "https://covidtracking.com/data/download/".to_string() + &state + "-history.csv";

    println!("{}", target);

    let response = reqwest::get(&target).await?;

    let path = Path::new("./cases.csv");

    let mut file = match File::create(&path) {
        Err(why) => panic!("couldn't create {}", why),
        Ok(file) => file,
    };
    
    let content =  response.text().await?;
    file.write_all(content.as_bytes())?;
    
    Ok(())
}