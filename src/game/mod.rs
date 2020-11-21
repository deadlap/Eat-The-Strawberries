pub mod world;
pub mod states;

pub enum StateSwitch {
    Menu,
    // Editor(Option<Level>),
    Play,//(Level),
    // PlayWith{
    //     lvl: Box<Level>,
    //     health: Health,
    // },
}
