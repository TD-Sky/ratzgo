pub trait Activable {
    fn active_mut(&mut self) -> &mut bool;

    fn active(mut self, set: bool) -> Self
    where
        Self: Sized,
    {
        *self.active_mut() = set;
        self
    }
}
