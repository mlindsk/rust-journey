fn place_holder_fn(mut s: String) -> String {
    s = s + "...";
    return s
}



// pub enum Node<'a> {
//     ChildLeft(&'a Option<Node<'a>>),
//     ChildRight(&'a Option<Node<'a>>),
//     Parent(&'a Option<Node<'a>>),
// }

// struct Node<T> {
//     left: Option<Box<Node<T>>>,
//     right: Option<Box<Node<T>>>,
//     value: T,
// }

fn main() {
    let s: String = "MyString".to_string();
    place_holder_fn(s);        
}
