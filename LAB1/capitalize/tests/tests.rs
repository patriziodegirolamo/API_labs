use capitalize::capitalize;


#[test]
fn check_string() {
    let stringa = "ciao mi chiamo  patti".to_string();
    let cap = capitalize(&stringa);
    assert_eq!(cap, "Ciao Mi Chiamo  Patti".to_string());
}

#[test]
#[ignore]
fn empty_string() {
    let stringa = "".to_string();
    let cap = capitalize(&stringa);
    assert_eq!(cap, "".to_string());
}

#[test]
#[ignore]
fn no_space_string() {
    let stringa = "ciaociao".to_string();
    let cap = capitalize(&stringa);
    assert_eq!(cap, "Ciaociao".to_string());
}

#[test]
#[ignore]
fn accenty_string() {
    let stringa = "àiò  peCora".to_string();
    let cap = capitalize(&stringa);
    assert_eq!(cap, "Àiò  PeCora".to_string());
}

#[test]
#[ignore]
fn already_maiusc_and_num_string() {
    let stringa = "àiò  Pecora 999".to_string();
    let cap = capitalize(&stringa);
    assert_eq!(cap, "Àiò  Pecora 999".to_string());
}