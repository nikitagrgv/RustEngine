#[derive(Eq, Hash, PartialEq, Copy, Clone, Debug)]
pub struct Entity(pub(in crate::ecs) usize);

impl Entity {
    pub(in crate::ecs) fn from_num(num: usize) -> Self {
        Self(num)
    }
    pub(in crate::ecs) fn to_num(&self) -> usize {
        self.0
    }
}
