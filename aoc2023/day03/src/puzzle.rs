#[derive(Debug, Clone)]
pub struct Number{ pub value: u32, pub from: usize }
impl Number {
  pub fn to(&self) -> usize { self.value.to_string().len() }
}

#[derive(Debug, Clone)]
pub struct Gear{ pub value: String, pub from: usize }
impl Gear {
  pub fn to(&self) -> usize { self.value.len() }
}
