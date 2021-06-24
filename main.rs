use curl::easy::Easy;
use run_script::ScriptOptions;
use std::fs::remove_file;
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
    fn exists(file: String) -> bool {
        if !std::path::Path::new(&file).exists() {
            return false;
        }
        return true;
    }
    fn delete_file(file: String) {
        remove_file(file).expect("Could not remove file");
    }
    fn create_if_not_exist(file: String) -> u8 {
        let file_tmp = &file;
        if !FileDo::exists(file_tmp.to_string()) {
            std::fs::File::create(file).expect("Failed to create file");
            1
        } else {
            0
        }
    }
}

fn main() -> std::io::Result<()> {
    let new_state: String = String::from('n');
    let get_covid_from_past: String = String::from('f');
    let file_name = "./state.txt".to_string();
    let beg_text: String = if FileDo::exists(file_name.to_owned()) {
        let mut owned_string: String =
            "To find Covid-19 cases from your past entry--press (".to_owned();
        owned_string.push_str(&get_covid_from_past);
        owned_string.push(')');
        owned_string.to_owned()
    } else {
        " ".to_string()
    };
    let mut opt: String = String::new();
    while opt != new_state && opt != get_covid_from_past {
        println!("{} | Press ({}) to enter new state:  ", beg_text, new_state);
        opt.clear();
        stdin().read_line(&mut opt).expect("Not a char");
        opt = opt.trim().to_string();
        // println!("{:?} is what you entered and {:?} is next", opt, new_state);
    }
    if FileDo::create_if_not_exist(file_name.to_owned()) == 1 || opt == new_state {
        FileDo::delete_file("./state.txt".to_owned());
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
    print!("\x1B[2J\x1B[1;1H");
    if std::path::Path::new(&path).exists() {
        let (_code, output, _error) = run_script::run(
            r#"
            Rscript GetData.R
            "#,
            &args,
            &options,
        )
        .unwrap();
        if output == "" {
            println!("What you entered as your state is probably not a state, if I'm wrong, let me know and report it at https://github.com/migechal/CoronaCounter/issues :)");
        } else {
            println!("Their are: {} cases in the state of {}", output, state);
        }
        FileDo::delete_file("./cases.csv".to_owned());
        Ok(())
    } else {
        println!("Error! File {} not found. (This issue is most likely not caused by you, please report it at https://github.com/migechal/CoronaCounter/issues", path);
        FileDo::delete_file("./cases.csv".to_owned());
        Err(std::io::Error::new(ErrorKind::NotFound, path))
    }
}
