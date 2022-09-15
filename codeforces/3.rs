use std::io::BufRead;

// https://codeforces.com/contest/1672/problem/B
 
fn is_constructible(s : &str) -> bool {
    // trivial cases: s of at leat length 2, last chat must be B
    if s.len() == 1 || &s[(s.len()-1)..] != "B" { return false ;}
 
    // number of Bs should be always <= number of As
    let mut a: usize = 0;
    let mut b: usize = 0;
 
    for char in s.chars() {
        if char == 'A' {
            a += 1;
        } else {
            b += 1;
        }
        if b > a { return false;}
    }
 
    let found = s.find('B');
    match found {
        Some(idx) if idx == s.len() -1 => { // here this means s is already a "good" string ,meaning a single op is needed
            return true;
        },
        Some(0) => { // B is at the beginning , this is a trivial case of non-constructible structure
            return false;
        },
        None => { return false; }, // s is only 'A's
        _ => { return true; }
    }
}
 
 
fn main() {
    let stdin = std::io::stdin();
    for line in stdin.lock().lines().skip(1) {
        println!("{}", if is_constructible(&line.unwrap()) { "YES"} else { "NO"});
    }
}
 
 
#[test]
fn test_1() {
    assert!(is_constructible("AAABB"));
    assert!(is_constructible("AABAB"));
    assert!(!is_constructible("ABB"));
    assert!(is_constructible("AAAAAAAAB"));
    assert!(is_constructible("ABABAB"));
}