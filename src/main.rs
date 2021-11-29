fn main() {
    let a = [10, 20, 30, 100, 40, 50, i32::MAX];
    println!("{:#?}",try_sum(&a));
    
    let a = [10, 20, 30, 100, 40, 50, 500];
    println!("{:#?}",try_sum(&a));
}

fn try_sum(a: &[i32]) -> Option<i32> {
    let mut it = a.iter();
    let sum = it.try_fold(0i32, |acc, &x| acc.checked_add(x));
    sum
}
