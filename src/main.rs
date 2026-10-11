mod app;
mod model;

use iced_exwlshell::layershell::application;
use iced_exwlshell::reexport::{Anchor, KeyboardInteractivity, Layer, LayerSize};
use iced_exwlshell::settings::{LayerShellSettings, Settings, StartMode};

use crate::app::{namespace, style, subscription, update, view};
use crate::model::Model;

fn main() -> Result<(), iced_exwlshell::Error> {
    application(Model::new, namespace, update, view)
        .subscription(subscription)
        .style(style)
        .settings(Settings {
            layer_settings: LayerShellSettings {
                layer: Layer::Bottom,
                keyboard_interactivity: KeyboardInteractivity::None,
                exclusive_zone: 0,
                events_transparent: true,
                size: LayerSize::px(300, 200),
                anchor: Anchor::Top | Anchor::Right,
                start_mode: StartMode::Active,
                ..Default::default()
            },
            ..Default::default()
        })
        .run()
}
