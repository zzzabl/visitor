fn main() {
    let mut animals: Vec<Box<dyn Animal>> =
        vec![Box::new(Sheep { wool: 2 }), Box::new(Camel { wool: 35 })];
    let mut barber = AnimalBarber { all_wool: 0 };
    barber.cut(&mut animals);
    println!("all_wool: {}", barber.all_wool);

    let mut hunter = StaticHunter { catched_animals: 0 };

    // Я же правмльно понимаю что в случае статической типизации я не смогу засунуть
    // сюда массив из разных зверей?

    hunter.hunt::<Sheep>(&mut Sheep { wool: 10 });
    hunter.hunt::<Camel>(&mut Camel { wool: 10 });
    hunter.hunt::<Crocodile>(&mut Crocodile {});
}

trait AnimalVisitor {
    fn visit_sheep(&mut self, sheep: &mut Sheep);
    fn visit_camel(&mut self, camel: &mut Camel);
    fn visit_crocodile(&mut self, crocodile: &mut Crocodile);
}

trait Animal {
    fn visit(&mut self, visitor: &mut dyn AnimalVisitor);
}
trait AnimalForZoo {
    fn visit<T: AnimalVisitor>(&mut self, visitor: &mut T);
}

struct Sheep {
    wool: i32,
}
impl Sheep {
    fn sing(&self) {
        println!("Baaaaaa...");
    }
}
impl Animal for Sheep {
    fn visit(&mut self, visitor: &mut dyn AnimalVisitor) {
        visitor.visit_sheep(self);
    }
}
impl AnimalForZoo for Sheep {
    fn visit<T: AnimalVisitor>(&mut self, visitor: &mut T) {
        visitor.visit_sheep(self)
    }
}

struct Camel {
    wool: i32,
}
impl Camel {
    fn spit(&self) {
        println!("Ugh!");
    }
}
impl Animal for Camel {
    fn visit(&mut self, visitor: &mut dyn AnimalVisitor) {
        visitor.visit_camel(self);
    }
}
impl AnimalForZoo for Camel {
    fn visit<T: AnimalVisitor>(&mut self, visitor: &mut T) {
        visitor.visit_camel(self)
    }
}

struct Crocodile {}
impl Animal for Crocodile {
    fn visit(&mut self, visitor: &mut dyn AnimalVisitor) {
        visitor.visit_crocodile(self)
    }
}
impl AnimalForZoo for Crocodile {
    fn visit<T: AnimalVisitor>(&mut self, visitor: &mut T) {
        visitor.visit_crocodile(self)
    }
}

struct AnimalBarber {
    all_wool: i32,
}
impl AnimalBarber {
    fn cut(&mut self, animals: &mut Vec<Box<dyn Animal>>) {
        for anymal in animals {
            anymal.visit(self);
        }
    }
}

impl AnimalVisitor for AnimalBarber {
    fn visit_sheep(&mut self, sheep: &mut Sheep) {
        self.all_wool += sheep.wool;
        sheep.wool = 0;
        sheep.sing();
    }

    fn visit_camel(&mut self, camel: &mut Camel) {
        self.all_wool += camel.wool;
        camel.wool = 0;
        camel.spit();
    }

    fn visit_crocodile(&mut self, _: &mut Crocodile) {
        println!("Aaaaa...")
    }
}

struct StaticHunter {
    catched_animals: i32,
}

impl StaticHunter {
    fn hunt<T: AnimalForZoo>(&mut self, animal: &mut T) {
        animal.visit(self);
    }
}

impl AnimalVisitor for StaticHunter {
    fn visit_sheep(&mut self, sheep: &mut Sheep) {
        sheep.sing();
        self.catched_animals += 1;
        println!("Catch wild sheep???");
    }

    fn visit_camel(&mut self, camel: &mut Camel) {
        camel.spit();
        self.catched_animals += 1;
        println!("Catch wild camel???");
    }

    fn visit_crocodile(&mut self, _crocodile: &mut Crocodile) {
        println!("Aaaaa...");
        self.catched_animals = 0;
    }
}
