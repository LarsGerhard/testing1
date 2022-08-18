/* Block comment explaining things!
This is just me messing around with basic rust components, as I have a Python background and there's a bit of a learning curve.
Enjoy!
 */

// You can include stuff from another file with
mod testing2;

// Main function. Pretty similar to most programming languages in that this is what gets run.
fn main() {
    {// My IDE is nice since it kinda typecasts for me. Here's an integer
        let x = 10;

        // Calling the exponent function brought into scope earlier. You can also apparently specify the size of the variable in declaration with _(type)
        let y = 10_i128.pow(32);

        // x isn't mutable, but I can replace it through shadowing with a new value
        let x = x + 4;

        // This is a vector array. I'm not sure yet what the difference is between a vector and tuple aside from how you call an element
        let z = [1, 2, 3, 4, 5, 6];

        // Here's the aforementioned tuple
        let list = ("I went to the store", 64, 32);

        // I don't ever use n1, so putting _n1 keeps it from complaining at compile time. _ throws away the number, if I don't care about assigning a value to a variable (i.e. a function spits out an argument that I don't care about)
        let (phrase, _n1, _) = (list.0, list.1, list.2);

        // Unlike Python (kinda sad) you can't just concatenate different variable types together in println!. So the brackets provide positional arguments for each variable I want to put in that spot as a string.
        // Also note that for a tuple I select the 3rd element with .2, while for a vector I select the 4th element more traditionally for an array with [3]
        println!("{0} {1} {2} {3} {4}", { x }, { y }, { list.2 }, { phrase }, { z[3] });

        // Here I'm calling a function I made that takes three arguments and returns one, and printing what it returns. This functions pretty similarly to Python on the call side, not so much on the actual function side.
        println!("{}", { cool_thing(x, list.0, 43.4) });
    }// Now we're done using our prior variables, so we can free up memory by limiting them to this scope

    // Making a new variable, this time a boolean
    let truth = true;

    // Similar to cases from Java. You take an input and return whatever matches that input.
    let num = match truth{
        true =>7,
        false =>6,
    };

    // More println!s for testing
    println!("{}",{num});

    // Because we released the string prior, we need to make a new one
    let phrase = "New phrase!";

    // .len acts similar to Java where it returns the length of the string. Note the . format shared with tuples
    println!("{}",{phrase.len()});

    // To avoid using too much memory, we'll make a new environment to use a package min
    {
        // How package components are brought into scope. The asterisk brings in everything at a penalty
        use std::cmp::*;

        // We can now use this imported function to determine what's smallest
        let x = min(4,7);

        println!("{}",{x});
    }
    //Now that we're out of the previous scope, for a one off use of that function we can directly call it like this for memory efficiency.

    {
        let x = std::cmp::max(4, 7);

        println!("{}", { x });
    }

    // Now I can call my method from the other file
    main2()

}

// My first function! Odd that Rust uses snake case, I find it really wierd. This'll be a couple lines as I dissect how it's different from making functions in Python
// Every argument needs to have its type predetermined. : is used to specify a type, which is done automatically when declaring variable types but not really when declaring arguments. Probably because it has no way to infer the type.
// Also, you need to specify what you return with -> (type).
fn cool_thing(x:i32,y:&str,z:f64) -> bool {

    // I can declare local variables in the brackets without affecting the variables outside
    {
        let x = 32.3;

        // Test to prove that it returns this x. Also apparently math functions are just implemented with .(func?!?!)
        println!("{0} {1} {2}", {x},{y},{z.sin()});
    }

    // Now it returns x declared in main
    println!("{0} {1} {2}", {x},{y},{z});

    // Basic if/then setup, pretty similar to Python and other languages
    if x < 32 {

        //Because we're at the end of the programme, we can forgo the semicolon and it'll return whatever doesn't have it (no need for return in this case)
        true
    } else {
        false
    }

}