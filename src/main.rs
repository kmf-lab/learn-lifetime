
mod lesson_1_reference;
mod lesson_2_struct;
mod lesson_3_function;

fn main() {
    lesson_1_reference::examples();
    lesson_2_struct::examples();
    lesson_3_function::examples();
}


#[cfg(test)]
mod tests {
    use rand::Rng;

    const ATTENDEE_COUNT:usize = 30;

    #[test]
    fn test_pick_range_confirm() {
        let x = 42;
        let mut rng = rand::thread_rng();

        let (min, max) = (0..1000)
            .map(|_| rng.gen_range(1..=x))
            .fold((u32::MAX, u32::MIN), |(min, max), val| {
                (min.min(val), max.max(val))
            });

        println!("Min: {}, Max: {}", min, max);
        assert_eq!(1, min);
        assert_eq!(x, max);
    }

    #[test]
    fn test_pick_random() {

        let mut rng = rand::thread_rng();

        println!("The winners are:");
        let first_result = rng.gen_range(1..=ATTENDEE_COUNT);
        println!("First Selected Winner: {}", first_result);

        let second_result;
        loop {
            let temp = rng.gen_range(1..=ATTENDEE_COUNT);
            if temp != first_result {
                second_result = temp;
                break;
            }
        }
        println!("Second Selected Winner: {}", second_result);

        assert!(
            first_result >= 1 && first_result <= ATTENDEE_COUNT,
            "Returned value is out of range"
        );
        assert!(
            second_result >= 1 && second_result <= ATTENDEE_COUNT,
            "Returned value is out of range"
        );
    }
}



