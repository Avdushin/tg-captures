#[derive(Clone, Copy)]
pub enum Screen {
    Start,
    Help,
    Menu,
}

#[derive(Debug, Clone, Copy)]
pub enum Callbacks {
    Start,
    Help,
    Menu,
}

impl std::fmt::Display for Callbacks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", match self {
            Callbacks::Start => "start",
            Callbacks::Help => "help",
            Callbacks::Menu => "menu",
        })
    }
}