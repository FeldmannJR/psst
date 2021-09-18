use druid::{
    widget::{Label}, Widget,
};

use crate::{
    data::{AppState}
};



pub fn home_widget() -> impl Widget<AppState> {
    Label::new("Nothing to see here!")
}

