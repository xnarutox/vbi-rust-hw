use std::io;

fn class1_hw1() {
    
    let org_arr = [1, 2,3,5,6,8, 10, 11];
    let sub_arr = [6,8,10];
    let mut is_child_arr = true;

    let sub_arr_first_pos = org_arr.iter().position(|&x| x == sub_arr[0]);

    if sub_arr_first_pos != None {
        let sub_arr_first_index = sub_arr_first_pos.unwrap();
        let sub_arr_index = sub_arr_first_index + 1;
        for x in 1..sub_arr.len()-1{
            if org_arr[sub_arr_index] == sub_arr[x] {
                continue;
            } else {
                is_child_arr = false;
            }
        }

        if is_child_arr {
            println!("Hello, org_arr contains sub_arr");
        } else {
            println!("Hello, org_arr does not sub_arr");
        }
        
    } else {
        println!("Hello, world! you are not my father");
    }

    
}

fn class1_hw2() {
    
    let str = "This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal. This is a regular paragraph with the default style of Normal.";

    println!("Please input your word.");

    let mut word = String::new();

    io::stdin()
        .read_line(&mut word)
        .expect("Failed to read line");
    word.pop();
    word.pop();
    
    let word_count = str.matches(&word).count();

    println!("You guessed: {}. we found {} times", word, word_count);

    
}

fn main() {
    class1_hw1();

    class1_hw2();
}
