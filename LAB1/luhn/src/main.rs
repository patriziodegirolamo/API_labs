use luhn::is_valid;
use clap::{Arg, Command};

fn args_parser() -> String{
    let app = Command::new("hello-clap")
        .version("1.0")
        .about("Says hello")
        .author("Michael Snoyman");

    let name_option = Arg::new("card")
        .long("card")
        .takes_value(true)
        .help("Who to say hello to")
        .required(true);

    let app = app.arg(name_option);

    let matches = app.get_matches();

    let card = matches.value_of("card")
        .expect("This can't be None, we said it was required");

    return card.to_string();
}


fn main() {
    //let code = "4539 3195 0343 6467".to_string();
    let code = args_parser();

    let mut valid_group = true;
    let mut count = 0;
    for group in code.split(" "){
        count = count + 1;
        if group.chars().count() != 4{
            valid_group = false;
        }
    }

    if count != 4 {
        valid_group = false;
    }

    if valid_group {
        valid_group = is_valid(&code);
    }

    else{
        println!("input card invalid");
    }
    println!("{}", valid_group);
}
