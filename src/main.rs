#[tokio::main]

async fn main() {

    // spawning 2 tokio tasks to execute my_function()
    //creating an em
    let mut handles = vec![];
    

    for i in 0..2 {
        let handle = tokio::spawn(async move {
            my_function().await
        });
        handles.push(handle);
    }
}


async fn my_function() {
    println!("I'm an async function!");
    let s1 = read_from_database().await;
    println!("First result: {s1}");
    let s2 = read_from_database().await;
    println!("Second result: {s2}");
}


async fn read_from_database() -> String{
    "DB_Result".to_owned()
}

// #![allow(unused)]
// fn main() {
// trait SimpleFuture {
//     type Output;
//     fn poll(&mut self, wake: fn()) -> Poll<Self::Output>;
// }

// enum Poll<T> {
//     Ready(T),
//     Pending,
// }
// }
