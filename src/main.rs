use std::fs;

fn main() {
    for entry in fs::read_dir("C:/Users/").unwrap() {
        let direntry = entry.unwrap();
        let path = direntry.path();
        let con: String;
        if path.is_dir() {
            let path_to_string = path.to_str();
            match path_to_string {
                None => panic!("wtf"),
                Some(_) => con = path_to_string.unwrap().to_string()
            }
            let split: Vec<&str> = con.split('/').collect();
            let new_path = fs::read_dir("C:/Users/".to_owned() + &split[split.len() - 1] + "/").unwrap();
            println!("{:?} is a dir", new_path);
        } else {
            println!("{:?} is a file", path);
        }
    }
}
