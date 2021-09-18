use druid::{LensExt, Widget, WidgetExt, widget::{List, Scroll, Tabs}};

use crate::{
    data::{AppState, MadeForYou, MadeForYouKind, Personalized},
    ui::playlist::playlist_widget,
    webapi::WebApi,
    widget::Async,
};

use super::utils::{error_widget, spinner_widget};

pub fn made_for_you() -> impl Widget<AppState> {
    Async::new(spinner_widget, || create_tabs(), error_widget)
        .on_deferred(|_| WebApi::global().get_made_for_you())
        .lens(AppState::personalized.then(Personalized::made_for_you))
}

pub fn create_tabs() -> impl Widget<MadeForYou> {
    let mut tabs = Tabs::new();
    let kinds = vec![
        MadeForYouKind::DailyMix,
        MadeForYouKind::Discover,
        MadeForYouKind::GenreMix,
        MadeForYouKind::ArtistMix,
        MadeForYouKind::DecadeMix,
        MadeForYouKind::UniquelyYours,
    ];

    for kind in kinds {
        let list = List::new(playlist_widget).lens(kind.clone());
        tabs.add_tab(kind.display_name(), Scroll::new(list).vertical().boxed());
    }
    tabs
}
