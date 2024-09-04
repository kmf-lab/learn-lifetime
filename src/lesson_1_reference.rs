
/***********************************************************/
/* Lesson 1: Reference Review - Borrowing and scope */
/***********************************************************/

/// Before we go into lifetimes, let's review the basics of references in Rust.
/// References allow a function or block to access a value without taking ownership
/// of it, which is called borrowing. There are two types of references in Rust:
/// immutable references and mutable references.
///
/// References do not hold ownership of the value, so they do not drop or
/// clean up the value when they go out of scope. However, references do require
/// the value they reference has a lifetime >= to the reference itself.
/// This is where lifetimes come into play.
///
/// Lifetimes ensure that all references are valid for the duration of their usage.
/// They help prevent dangling references, which can lead to undefined behavior.
///
/// Imagine you lend a book to a friend (reference). You need to ensure that the book
/// remains available (valid) for as long as your friend needs it. Lifetimes in Rust
/// make sure that when the reference is still in use, the data it points to hasn’t been
/// dropped.

/********************/
/*   Vocabulary     */
/********************/

/// Immutable Reference:  A reference to a value that cannot be changed. Multiple immutable
///                       references can exist at the same time as long as there are no mutable.
/// Mutable Reference: A reference that allows the underlying value to be changed.
///                    Only one mutable reference to a particular value can exist at a time
///                    to prevent data races. This is enforced by Rust's borrow checker.
/// Lifetime:    A named region of code during which a REFERENCE is valid.  This is often
///              aligned with the scope defined by { } but could be smaller based on last
///              usage.  It could even be larger than a scope by naming the lifetime and
///              defining it outside the scope.
///
/// Elision:     The compiler's ability to infer lifetimes based on the structure of the code
///              Similar to the english contraction "I'm" vs "I Am"
///
///              It is pronounced as "ih-LIZH-uhn" where:
///                       ih sounds like the "i" in "kit"
///                       LIZH sounds like the "lis" in "collision"
///                       uhn sounds like the "un" in "button"
///                                   'collision but ill'
///              Say it with me.
/// Elided:      The act of removing or omitting something.
///              It is pronounced as "ih-LY-did" where:
///                      ih sounds like the "i" in "kit"
///                      LY sounds like the "ly" in "fly"
///                      did sounds like the word "did"
///              Say it with me.

////////////////////////////////////////////////////////////////
/* Lesson 1: Reference Review - Borrowing and scope */
////////////////////////////////////////////////////////////////

pub(crate) fn examples() {

    println!(" --------------- lesson 1 example 1 ---------------");
    {
        let data = String::from("Hello");
        let reference1 = &data;
        let reference2 = &data;
        //drop(data); // this will cause an error
        //data.push_str(" World"); // this will cause an error
        println!("reference1: {}", reference1);
        println!("reference2: {}", reference2);
        drop(data);
    }


    println!(" --------------- lesson 1 example 2 ---------------");
    {
        let mut data = String::from("Hello");

        let reference1 = &mut data;
        //let reference2 = &data; /// this will cause an error
        reference1.push_str(" World");

        println!("reference1: {}", reference1);
        println!("data: {}", &data);
    }


    println!(" --------------- lesson 1 example 3 ---------------");
    fn take_ref(my_text: &String) {
        println!("{:?}",my_text); //borrow ref and return nothing
    }
    {
        let s:String = String::from("message");
        take_ref(&s); //note the owned value s must be held while ref was used
        println!("{:?}",s);
    }


    println!(" --------------- lesson 1 example 4 ---------------");
    fn cant_make_ref(my_text: &String) -> &String {
        my_text //we can NOT pass in owned object and return &my_text, why?
    }
    {
        let s:String = String::from("message");
        let r = cant_make_ref(&s);
        println!("{:?}",r);
    }

    println!(" --------------- lesson 1 example 5 ---------------");
    //what is a &'static lifetime ?
    //it may be tempting to use 'static for everything; but it is not a good idea
    fn make_string(my_text: &'static str) -> String {
        String::from(my_text)
    }
    {
        let message = make_string("message"); // these literal bytes are in the binary
        println!("{:?}",message);
    }

}

// For more 'details' read the Rustonomicon:  https://doc.rust-lang.org/nomicon/lifetimes.html
// Note this quote:
// "Historically, Rust kept the borrow alive until the end of scope, so these examples might fail
// to compile with older compilers. Also, there are still some corner cases where Rust fails to
// properly shorten the live part of the borrow and fails to compile even when it looks like it
// should. These'll be solved over time."
// * From our previous lessons we saw examples of shortened scopes.
// * Caution the Rustonomicon has strange examples.
//
//
// Lifetimes in Rust act as placeholders, similar to generics, that specify how long
// references are valid, the specific duration is determined by the data they reference.
//
// Every reference has a lifetime. In very early releases of Rust developers had to
// annotate every & reference with a lifetime. Elision added to simplify this
// was formally part of the 2015 release.
//
// Due to lifetime elision rules, most lifetimes are inferred by the compiler and not
// explicitly specified.
//       ( The inferred lifetimes may also bring some baggage, more on this later)
//
// Elision rules are as follows:
//
// 1.  Each elided lifetime in input position becomes a distinct lifetime parameter.
//
// 2. If there is exactly one input lifetime position (elided or not), that lifetime is
//    assigned to all elided output lifetimes.
//
// 3. If there are multiple input lifetime positions, but one of them is &self or &mut self,
//    the lifetime of self is assigned to all elided output lifetimes.


/*  Example from The Rustonomicon      https://doc.rust-lang.org/nomicon/lifetimes.html

fn print(s: &str);                                      // elided
fn print<'a>(s: &'a str);                               // expanded  (IDEs often show this)

fn debug(lvl: usize, s: &str);                          // elided
fn debug<'a>(lvl: usize, s: &'a str);                   // expanded

fn substr(s: &str, until: usize) -> &str;               // elided
fn substr<'a>(s: &'a str, until: usize) -> &'a str;     // expanded

fn get_str() -> &str;                                   // ILLEGAL

fn frob(s: &str, t: &str) -> &str;                      // ILLEGAL (will see example later)

fn get_mut(&mut self) -> &mut T;                        // elided
fn get_mut<'a>(&'a mut self) -> &'a mut T;              // expanded

fn args<T: ToCStr>(&mut self, args: &[T]) -> &mut Command                  // elided
fn args<'a, 'b, T: ToCStr>(&'a mut self, args: &'b [T]) -> &'a mut Command // expanded

fn new(buf: &mut [u8]) -> BufWriter;                    // elided
fn new(buf: &mut [u8]) -> BufWriter<'_>;                // elided (with `rust_2018_idioms`)
fn new<'a>(buf: &'a mut [u8]) -> BufWriter<'a>          // expanded

 */

//  Note the last example, the underscore is a placeholder for the lifetime. It is not clear that
//  BufWriter is borrowing the buffer for the lifetime of the buffer. The lifetime is inferred
//  by the compiler.  We use the underscore to indicate that the lifetime is inferred for clarity.

