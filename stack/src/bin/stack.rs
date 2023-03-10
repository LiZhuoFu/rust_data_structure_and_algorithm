use stack::stack::Stack;
fn main() {
     let mut s = Stack::new();
     s.push(1); s.push(2); s.push(4);
     println!("top {:?}, size {}",s.peek().unwrap(), s.size());
     println!("pop {:?}, size {}",s.pop().unwrap(), s.size());
     println!("is_empty:{}, stack:{:?}", s.is_empty(), s);
     }