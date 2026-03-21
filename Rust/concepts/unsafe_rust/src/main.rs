
fn main() {
    println!("A peep into unsafe rust...");

    let mut nums = vec![1, 2, 3, 4, 5, 6, 7];

    let (first_half, second_half) = nums.split_at_mut(7);

    first_half[0] = 12;

    println!("len of first half: {}", first_half.len());

    println!("first half: {:?}", first_half);
    println!("second half: {:?}", second_half); 
}
