use curl::easy::Easy;
use run_script::ScriptOptions;
use std::fs::{remove_file, File};
use std::io::{prelude::*, stdin, ErrorKind};
use std::str;

struct FileDo {}

impl FileDo {
    fn write_to_file(file: String, text: String) {
        let mut file = std::fs::File::create(file).expect("Failed to open file");
        file.write_all(text.as_bytes()).expect("Write failed");
    }
    fn read_from_file(file: String) -> String {
        let mut file = std::fs::File::open(file).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        return contents;
    }
    fn _delete_file(file: String) {
        remove_file(file).expect("Could not remove file");
    }
    fn create_if_not_exist(file: String) -> u8 {
        if !std::path::Path::new(&file).exists() {
            File::create(file).expect("Failed to create file");
            1
        } else {
            0
        }
    }
    // fn is_empty_file(file: String) -> bool {
    //     use std::fs;
    //     if !std::path::Path(&file).as_bytes() {
    //         fs::File::create(file).expect("Failed to create file");
    //         return false;
    //     } else {
    //         return true;
    //     }
    // }
}

fn main() -> std::io::Result<()> {
    let file_name = "./state.txt".to_string();

    if FileDo::create_if_not_exist(file_name.to_owned()) == 1 {
        println!("Enter your state name:  ");

        let mut state_name = String::new();
        stdin().read_line(&mut state_name).expect("Not a string");
        state_name.pop(); // remove newline
        FileDo::write_to_file(file_name.to_owned(), state_name.to_lowercase());
    }

    let state = FileDo::read_from_file("./state.txt".to_owned()).to_lowercase();

    let path = "./cases.csv";

    let mut covid = Easy::new();

    covid
        .url(&("https://covidtracking.com/data/download/".to_string() + &state + "-history.csv"))
        .unwrap();

    let mut data_from_web: String = String::new();
    {
        let mut transfer_data = covid.transfer();
        transfer_data
            .write_function(|data| {
                data_from_web.push_str(str::from_utf8(data).unwrap());
                Ok(data.len())
            })
            .unwrap();
        transfer_data.perform().unwrap();
    }

    FileDo::write_to_file(path.to_string(), data_from_web);

    let options = ScriptOptions::new();

    let args = vec![];

    if std::path::Path::new(&path).exists() {
        let (_code, output, _error) = run_script::run(
            r#"
            Rscript GetData.R
           "#,
            &args,
            &options,
        )
        .unwrap();
        println!("Their are: {} cases in the state of {}", output, state);
        Ok(())
    } else {
        println!("Error! File {} not found. (This issue is most likely not caused by you, please report it at https://github.com/migechal/CoronaCounter/issues", path);
        Err(std::io::Error::new(ErrorKind::NotFound, path))
    }
}
