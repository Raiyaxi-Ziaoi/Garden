use std::io::Write;

fn parse_print(data: String) {
    let mut i = 0;
    for line in data.lines() {
        i += 1;
        println!("{0}. {1}", i, line.split(",").collect::<Vec<&str>>()[0]);
    }
}

fn get_input() -> i32 {
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line!");
    return input.trim().parse().expect("Input not an integer!");
}

fn main() -> std::io::Result<()> {
    if !std::path::Path::new("~.dat").exists() { std::fs::File::create("~.dat")?; }

    let data = std::fs::read_to_string("~.dat").expect("Unable to read file!");
    parse_print(data.clone());

    let input = get_input(); 
    
    let array: Vec<&str> = data.lines().collect();
    std::process::Command::new("cmd.exe").arg("/C").arg("start")
                            .arg(array[input as usize - 1].split(",").collect::<Vec<&str>>()[1])
                            .output().expect("Failed to execute command");

    Ok(())
}
