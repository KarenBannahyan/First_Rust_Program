use std::result;

fn main() {
    let arr: [i32; 10] = [-1,3,5,7,8,10,24,37,42,150];
    let result = bin_search(&arr,24);
    match result {
        Some((found_value,found_index)) => println!("Value {found_value}, index {found_index}"),
        None => println!("Not found")
    } 
}

fn bin_search(arr: &[i32], desired_value: i32) -> Option<(i32, usize)> {
    let mut low_bound: usize = 0;
    let mut up_bound: usize = arr.len() - 1;
    let mut i: usize = 0;

    while low_bound <= up_bound {
        i+= 1;

        let mid: usize = (up_bound + low_bound) / 2;

        let mid_value: i32 = arr[mid];

        if mid_value == desired_value {
            return Some((mid_value, mid));
        } else if desired_value > mid_value {
            low_bound = mid + 1;
        } else {
            up_bound = mid + 1;
        }


        println!("Step {i}");
    }
    None
}