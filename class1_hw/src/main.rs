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
    println!("Hello, home work 2");
}

fn main() {
    class1_hw1();

    class1_hw2();
}
