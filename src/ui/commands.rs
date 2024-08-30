pub enum UI {
    SendMessage(String),

    SetInterface(String),
    SetLanguage(String),
    SetUsername(String),
}

pub enum Net {
    UpdateUsername(String),
}
