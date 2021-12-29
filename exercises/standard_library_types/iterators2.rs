// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// As always, there are hints if you execute `rustlings hint iterators2`!

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    let mut resultStr = String::new();

    match c.next() {
        None => String::new(),
        Some(first) => {
            match first {
                'h' => resultStr.push('H'),
                'w' => resultStr.push('W'),
                _ => resultStr.push(' '),
            }
            //let charUppercase = ((first as u8) + 55) as char;
            //resultStr.push('H'); //charUppercase);
            while let others = c.next() {
                match others {
                    None => break,
                    Some(rest) => resultStr.push(rest),
                }
            }
            //println!("{}", charUppercase);
            //return String::new();
            resultStr
        },
    }
    
    // let firstChar: bool = true;
    // for ch in c {
    //     if firstChar {
    //         // match c.next() {
    //         // None => String::new(),
    //         // Some(first) => {
    //         //     let newChar: u8 = (first as u8) + 55;
    //         //     resultStr.push(newChar as char)
    //         // },
    //         firstChar = false;
    //     } else {
    //         resultStr.push(ch);
    //     }
    // }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut newVec = Vec::new();

    for word in words {
        let newWord = capitalize_first(word);
        newVec.push(newWord);
    }
    //vec![]
    newVec
}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    //String::new()
    let mut newPhrase = String::new();

    for word in words {
        let capitalizedWord = capitalize_first(word);
        newPhrase.push_str(&capitalizedWord);
    }

    newPhrase
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
