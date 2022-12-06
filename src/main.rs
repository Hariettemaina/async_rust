use std::time::Duration; //import Duration from standard library
use tokio::time::sleep; //import sleep from tokio

#[tokio::main]
async fn main() {

    // spawning 2 tokio tasks to execute my_function()
    //creating an empty vector to store our task handle
    let mut handles = vec![];
    
    // create a for loop with four iterations
    for i in 0..2 {
        // calling the spawn funtion on the tokio module
        let handle = tokio::spawn(async move {
            my_function(i).await
        });
        //tokio spawn takes a future as an argument and returns a joinhandle
        handles.push(handle);
    }
    //loop through the task handles and await them 
    for handle in handles{
        handle.await.unwrap();
        //handle returns a result type which could be an error if the tasks panics 
    }
}

//updating my_fuction to accept an integer
async fn my_function(i: i32) {
    println!("[{i}] I'm an async function!");
    let s1 = read_from_database().await;
    println!("[{i}] First result: {s1}");
    let s2 = read_from_database().await;
    println!("[{i}] Second result: {s2}");
}


async fn read_from_database() -> String{
    sleep(Duration::from_millis(50)).await; // stops the current future fro executing for given duration instead of an entire thread
    
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
