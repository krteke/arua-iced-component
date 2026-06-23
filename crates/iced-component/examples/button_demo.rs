//! Visual Iced demo for `AnimatedButton`.

use std::time::Duration;

use iced::widget::{column, container, row, text};
use iced::{Element, Fill, Subscription, Task, Theme, application, time};
use iced_component::button::{AnimatedButton, AnimatedIconButton, ButtonEvent, IconSource};
use iced_component::component::ComponentContext;
use iced_component::motion::{MotionPreferences, MotionPreferencesController};
use iced_component::{MotionError, MotionRuntime};

const MOTION_ICON: &[u8] = br#"
<svg viewBox="0 0 16 16" xmlns="http://www.w3.org/2000/svg">
  <path d="M2 4h8v1.5H2zM12 3a2 2 0 1 1 0 4 2 2 0 0 1 0-4zM6 9h8v1.5H6zM4 8a2 2 0 1 0 0 4 2 2 0 0 0 0-4z"/>
</svg>
"#;

fn main() -> iced::Result {
    application(Demo::new, update, view)
        .title("aura-iced-component button demo")
        .subscription(subscription)
        .theme(theme)
        .window_size([420.0, 260.0])
        .run()
}

struct Demo {
    runtime: MotionRuntime,
    context: ComponentContext,
    reduce_motion: MotionPreferencesController,
    save_button: AnimatedButton,
    reset_button: AnimatedButton,
    motion_button: AnimatedIconButton,
    clicks: u32,
    motion_error: Option<String>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    Tick,
    SaveButton(ButtonEvent<SaveAction>),
    ResetButton(ButtonEvent<ResetAction>),
    MotionButton(ButtonEvent<MotionAction>),
}

#[derive(Debug, Clone, Copy)]
enum SaveAction {
    Save,
}

#[derive(Debug, Clone, Copy)]
enum ResetAction {
    Reset,
}

#[derive(Debug, Clone, Copy)]
enum MotionAction {
    Toggle,
}

impl Demo {
    fn new() -> Self {
        let (preferences, reduce_motion) = MotionPreferences::new(false);
        let mut runtime = MotionRuntime::new();
        let context = ComponentContext::current().with_motion_preferences(preferences);
        let mut save_button = AnimatedButton::suggested("Save");
        let mut reset_button = AnimatedButton::standard("Reset").flat();
        let mut motion_button = AnimatedIconButton::suggested(IconSource::svg_bytes(MOTION_ICON));

        save_button.register(&mut runtime, &context);
        reset_button.register(&mut runtime, &context);
        motion_button.register(&mut runtime, &context);

        Self {
            runtime,
            context,
            reduce_motion,
            save_button,
            reset_button,
            motion_button,
            clicks: 0,
            motion_error: None,
        }
    }
}

fn update(state: &mut Demo, message: Message) -> Task<Message> {
    match message {
        Message::Tick => {
            state
                .runtime
                .tick(iced_component::motion::Duration::from_millis(16.0));
        }
        Message::SaveButton(event) => {
            let result = state.save_button.update_event_with(
                event,
                &mut state.runtime,
                |SaveAction::Save| {
                    state.clicks += 1;
                },
            );
            record_motion_result(state, result);
        }
        Message::ResetButton(event) => {
            let result = state.reset_button.update_event_with(
                event,
                &mut state.runtime,
                |ResetAction::Reset| {
                    state.clicks = 0;
                },
            );
            record_motion_result(state, result);
        }
        Message::MotionButton(event) => {
            let result = state.motion_button.update_event_with(
                event,
                &mut state.runtime,
                |MotionAction::Toggle| {
                    let next = !state.reduce_motion.reduce_motion();
                    state.reduce_motion.set_reduce_motion(next);
                },
            );
            record_motion_result(state, result);
        }
    }

    Task::none()
}

fn record_motion_result(state: &mut Demo, result: Result<bool, MotionError>) {
    match result {
        Ok(_) => state.motion_error = None,
        Err(error) => state.motion_error = Some(error.to_string()),
    }
}

fn subscription(_state: &Demo) -> Subscription<Message> {
    time::every(Duration::from_millis(16)).map(|_| Message::Tick)
}

fn theme(_state: &Demo) -> Theme {
    Theme::Light
}

fn view(state: &Demo) -> Element<'_, Message> {
    let save = state
        .save_button
        .view(&state.runtime, &state.context)
        .on_press(SaveAction::Save)
        .map_event(Message::SaveButton);
    let reset = state
        .reset_button
        .view(&state.runtime, &state.context)
        .on_press(ResetAction::Reset)
        .map_event(Message::ResetButton);
    let motion = state
        .motion_button
        .view(&state.runtime, &state.context)
        .on_press(MotionAction::Toggle)
        .map_event(Message::MotionButton);

    let snapshot = state
        .save_button
        .snapshot(&state.runtime, &state.context)
        .expect("button motion handle belongs to the demo runtime");

    let reduce_label = if state.reduce_motion.reduce_motion() {
        "Reduce motion: on"
    } else {
        "Reduce motion: off"
    };

    let content = column![
        text("AnimatedButton").size(28),
        text("Hover, press, and toggle reduced motion to see the component runtime path."),
        row![save, reset, motion, text(reduce_label).size(14),].spacing(12),
        text(format!(
            "motion: scale {:.2}, shadow_y {:.2}, bg_alpha {:.2}",
            snapshot.motion.scale, snapshot.motion.shadow_y, snapshot.motion.bg_alpha
        ))
        .size(14),
        text(
            state
                .motion_error
                .as_deref()
                .unwrap_or("motion runtime: ok")
        )
        .size(14),
    ]
    .spacing(16);

    container(content)
        .padding(24)
        .center_x(Fill)
        .center_y(Fill)
        .into()
}
