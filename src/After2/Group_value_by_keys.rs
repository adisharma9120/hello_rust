/* "Write a function that takes a vector of tuples (each tuple containing a key and a value) and 
returns a HashMap where the keys are the unique keys from the input tuples, 
and the values are vectors of all corresponding values associated with each key.
 */

 use std::collections::HashMap;

 fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32>{
      
    let mut hm = HashMap::new();
    for (key, value ) in vec {
        hm.insert(key, value);
    }
     return hm;

 }
  fn main(){
    let input_vec = vec![(String::from("Aditya"), 22), (String::from("Sharma"), 32)];
    let hm = group_values_by_keys(input_vec);
     
    println!("{:?}", hm);
  }

 use std::collections::HashMap;

 fn group_of_key_value( vec:Vec<(String, i32)>)-> HashMap<String, i32>{

    let mut hashmap = HashMap::new();
    for (key, value ) in vec {
        hashmap.insert(key , value);
    }
    return hashmap;
 }