use futures::executor::block_on;

async fn  task_hello(){
    println!("Hello")
}

fn main() {
    let t1 = task_hello();
    let t2 = task_hello();
    let t3 = task_hello();
    for t in [t1, t2, t3] {
        block_on(t)
    }
}
