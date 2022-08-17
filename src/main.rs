use std::i128;

fn main() {
    let x = 10;

    let y = i128::pow(10,32);

    let x = x + 4;

    let z = [1,2,3,4,5,6];

    let list = ("I went to the store",64,32);

    let (phrase, _n1, _) = (list.0,list.1,list.2);

    println!("{0} {1} {2} {3} {4}",{x},{y},{list}.2, {phrase},{z[3]});

    println!("{}",{cool_thing(x,list.0,43.4)});

    let truth = true;

    let num = match truth{
        true =>7,
        false =>6,
    };
    println!("{}",{num});
}

fn cool_thing(x:i32,y:&str,z:f64) -> bool {
    {
        let x = 32.3;
        println!("{0} {1} {2}", {x},{y},{z});
    }
    println!("{0} {1} {2}", {x},{y},{z});

    if x < 32 {
        true
    } else {
        false
    }
}