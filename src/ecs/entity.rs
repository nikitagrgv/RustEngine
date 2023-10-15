#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Entity(pub(super) usize);

impl Entity {
    pub(super) fn from_num(num: usize) -> Self {
        Self(num)
    }
    pub(super) fn to_num(&self) -> usize {
        self.0
    }
}
