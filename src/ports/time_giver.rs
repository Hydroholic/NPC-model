#[derive(Debug, PartialEq)]
pub enum TimeState {
    Morning, Night
}


pub trait TimeGiver {
    fn give_time(&self) -> &TimeState;
}
