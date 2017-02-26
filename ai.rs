use std::io;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fmt;

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


struct Factory {
    id: i32,
    owner: i32,
    cyborg_count: i32,
    production: i32
}

struct Troop {
    id: i32,
    owner: i32,
    factory_start: i32,
    factory_end: i32,
    cyborg_count: i32,
    turn_remaining: i32
}


fn main() {
    let mut factory_distance: HashMap<(i32, i32), i32> = HashMap::new();

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

        factory_distance.insert((factory_1, factory_2), distance);
        print_err!("{:?}", factory_distance.get(&(factory_1, factory_2)))
    }

    // game loop
    loop {
        let mut troops: LinkedList<Troop> = LinkedList::new();
        let mut factories: LinkedList<Factory> = LinkedList::new();

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

            match entity_type.as_ref() {
                "FACTORY" => factories.push_back(Factory{id: entity_id, owner: arg_1, cyborg_count: arg_2, production: arg_3}),
                "TROOP" => troops.push_back(Troop{id: entity_id, owner: arg_1, factory_start: arg_2, factory_end: arg_3, cyborg_count: arg_4, turn_remaining: arg_5}),
                _ => print_err!("???????")
            }


        }
        
        }

        // Write an action using println!("message...");
        // To debug: print_err!("Debug message...");


        // Any valid action, such as "WAIT" or "MOVE source destination cyborgs"
        println!("WAIT");
    }
}
