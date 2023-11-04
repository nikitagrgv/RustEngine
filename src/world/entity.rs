#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Entity(pub(in crate::world) usize);

impl Entity {
    pub(in crate::world) fn from_num(num: usize) -> Self {
        Self(num)
    }
    pub fn to_num(&self) -> usize {
        self.0
    }
}
