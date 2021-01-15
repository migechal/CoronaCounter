
struct FileDo;
impl FileDo{
    fn write_to_file(file: String, text: String){
        use std::io::Write;
        let mut file = std::fs::File::create(file).expect("create failed");
        file.write_all(text.as_bytes()).expect("write failed");
    }
    fn read_from_file(file: String) -> String{
        use std::io::Read;
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


fn main(){
    use std::io::{stdin, stdout, Write, Read};
    use curl::easy::Easy;
    let mut covid = Easy::new();
    if FileDo::create_if_not_exist("state.txt".to_owned()) == 1{
        println!("Enter your state name:  ");
        let mut state_name = String::new();
        stdin().read_line(&mut state_name).expect("Not a string");
        FileDo::write_to_file("state.txt".to_owned(), state_name.to_lowercase());
    }
    let state :String = FileDo::read_from_file("state.txt".to_owned()).to_lowercase();

    let link :String = "https://covidtracking.com/data/download".to_owned() + &state + "-history.cvs";

    covid.url(&link).unwrap();
    covid.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
}   