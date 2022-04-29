use std::cmp::PartialOrd;

fn largest_element<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];
    // why? using `&e` is copying?
    for e in list {
        if e > largest {
            largest = &e
        }
    }
    largest
}

fn main() {
    let ilist = [1, 2, 3, 4, 5];
    let slist = [String::from("abc"), String::from("def")];
    println!("the largest of {:?} is {}", &ilist, largest_element(&ilist));
    println!("the largest of {:?} is {}", &slist, largest_element(&slist));
}
