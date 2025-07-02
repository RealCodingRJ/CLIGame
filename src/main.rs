use std::fs;
use std::io::Write;
use std::path::Path;

fn main() {
    get_file()
}

fn create_point_add(a: i32)-> i32 {

    return a.abs() + 2;
}

fn create_point_sub(s: i32) -> i32 {

    return s.abs()  - 1;
}


fn get_file() {

    let mut file =
        fs::File::create("main.game.data").unwrap();

    let path = Path::new("main.game.data");

    let message = "TITLE: CLI Game\n\n";
    let objective = "Objective: | Guess a Word And Add Point |";


    if path.exists() {

        file.write(message.as_ref()).unwrap();
        file.write(objective.as_ref()).unwrap();

    }


}