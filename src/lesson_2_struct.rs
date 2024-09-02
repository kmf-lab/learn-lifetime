

/***********************************************************/
/* Lesson 2: Struct and self */
/***********************************************************/

/// Both the easiest and most complex examples of lifetimes can be seen when adding methods
/// to a struct. This is because every reference in a struct needs a lifetime, and that lifetime
/// is part of the type of the struct. References returned by implementing methods returning must
/// have the same lifetime as the struct.  In many ways this is classic OO programming, but with
/// the added complexity of lifetimes.
///
/********************/
/*   Vocabulary     */
/********************/

/// Struct: A data structure that groups together values of different typed fields under a name.
/// Implementation: The block of code that defines the methods of a struct.
/// Method: A function that is defined within the context of a struct. (Think OO programming)
/// Ownership: The concept of a single value having a single owner that is responsible for
///            cleaning up the value when it goes out of scope.
/// Drop: The process of cleaning up a value when it goes out of scope.
/// Lifetime: A named region of code during which a REFERENCE is valid.
///
////////////////////////////////////////////////////////////////
/* Lesson 2: Struct and self - Simple lifetime */
////////////////////////////////////////////////////////////////

pub(crate) fn examples() {

    // self examples on a simple struct



    // 1) Scope and Ownership
    println!(" --------------- lesson 1 example 1 ---------------");

    {
        struct SimpleStruct {
            a: i32,
            b: i32,
        }

        //struct with unified lifetimes
        #[derive(Debug)]
        struct unified_struct<'a> {
            a: &'a str,
            b: &'a str
        }

        //TODO

    }

/*


Methods on structs must explicitly declare lifetimes only if they reference data within the struct.

When defining methods for a struct that borrow data from the struct, you need to specify
lifetimes to ensure that the references are valid for the required duration.
Multiple references in a function or method can have different lifetimes.
 */

    {
        println!(" --------------- lesson complex example ---------------");

        //struct with split lifetimes

        #[derive(Debug)]
        struct split_struct<'a,'b> {
            a: &'a str,
            b: &'b str
        }

        fn some_strange_function<'a,'b>(d:split_struct<'a,'b>) -> &'a str {
            let split_struct {a, b} = d;
            println!("{:?} {:?}",a,b);
            a
        }

        { //after the other??
            let a_string = "aaa".to_string();
            let mut a_ref;
            {
                let b_string = "bbb".to_string();
                let mut data = split_struct {a: &a_string, b: &b_string};
                println!("{:?}",&data);

                a_ref = some_strange_function(data);

            }
            println!("{:?}",a_ref);

        }

        impl <'a,'b>split_struct<'a,'b> {
            fn consume(self) {
            }
            fn consume_return_a(self) -> &'a str {
                self.a
            }
            fn consume_return_b(self) -> &'b str {
                self.b
            }
            fn process_a(self: &Self) -> &str {
                self.a
            }

            fn process_b(self: &Self) -> &str {
                self.b
            }

            fn update_a(self: &mut Self, a: &'a str) -> &str {
                self.a = a;
                a
            }

            fn update_b(self: &mut Self, b: &'b str) -> &str {
                self.b = b;
                b
            }
        }

        {
            let a_string = "aaa".to_string();
            let mut a_ref;
            {
                let b_string = "bbb".to_string();
                let mut data = split_struct {a: &a_string, b: &b_string};

                a_ref = data.consume_return_a();
                //a_ref = data.consume_return_b(); //this can not work.


            }
            println!("{:?}",a_ref);
        }

        {
            let a_string = "aaa".to_string();
            let mut a_ref;

            {
                let b_string = "bbb".to_string();
                let mut data = split_struct {a: &a_string, b: &b_string};

                //if we assign outside our scope then the compiler discovers the problem
                // because data holds b and b_string is restricted to this scope.
                //a_ref = data.process_a(); //this is not allowed, it was not moved out of data


                let split_struct {a,b} = data; // this is a "partial move"

                a_ref = a; //here we can share the &str out of this scope with data

                // the rest of these lines are probably very bad practice
                // BUT lets do it anyway. a future version of Rust may stop us.

                println!("a:{:?} b:{:?}",data.a,data.b); //questionable practice

                data.a=""; //questionable practice

                let b_ref = data.process_b(); //questionable practice
                println!("{:?} ",b_ref);

                let x_ref = data.process_a(); //questionable practice
                println!("{:?} ",x_ref);

                println!("{:?}",data);
                let x = data.consume_return_b(); //questionable practice
                println!("{:?}",x);


            };
            println!("{:?}",a_ref);


        }

    }



    {


    }
}