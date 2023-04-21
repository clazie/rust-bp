fn print_number(n: i64) {
  println!(" A number: {}", n);
}
// fn print_number(n: i32) {
//    println!(" A number: {}", n);
// }

fn square(n: i32) -> i32 {
  n * n
}

fn t(n: i32) -> i32 {
  let mut number = n;
  while number != 1 {
    if number % 2 == 0 {
      number = number / 2;
    } else {
      number = number * 3 + 1;
    }
  }
  number
}

fn main() {
  let a: i32 = 3;
  let arr = [3, 1, 4];
  
  println!("-----");
    print_number(a.into());
  println!("square: {}", square(4));
  println!("Hello, world!");
  println!("{}",a);
  println!("arr ist {:#?}", arr);
  
  println!("-----");
  let x = 5;
  let b = if x >= 5 { 100 } else { 0 };
  println!("{}", b);
  
  println!("-----");
  let mut m : i32 = 4;
  println!("{}", m);
  
  println!("-----");
  m = t(m);
  println!("{}", m);

  println!("-----");
  for i in 1..10 {
    println!("{}", i);
  }

  println!("-----");
  let arr = [3, 27, 42];
  for elem in &arr {
    println!("{}", elem);
  }

  let s : String = "hi".to_string();
  println!("s: {}", s);

  let b = s.clone(); // Let b = s würde "hi" von s zu b bewegen. s wäre dann nicht mehr benutzbar.
  println!("b: {}", b);
  println!("s: {}", s);

  fn greet(name: &mut String) {
    *name = "Tobilein".to_string();
    println!("Hello {}", name);
  }
  fn say_goodbye(name: &String) {
    println!("Goodbye {}", name);
  }

  let mut peter = "peter".to_string();
  greet(&mut peter);
  say_goodbye(&peter);

struct Monster {
  health: u8,
  strength: u8
}

impl Monster {
  /// Returns a monster with the specified
  /// strength

  fn with_strength(strength: u8) -> Self {
    Monster {
      health: 100,
      strength: strength,
    }
  }

  /// Returns a Monster with strength 10
  pub fn weak() -> Self {
      Self::with_strength(10)
  }

  /// Returns if the monster is alive
  pub fn is_alive(&self) -> bool{
    self.health > 0
  }

  /// Returns the monster's current attack
  /// strength. If the monster has less than 20 health points, its attack is only
  /// half as strong.
  fn attack_strength(&self) -> u8 {
    if self.health < 20 {
      self.strength / 2
    } else {
      self.strength
    }
  }

  /// Reduces the mMonster's health points
  /// according to the incomming attack's strength
  fn endure_attack(&mut self, strength: u8) {
    self.health = self.health.saturating_sub(strength);
  }


}

let mut wolfgang:Monster = Monster::weak();
let mut sabine:Monster = Monster::with_strength(13);

while wolfgang.is_alive() && sabine.is_alive() {
  wolfgang.endure_attack(sabine.attack_strength());
  sabine.endure_attack(wolfgang.attack_strength());
  println!(
    "Wolfgang: {} HP, Sabine: {} HP",
    wolfgang.health,
    sabine.health,
  )  
}

}