pub fn run() {
    // print to console
    println!("hello from print");

    //basic formatting
    println!("{} looks like {}", "ben", "neb");

    //positional arguments
    println!("{0} is from {1} and {0} likes {2}", "ben", "lincoln", "music");

    // named arguments
    println!("{name} likes {activity}",
    name = "ben",
    activity = "running"
    );

    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);
    }


