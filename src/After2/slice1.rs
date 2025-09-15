fn main() {
    // Example 1
    example1();

    // Example 2
    example2();
}

// ----------------- Example 1 -----------------
fn example1() {
    let name = String::from("Hello World");

    let ans = first_word(name.clone()); // clone kyunki ownership ja rahi hai
    let ans2 = last_word(name);

    println!("First word: {}", ans);
    println!("Last word: {}", ans2);
}

fn first_word(s: String) -> String {
    let mut ans = String::new();
    for i in s.chars() {
        if i == ' ' {
            break;
        }
        ans.push(i);
    }
    ans
}

fn last_word(s: String) -> String {
    let mut ans = String::new();
    let mut collect = false;

    for i in s.chars().rev() {
        if i == ' ' {
            break;
        }
        ans.push(i);
        collect = true;
    }

    if collect {
        return ans.chars().rev().collect(); // ulta se sahi order me convert
    }

    ans
}

// ----------------- Example 2 -----------------
fn example2() {
    let mut word = String::from("hellor world");
    let word2 = &word[0..5];

    println!("Slice word2: {}", word2);

    word.clear();
}
