use std::io;

macro_rules! print_err {
    ($($arg:tt)*) => (
        {
            use std::io::Write;
            writeln!(&mut ::std::io::stderr(), $($arg)*).ok();
        }
    )
}

macro_rules! parse_input {
    ($x:expr, $t:ident) => ($x.trim().parse::<$t>().unwrap())
}

/**
 * Auto-generated code below aims at helping you parse
 * the standard input according to the problem statement.
 **/
fn main() {
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let factory_count = parse_input!(input_line, i32); // the number of factories
    let mut input_line = String::new();
    io::stdin().read_line(&mut input_line).unwrap();
    let link_count = parse_input!(input_line, i32); // the number of links between factories
    for i in 0..link_count as usize {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let inputs = input_line.split(" ").collect::<Vec<_>>();
        let factory_1 = parse_input!(inputs[0], i32);
        let factory_2 = parse_input!(inputs[1], i32);
        let distance = parse_input!(inputs[2], i32);
    }

    // game loop
    loop {
        let mut input_line = String::new();
        io::stdin().read_line(&mut input_line).unwrap();
        let entity_count = parse_input!(input_line, i32); // the number of entities (e.g. factories and troops)
        for i in 0..entity_count as usize {
            let mut input_line = String::new();
            io::stdin().read_line(&mut input_line).unwrap();
            let inputs = input_line.split(" ").collect::<Vec<_>>();
            let entity_id = parse_input!(inputs[0], i32);
            let entity_type = inputs[1].trim().to_string();
            let arg_1 = parse_input!(inputs[2], i32);
            let arg_2 = parse_input!(inputs[3], i32);
            let arg_3 = parse_input!(inputs[4], i32);
            let arg_4 = parse_input!(inputs[5], i32);
            let arg_5 = parse_input!(inputs[6], i32);
        }

        // Write an action using println!("message...");
        // To debug: print_err!("Debug message...");


        // Any valid action, such as "WAIT" or "MOVE source destination cyborgs"
        println!("WAIT");
    }
}
