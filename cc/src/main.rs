fn main() {
    println!("The sum of multiples of 3 and 5 from 0 to 1000 is {}.", sum_multiples());
}

//method returns the sum of multiples of 3 and 5
fn sum_multiples() -> i32 {
    let mut num = 0;
    for index in 0..1000{
        if index % 3 == 0 || index % 5 == 0{
            num+=index;
        }
    }
    return num;
}
