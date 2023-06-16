use crate::model::channel::Channel;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Guild {
    pub id: String,
    pub name: String,
    pub channels: Vec<Channel>,
}
