fn main() {
    let numbers = vec![1, 2, 3, 4, 5];

    // 1. map → har value ko double karna
    let doubled: Vec<_> = numbers.iter().map(|x| x * 2).collect();
    println!("Doubled: {:?}", doubled); // [2, 4, 6, 8, 10]

    // 2. filter → sirf even numbers lena
    let evens: Vec<_> = numbers.iter().filter(|x| *x % 2 == 0).collect();
    println!("Evens: {:?}", evens); // [2, 4]

    // 3. take → pehle 2 numbers lena
    let first_two: Vec<_> = numbers.iter().take(2).collect();
    println!("First two: {:?}", first_two); // [1, 2]

    // 4. skip → pehle 2 chhodna
    let skip_two: Vec<_> = numbers.iter().skip(2).collect();
    println!("Skip two: {:?}", skip_two); // [3, 4, 5]

    // 5. enumerate → index ke sath values
    for (i, val) in numbers.iter().enumerate() {
        println!("Index {} has value {}", i, val);
    }

    // 6. sum → sabka total nikalna
    let total: i32 = numbers.iter().sum();
    println!("Sum: {}", total); // 15

    // 7. any → check karo koi number > 4 hai kya
    let has_gt4 = numbers.iter().any(|x| *x > 4);
    println!("Any greater than 4? {}", has_gt4); // true

    // 8. all → check karo sabhi number < 10 hain kya
    let all_lt10 = numbers.iter().all(|x| *x < 10);
    println!("All less than 10? {}", all_lt10); // true
}