struct SeaCreature {
  pub name: String,
  noise: String,
}

impl NoiseMaker for SeaCreature {
  fn make_noise(&self) {
      println!("{}", &self.get_sound());
  }
}



trait LoudNoiseMaker: NoiseMaker {
  fn make_alot_of_noise(&self) {
      self.make_noise();
      self.make_noise();
      self.make_noise();
  }

  fn static_make_noise(creature: &SeaCreature) {
    // we know the real type
    creature.make_noise();
  }
  
  fn dynamic_make_noise(noise_maker: &dyn NoiseMaker) {
    // we don't know the real type
    noise_maker.make_noise();
  }
  
  fn generic_make_noise<T>(creature: &T)
  where
    T: NoiseMaker,
  {
    // we know the real type at compile-time
    creature.make_noise();
  }

  fn generic_make_noise(creature: &impl NoiseMaker)
  {
    // we know the real type at compile-time
  creature.make_noise();
  }
}

impl LoudNoiseMaker for SeaCreature {
  pub fn get_sound(&self) -> &str {
    &self.noise
  }
}
trait NoiseMaker {
  fn make_noise(&self);
  
  fn make_alot_of_noise(&self){
      self.make_noise();
      self.make_noise();
      self.make_noise();
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

  println!("{}", creature.get_sound());
  creature.make_noise();
  creature.make_alot_of_noise();
  static_make_noise(creature);
  dynamic_make_noise(creature);
  generic_make_noise(creature);

  for a in ocean.animals.iter() {
    a.make_noise();

  };
}