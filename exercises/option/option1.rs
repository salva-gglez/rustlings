// option1.rs
// Make me compile! Execute `rustlings hint option1` for hints


// you can modify anything EXCEPT for this function's sig
fn print_number(maybe_number: Option<u16>) {
    match maybe_number {
        Some(maybe_number) => println!("printing: {}", maybe_number),
        None => println!("No number supplied")
    };
}

fn main() {
    print_number(Some(13));
    print_number(Some(99));

    let mut numbers: [Option<u16>; 5] = [None; 5];
    for iter in 0..5 {
        let number_to_add: u16 = {
            ((iter * 1235) + 2) / (4 * 16)
        };

        if iter == 2 {
            numbers[iter as usize] = None;
        } else {
            numbers[iter as usize] = Some(number_to_add);
        }
    }

    for iter in 0..5 {
        print_number(numbers[iter as usize]);
    }
}
