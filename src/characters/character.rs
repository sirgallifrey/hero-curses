pub trait Character {
  fn get_hit_points(&self) -> u8;
  fn get_strenght(&self) -> u8;
  fn get_dexterety(&self) -> u8;
  fn get_constitution(&self) -> u8;
  fn get_inteligence(&self) -> u8;
  fn get_wisdom(&self) -> u8;
  fn get_charisma(&self) -> u8;
}

pub enum PlayerClass {
  Warrior,
  Mage
}

pub struct Player {
  pub level: u8,
  pub class: PlayerClass,

  pub strenght: u8,
  pub dexterety: u8,
  pub constitution: u8,
  pub inteligence: u8,
  pub wisdom: u8,
  pub charisma: u8,
}

impl Character for Player {
  fn get_charisma(&self) -> u8 {
    self.charisma
  }
  fn get_constitution(&self) -> u8 {
    self.constitution
  }
  fn get_dexterety(&self) -> u8 {
    self.dexterety
  }
  fn get_inteligence(&self) -> u8 {
    self.inteligence
  }
  fn get_strenght(&self) -> u8 {
    self.strenght
  }
  fn get_wisdom(&self) -> u8 {
    self.wisdom
  }
  fn get_hit_points(&self) -> u8 {
    match &self.class {
      PlayerClass::Warrior => self.level * 10 * (self.constitution % 10),
      PlayerClass::Mage => self.level * 7 * (self.constitution % 10)
    }
  }
}