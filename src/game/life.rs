use std::cmp;

#[allow(dead_code)]
pub trait HasLife {
    fn is_alive(&self) -> bool;
    fn get_life_current(&self) -> u32;
    fn get_life_max(&self) -> u32;
    fn set_life(&mut self, delta: u32);

    fn heal(&mut self, delta: u32) {
        let new_life = cmp::min(self.get_life_max(), self.get_life_current() + delta);
        self.set_life(new_life);
    }

    fn damage(&mut self, delta: u32) {
        self.set_life(self.get_life_current() - delta);
    }
}
