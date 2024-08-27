fn main() {
    // Passare variabili dentro le funzioni
    // passing_vars_inside_functions();

    // Ownership
    // ownership();

    // Cloning
    // cloning();

    // Immutable Borrowing
    // let v = vec![1, 2, 3, 4, 5];
    // show_all_data(&v);

    // Mutable Borrowing
    // let mut word = String::from("Hello");
    // update_word(&mut word);
}


// Ownership
fn ownership() {
    let s1 = String::from("ciao");
    let s2 = s1; // s1 è stato spostato in s2, s1 non è più valido

    let s1 = String::from("nuovo valore"); // Riassegnazione s1
    println!("Valore di s1: {}", s1); // nuovo valore
    println!("Valore di s2: {}", s2); // ciao

    // let v1 = vec![1, 2, 3];
    // let v2 = v1;  // `v1` è ora non più valido
    // println!("{:?}", v1); // Errore: value borrowed here after move
}


// Cloning
fn cloning() {
    let mut s1 = String::from("ciao");
    let mut s2 = s1.clone(); // deep copy di s1 in s2

    println!("Valore di s1: {}", s1); // ciao
    println!("Valore di s2: {}", s2); // ciao

    s1 = String::from("Hello");
    println!("Valore di s1: {}", s1); // Hello

    s2 = String::from("World");
    println!("Valore di s2: {}", s2); // World
}


// Ownership dei dati scalari
fn sum(x: i32, y: i32) -> i32 {
    x + y
}

fn passing_vars_inside_functions() {
    let x = 10;
    let y = 20;
    print!("{}", sum(x, y));
}


// Immutable Borrowing
fn show_all_data(slice: &[i32]) {
    for &value in slice {
        println!("{}", value);
    }
}

// Mutable Borrowing
fn update_word(word: &mut String) {
    word.push_str(" World");
    println!("{}", word);
}
