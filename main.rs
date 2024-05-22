fn main() {

    //% Pointers e References

    //# Primitive Arrays
    /* let array1 = [1, 2, 3, 4, 5];
    let array2 = array1;
    println!("{:?}", (array1, array2)); */
    // ([1, 2, 3, 4, 5], [1, 2, 3, 4, 5])

    //# Vec
    /* let vec1 = vec![1, 2, 3, 4, 5];
    let vec2 = vec1.clone();
    println!("{:?}", (vec1, vec2)); */
    // ([1, 2, 3, 4, 5], [1, 2, 3, 4, 5])

    /* let vec3 = vec![1, 2, 3, 4, 5];
    let vec4 = &vec3;
    println!("{:?}", (vec3, vec4)); */
    // ([1, 2, 3, 4, 5], [1, 2, 3, 4, 5])


    //% Ownership

    /* let s1 = String::from("ciao");
    let s2 = s1;

    let s1 = String::from("nuovo valore");
    println!("Valore di s1: {}", s1); // nuovo valore
    println!("Valore di s2: {}", s2); // ciao */


    //, Deep copy
    /* let mut s1 = String::from("ciao");
    let mut s2 = s1.clone();
    println!("Valore di s1: {}", s1); // ciao
    println!("Valore di s2: {}", s2); // ciao

    s1 = String::from("Hello");
    println!("Valore di s1: {}", s1); // Hello
    s2 = String::from("World");
    println!("Valore di s2: {}", s2); // World */


    //, Scope ownership
    /* let s1 = String::from("ciao");
    {
        let s2 = s1;
        println!("Valore di s2{}", s2); // Qui s2 è accessibile
     }
    println!("Valore di s2{}", s2);  */ //. Qui s2 non è raggiungibile


    //, Ownership dei dati scalari
    /* let x = 10;
    let y = x;
    println!("Valore di x: {}", x); // 10, x è ancora valido e accessibile
    println!("Valore di y: {}", y); // 10, y ha lo stesso valore di x */



    //% Borrowing

    //, Immutable Borrowing
    /* fn show_all_data(slice: &[i32]) {
        for &value in slice {
            println!("{}", value);
        }
    }
    let data = vec![1, 2, 3, 4];
    show_all_data(&data); */ // Borrow immutabile di data


    //, Mutable Borrowing
    /* let mut numero = 10;
    incrementa(&mut numero); // Passiamo un riferimento mutabile alla funzione
    println!("Il numero incrementato è: {}", numero); // Stampiamo il valore incrementato

    fn incrementa(n: &mut i32) {
        *n += 1; // 11: Incrementiamo il valore a cui il riferimento punta
    } */


    //# Evitare Race conditions
    /* let mut vec_1 : Vec<i32> = vec![1, 2, 3];&
    let ref1 : &mut Vec<i32> = &mut vec_1;
    let ref2 : &mut Vec<i32> = &mut vec_1;

    println!("ref1: {:?}. ref2: {:?}", ref1, ref2); */ //. Errore

    /* let mut vec_1 : Vec<i32> = vec![1, 2, 3];
    let ref1 : &Vec<i32> = &vec_1;
    let ref2 : &Vec<i32> = &vec_1;
    println!("ref1: {:?}, ref2: {:?}", ref1, ref2);

    let ref3 : &mut Vec<i32> = &mut vec_1;
    println!("ref3: {:?}", ref3); */ // Questo funziona


    //# Evitare dangling pointers
    /* let mut numero = 10;
    {
        let r1 = &numero; // r1 è un riferimento immutabile a numero
        println!("r1: {}", r1);  // OK: leggere attraverso r1 è sicuro

        //. Errore! Non possiamo creare un riferimento mutabile mentre r1 esiste.
        let r2 = &mut numero;

    } // r1 esce dallo scope qui

    {
        // OK: ora possiamo avere un riferimento mutabile perché non ci sono riferimenti immutabili
        let r2 = &mut numero;
        *r2 += 1; // OK: modifichiamo numero attraverso r2
        println!("r2: {}", r2); // OK: leggere attraverso r2 è sicuro

        //. Errore! Non possiamo avere due riferimenti mutabili nello stesso scope.
        let r3 = &mut numero;
    } */ // r2 esce dallo scope qui. Possiamo creare nuovamente riferimenti mutabili o immutabili a numero


    //% Dereferencing

    /* let x = 5;
    let y = &x; // y è un riferimento a x

    assert_eq!(5, x);
    assert_eq!(5, *y);
    println!("{x}, {y}"); */ // 5, 5

    //, Dereferencing con Mutabilità

    /* let mut data : i32 = 42;
    let ref1 : &mut i32 = &mut data;
    let deref_ref1 : i32 = *ref1 + 10;
    println!("{ref1}, {deref_ref1}" ); */ // 42, 52

    //, Deref non possibile
    /* let mut data : Vec<i32> = vec![1,2,3];
    let ref1 : &mut Vec<i32> = &mut data; */

    //let deref_ref1 : Vec<i32> = *ref1; //. Errore

    //? Clona i dati invece di tentare di spostarli
    /* let deref_ref1: Vec<i32> = ref1.clone();
    println!("{:?}, {:?}", ref1, deref_ref1); */ // [1, 2, 3], [1, 2, 3]
}