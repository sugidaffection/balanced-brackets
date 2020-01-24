use std::io;
use std::collections::HashMap;

fn is_balanced(exp: String) -> bool{
    let e = "({[".chars().zip(")}]".chars());
    let map: HashMap<_,_> = e.into_iter().collect();

    let mut queue:Vec<char> = Vec::new();
    let mut unbalanced = false;

    exp.chars().for_each(|c| {
        if let Some(map) = map.get(&c){
            queue.push(*map);
        }
        else if map.values().any(|&s| s == c) && !(!queue.is_empty() && c == queue.pop().unwrap()) {
            unbalanced = true;
            return
        }
    });
    queue.is_empty() || unbalanced
}

fn main() {
    let mut n: String = String::new();
    println!("Your Input : ");
    io::stdin().read_line(&mut n).expect("Can't reading input");
    let balanced: bool = is_balanced(n.trim().to_owned());
    println!("{}", if balanced { "Balanced" } else { "Unbalanced" });

}
