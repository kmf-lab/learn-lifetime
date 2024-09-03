

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

    println!(" --------------- lesson 2 example 1 ---------------");
    {
        //we have no references and do not need any lifetimes
        struct SimpleStruct {
            a: i32,
            b: i32,
        }
        impl SimpleStruct {
            fn consume(self) {}
            fn consume_return_a(self) -> i32 {
                self.a
            }
            fn process_a(self: &Self) -> &i32 { // Elided
                &self.a
            }
            fn update_a(self: &mut Self, a: i32) -> &i32 { // Elided
                self.a = a;
                &self.a
            }
            fn update_b1(self: &mut Self, b: &i32) -> &i32 { // Elided
                self.b = *b;
                &self.b
            }
            //false warning in RR, we broke the rules
            //we can fix this by moving <'goober> way up to impl<'goober> for better clarity
            fn update_b2<'goober>(self: &mut Self, b: &'goober i32) -> &'goober i32 {
                self.b = *b;
                &b
            }
        }
        {
            let mut s = SimpleStruct { a: 1, b: 2 };
            let new_b = 7;
            let _ref_b = s.update_b1(&new_b); //typical use case
            let ref_b = s.update_b2(&new_b); //strange use case
            println!("{:?} {:?}", ref_b, s.b);
            s.b = 10;
            println!("{:?} {:?}", ref_b, s.b);
        }
    }

    {
        println!(" --------------- lesson 2 example 2 ---------------");
        //I could use str but used String for simplicity

        //struct with unified lifetimes
        #[derive(Debug)]
        struct unified_struct<'a> {
            a: &'a String, // we typically give all refs the same lifetime
            b: &'a String
        }

        // the implementation block is generic over a lifetime 'a
        // the unified_struct is to use this lifetime 'a
        impl<'a> unified_struct<'a> {
            fn consume(self) {
            }
            //strange case where lifetime of a ref is outside our struct
            fn consume_return_a(self) -> &'a String { //consumed/dropped struct
                self.a
            }
            fn process_a(self: &Self) -> & String { //this ref is the same lifetime as struct
                self.a
            }
            fn update_a(self: & mut Self, a: &'a String) -> & String {
                self.a = a; // passing in a new ref requires us to tie these lifetimes together
                self.a
            }
            //note this is different from goob above because self.b is a ref
            fn update_b(self: &mut Self, b: &'a String) -> & String {
                self.b = b; //must be 'a because we set it here
                b
            }
        }

        {
          let a:String = String::from("a");
          let a_ref = &a;
            {
                let b: String = String::from("b");
                {
                    //within this struct we tied a and b together
                    let s = unified_struct { a: a_ref, b: &b };
                    println!("{:?}",&s);
                    //drop(b); //this will cause an error
                    println!("{:?}",s.a); //what was "inferred" here? baggage?

                }
            }
          println!("{:?}",a_ref); //what was "inferred" here?
        }
    }


    {
        println!(" ---------------  lesson 2 example 3  ---------------");

        //struct with split lifetimes

        #[derive(Debug)]
        struct split_struct<'a,'b> {
            a: &'a String,
            b: &'b String
        }

        fn some_strange_function<'a,'b>(d:split_struct<'a,'b>) -> &'a String {
            let split_struct {a, b} = d; //this is a "partial move" and a "destructuring"
            println!("{:?} {:?}",a,b);
            a
        }

        {
            let a_string = String::from("aaa");
            let mut a_ref;
            {
                let b_string = String::from("bbb");
                let mut data = split_struct {a: &a_string, b: &b_string};
                println!("{:?}",&data);
                a_ref = some_strange_function(data);

            }
            println!("{:?}",a_ref);

        }

        println!(" ---------------  lesson 2 example 3.5  ---------------");

        impl <'a,'b>split_struct<'a,'b> {
            fn consume(self) {
            }
            fn consume_return_a(self) -> &'a String {
                self.a
            }
            fn consume_return_b(self) -> &'b String {
                self.b
            }
            fn process_a(self: &Self) -> &String {
                self.a
            }
            fn process_b(self: &Self) -> &String {
                self.b
            }
            fn update_a(self: &mut Self, a: &'a String) -> &String {
                self.a = a;
                a
            }
            fn update_b(self: &mut Self, b: &'b String) -> &String {
                self.b = b;
                b
            }
        }

        {
            let a_string = String::from("aaa");
            let mut a_ref;
            {
                let b_string = String::from("bbb");
                let mut data = split_struct {a: &a_string, b: &b_string};

                a_ref = data.consume_return_a();
               // a_ref = data.consume_return_b(); //this can not work. what was inferred?

            }
            println!("{:?}",a_ref);
        }

        {
            let a_string = String::from("aaa");
            let mut a_ref;
            let text = String::from("");

            {
                let b_string = String::from("bbb");
                let mut data = split_struct {a: &a_string, b: &b_string};

                //if we assign outside our scope then the compiler discovers the problem
                // because data holds b and b_string is restricted to this scope.
                //a_ref = data.process_a(); //this is not allowed, it was not moved out of data


                let split_struct {a,b} = data; // this is a "partial move"

                a_ref = a; //here we can share the &str out of this scope with data

                // the rest of these lines are probably very bad practice
                // BUT lets do it anyway. a future version of Rust may stop us.

                println!("a:{:?} b:{:?}",data.a,data.b); //questionable practice

                data.a=&text; //questionable practice

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
}