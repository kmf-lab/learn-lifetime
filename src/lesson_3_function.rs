
/***********************************************************/
/* Lesson 3: Complex lifetime applications*/
/***********************************************************/

/// In this lesson, we will push the boundaries of what we've learned so far about
/// lifetimes in Rust. We'll dive deeper into more complex lifetime scenarios, including
/// lifetime elision, lifetime subtyping, and Higher-Rank Trait Bounds (HRTBs). Understanding
/// these concepts will give you the tools to write more flexible and reusable Rust code,
/// especially when dealing with functions and structs that involve multiple lifetimes.

/********************/
/*   Vocabulary     */
/********************/

/// Lifetime Elision: The process by which the Rust compiler automatically infers lifetimes
///                   in function signatures, reducing the need for explicit annotations.
/// Lifetime Subtyping: A concept where one lifetime is a subtype of another, meaning one
///                     lifetime can be shorter than another while still satisfying the
///                     requirements of the code.
/// Higher-Rank Trait Bounds (HRTBs): A way to express that a function must work for all
///                                   possible lifetimes, making your code more flexible.

////////////////////////////////////////////////////////////////
/* Lesson 3: Functions */
////////////////////////////////////////////////////////////////

pub(crate) fn examples() {

    println!(" --------------- lesson 3 example 1 ---------------");
    //using lifetime as a guard

    fn get_length_with_lifetime<'a>(s: &'a str) -> (usize, &'a str) {
        //lifetime is 'a and it is the same as the return value
        (s.len(), s)
    }

    {
        let mut s = String::from("Hello, world!");

        {
            // Borrowing the string immutably to get its length and keeping the reference
            let (length, _s_ref) = get_length_with_lifetime(&s);

            // Attempting to modify the string while it's borrowed will cause a compile-time error
            // This line will cause a compile-time error because `s` is still borrowed immutably
            //s.push_str(" New text");

            println!("Length with lifetime: {}", length);

            drop(_s_ref); //we must force _s_ref to live until now to protect the derived length

        } // The immutable borrow ends here

        // Now that the borrow is over, we can modify the string
        s.push_str(" New text");
        println!("String after modification: {}", s);
    }

    println!(" --------------- lesson 3 example 2 ---------------");
    // classic example of returning the shortest string
    // When you have multiple references with the same named lifetime, Rust ensures that
    // none of these references are used after the shortest-lived owned value expires. This
    // prevents any reference from outliving the data it points to, maintaining memory safety.
    fn shortest_length<'a>(x: &'a String,
                           y: &'a String,
                           z: &'a String) -> &'a String {
        let mut vec = vec![x, y, z];
        vec.sort_by(|a, b| a.len().cmp(&b.len()));
        vec[0]
    }
    {
        let x = String::from("Hello");
        let y = String::from("World");
        let z = String::from("!");

        let shortest = shortest_length(&x, &y, &z);
        println!("Shortest string: {}", shortest);
    }
    /////////
    //an alternate approach is to use lifetime subtyping
    ////////
    fn shortest_length_with_lifetime_subtyping<'a,'b:'a>(x: &'a String,
                                                         y: &'b String,
                                                         z: &'b String) -> &'a String {
        let mut vec = vec![x, y, z];
        vec.sort_by(|a, b| a.len().cmp(&b.len()));
        vec[0]
    }
    println!(" --------------- lesson 3 example 3 ---------------");
    //if we only return one of the references that is the only thing needing a lifetime
    fn shortest_length_broken<'a>(x: &'a String,
                                  y: & String,
                                  z: & String) -> &'a str {
        println!("{} {} {}",x,y,z);
        x.as_str() //also returning a slice to show it is not the same reference
    }
    {
        let x = String::from("Hello");
        let y = String::from("World");
        let z = String::from("!");
        let shortest = shortest_length_broken(&x, &y, &z);
        println!("Shortest string broken: {}", shortest);
    }


    println!(" --------------- lesson 3 example 4 ---------------");
    fn do_something1(x: String, y: &String) -> &String { // Elided, we have only one reference
        y
    }

    //after we have two we must decorate with lifetimes
    //see how we used a lifetime to make clear we will not be returning x
    //I made up the names not_returned and shared to make it clear. you can use any name.
    fn do_something2<'not_returned,'shared>(x: &'not_returned String, y: &'shared String) -> &'shared String {
        println!("{} {}",x,y);
        y
    }
    {
        let x = String::from("Hello");
        let y = String::from("World");

        let result2 = do_something1(x, &y);
        println!("Result: {}", result2);

        let z = String::from("!");
        let y_ref = do_something2(&z, &y);
        println!("Result: {}", y_ref);

    }

    println!(" --------------- lesson 3 example 5 ---------------");
    //how we could create a new owned value yet return a reference to it
    //this is a bit of a strange case, but it is possible

    //we know this is not allowed:
    //   fn attempted_return_owned<'a>(text: String) -> &'a String {
    //      &text //we had to be clear about its lifetime but could still not return it
    //   }
    {
      struct HoldingStruct {
          text:String
      }

      let mut holding = HoldingStruct { text: String::from("Hello") };

      // this strange method puts the new owned value into the outer scope and returns a ref.
      fn hold_and_ref<'a>(holding: &'a mut HoldingStruct, text: String) -> &'a String {
          holding.text = text; //we just took ownership of text into holding
          &holding.text
      }

      println!("{}",holding.text);
      println!("{}",hold_and_ref(&mut holding,String::from("world")));
      println!("{}",holding.text);
    }

    println!(" --------------- lesson 3 example 6 ---------------");
         // HRTB (Higher-Rank Trait Bounds) in Action
    {

        fn apply_to_str<F>(text: &str, f: F) -> & str
        where
             F: for<'a> Fn(&'a str) -> &'a str,
        {
            f(text)
        }

        {

            // Use the function with a closure that returns the first 3 characters of the string
            let result1 = apply_to_str("Hello", |s| &s[0..3]);
            println!("{}", result1);  // Output: Hel

            // // Use the function with a closure that converts the string to uppercase
            // let result2 = apply_to_str("Hello", |s| &s.to_uppercase());
            // println!("{}", result2);  // Output: HELLO
            //
            // // Use the function with a closure that reverses the string
            // let result3 = apply_to_str("Hello", |s| &s.chars().rev().collect::<String>());
            // println!("{}", result3);  // Output: olleH
            //
            // // Use the function with a closure that repeats the string twice
            // let result4 = apply_to_str("Hello", |s| &format!("{}{}", s, s));
            // println!("{}", result4);  // Output: HelloHello
            //
            // // Use the function with a closure that extracts a substring and converts it to uppercase
            // let result5 = apply_to_str("Hello, world!", |s| &s[7..12].to_uppercase());
            // println!("{}", result5);  // Output: WORLD
            //
            // Example where `for<'a>` is necessary
            // let result6 = apply_to_str("Hello", |s| {
            //     let part1: &str = &s[0..2];
            //     let part2: &str = &s[2..];
            //     &format!("{}-{}", part1, part2)
            // });
            // println!("{}", result6);  // Output: He-llo
        }

    }

    /*

    End with HRTB

    F: for<'a> Fn(&'a str) -> &'a str: This line is using HRTBs. It means that the function f
     can accept a reference with any lifetime 'a, and it returns a reference with the same
      lifetime 'a.

    HRTBs in Action: The for<'a> part indicates that the trait bound Fn(&'a str) ->
    &'a str must hold true for any lifetime 'a. In other words, the function f
     must work for any possible lifetime of the string slice it receives.

    Why It’s Useful:
    Without HRTBs, you’d be forced to work with specific, explicitly named lifetimes.
    HRTBs allow more flexibility, letting you write more generic and reusable code that
    works with references of any lifetime.
    In this example, apply_to_str can take any function that transforms a string slice and
    returns a slice, regardless of the specific lifetime of the slice, thanks to the use of HRTBs.

 */
}

