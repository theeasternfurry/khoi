use std::time::Duration;

use iced::widget::{column, progress_bar, text};
use iced::{Background, Border, Color, Element, Length, Subscription, Task, Theme};
use iced_exwlshell::to_layer_message;

use crate::model::Model;

#[to_layer_message]
#[derive(Debug, Clone)]
pub enum Message {
    Tick,
}

pub fn namespace() -> String {
    String::from("Khoi")
}

pub fn subscription(_model: &Model) -> Subscription<Message> {
    iced::time::every(Duration::from_secs(1)).map(|_| Message::Tick)
}

pub fn update(model: &mut Model, message: Message) -> Task<Message> {
    if let Message::Tick = message { model.refresh() }

    Task::none()
}

fn meter<'a>(title: &str, percent: f32, detail: String) -> Element<'a, Message> {
    let percent = percent.clamp(0.0, 100.0);

    column![
        text(format!("{title}  {detail}")).size(13),
        progress_bar(0.0..=100.0, percent)
            .length(Length::Fill)
            .girth(10)
            .style(|theme: &Theme| {
                let palette = theme.palette();

                progress_bar::Style {
                    background: Background::Color(Color { a: 0.15, ..palette.text }),
                    bar: Background::Color(palette.text),
                    border: Border {
                        radius: 3.0.into(),
                        ..Border::default()
                    },
                }
            }),
    ]
    .spacing(4)
    .into()
}

pub fn view(model: &Model) -> Element<'_, Message> {
    let cpu = model.cpu_percent();
    let memory = model.memory();
    let swap = model.swap();

    let disk: Element<'_, Message> = match model.disk() {
        Some(disk) => meter(
            "DISK /",
            disk.percent() as f32,
            format!("{:.1} / {:.1} GiB", disk.used_gib, disk.total_gib),
        ),
        None => text("Disk: unavailable").size(13).into(),
    };

    column![
        meter("CPU", cpu, format!("{cpu:.1}%")),
        meter(
            "RAM",
            memory.percent() as f32,
            format!("{:.1} / {:.1} GiB", memory.used_gib, memory.total_gib),
        ),
        meter(
            "SWAP",
            swap.percent() as f32,
            format!("{:.1} / {:.1} GiB", swap.used_gib, swap.total_gib),
        ),
        disk,
    ]
    .spacing(10)
    .padding(16)
    .width(Length::Fill)
    .into()
}

pub fn style(_model: &Model, theme: &Theme) -> iced::theme::Style {
    iced::theme::Style {
        background_color: Color::TRANSPARENT,
        text_color: theme.palette().text,
    }
}
