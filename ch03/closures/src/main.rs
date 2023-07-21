mod closure {
    fn add_doubles(closure: fn(i32) -> i32, x: i32, y: i32) -> i32 {
        return closure(x) + closure(y);
    }

    fn add_tripples(closure: Box<dyn Fn(i32) -> i32>, x: i32, y: i32) -> i32 {
        return closure(x) + closure(y);
    }

    pub fn execute_closure() {
        let closure_double = |int_input| return int_input * 2;
        let outcome = add_doubles(closure_double, 2, 3);
        println!("{outcome}");

        let step = 3;
        let closure_tripple = move |int_input| return int_input * step;
        let value = add_tripples(Box::new(closure_tripple), 2, 3);
        println!("{value}");
    }
}
mod thread_process {
    use std::thread::JoinHandle;
    use std::{thread, time};

    fn do_something(number: i8) -> i8 {
        println!("number {} is running", number);
        let two_seconds = time::Duration::new(2, 0);
        thread::sleep(two_seconds);
        return 2;
    }

    pub fn execute_thread() {
        let now = time::Instant::now();
        let thread_one: JoinHandle<i8> = thread::spawn(|| do_something(1));
        let thread_two: JoinHandle<i8> = thread::spawn(|| do_something(2));
        let thread_three: JoinHandle<i8> = thread::spawn(|| do_something(3));

        let one = thread_one.join();
        let two = thread_two.join();
        let three = thread_three.join();

        // let one: i8 = do_something(1);
        // let two: i8 = do_something(2);
        // let three: i8 = do_something(3);
        println!("Time elapsed: {:?}", now.elapsed());
        println!("Result {}", one.unwrap() + two.unwrap() + three.unwrap());
    }
}

mod async_await {
    use futures::{executor::block_on, future::join_all, join};
    use std::{thread, time, vec::Vec};

    async fn do_something(number: i8) -> i8 {
        println!("number {} is running", number);
        let two_seconds = time::Duration::new(5, 0);
        thread::sleep(two_seconds);
        return 2;
    }

    pub fn execute_async() {
        let now = time::Instant::now();

        let async_outcome = async {
            let mut futures_vec = Vec::new();
            let future_four = do_something(4);
            let future_five = do_something(5);

            futures_vec.push(future_four);
            futures_vec.push(future_five);

            let handles = futures_vec
                .into_iter()
                .map(async_std::task::spawn)
                .collect::<Vec<_>>();

            let results = join_all(handles).await;
            return results.into_iter().sum::<i8>();
        };
        let result = block_on(async_outcome);
        println!("time elapsed for join vec {:?}", now.elapsed());
        println!("Here is the result: {:?}", result);
    }
}

fn main() {
    // closure::execute_closure();
    // thread_process::execute_thread();
    async_await::execute_async();
}
