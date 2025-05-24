use std::collections::HashSet;

fn main() {
    let set: HashSet<char> = "school".chars().collect();
    println!("Characters in 'school': {:?}", set);
}
