//All state is located inside this
pub struct App {
    mode: Mode,
}

pub enum Mode {
    GuildSelect,
    // ChannelSelect,
    // ChannelInteract,
}

impl App {
    pub fn new() -> App {
        App {
            mode: Mode::GuildSelect,
        }
    }
}
