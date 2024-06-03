fn main() {
    let string_array = vec!["Hey", "Hi,", "Hello", "Hi", "Crazy", "Right"];

    println!("{:?}\n", string_array);

    for i in string_array {
        println!("{}\n", i);
    }
}
