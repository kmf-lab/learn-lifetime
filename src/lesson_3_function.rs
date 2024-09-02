/*

End with HRTB

fn apply_to_str<F>(f: F) -> &str
where
    F: for<'a> Fn(&'a str) -> &'a str,
{
    let s = String::from("Hello");
    f(&s)
}

fn main() {
    let result = apply_to_str(|s| &s[0..3]);
    println!("{}", result);  // Output: Hel
}

F: for<'a> Fn(&'a str) -> &'a str: This line is using HRTBs. It means that the function f can accept a reference with any lifetime 'a, and it returns a reference with the same lifetime 'a.

HRTBs in Action: The for<'a> part indicates that the trait bound Fn(&'a str) -> &'a str must hold true for any lifetime 'a. In other words, the function f must work for any possible lifetime of the string slice it receives.

Why It’s Useful:
Without HRTBs, you’d be forced to work with specific, explicitly named lifetimes. HRTBs allow more flexibility, letting you write more generic and reusable code that works with references of any lifetime.
In this example, apply_to_str can take any function that transforms a string slice and returns a slice, regardless of the specific lifetime of the slice, thanks to the use of HRTBs.

 */

// complex lifetime examples


/*





Lifetime elision rules allow the Rust compiler to automatically determine lifetimes in many cases,
reducing the need for explicit lifetime annotations in function signatures and method implementations.


In functions with a single reference parameter, the lifetime of the reference parameter is inferred to be the same as the lifetime of the function's return value (if it returns a reference).
( The inferred lifetime may also bring some baggage with it, more on this later)

When a function has a single reference parameter, Rust infers that the returned reference (if any)
 must be valid for the same lifetime as the input reference. This means the returned reference
 is tied to the same original owner as the input reference and cannot outlive it.




In more complex functions or methods, different reference parameters may have different lifetimes.
 Rust's lifetime system allows you to specify and enforce these different lifetimes to ensure safe borrowing.
All references with the same named lifetime are bound by the shortest-lived reference they point to.

If multiple references share the same named lifetime, the actual lifespan is defined by the owned
 resources these references are pointing to. This ensures that references do not outlive the data they point to.

When you have multiple references with the same named lifetime, Rust ensures that all of these
references are not used after the shortest-lived reference expires. This prevents any reference
from outliving the data it points to, maintaining memory safety.



*/


/*

When you have multiple references with the same named lifetime, Rust ensures that all of these
 references are not used after the shortest-lived reference expires. This prevents any reference
  from outliving the data it points to, maintaining memory safety.

For example, in the function shortest_lifetime, both x and y have the same lifetime 'a. The
returned reference will be valid only within the scope where both x and y are valid.
If x and y have different valid durations, the reference must be dropped or cease to be
 used after the shorter duration expires.

By enforcing these rules, Rust ensures that references do not outlive the data they point to,
which prevents dangling references and ensures safe memory access.

 */



//   rustonomicon .. quotes.
//

pub(crate) fn examples() {

    //guard example
    fn get_length_with_lifetime<'a>(s: &'a str) -> (usize, &'a str) { //lifetime is 'a and it is the same as the return value
        (s.len(), s)
    }

    {
        let mut s = String::from("Hello, world!");

        {
            // Borrowing the string immutably to get its length and keeping the reference
            let (length, _s_ref) = get_length_with_lifetime(&s);

            // Attempting to modify the string while it's borrowed will cause a compile-time error
            // This line will cause a compile-time error because `s` is still borrowed immutably
            s.push_str(" New text");

            println!("Length with lifetime: {}", length);

            //  drop(_s_ref);

        } // The immutable borrow ends here

        // Now that the borrow is over, we can modify the string
        s.push_str(" New text");
        println!("String after modification: {}", s);
    }

    //TODO: string example with 3 refences, sort and return the shortest

    //TODO: show hardcoded first example need now have liftime on the unused

    //TODO:struct passedi to an object - put in previous

    //TODO: only 1 ref and owned object

    //TODO:  ref in leas to ref out,
    //     try example pasing in mut struct to populate and return ref...

    //TODO: move async 'a if we have time


    // pub(crate) fn examples2<'b>() {
    //
    //     //other examples
    //     {                                                     //---------------+--  'a
    //         let mut data: Vec<i32> = vec![1, 2, 3];
    //         {                                                  //----------+--- 'b
    //             let x: &'b i32 = Index::index::<'b>(& data, 0);
    //             {                                              //----------+--- 'c
    //                 Vec::push(& mut data, 4);
    //             }
    //             println!("{}",x);
    //         }
    //     }
    //
    // }
}