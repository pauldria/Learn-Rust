fn main() {
    let c: f64 = 20.0;
    let f1 = convert_to_fahrenheit(c);
    let f2 = convert_degrees(c, false);

    println!("Converting {} to {} (method 1)", c, f1);
    println!("Converting {} to {} (method 2)", c, f2);

    let fib20 = fib(20);
    println!("20th Fibonacci number is {}", fib20);

    // This is slow!
    // let fib50 = fib(50);
    // println!("50th Fibonacci number is {}", fib50);

    let fib50 = fib_fast(50);
    println!("50th Fibonacci number (fast) is {}", fib50);

    twelve_days_of_christmas();

    // String playtime
    let mut s = String::from("hello");
    s.push_str(" hello!");

}

fn convert_to_celsius(f: f64) -> f64 {
    return (f - 32.0) / 1.8;
}

fn convert_to_fahrenheit(c: f64) -> f64 {
    return 1.8 * c + 32.0;
}

fn convert_degrees(t: f64, to_celsius: bool) -> f64 {
    if to_celsius {
        return convert_to_celsius(t);
    }
    else {
        return convert_to_fahrenheit(t);
    }
}

fn fib(n: i64) -> i64 {
    return if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        fib(n - 1) + fib(n - 2)
    }
}

fn fib_fast(n: i64) -> i64 {
    return if n == 0 {
        0
    } else if n == 1 {
        1
    } else {
        let mut f = 1;
        let mut prev1 = 1;
        let mut tmp = 0;

        let mut i = 2;
        while i < n {
            tmp = f;
            f = f + prev1;
            prev1 = tmp;
            i += 1;
        }
        f
    }
}

fn twelve_days_of_christmas() -> () {
    let ordinals = ["first", "second", "third", "fourth", "fifth", "sixth", "seventh", "eighth", "ninth", "tenth", "eleventh", "twelfth"];
    let gifts = ["a partridge in a pear tree!",
                          "two turtle doves",
                          "three French hens",
                          "four calling birds",
                          "five golden rings",
                          "six geese-a-laying",
                          "seven swans-a-swimming",
                          "eight maids-a-milking",
                          "nine ladies dancing",
                          "ten lords-a-leaping",
                          "eleven pipers piping",
                          "twelve drummers drumming"];

    let mut i= 0;
    while i < 12 {
        println!("On the {} day of Christmas, my true love gave to me:", ordinals[i]);
        let mut j = i;
        while j > 0 {
            println!("{}", gifts[j]);
            j -= 1;
        }
        if i == 0 {
            println!("{}", gifts[0]);
        }
        else {
            println!("and {}", gifts[0]);
        }
        i += 1;
    }
}