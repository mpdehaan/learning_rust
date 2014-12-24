struct Monster {
    health: int,
    attack: int
}

impl Monster {
    fn attack(&self) {
        println!("The monster attacks for {} damage.", self.attack);
    }

    fn foo() {
        println!("I'm a class method!");
    }

}

fn main() {

    let m = Monster { health: 10, attack: 20 };

    m.attack();
    Monster::foo();


}


