use std::{env, ops::Range};

fn main() {
    let args = env::args().collect::<Vec<String>>();
    if args.len() == 2 {
        let param = args.get(1).unwrap();
        if is_isogram(param) {
            println!("Yes! {param} is an isogram.");
        } else {
            println!("Nope! {param} is no isogram.");
        }
    }
}

pub fn is_isogram(value: &str) -> bool {
    //implementation goes here!
    if value.len() == 0 {
        // empty strings are always isograms
        return true;
    }

    let mut counts : Vec<i32> = vec![0;256];
    for c in 0..255 {
        for cc in value.chars() {
            let ccc = cc.to_ascii_lowercase();
            if c == (ccc as usize) {
                counts[ccc as usize] +=  1;
            }            
        }
    }

    let mut letter_count = 0;
    for count in counts {
        if count > 0 && letter_count == 0{
            //init, first run
            letter_count = count;
        }else if  count > 0{
            if letter_count != count{
                return false;
            }
        }
    }
    return true;

    //       c
    //   o   t    t   o
    //   ^             
/* 

    let mut last_count = None;

    for c in value.chars() {
        let mut count = 0;
        
        for cc in value.chars() {
            if c == cc {
                count = count + 1;
            }
        }

        if let Some(x) = last_count {
            if x != count {
                return false;
            }
        }else {
            //first character
            last_count = Some(count);
        }
    }*/

    true
}

#[cfg(test)]
mod isogram_tests {
    use crate::is_isogram;

    #[test]
    fn empty_string_should_not_be_an_isogram() {
        assert!(is_isogram("otto"));
    }

    #[test]
    fn a_single_character_is_an_isogram(){
        assert!(is_isogram("a"));
    }
}
