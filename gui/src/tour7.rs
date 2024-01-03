
impl SeaCreature {
  pub fn get_sound(&self) -> &str {
      &self.noise
  }
}

fn main() {
  let creature = SeaCreature {
      name: String::from("Ferris"),
      noise: String::from("blub"),
  };
  println!("{}", creature.get_sound());
}

struct SeaCreature {
  pub name: String,
  noise: String,
}

impl SeaCreature {
  pub fn get_sound(&self) -> &str {
      &self.noise
  }
}

trait NoiseMaker {
  fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
  fn make_noise(&self) {
      println!("{}", &self.get_sound());
  }
}

fn main() {
  let creature = SeaCreature {
      name: String::from("Ferris"),
      noise: String::from("blub"),
  };
  creature.make_noise();
}

trait NoiseMaker {
  fn make_noise(&self);
  
  fn make_alot_of_noise(&self){
      self.make_noise();
      self.make_noise();
      self.make_noise();
  }
}

impl NoiseMaker for SeaCreature {
  fn make_noise(&self) {
      println!("{}", &self.get_sound());
  }
}

fn main() {
  let creature = SeaCreature {
      name: String::from("Ferris"),
      noise: String::from("blub"),
  };
  creature.make_alot_of_noise();
}


impl SeaCreature {
  pub fn get_sound(&self) -> &str {
      &self.noise
  }
}

trait NoiseMaker {
  fn make_noise(&self);
}

trait LoudNoiseMaker: NoiseMaker {
  fn make_alot_of_noise(&self) {
      self.make_noise();
      self.make_noise();
      self.make_noise();
  }
}

impl NoiseMaker for SeaCreature {
  fn make_noise(&self) {
      println!("{}", &self.get_sound());
  }
}

impl LoudNoiseMaker for SeaCreature {}

fn main() {
  let creature = SeaCreature {
      name: String::from("Ferris"),
      noise: String::from("blub"),
  };
  creature.make_alot_of_noise();
}
trait NoiseMaker {
  fn make_noise(&self);
}


fn static_make_noise(creature: &SeaCreature) {
  // we know the real type
  creature.make_noise();
}

fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
  // we don't know the real type
  noise_maker.make_noise();
}

fn main() {
  let creature = SeaCreature {
      name: String::from("Ferris"),
      noise: String::from("blub"),
  };
  static_make_noise(&creature);
  dynamic_make_noise(&creature);
}

trait NoiseMaker {
  fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
  fn make_noise(&self) {
      println!("{}", &self.get_sound());
  }
}

fn generic_make_noise<T>(creature: &T)
where
  T: NoiseMaker,
{
  // we know the real type at compile-time
  creature.make_noise();
}

fn main() {
  let creature = SeaCreature {
      name: String::from("Ferris"),
      noise: String::from("blub"),
  };
  generic_make_noise(&creature);
}

impl SeaCreature {
  pub fn get_sound(&self) -> &str {
      &self.noise
  }
}

trait NoiseMaker {
  fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
  fn make_noise(&self) {
      println!("{}", &self.get_sound());
  }
}

fn generic_make_noise(creature: &impl NoiseMaker)
{
  // we know the real type at compile-time
  creature.make_noise();
}

struct SeaCreature {
  pub name: String,
  noise: String,
}

impl SeaCreature {
  pub fn get_sound(&self) -> &str {
      &self.noise
  }
}

trait NoiseMaker {
  fn make_noise(&self);
}

impl NoiseMaker for SeaCreature {
  fn make_noise(&self) {
      println!("{}", &self.get_sound());
  }
}

struct Ocean {
  animals: Vec<Box<dyn NoiseMaker>>,
}

fn main() {
  let ferris = SeaCreature {
      name: String::from("Ferris"),
      noise: String::from("blub"),
  };
  let sarah = SeaCreature {
      name: String::from("Sarah"),
      noise: String::from("swish"),
  };
  let ocean = Ocean {
      animals: vec![Box::new(ferris), Box::new(sarah)],
  };
  for a in ocean.animals.iter() {
      a.make_noise();
  }
}
