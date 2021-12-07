use std::thread;
use std::time::Duration;

fn main() {
    let nums = vec![1, 2, 3];
    for num in nums {
        println!("{}", num);
    }
    // let simulated_user_specified_value = 10;
    // let simulated_random_number = 7;

    // generate_workout(simulated_user_specified_value, simulated_random_number);
}

#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_my_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> {
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {

    use super::*;
    // #[test]
    // fn iterator_demonstration() {
    //     let v1 = vec![1, 2, 3];

    //     let mut v1_iter = v1.iter();

    //     assert_eq!(v1_iter.next(), Some(&1));
    //     assert_eq!(v1_iter.next(), Some(&2));
    //     assert_eq!(v1_iter.next(), Some(&3));
    //     assert_eq!(v1_iter.next(), None);
    // }

    // #[test]
    // fn iterator_sum() {
    //     let v1 = vec![1, 2, 3];

    //     let v1_iter = v1.iter();

    //     let total: i32 = v1_iter.sum();

    //     assert_eq!(total, 6);
    // }

    // #[test]
    // fn sum() {
    //     let v1 = vec![1, 2, 3];
    //     let sum: Vec<u32> = v1.iter().map(|num| num + 1).collect();
    //     println!("{:?}", sum);
    //     assert_eq!(sum, vec![2, 3, 4])
    // }

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_my_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}

struct Cacher<T>
where
    T: Fn(u32) -> u32,
{
    calculation: T,
    value: Option<u32>,
}

impl<T> Cacher<T>
where
    T: Fn(u32) -> u32,
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
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
        println!("Next, do {} situps!", expensive_result.value(intensity));
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                expensive_result.value(intensity)
            );
        }
    }
}
