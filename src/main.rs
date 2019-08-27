/*

You control a queen ant. You must make choices in order to keep your kingdom alive.
Watch out for other players and make sure they don't steal your food!
Protect your disciples and safeguard your babies!

*/

// use ws;
// use ws::{connect, Handler, Sender, Handshake, Result, Message, CloseCode};
// use std::Cell::RefCell;
// use std::rc::Rc;

fn main() {
	
}

struct Client {
	out: Sender,
	
}

// Kinda like cheats, but some, like mama, are normal for gameplay

struct Base {
	food: f64, // The amount of food in store
	workers: Vec<Worker>,
	soldiers: Vec<Soldier>,
	babies: Vec<Baby>,
}

enum Quality {
	Armor, // Increases defense by a factor of 2
	Builder, // Can build bases
	Decrepit, // Has no attack and consumes no food, but doesn't do much of anything else
	Giver, // Produces abnormally large amounts of food
	Invincible, // Cannot die
	Mama, // Can pick up babies
	Nacho, // Has a strong defense
	Pincerro, // Has a strong attack
}

struct Worker {
	name: String,
	cost: f64, // food per second
	production: f64, // food per second
	attack: f64, // dmg per second
	defense: f64, // dmg divider
	health: f64, // health
	qualities: Vec<Quality>,
}

struct Soldier {
	name: String,
	cost: f64,
	production: f64,
	attack: f64,
	defense: f64,
	health: f64,
	qualities: Vec<Quality>,
}

struct Baby {
	name: String,
	ant_type: AntType,
	cost: f64,
	production: f64,
	attack: f64,
	defense: f64
	health: f64,
	qualities: Vec<Quality>,
}