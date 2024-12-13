fn main() {
    let mut todos: Vec<String> = vec![];
    println!("Enter a comma-separated list of to-dos:\n");


    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    for todo in input.trim().split(", ") {
        todos.push(todo.to_string());
    }

    // Print the to dos all on the same line seperated by commas
    print!("To-dos: ");
    for (i, todo) in todos.iter().enumerate() {
        if i == todos.len() - 1 {
            print!("{}", todo);
        } else {
            print!("{}, ", todo);
        }
    }
}
