use std::thread;
use std::time::Duration;
use std::collections::HashMap;
use std::hash::Hash;


struct Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        K: Hash + Eq + Copy, // required to work properly
        V: Copy,  // required to work properly
{
    calculation: T,
    value: HashMap<K, V>,
}

impl<T, K, V> Cacher<T, K, V>
    where
        T: Fn(K) -> V,
        K: Hash + Eq + Copy,
        V: Copy,
{
    fn new(calculation: T) -> Cacher<T, K, V> {
        Cacher {
            calculation,
            value: HashMap::new(),
        }
    }

    fn value(&mut self, arg: K) -> V {
        match self.value.get(&arg) {
            Some(v) => *v, // deref will copy the value
            None => {
                let v = (self.calculation)(arg);
                self.value.insert(arg, v);
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
        println!("Today, do {} push-ups!", expensive_result.value(intensity));
        println!("Next, do {} sit-ups!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!("Today, run for {} minutes", expensive_result.value(intensity));
        }
    }
}

fn main() {
    let simulated_user_specified_value = 26;
    let simulated_random_number = 3;

    generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn call_with_different_values() {
        let mut c = Cacher::new(|a| a);

        let v1 = c.value(1);
        let v2 = c.value(2);

        assert_eq!(v2, 2);
    }
}
