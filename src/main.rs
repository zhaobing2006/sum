fn main() {
    let  array:[u32;5] = [1,5,7,8,11];   
    let sum = sum_compute(&array);
    match sum{
        None=>println!(" error."),
        Some(value)=>println!("The summer value or array is {}", value)
    }
}


fn sum_compute(p_array:&[u32])->Option<u32>{
    let mut sum: u32 = 0;
    for i in p_array.iter(){
        sum += i
    }
    Some(sum)
}