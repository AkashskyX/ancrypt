use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};
use threadpool::ThreadPool;
use std::env;

fn main() {
    

    let default_pool_size = 4095;
   // env::set_var("POOL_SIZE", "5000");
    let pool_size = env::var("POOL_SIZE")
    .ok()
    .and_then(|val| val.parse().ok())
    .unwrap_or(default_pool_size);


    println!("POOOOL: {}" , pool_size);
    let pool =  ThreadPool::new(pool_size);


      
    let counter = Arc::new(Mutex::new(0));
    let start = Instant::now();



    for i in 0..2 {
        let counter = Arc::clone(&counter);

       pool.execute(move || {

        sleep(Duration::from_micros(1));
        let mut num = counter.lock().unwrap();
        *num += 1;

        println!("This is thread number {} from the THREADPOOL : {}" , i , i + 1 )

       })
    }



    pool.join();







    

    println!("THe final count is: {:?}", counter.lock().unwrap());
    println!("Elapsed time: {:?}", start.elapsed());
}
