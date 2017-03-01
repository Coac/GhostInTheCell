use std::io;
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fmt;
use std::time::Instant;
extern crate rand;
use rand::Rng;

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
    cyborg_count_combat: i32,
    cyborg_remaining: i32 // For random strategy
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
    start: Instant
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
            troop_commands: LinkedList::new(),
            start: Instant::now()
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

            self.factories.insert(i, Factory{id: i, owner: -99, cyborg_count: -99, production: -99, distances: distances, cyborg_count_combat: 0, cyborg_remaining: 0});
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
                factory.cyborg_remaining = factory.cyborg_count;
            } else if entity_type == "TROOP" {
                self.troops.push_back(Troop{id: entity_id, owner: arg_1, factory_start: arg_2, factory_end: arg_3, cyborg_count: arg_4, turn_remaining: arg_5});
            }

        }

    }

    fn random_strategy(&mut self) {
        let factory_count: i32 = self.factories.len() as i32;
        for (id, factory) in  &mut self.factories {
            if !factory.is_player() { continue }

            let cyborg_count = rand::thread_rng().gen_range(0, factory.cyborg_remaining + 1);

            if cyborg_count == 0 { continue }

            factory.cyborg_remaining -= cyborg_count;

            let mut target = rnd_range(self.start, factory_count);

            while target == factory.id {
                target = rnd_range(self.start, factory_count);
            }

            self.troop_commands.push_back(Troop{id: 999, owner: 1, factory_start: factory.id, factory_end: target, cyborg_count: cyborg_count, turn_remaining: 10});
        }

    }

    fn max_strategy(&mut self) {
        let max_factory_option = self.factories.iter()
            .filter(|&(ind, fac)| fac.is_player())
            .max_by_key(|&(ind, fac)| fac.cyborg_remaining);

        if !max_factory_option.is_some() { return }

        let mut max_factory = max_factory_option.unwrap().1;

        //print_err!("Ordering {}  {}", max_factory.id, max_factory.cyborg_count);

        // Closest factory
        for &(distance, id2) in max_factory.distances.iter() {
            let factory2 = self.factories.get(&id2).unwrap();
            if !factory2.is_player() && factory2.production > 0 {
                //self.commands.push(format!("MOVE {} {} {}", max_factory.id, id2, max_factory.cyborg_count));
                self.troop_commands.push_back(Troop{id: 999, owner: 1, factory_start: max_factory.id, factory_end: id2, cyborg_count: max_factory.cyborg_remaining, turn_remaining: distance});
                break;
            }
        }

    }

    fn neutral_first_strategy(&mut self) {
        let factories_immu = self.factories.clone();
        for (id, factory) in self.factories.iter_mut() {

            if factory.is_player() {
                for &(distance, id2) in factory.distances.iter() {
                    let fac_target = factories_immu.get(&id2).unwrap();
                    if fac_target.production > 0 {

                        if fac_target.is_neutral() && fac_target.cyborg_count < factory.cyborg_remaining {
                            let mut is_enemy_closest = false;
                            // Check if fac_target is the closest
                            for &(distance, id2) in fac_target.distances.iter() {

                                if id2 == factory.id {
                                    is_enemy_closest = false;
                                    break;
                                }

                                let fac = factories_immu.get(&id2).unwrap();
                                if fac.is_enemy() {
                                    is_enemy_closest = true;
                                    break;
                                }

                            }
                            if !is_enemy_closest {
                                self.troop_commands.push_back(Troop{id: 999, owner: 1, factory_start: factory.id, factory_end: fac_target.id, cyborg_count: fac_target.cyborg_count +1, turn_remaining: distance});
                                factory.cyborg_remaining -= fac_target.cyborg_count +1;
                            }
                        }
                    }
                }
            }

        }

        if self.troop_commands.len() == 0 {
            self.max_strategy();
        }

    }

    fn defend_strategy(&mut self) {

        for (id, factory) in self.factories.iter_mut() {
            if !factory.is_player() { continue }

            let mut enemy_count = 0;
            for troop in self.troops.iter() {
                if troop.factory_end == factory.id {
                    if troop.is_enemy() {
                        enemy_count += troop.cyborg_count;
                    }
                }
            }
            if enemy_count >= factory.cyborg_count {
                print_err!("Need defense id:{}", factory.id);
                factory.cyborg_remaining = 0;
            } else {
                factory.cyborg_remaining -= enemy_count;
                if factory.cyborg_remaining > 15 {
                    factory.cyborg_remaining -= 10;
                    self.commands.push(format!("INC {}", id));
                }
            }
        }

        for (id, factory) in self.factories.iter() {
            if !factory.is_player() { continue }
            if factory.production == 0 { continue }

            let mut turn = -1;
            let mut state = self.clone();
            while !state.factories.get(&id).unwrap().is_enemy() && turn < 20 {
                //print_err!("turn : {} id{} owner{}", turn, state.factories.get(&id).unwrap().id, state.factories.get(&id).unwrap().owner);
                state.sim_next_turn();
                turn += 1;
            }
            if turn < 20 {
                let captured_fac = state.factories.get(&id).unwrap();
                let mut need_cyborg = captured_fac.cyborg_count - turn * captured_fac.production;
                print_err!("factory {} will captured in {} turns. Defend {}", id, turn, need_cyborg);

                if need_cyborg < 0 { need_cyborg = 2 }

                for &(distance, id2) in factory.distances.iter() {
                    if distance > turn { break }

                    let mut factory_renfort = self.factories.get(&id2).unwrap();
                    if !factory_renfort.is_player() { continue }
                    if factory_renfort.cyborg_remaining < need_cyborg { continue }

                    //factory_renfort.cyborg_remaining -= 10;
                    self.troop_commands.push_back(Troop{id: 999, owner: 1, factory_start: factory_renfort.id, factory_end: factory.id, cyborg_count: need_cyborg, turn_remaining: distance});
                }

            }

        }

        if self.troop_commands.len() == 0 {
            self.neutral_first_strategy();
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
        if self.bomb_count == 0 { return }

        // Get the target
        let mut aimed_factory: &Factory = &Factory{id: -99, owner: -99, cyborg_count: -99, production: -99, distances: Vec::new(), cyborg_count_combat: 0, cyborg_remaining: 0};
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

        let mut final_command = "MSG El Psy Congroo".to_string();
        for command in self.commands.iter() {
            final_command.push_str(";");
            final_command.push_str(&command);
        }

        for troop in self.troop_commands.iter() {
            final_command.push_str(";");
            final_command.push_str(&format!("MOVE {} {} {}", troop.factory_start, troop.factory_end, troop.cyborg_count));
        }

        println!("{}", final_command);

        self.troop_commands.clear();
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

        /*
        for troop in self.troop_commands.iter_mut() {
            self.troops.push_back(troop.clone());
        }
        */
        //self.troops.extend(self.troop_commands.iter());
        self.troops.append(&mut self.troop_commands);

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

                    //print_err!("Captured id:{} owner:{} count:{}", factory.id, factory.owner, factory.cyborg_count);

                }

                factory.cyborg_count_combat = 0;
            }
        }

        // Bombs



    }

}


fn rnd_range(start: Instant, max: i32) -> i32 {
    return (start.elapsed().subsec_nanos() % max as u32) as i32;
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

        print_err!("Id1:{} Id2:{} Distance:{}", factory_1, factory_2, distance);
        game_state.factory_distance.insert((factory_1, factory_2), distance);
    }
    game_state.init_factories_distance(factory_count);


    // game loop
    loop {
        let start = Instant::now();

        game_state.init_entities();

        /*
        let mut max_game = game_state.clone();
        let mut max_score = game_state.evaluate();
        for i in 0..1000 {
            let mut game_cloned = game_state.clone();
            game_cloned.random_strategy();
            game_cloned.random_strategy();

            let mut game_cloned_cloned = game_cloned.clone();
            game_cloned_cloned.sim_next_turn();
            let mut score = game_cloned_cloned.evaluate();
            if score > max_score {
                max_score = score;
                max_game = game_cloned;
            }

            let elapsed = start.elapsed();
            if elapsed.subsec_nanos() / 1_000_000 > 49 { break }
        }


        max_game.compute_bomb();
        max_game.print_commands();

        game_state = max_game;

        */
        //game_state.neutral_first_strategy();
        game_state.defend_strategy();
        game_state.compute_bomb();
        game_state.print_commands();


        let elapsed = start.elapsed();
        print_err!("Elapsed: {} ms",
             (elapsed.as_secs() * 1_000) + (elapsed.subsec_nanos() / 1_000_000) as u64);

    }
}
