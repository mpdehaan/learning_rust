struct Monster {
    health: int,
    attack: int
}

fn main() {

    let m = Monster { health: 10, attack: 20 };

    println!("{}", m.health.to_string());
    println!("{}", m.attack.to_string());

}


