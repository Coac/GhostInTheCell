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


#[derive(Clone)]
struct Factory {
    id: i32,
    owner: i32,
    cyborg_count: i32,
    production: i32,
    distances: Vec<(i32, i32)>, // (distance, id)
    cyborg_count_combat: i32
}

trait HasOwner {
    fn get_owner(&self) -> i32;


    fn is_player(&self) -> bool {
        return self.get_owner() == 1;
    }

    fn is_enemy(&self) -> bool {
        return self.get_owner() == -1;
    }

    fn is_neutral(&self) -> bool {
        return self.get_owner() == 0;
    }
}

impl HasOwner for Factory {
    fn get_owner(&self) -> i32 { self.owner }
}

#[derive(Clone)]
struct Troop {
    id: i32,
    owner: i32,
    factory_start: i32,
    factory_end: i32,
    cyborg_count: i32,
    turn_remaining: i32
}

impl HasOwner for Troop {
    fn get_owner(&self) -> i32 { self.owner }
}


#[derive(Clone)]
struct GameState {
    factory_distance: HashMap<(i32, i32), i32>,
    factories: HashMap<i32, Factory>,
    troops: LinkedList<Troop>,
    commands: Vec<String>,
    bomb_count: i32,
    bomb_last: i32,
    troop_commands: LinkedList<Troop>,
}


impl GameState {
    fn new() -> GameState {
        GameState {
            factory_distance: HashMap::new(),
            factories: HashMap::new(),
            troops: LinkedList::new(),
            commands: Vec::new(),
            bomb_count: 2,
            bomb_last: -99,
            troop_commands: LinkedList::new()
        }
    }

    fn init_factories_distance(&mut self, factory_count: i32) {
        for i in 0..factory_count as i32 {
            let mut distances: Vec<(i32, i32)> = Vec::new();
            for (&(id1, id2), distance) in self.factory_distance.iter() {
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

            self.factories.insert(i, Factory{id: i, owner: -99, cyborg_count: -99, production: -99, distances: distances, cyborg_count_combat: 0});
        }
    }

    fn init_entities(&mut self) {
        self.troops.clear();

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
                let mut factory = self.factories.get_mut(&entity_id).unwrap();
                factory.owner = arg_1;
                factory.cyborg_count = arg_2;
                factory.production = arg_3;
            } else if entity_type == "TROOP" {
                self.troops.push_back(Troop{id: entity_id, owner: arg_1, factory_start: arg_2, factory_end: arg_3, cyborg_count: arg_4, turn_remaining: arg_5});
            }

        }

    }

    fn max_strategy(&mut self) {
        let max_factory_option = self.factories.iter()
            .filter(|&(ind, fac)| fac.is_player())
            .max_by_key(|&(ind, fac)| fac.cyborg_count);

        if !max_factory_option.is_some() { return }

        let mut max_factory = max_factory_option.unwrap().1;

        //print_err!("Ordering {}  {}", max_factory.id, max_factory.cyborg_count);

        // Closest factory
        for &(distance, id2) in max_factory.distances.iter() {
            let factory2 = self.factories.get(&id2).unwrap();
            if !factory2.is_player() && factory2.production > 0 {
                self.commands.push(format!("MOVE {} {} {}", max_factory.id, id2, max_factory.cyborg_count));
                self.troop_commands.push_back(Troop{id: 999, owner: 1, factory_start: max_factory.id, factory_end: id2, cyborg_count: max_factory.cyborg_count, turn_remaining: distance});
                break;
            }
        }

    }

    fn swarm_strategy(&mut self) {
        for (id, factory) in self.factories.iter() {
            if factory.is_player() && factory.cyborg_count > self.factories.len() as i32 {
                let mut id1 = factory.id;
                let mut cyborg_count = factory.cyborg_count;

                for (id, factory2) in self.factories.iter() {
                    if !factory2.is_player() {
                        let mut id2 = factory2.id;
                        self.commands.push(format!("MOVE {} {} {}", id1, id2, 1));
                    }
                }
            }
        }
    }

    fn compute_bomb(&mut self) {
        print_err!("BOMB COUNT {}", self.bomb_count);

        if self.bomb_count == 0 { return }

        // Get the target
        let mut aimed_factory: &Factory = &Factory{id: -99, owner: -99, cyborg_count: -99, production: -99, distances: Vec::new(), cyborg_count_combat: 0};
        for (id, factory) in self.factories.iter() {
            if factory.is_enemy() && factory.cyborg_count > aimed_factory.cyborg_count && factory.production > 2 && self.bomb_last != factory.id {
                aimed_factory = factory;
            }
        }

        if aimed_factory.id < 0 { return }

        // Get the source (the closest)
        for &(distance, id2) in aimed_factory.distances.iter() {
            let factory2 = self.factories.get(&id2).unwrap();
            if factory2.is_player() {
                self.bomb_count -= 1;
                self.bomb_last = aimed_factory.id;
                self.commands.push(format!("BOMB {} {}", id2, aimed_factory.id));
                return;
            }
        }

    }

    fn print_factories(&mut self) {
        for (id, factory) in self.factories.iter() {
            print_err!("{} {} {} {}", factory.id, factory.owner, factory.cyborg_count, factory.production);
        }
    }

    fn print_commands(&mut self) {
        if self.commands.len() > 0 {
            let mut final_command = "MSG El Psy Congroo".to_string();
            for command in self.commands.iter() {
                final_command.push_str(";");
                final_command.push_str(&command);
            }
            println!("{}", final_command);
        } else {
            println!("WAIT");
        }

        self.commands.clear();
    }

    fn evaluate(&mut self) -> i32 {
        let mut score: i32 = 0;

        // Cyborg in factories
        for (id, factory) in self.factories.iter() {
            if factory.is_player() {
                score += factory.cyborg_count;
                score += factory.production * 10;
            } else if factory.is_enemy() {
                score -= factory.cyborg_count;
                score -= factory.production * 10;
            }

        }

        // Cyborg in troops
        for troop in self.troops.iter() {
            if troop.is_player() {
                score += troop.cyborg_count;
            } else if troop.is_enemy() {
                score -= troop.cyborg_count;
            }

        }

        // Factories that will be catpured
        for (id, factory) in self.factories.iter() {
            let mut cyborg_count: i32 = factory.cyborg_count * factory.owner;
            for troop in self.troops.iter() {
                if factory.id == troop.factory_end {
                    cyborg_count += troop.cyborg_count * troop.owner;
                }

            }

            if cyborg_count > 0 {
                score += factory.production * 10;
            } else if cyborg_count < 0 {
                score -= factory.production * 10;
            }

        }

        //print_err!("Score : {}", score);
        return score;
    }

    fn test(&mut self) {
        let mut fac = self.clone();

        fac.commands = Vec::new();

    }

    fn sim_next_turn(&mut self) {
        // Troops Moving
        for troop in self.troops.iter_mut() {
            troop.turn_remaining -= 1;
            if troop.turn_remaining == 0 {
                let mut factory = self.factories.get_mut(&troop.factory_end).unwrap();

                if factory.owner == troop.owner {
                    factory.cyborg_count += troop.cyborg_count
                } else {
                    factory.cyborg_count_combat += troop.cyborg_count * troop.owner;
                }

            }
        }
        self.troops = self.troops.iter()
            .filter(|troop| troop.turn_remaining > 0)
            .map(|troop| troop.clone())
            .collect();

        // Orders Execution
        for troop in self.troop_commands.iter_mut() {
            let mut factory = self.factories.get_mut(&troop.factory_start).unwrap();
            factory.cyborg_count -= troop.cyborg_count;
        }
        self.troops.append(&mut self.troop_commands);
        self.troop_commands.clear();


        // Production & Combat
        for (id, factory) in self.factories.iter_mut() {
            // Production
            if !factory.is_neutral() {
                factory.cyborg_count += factory.production;
            }

            // Combat
            if factory.cyborg_count_combat != 0 {
                if factory.is_neutral() {
                    if factory.cyborg_count_combat < 0 {
                        factory.cyborg_count += factory.cyborg_count_combat;
                        if factory.cyborg_count < 0 {
                            factory.owner = -1;
                            factory.cyborg_count *= -1;
                        }
                    } else if factory.cyborg_count_combat > 0 {
                        factory.cyborg_count -= factory.cyborg_count_combat;
                        if factory.cyborg_count < 0 {
                            factory.owner = 1;
                            factory.cyborg_count *= -1;
                        }
                    }
                } else {
                    factory.cyborg_count = factory.cyborg_count * factory.owner + factory.cyborg_count_combat;
                    if factory.cyborg_count < 0 {
                        factory.cyborg_count *= -1;
                        factory.owner = -1;
                    } else if factory.cyborg_count > 0 {
                        factory.owner = 1;
                    }
                }

                factory.cyborg_count_combat = 0;
            }
        }

        // Bombs



    }

}


fn main() {
    let mut game_state: GameState = GameState::new();

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

        game_state.factory_distance.insert((factory_1, factory_2), distance);
    }
    game_state.init_factories_distance(factory_count);


    // game loop
    loop {
        let start = Instant::now();

        game_state.init_entities();

        game_state.max_strategy();
        game_state.compute_bomb();
       // game_state.print_factories();

        game_state.test();

        game_state.evaluate();

        game_state.print_commands();

        let mut game_cloned = game_state.clone();
        for i in 0..1000 {
            game_cloned.sim_next_turn();
            game_cloned.evaluate();
            game_cloned.max_strategy();
        }


        let elapsed = start.elapsed();
        print_err!("Elapsed: {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);
    }
}
