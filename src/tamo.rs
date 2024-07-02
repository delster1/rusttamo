use std::fs::{File, OpenOptions};
use std::io::{self, BufRead, BufReader, Write};
use std::path::Path;
use std::fmt;
#[derive(Clone, Debug)]
pub struct Tamo {
    name: String,
    age: f32,
    hunger: f32,
    thirst: f32,
    happiness: f32,
    energy: f32,
    room: TamoRoom,
}


#[derive(Clone, Debug)]
struct TamoRoom {
    food: u32,
    water : u32,

}
impl Tamo {
    pub fn build(name: String) -> Tamo {
        Tamo {
            name,
            age: 0.0,
            hunger: 10.0, // out of 100
            thirst: 10.0, // out of 100
            happiness: 100.0, // out of 100
            energy: 100.0, // out of 100
            room : TamoRoom {food: 100, water: 100}
        }
    }

    pub fn save_tamo(&self) -> io::Result<()> {
        let tamo_string = format!(
            "{},{},{},{},{},{}, {}, {}\n",
            self.name,
            self.age,
            self.hunger,
            self.thirst,
            self.happiness,
            self.energy,
            self.room.food,
            self.room.water
        );

        let mut file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open("tamo.txt")?;
        file.write_all(tamo_string.as_bytes())?;
        Ok(())
    }

    pub fn feed(&mut self ) {
        if self.hunger < 2.0 {
            println!("Can't feed tamo, they're already full...");
            self.room.food = self.room.food + 10;
            return
        }
        self.hunger -= 1.0;
        println!("Fed tamo!");
    }

    pub fn quench(&mut self) {
        if self.thirst < 2.0 {
            println!("Can't quench tamo, they're already full...");
            self.room.water = self.room.water + 10;
            return
        }
        self.thirst -= 1.0;
        self.energy += 5.0;
        println!("gave tamo something to drink!")
    }

    pub fn play(&mut self) {
        self.happiness += 10.0;
        self.energy -= 20.0;
        println!("played!");
    }

    pub fn eat(&mut self){
        if self.room.food >= 5 && self.hunger > 30.0 && self.energy > 1.0{
            self.room.food -= 5;
            self.hunger -= 30.0;
            self.energy -= 10.0;
            self.happiness += 5.0;
            println!("ate!");
            return
        }

    }

    pub fn drink(&mut self){
        if self.room.water >= 5 && self.thirst > 30.0 && self.energy > 1.0 {
            self.room.water -= 5;
            self.thirst -= 30.0;
            self.energy -= 10.0;
            self.happiness += 5.0;
            println!("drink!");
            return
        }
    }

    pub fn rest(&mut self){
        if self.energy < 80.0 {
            self.energy += 20.0 + (20.0 * (self.happiness / 100.0)) + (20.0 * (100.0 - self.thirst) / 100.0) + (20.0 * (100.0 - self.hunger) );
            self.happiness += 10.0;
            println!("rested!");
        }
        
    }

    pub fn test_dead(&mut self) -> bool {
        if self.energy < 1.0 {
            println!("You killed your tamogachi!");
            return true;
        };
        return false;
    }

    pub fn kill(&mut self) {
        self.age = f32::MAX;
    }
    pub fn time_pass(&mut self) { // maybe to happen every minute
        self.age += 0.01;
        self.happiness -= 0.05;
        self.thirst += 0.02;
        self.hunger += 0.01;
        self.energy -= 0.05;
        if self.energy < 1.0 {
            self.kill();
            return
        }
        if self.hunger > 100.0 || self.thirst > 100.0 {
            self.energy -= 1.0;
        }
    }
}

impl fmt::Display for Tamo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Name: {}, Age: {}, Hunger: {}, Thirst: {}, Happiness: {}, Energy: {}, Room: {}",
            self.name,
            self.age,
            self.hunger,
            self.thirst,
            self.happiness,
            self.energy,
            self.room
        )
    }
}
impl fmt::Display for TamoRoom {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "food: {}, water: {}",
            self.food,
            self.water,
            
        )
    }
}
pub fn load_tamo(path: &str) -> io::Result<Tamo> {
    if !Path::new(path).exists() {
        return Err(io::Error::new(io::ErrorKind::NotFound, "Config file not found"));
    }

    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut line = String::new();

    reader.read_line(&mut line)?;
    let parts: Vec<&str> = line.trim().split(',').collect();

    if parts.len() != 8 {
        println!("{:?}", parts);
        return Err(io::Error::new(io::ErrorKind::InvalidData, "Invalid config format"));
    }

    let name = parts[0].to_string();
    let age = parts[1].parse().unwrap_or(0.0);
    let hunger = parts[2].parse().unwrap_or(0.0);
    let thirst = parts[3].parse().unwrap_or(0.0);
    let happiness = parts[4].parse().unwrap_or(0.0);

    let energy= parts[5].parse().unwrap_or(0.0);
    let food: u32 = parts[6].parse().unwrap_or(0);
    let water : u32 = parts[7].parse().unwrap_or(0);
    let room = TamoRoom {food: food, water: water};
    Ok(Tamo {
        name,
        age,
        hunger,
        thirst,
        happiness,
        energy,
        room,
    })
}
