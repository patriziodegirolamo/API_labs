use minesweeper::annotate;

fn print_type_of<T>(_: &T) {
    println!("{}", std::any::type_name::<T>())
}

fn main() {

    let asterisco = "*".as_bytes().get(0).unwrap();
    println!("{:?}", asterisco);
    let border = [
        " * * ",
        "  *  ",
        "  *  ",
        "     ",
    ];
    let mut minefield = Vec::<String>::with_capacity(border.len());


    for i in 0..border.len(){
        minefield.push(String::new());

        for j in 0..border[i].len(){
            let item = border[i].as_bytes()[j];
            println!("{}", item);
            println!("{}", item == *asterisco);
            match item {
                42 => {minefield[i].push('*')}
                _ => {minefield[i].push('.')}
            }
        }
    }
    println!("{:?}", minefield);



}
