
struct FileDo;
impl FileDo{
    fn writeToFile(file: String, text: String){
        use std::io::Write;
        let mut file = std::fs::File::create(file).expect("create failed");
        file.write_all(text.as_bytes()).expect("write failed");
    }
    fn readFromFile(file: String) -> String{
        use std::io::Read;
        let mut File = std::fs::File::open(file).unwrap(); 
        let mut contents = String::new();
        File.read_to_string(&mut contents).unwrap();
        return contents;
    }
    fn delete(file: String){
        use std::fs;
        fs::remove_file(file).expect("could not remove file");
    }
}
use xlsxwriter::{DateTime as XLSDateTime, Format, Workbook, Worksheet};
fn seekExcel(sheet: &mut Worksheet, state: String){

}


fn main(){
    use std::io::{stdout, Write, Read};
    use curl::easy::Easy;
    let mut covid = Easy::new();
    let mut state :&str;
    covid.url(String::from("https://covidtracking.com/data/download").unwrap();
    covid.write_function(|data| {
        stdout().write_all(data).unwrap();
        Ok(data.len())
    }).unwrap();
}   