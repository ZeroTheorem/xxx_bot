#[derive(Clone, Default)]
pub enum State {
    #[default]
    Start,
    ReceiveMonth,
    ReceiveYear,
}
