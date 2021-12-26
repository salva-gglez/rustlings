// option2.rs
// Make me compile! Execute `rustlings hint option2` for hints

fn main() {
    let optional_word = Some(String::from("rustlings"));
    // TODO: Make this an if let statement whose value is "Some" type
    if let word = match optional_word {
            Some(s) => s,
            None => "".to_string(),
        }
    {
        println!("The word is: {}", word);
    } else {
        println!("The optional word doesn't contain anything");
    }

    let mut optional_integers_vec: Vec<Option<i8>> = Vec::new();
    for x in 1..10 {
        optional_integers_vec.push(Some(x));
    }

    // TODO: make this a while let statement - remember that vector.pop also adds another layer of Option<T>
    // You can stack `Option<T>`'s into while let and if let
    while let elem = optional_integers_vec.pop() {
        //integer = optional_integers_vec.pop() {
        
        let vecElem = match elem {
            Some(i) => i,
            None => break,
        };

        let integer = match vecElem {
            Some(i) => i,
            None => continue,
        };

        println!("current value: {}", integer);
        
        /*if elem {
            println!("true")
        } else {
            println!("false")
        }*/
    }

}
