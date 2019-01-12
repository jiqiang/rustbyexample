use std::collections::HashMap;
use std::thread;
use std::time::Duration;

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    cache: HashMap<u32, u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            cache: HashMap::new(),
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.cache.get(&arg) {
            Some(&v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.cache.insert(arg, v);
                v
            }
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        num
    });

    if intensity < 25 {
        println!("Today, do {} pushups!", expensive_result.value(intensity));
        println!("Next, do {} situps", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
    println!("{}", expensive_result.value(5));
    println!("{}", expensive_result.value(6));
    println!("{}", expensive_result.value(5));
    println!("{}", expensive_result.value(6));
    println!("{}", expensive_result.value(5));
    println!("{}", expensive_result.value(6));
}

fn main() {
    let simulated_user_specified_value = 30;
    let simulated_random_number = 3;
    generate_workout(simulated_user_specified_value, simulated_random_number);
}
