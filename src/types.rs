#[derive(Default, Clone, Debug)]
pub struct Player {
    pub id: u32,
    pub name: String,
    pub life: u32,
    pub speed: u32,
    pub position: Position,
}

#[derive(Default, Clone, Debug)]
pub struct Position {
    x: u32,
    y: u32,
}

impl Player {
    pub fn new() -> Player {
        Player {
            id: 0u32,
            name: String::from(""),
            life: 100u32,
            speed: 0u32,
            position: Position::default(),
        }
    }
    // fn with_speed(self, s: u32) -> Self {
    //     self.speed = s;
    //     self
    // }
    pub fn eat(&mut self, v: Veggie) {
        println!("Player {} eats {:?}", self.id, v);
        self.life += 1;
    }
    pub fn jump(&mut self, dir: Direction) {
        match dir {
            Direction::Up => {
                println!("Player {} has jumped {:?}", self.id, dir);
                self.position.y += self.speed
            }
            Direction::Down => {
                println!("Player {} has jumped {:?}", self.id, dir);
                self.position.y -= self.speed
            }
            Direction::Left => {
                println!("Player {} has jumped {:?}", self.id, dir);
                self.position.x -= self.speed
            }
            Direction::Right => {
                println!("Player {} has jumped {:?}", self.id, dir);
                self.position.x += self.speed
            }
        }
    }
}

#[derive(Clone, Debug)]
pub enum Veggie {
    Potato(u16),
    Cucumber(u16),
    Carrot(u16),
}

impl Default for Veggie {
    fn default() -> Self {
        Veggie::Potato(1)
    }
}

impl Veggie {
    pub fn potato() -> Self {
        Veggie::Potato(1)
    }
    pub fn carrot() -> Self {
        Veggie::Carrot(10)
    }
    pub fn cucumber() -> Self {
        Veggie::Cucumber(5)
    }
}

#[derive(Clone, Debug)]
pub enum Action {
    Eat(Veggie),
    Jump(Direction),
}

#[derive(Clone, Debug)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}
