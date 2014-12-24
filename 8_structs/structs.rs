struct Monster {
    health: int,
    attack: int
}

impl Monster {

    fn new(health: int, attack: int) -> Monster {
        Monster { health:health, attack:attack }
    }

    fn attack(&self) {
        println!("The monster attacks for {} damage.", self.attack);
    }

    fn foo() {
        println!("I'm a class method!");
    }

}

fn main() {

    let m = Monster::new(20,40);

    m.attack();
    Monster::foo();


}


