use std::io;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fmt;
use std::time::Instant;


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
    production: i32,
    distances: Vec<(i32, i32)> // (distance, id)
}

impl Factory {
    fn up(&mut self) {
        self.owner += 1;
    }
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
    }
    init_factories_distance(factory_count, &mut factories, &factory_distance);


    // game loop
    loop {
        let start = Instant::now();

        let mut troops: LinkedList<Troop> = LinkedList::new();
        init_entities(&mut troops, &mut factories);
        let mut commands = Vec::new();


        max_strategy(&mut factories, &mut commands);
        print_factories(&factories);

        print_commands(&commands);


        let elapsed = start.elapsed();
        print_err!("Elapsed: {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
    }
}

fn init_factories_distance(factory_count: i32, factories: &mut HashMap<i32, Factory>, factory_distance: &HashMap<(i32, i32), i32>) {
    for i in 0..factory_count as i32 {
        let mut distances: Vec<(i32, i32)> = Vec::new();
        for (&(id1, id2), distance) in factory_distance.iter() {
            if id1 == i {
                distances.push((*distance, id2));
            } else if id2 == i {
                distances.push((*distance, id1));
            }

        }


        distances.sort();
        for &(id1, id2) in distances.iter() {
            print_err!("Distance:{} Id:{}", id1, id2);
        }

        print_err!("---------");

        factories.insert(i, Factory{id: i, owner: -99, cyborg_count: -99, production: -99, distances: distances});
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
            let mut factory = factories.get_mut(&entity_id).unwrap();
            factory.owner = arg_1;
            factory.cyborg_count = arg_2;
            factory.production = arg_3;
        } else {
            troops.push_back(Troop{id: entity_id, owner: arg_1, factory_start: arg_2, factory_end: arg_3, cyborg_count: arg_4, turn_remaining: arg_5});
        }

    }
}

fn max_strategy(factories: &mut HashMap<i32, Factory>, commands: &mut Vec<String>) {
    let mut id1 = -999;
    let mut cyborg_count = -999;

    // Get max
    for (id, factory) in factories.iter() {
        if factory.owner == 1 && factory.cyborg_count > cyborg_count {
            id1 = factory.id;
            cyborg_count = factory.cyborg_count;
        }
    }

    print_err!("{}  {}", id1, cyborg_count);

    let factory = factories.get(&id1).unwrap();

    for &(distance, id2) in factory.distances.iter() {
        let factory2 = factories.get(&id2).unwrap();
        if factory2.owner != 1 {
            commands.push(format!("MOVE {} {} {}", id1, id2, cyborg_count - 1));
            break;
        }
    }
}

fn swarm_strategy(factories: &mut HashMap<i32, Factory>, commands: &mut Vec<String>) {
    for (id, factory) in factories.iter() {
        if factory.owner == 1 && factory.cyborg_count > factories.len() as i32 {
            let mut id1 = factory.id;
            let mut cyborg_count = factory.cyborg_count;

            for (id, factory2) in factories.iter() {
                if factory2.owner != 1 {
                    let mut id2 = factory2.id;
                    commands.push(format!("MOVE {} {} {}", id1, id2, 1));
                }
            }
        }
    }

}

fn print_factories(factories: &HashMap<i32, Factory>) {
    for (id, factory) in factories.iter() {
        print_err!("{} {} {} {}", factory.id, factory.owner, factory.cyborg_count, factory.production);
    }
}

fn print_commands(commands: &Vec<String>) {
    if commands.len() > 0 {
        let mut final_command = "MSG El Psy Congroo".to_string();
        for command in commands.iter() {
            final_command.push_str(";");
            final_command.push_str(&command);
        }
        println!("{}", final_command);
    } else {
        println!("WAIT");
    }
}
