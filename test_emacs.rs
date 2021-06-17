use std::collections::HashMap;
fn fibonacci(num: i32, fib_store: &mut HashMap<i32, i32>) -> i32 {
    //base case

    if num == 0 {
        return 0;
    } else if num <= 2 {
        return 1;
    }

    match fib_store.get(&num) {
        Some(number) => {
            return *number as i32;
        }

        _ => {
            let results = fibonacci(num - 1, fib_store) + fibonacci(num - 2, fib_store);
            fib_store.insert(num, results);
            results
        }
    }
}

fn main() {
    let mut fib_store = HashMap::new();
    let results = fibonacci(15, &mut fib_store);
    println!("The results are  {results}", results = results)
}
