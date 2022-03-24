use clap::{Arg, Command};
use capitalize::capitalize;

fn args_parser() -> String{
    let app = Command::new("hello-clap")
        .version("1.0")
        .about("Says hello")
        .author("Michael Snoyman");

    let name_option = Arg::new("frase")
        .long("frase")
        .takes_value(true)
        .help("Who to say hello to")
        .required(true);

    let app = app.arg(name_option);

    let matches = app.get_matches();

    let frase = matches.value_of("frase")
        .expect("This can't be None, we said it was required");

    return frase.to_string();
}



fn main() {

    let frase = args_parser();



    println!("{}", capitalize(&frase));

}

