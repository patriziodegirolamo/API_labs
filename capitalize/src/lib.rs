pub fn capitalize(s: &str) -> String {
    let mut inizio = true;
    let mut cap_string = String::new();
    for car in s.chars(){
        match car{
            ' ' => {
                inizio = true;
                cap_string.push(car);
            },
            _ if inizio => {
                inizio = false;
                cap_string.push_str(car.to_uppercase().to_string().as_str())
            },
            _ => { cap_string.push(car);}
        }
    }
    return cap_string;
}
