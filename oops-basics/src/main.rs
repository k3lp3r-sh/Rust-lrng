struct Player {
    name: String,
    health: i32,
    damage: i32,
}

impl Player {
    pub fn new(name: String, health: i32, damage: i32) -> Player {
        Player {
            name: name,
            health: health,
            damage: damage,
        }
    }
}

impl Player {
    pub fn increment_health(&mut self) {
        self.health += 1;
    }
}
fn main() {
    let nitish = Player::new(String::from("nitish"), 100, 250);
    println!(
        "the health of character {} is {} with damage upto {}",
        nitish.name, nitish.health, nitish.damage
    )
}
