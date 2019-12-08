extern crate rand;

use std::io;
//needed for user input
use std::io::Write;
//needed for io::stdout().flush()
use rand::prelude::*;
//for random

mod pokemon;



fn main(){
	//initialize variables:
	let mut round = 1;
	let mut game_end = 0;
	let mut choice;

	//creates pokemon
	let mut pokemon1 = pokemon::init();
	let mut pokemon2 = pokemon::init();

	print!("Enter name for pokemon 1: ");
	io::stdout().flush();
	let mut name1 = String::new();
	io::stdin().read_line(&mut name1).expect("Error");
	pokemon1.set_name(name1);

	print!("Enter name for pokemon 2: ");
	io::stdout().flush();
	let mut name2 = String::new();
	io::stdin().read_line(&mut name2).expect("Error");
	pokemon2.set_name(name2);

	//printing the stats of the pokemon
	pokemon::print_stats(&pokemon1, &pokemon2);

	//a loop is a single round
	loop{
		if random(){
			//the scenario where pokemon 1 starts

			if game_end == 1{
				break;
			}

			//pokemon 1's turn
			choice = pokemon::turn(&pokemon1, &round, &pokemon2);
			if choice == 1{
				//look for a random number from 5 to 10
				//60% chance to succeed
				let mut rng = thread_rng();
				//damage is from 5 to 10
				let damage:i32 = rng.gen_range(5, 11);
				//prob is from 0-1
				//so if prob < 0.4 it is a success
				let prob:f64 = rng.gen();
				if prob > 0.4 && pokemon::check_mana(&pokemon1){
					pokemon::attack_success(&mut pokemon1, &mut pokemon2, damage);
				}else{
					pokemon::attack_fail(&mut pokemon1);
				}
				pokemon::print_stats(&pokemon1, &pokemon2);
				game_end = pokemon::game_end(&pokemon1, &pokemon2);
			}else if choice == 2 {
				pokemon::buy_mp_potion(&mut pokemon1);
				pokemon::print_stats(&pokemon1, &pokemon2);
			}else if choice == 3{
				pokemon::buy_hp_potion(&mut pokemon1);
				pokemon::print_stats(&pokemon1, &pokemon2);
			}else if choice == 4{
				println!("The game has ended! YOU LOSE");
				break;
			}


			if game_end == 1{
				break;
			}

			//pokemon 2's turn
			choice = pokemon::turn(&pokemon2, &round, &pokemon1);
			if choice == 1{
				//look for a random number from 5 to 10
				//60% chance to succeed
				let mut rng = thread_rng();
				//damage is from 5 to 10
				let damage:i32 = rng.gen_range(5, 11);
				//prob is from 0-1
				//so if prob < 0.4 it is a success
				let prob:f64 = rng.gen();
				if prob > 0.4 && pokemon::check_mana(&pokemon2){
					pokemon::attack_success(&mut pokemon2, &mut pokemon1, damage);
				}else{
					pokemon::attack_fail(&mut pokemon2);
				}
				pokemon::print_stats(&pokemon1, &pokemon2);
				game_end = pokemon::game_end(&pokemon1, &pokemon2);
			}else if choice == 2 {
				pokemon::buy_mp_potion(&mut pokemon2);
				pokemon::print_stats(&pokemon1, &pokemon2);
			}else if choice == 3{
				pokemon::buy_hp_potion(&mut pokemon2);
				pokemon::print_stats(&pokemon1, &pokemon2);
			}else if choice == 4{
				println!("The game has ended! YOU LOSE");
				break;
			}

			if game_end == 1{
				break;
			}
		}else{
			// the scenario where pokemon 2 starts!

			if game_end == 1{
				break;
			}

			//pokemon 2's turn
			choice = pokemon::turn(&pokemon2, &round, &pokemon1);
			if choice == 1 {
				//look for a random number from 5 to 10
				//60% chance to succeed
				let mut rng = thread_rng();
				//damage is from 5 to 10
				let damage:i32 = rng.gen_range(5, 11);
				//prob is from 0-1
				//so if prob < 0.4 it is a success
				let prob:f64 = rng.gen();
				if prob > 0.4 && pokemon::check_mana(&pokemon2){
					pokemon::attack_success(&mut pokemon2, &mut pokemon1, damage);
				}else{
					pokemon::attack_fail(&mut pokemon2);
				}
				pokemon::print_stats(&pokemon1, &pokemon2);
				game_end = pokemon::game_end(&pokemon1, &pokemon2);
			}else if choice == 2 {
				pokemon::buy_mp_potion(&mut pokemon2);
				pokemon::print_stats(&pokemon1, &pokemon2);
			}else if choice == 3{
				pokemon::buy_hp_potion(&mut pokemon2);
				pokemon::print_stats(&pokemon1, &pokemon2);
			}else if choice == 4{
				println!("The game has ended! YOU LOSE");
				break;
			}

			if game_end == 1{
				break;
			}

			//pokemon 1's turn
			choice = pokemon::turn(&pokemon1, &round, &pokemon2);
			if choice == 1 {
				//look for a random number from 5 to 10
				//60% chance to succeed
				let mut rng = thread_rng();
				//damage is from 5 to 10
				let damage:i32 = rng.gen_range(5, 11);
				//prob is from 0-1
				//so if prob < 0.4 it is a success
				let prob:f64 = rng.gen();
				if prob > 0.4 && pokemon::check_mana(&pokemon1){
					pokemon::attack_success(&mut pokemon1, &mut pokemon2, damage);
				}else{
					pokemon::attack_fail(&mut pokemon1);
				}
				pokemon::print_stats(&pokemon1, &pokemon2);
				game_end = pokemon::game_end(&pokemon1, &pokemon2);
			}else if choice == 2 {
				pokemon::buy_mp_potion(&mut pokemon1);
				pokemon::print_stats(&pokemon1, &pokemon2);
			}else if choice == 3{
				pokemon::buy_hp_potion(&mut pokemon1);
				pokemon::print_stats(&pokemon1, &pokemon2);
			}else if choice == 4{
				println!("The game has ended! YOU LOSE");
				break;
			}

			if game_end == 1{
				break;
			}
		}
		round = round + 1;
	}
	
}