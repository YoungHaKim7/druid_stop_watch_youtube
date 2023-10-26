use druid::widget::{Controller, ControllerHost, CrossAxisAlignment, Flex, Label};
use druid::{
    AppLauncher, BoxConstraints, Data, Env, Event, EventCtx, LayoutCtx, Lens, LifeCycle,
    LifeCycleCtx, PaintCtx, UpdateCtx, Widget, WidgetExt, WindowDesc,
};
use std::time::Duration;

// impl<W: Widget<StopwatchState>> Controller<StopwatchState, W> for StopwatchWidget {
//     fn event(
//         &mut self,
//         child: &mut W,
//         ctx: &mut EventCtx,
//         event: &Event,
//         data: &mut StopwatchState,
//         env: &Env,
//     ) {
//         match event {
//             Event::WindowConnected => {
//                 let timer_id = ctx.request_timer(Duration::from_secs(1));
//                 ctx.set_handled();
//             }
//             Event::Timer(timer_id) => {
//                 data.time = data.get_time();
//                 ctx.request_paint();
//                 let timer_id = ctx.request_timer(Duration::from_secs(1));
//                 ctx.set_handled();
//             }
//             Event::Command(cmd) => match cmd {
//                 "start" => {
//                     data.is_running = true;
//                     ctx.set_handled();
//                 }
//                 "stop" => {
//                     data.is_running = false;
//                     ctx.set_handled();
//                 }
//                 _ => {}
//             },
//             _ => {}
//         }
//         child.event(ctx, event, data, env);
//         ctx.window().set_always_on_top(true);
//         ctx.window().show_titlebar(false);
//         // ctx.window().handle_titlebar(true);
//     }
// }

impl StopwatchState {
    fn get_elapsed_time(&mut self) -> String {
        let total_seconds = self.elapsed_time;
        let mut seconds_remaining = total_seconds;

        let print_min_remaing = seconds_remaining / 60;
        let print_seconds_remaing = seconds_remaining % 60;
        seconds_remaining -= 1;
        return format!(
            "\r\t\t방송 시작:   {}  분(min) {} 초(sec)   전 ",
            print_min_remaing, print_seconds_remaing
        );
    }
}

// impl Widget<StopwatchState> for StopwatchWidget {
//     fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut StopwatchState, env: &Env) {
//         match event {
//             Event::Timer(timer_id) => {
//                 if data.is_running {
//                     data.elapsed_time += &1;
//                     ctx.request_paint();
//                 }
//                 ctx.request_timer(Duration::from_secs(1));
//                 ctx.set_handled();
//             }
//             _ => {}
//         }

//         // Wrap the elapsed_time field in a RefCell so that we can safely borrow it.
//         let elapsed_time = &mut data.elapsed_time;

//         // Layout and draw the elapsed_time field.
//         // ctx.layout_child(elapsed_time, env, ctx.size());
//         // ctx.draw(elapsed_time, env);
//     }
// }

#[derive(Clone, Data, Lens)]
pub struct StopwatchState {
    pub(crate) is_running: bool,
    pub(crate) elapsed_time: i32,
}

impl<W: Widget<StopwatchState>> Controller<StopwatchState, W> for StopwatchWidget {
    fn lifecycle(
        &mut self,
        _: &mut LifeCycleCtx<'_, '_>,
        _: &LifeCycle,
        _: &StopwatchState,
        _: &Env,
    ) {
        // TODO: Implement this method.
    }

    fn update(
        &mut self,
        _: &mut UpdateCtx<'_, '_>,
        _: &StopwatchState,
        _: &StopwatchState,
        _: &Env,
    ) {
        // TODO: Implement this method.
    }

    fn layout(
        &mut self,
        _: &mut LayoutCtx<'_, '_>,
        _: &BoxConstraints,
        _: &StopwatchState,
        _: &Env,
    ) -> druid::Size {
        // TODO: Implement this method.
        druid::Size::ZERO
    }

    fn paint(&mut self, _: &mut PaintCtx<'_, '_, '_>, _: &StopwatchState, _: &Env) {
        // TODO: Implement this method.
    }
}

struct StopwatchWidget(StopwatchState);

impl Widget<StopwatchState> for StopwatchWidget {
    fn event(&mut self, ctx: &mut EventCtx, event: &Event, data: &mut StopwatchState, env: &Env) {
        match event {
            Event::Timer(timer_id) => {
                if data.is_running {
                    data.get_elapsed_time();
                    ctx.request_paint();
                }
                ctx.request_timer(Duration::from_secs(1));
                ctx.set_handled();
            }
            Event::Command(cmd) => match cmd {
                "start" => {
                    data.is_running = true;
                    ctx.set_handled();
                }
                "stop" => {
                    data.is_running = false;
                    ctx.set_handled();
                }
                _ => {}
            },
            _ => {}
        }
    }
}

pub fn build_root_widget() -> impl Widget<StopwatchState> {
    let display = Label::new(|data: &i32, _env: &_| data.to_string())
        .with_text_size(16.0)
        .lens(StopwatchState::elapsed_time)
        .padding(5.0);

    let display = ControllerHost::new(display, StopwatchWidget);

    Flex::column()
        .with_flex_spacer(0.2)
        .with_child(display(StopwatchState::elaspsed_time))
        .cross_axis_alignment(CrossAxisAlignment::Start)
}

pub fn main() {
    // describe the main window
    let main_window = WindowDesc::new(build_root_widget())
        .title("NewYork_Time: EST!")
        .window_size((160.0, 90.0))
        .resizable(false);

    // create the initial app state
    let initial_state: StopwatchState = StopwatchState {
        is_running: true,
        elapsed_time: 0,
    };

    // start the application. Here we pass in the application state.
    AppLauncher::with_window(main_window)
        .log_to_console()
        .launch(initial_state)
        .expect("Failed to launch application");
}
