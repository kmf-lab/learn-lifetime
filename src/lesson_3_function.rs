
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
    fn do_something2<'not_returned,'shared>(x: &'not_returned String,
                                            y: &'shared String) -> &'shared String {
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
    // There is very little documentation on higher-ranked lifetimes, so let's break this down.

    {
        fn apply_to_str<F>(text: &str, f: F) -> &str
        where
        // If we only had one reference, lifetimes could be elided. But we have more than one,
        // so we must specify how lifetimes relate to each other.
        // Here, F is a closure that takes two references with different lifetimes.
            F: for<'a, 'goober> Fn(&'a str, &'goober str) -> &'a str,
        // The for<'a, 'goober> syntax serves a similar purpose to impl<'a, 'goober>,
        // but it's used here to make the closure flexible over lifetimes.
        {
            // We pass the main text and "goober" but that could be version or trace data...
            f(text, "goober")
        }
        {
            // Use the function with a closure, returns the first 4 characters of the main string
            let result1 = apply_to_str("Hello", |s, g| &s[0..4]);
            println!("{}", result1);  // Output: Hel
        }

        // Without HRTBs, you would be forced to work with specific, explicitly named lifetimes.
        // HRTBs allow more flexibility, letting you write more generic and reusable code that
        // works with references of any lifetime without having to introduce a struct or trait.
        //
        // In this example, apply_to_str can take any function that transforms a string slice.
        // Because the function signature takes two references, lifetimes are required. However,
        // without the for<'a, 'goober> syntax, we would have to specify fixed lifetimes in the
        // trait, which would make the code less flexible.
    }
}

