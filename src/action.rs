#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Quit,
    BindsSelection,
    IncreaseTarget,
    DecreaseTarget,
    StartBenchmark,
    RegisterClick(char),
    Reset,
    Tick,
    None,
}
