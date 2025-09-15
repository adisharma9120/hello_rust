/* fn main() {
    let name = String::from("Hello World");

    let ans = first_word(name.clone()); // clone kyunki ownership ja rahi hai
    let ans2 = last_word(name);

    println!("First word: {}", ans);
    println!("Last word: {}", ans2);
}

fn first_word(s: String) -> String {
    let mut ans = String::from("");
    for i in s.chars() {
        if i == ' ' {
            break;
        }
        ans.push(i); // push ek char leta hai, to_string() ki zarurat nahi
    }
    return ans;
}

fn last_word(s: String) -> String {
    let mut ans = String::from("");
    let mut collect = false;

    for i in s.chars().rev() { // reverse loop
        if i == ' ' {
            break;
        }
        ans.push(i);
        collect = true;
    }

    if collect {
        return ans.chars().rev().collect(); // kyunki reverse me ulta collect hua
    }

    return ans;

}
 */

 fn main()
 {
    let mut word = String::from("hellor world");
    let word2 = &word[0..5];

    println!("{}", word2);
    
    word.clear();
 }