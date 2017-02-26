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
    let mut factories: HashMap<i32, Factory> = HashMap::new();

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
        init_entities(&mut troops, &mut factories);


        let mut id1 = -1;
        let mut id2 = 0;
        let mut cyborg_count = 0;
        for (id, factory) in factories.iter() {
            if factory.owner == 1 && factory.cyborg_count > 10{
                id1 = factory.id;
                cyborg_count = factory.cyborg_count;
            }
            if factory.owner == 0 {
                id2 = factory.id;
            }
            print_err!("{} {} {} {}", factory.id, factory.owner, factory.cyborg_count, factory.production);
        }


        // Write an action using println!("message...");
        // To debug: print_err!("Debug message...");


        // Any valid action, such as "WAIT" or "MOVE source destination cyborgs"
        if id1 != -1 && id1 != id2 {
            println!("MOVE {} {} {}", id1, id2, cyborg_count);
        } else {
            println!("WAIT");
        }

    }
}

fn init_entities(troops: &mut LinkedList<Troop>, factories: &mut HashMap<i32, Factory>) {
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

        if entity_type == "FACTORY" {
            factories.insert(entity_id, Factory{id: entity_id, owner: arg_1, cyborg_count: arg_2, production: arg_3});
        } else {
            troops.push_back(Troop{id: entity_id, owner: arg_1, factory_start: arg_2, factory_end: arg_3, cyborg_count: arg_4, turn_remaining: arg_5});
        }

    }
}
