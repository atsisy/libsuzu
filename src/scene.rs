pub mod end_scene;
pub mod save_scene;
pub mod scenario_scene;
pub mod shop_scene;
pub mod suzuna_scene;
pub mod title_scene;

use std::str::FromStr;

use ggez::input as ginput;
use crate::libsuzu::core::Clock;
use crate::libsuzu::device as tdev;
use crate::libsuzu::numeric;

use crate::core::SuzuContext;

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SceneTransition {
    Keep,
    Reset,
    SwapTransition,
    StackingTransition,
    PoppingTransition,
}

impl FromStr for SceneTransition {
    type Err = ();

    fn from_str(scene_str: &str) -> Result<Self, Self::Err> {
        match scene_str {
            "Keep" => Ok(Self::Keep),
            "Reset" => Ok(Self::Reset),
            "SwapTransition" => Ok(Self::SwapTransition),
            "StackingTransition" => Ok(Self::StackingTransition),
            "PoppingTransition" => Ok(Self::PoppingTransition),
            _ => panic!("Error: EventTrigger::from_str"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum SceneID {
    Null,
    MainDesk,
    Scenario,
    SuzunaShop,
    DayResult,
    Save,
    Copying,
    End,
    Title,
}

impl FromStr for SceneID {
    type Err = ();

    fn from_str(scene_str: &str) -> Result<Self, Self::Err> {
        match scene_str {
            "MainDesk" => Ok(Self::MainDesk),
            "Scenario" => Ok(Self::Scenario),
            "SuzunaShop" => Ok(Self::SuzunaShop),
            "WorkResult" => Ok(Self::DayResult),
            "Save" => Ok(Self::Save),
            "Title" => Ok(Self::Title),
            "End" => Ok(Self::End),
            _ => Err(()),
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum DrawRequest {
    InitDraw,
    Draw,
    Skip,
}

impl DrawRequest {
    pub fn is_not_skip(&self) -> bool {
        self != &DrawRequest::Skip
    }
}

impl std::ops::BitOr for DrawRequest {
    type Output = Self;

    fn bitor(self, rhs: Self) -> Self::Output {
        match self {
            Self::InitDraw => DrawRequest::InitDraw,
            Self::Draw => DrawRequest::InitDraw,
            Self::Skip => rhs,
        }
    }
}

impl std::ops::BitOrAssign for DrawRequest {
    fn bitor_assign(&mut self, rhs: Self) {
        *self = *self | rhs;
    }
}

pub trait SceneManager {
    fn key_down_event<'a>(&mut self, _: &mut SuzuContext<'a>, _vkey: tdev::VirtualKey) {}

    fn key_up_event<'a>(&mut self, _ctx: &mut SuzuContext<'a>, _vkey: tdev::VirtualKey) {}

    fn mouse_motion_event<'a>(
        &mut self,
        _ctx: &mut SuzuContext<'a>,
        _point: numeric::Point2f,
        _offset: numeric::Vector2f,
    ) {
    }

    fn mouse_button_down_event<'a>(
        &mut self,
        _ctx: &mut SuzuContext<'a>,
        _button: ginput::mouse::MouseButton,
        _point: numeric::Point2f,
    ) {
    }

    fn mouse_button_up_event<'a>(
        &mut self,
        _ctx: &mut SuzuContext<'a>,
        _button: ginput::mouse::MouseButton,
        _point: numeric::Point2f,
    ) {
    }

    fn mouse_wheel_event<'a>(
        &mut self,
        _ctx: &mut SuzuContext<'a>,
        _point: numeric::Point2f,
        _x: f32,
        _y: f32,
    ) {
    }

    fn scene_popping_return_handler<'a>(&mut self, _: &mut SuzuContext<'a>) {}

    fn pre_process<'a>(&mut self, _ctx: &mut SuzuContext<'a>);

    fn drawing_process(&mut self, ctx: &mut ggez::Context);
    fn post_process<'a>(&mut self, ctx: &mut SuzuContext<'a>) -> SceneTransition;
    fn transition(&self) -> SceneID;

    fn get_current_clock(&self) -> Clock;

    fn update_current_clock(&mut self);

    fn focus_event<'a>(&mut self, _ctx: &mut SuzuContext<'a>) {}
    fn unfocus_event<'a>(&mut self, _ctx: &mut SuzuContext<'a>) {}
}

pub struct NullScene {}

impl NullScene {
    pub fn new() -> Self {
        NullScene {}
    }
}

impl SceneManager for NullScene {
    fn pre_process<'a>(&mut self, _ctx: &mut SuzuContext<'a>) {}

    fn drawing_process(&mut self, _ctx: &mut ggez::Context) {}
    fn post_process<'a>(&mut self, _ctx: &mut SuzuContext<'a>) -> SceneTransition {
        SceneTransition::Keep
    }

    fn transition(&self) -> SceneID {
        SceneID::Null
    }

    fn get_current_clock(&self) -> Clock {
        0
    }

    fn update_current_clock(&mut self) {}
}

///
/// # ????????????????????????????????????????????????????????????
///
/// ## run_time
/// ?????????????????????
///
/// ## func
/// run_time???????????????????????????
///
pub struct DelayEvent<T> {
    pub run_time: Clock,
    pub func: Box<dyn FnOnce(&mut T, &mut SuzuContext, Clock) -> ()>,
}

impl<T> DelayEvent<T> {
    pub fn new(f: Box<dyn FnOnce(&mut T, &mut SuzuContext, Clock) -> ()>, t: Clock) -> Self {
        DelayEvent::<T> {
            run_time: t,
            func: f,
        }
    }
}

///
/// # ????????????????????????????????????????????????
///
/// ## list
/// ??????????????????????????????, run_time???????????????????????????
///
pub struct DelayEventList<T> {
    list: Vec<DelayEvent<T>>,
}

impl<T> DelayEventList<T> {
    pub fn new() -> Self {
        DelayEventList::<T> { list: Vec::new() }
    }

    pub fn add_event(
        &mut self,
        f: Box<dyn FnOnce(&mut T, &mut SuzuContext, Clock) -> ()>,
        t: Clock,
    ) -> &mut Self {
        self.add(DelayEvent::new(f, t))
    }

    pub fn add(&mut self, event: DelayEvent<T>) -> &mut Self {
        self.list.push(event);
        self.list.sort_by(|o1, o2| o2.run_time.cmp(&o1.run_time));
        self
    }

    pub fn move_top(&mut self) -> Option<DelayEvent<T>> {
        if self.list.len() > 0 {
            self.list.pop()
        } else {
            None
        }
    }

    pub fn len(&self) -> usize {
        self.list.len()
    }

    pub fn clear(&mut self) {
        self.list.clear();
    }
}

#[macro_export]
macro_rules! flush_delay_event {
    ($slf: expr, $event_list: expr, $ctx: expr, $t: expr) => {{
	let mut macro_loop_count: usize = 0;

	while let Some(event) = $event_list.move_top() {
            // ????????????????????????????????????????????????????????????????????????????????????????????????????????????
            if event.run_time > $t {
                $event_list.add(event);
                break;
            }

            // ???????????????????????????????????????self???????????????????????????????????????
            (event.func)($slf, $ctx, $t);
	    macro_loop_count += 1;
        }

	macro_loop_count
    }};
}

#[macro_export]
macro_rules! flush_delay_event_and_redraw_check {
    ($slf: expr, $event_list: expr, $ctx: expr, $t: expr, $handle:expr) => {{
        if flush_delay_event!($slf, $event_list, $ctx, $t) > 0 {
            $ctx.process_utility.redraw();
            let _ = $handle;
        }
    }};
}

#[macro_export]
macro_rules! add_delay_event {
    ($event_list: expr, $event: expr, $delay: expr) => {{
        $event_list.add_event(Box::new($event), $delay);
    }};
}
