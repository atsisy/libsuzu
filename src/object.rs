pub mod character_factory;
pub mod collision;
pub mod effect;
pub mod effect_object;
pub mod end_object;
pub mod map_object;
pub mod move_fn;
pub mod notify;
pub mod save_scene_object;
pub mod scenario;
pub mod scenario_object;
pub mod shop_object;
pub mod simulation_ui;
pub mod task_object;
pub mod task_result_object;
pub mod title_object;
pub mod util_object;

use ggez::graphics as ggraphics;

use crate::libsuzu::core::Clock;
use crate::libsuzu::graphics::drawable::*;
use crate::libsuzu::graphics::object as tobj;
use crate::libsuzu::graphics::object::sub_screen::SubScreen;
use crate::libsuzu::graphics::object::*;
use crate::impl_node2d_for_wrapped;
use crate::impl_transform_object_for_wrapped;
use crate::libsuzu::numeric;

use crate::core::map_parser as mp;
use crate::core::{SuzuContext, TextureID};

pub trait Clickable {
    fn button_down<'a>(
        &mut self,
        _ctx: &mut SuzuContext<'a>,
        _: Clock,
        _button: ggez::input::mouse::MouseButton,
        _point: numeric::Point2f,
    ) {
    }

    fn button_up(
        &mut self,
        _ctx: &mut SuzuContext,
        _: Clock,
        _button: ggez::input::mouse::MouseButton,
        _point: numeric::Point2f,
    ) {
    }

    fn on_click<'a>(
        &mut self,
        _ctx: &mut SuzuContext<'a>,
        _: Clock,
        _button: ggez::input::mouse::MouseButton,
        _point: numeric::Point2f,
    ) {
    }

    fn clickable_status(
        &mut self,
        _ctx: &mut ggez::Context,
        _point: numeric::Point2f,
    ) -> ggez::input::mouse::CursorIcon {
        ggez::input::mouse::CursorIcon::Default
    }
}

pub struct BlackOutParam {
    pub black_out: Clock,
    pub black_keep: Clock,
    pub black_return: Clock,
}

impl BlackOutParam {
    pub fn new(black_out: Clock, black_keep: Clock, black_return: Clock) -> Self {
        BlackOutParam {
            black_out: black_out,
            black_keep: black_keep,
            black_return: black_return,
        }
    }
}

pub struct BlackOutTexture {
    texture: EffectableWrap<MovableWrap<Texture>>,
}

impl BlackOutTexture {
    pub fn new<'a>(
        ctx: &mut SuzuContext<'a>,
        texture_id: TextureID,
        pos: numeric::Point2f,
        drawing_depth: i8,
        now: Clock,
    ) -> Self {
        BlackOutTexture {
            texture: EffectableWrap::new(
                MovableWrap::new(
                    Box::new(Texture::new(
                        ctx.ref_texture(texture_id),
                        pos,
                        numeric::Vector2f::new(1.0, 1.0),
                        0.0,
                        drawing_depth,
                    )),
                    None,
                    now,
                ),
                vec![],
            ),
        }
    }

    pub fn run_black_out(&mut self, param: BlackOutParam, now: Clock) {
        self.texture.clear_effect();
        self.texture.add_effect(vec![
            effect::fade_in(param.black_out, now),
            effect::fade_out(param.black_return, now + param.black_out + param.black_keep),
        ]);
    }
}

impl Drawable for BlackOutTexture {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.texture.draw(ctx)
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.texture.hide()
    }

    #[inline(always)]
    fn appear(&mut self) {
        self.texture.appear()
    }

    #[inline(always)]
    fn is_visible(&self) -> bool {
        self.texture.is_visible()
    }

    #[inline(always)]
    fn set_drawing_depth(&mut self, depth: i8) {
        self.texture.set_drawing_depth(depth)
    }

    #[inline(always)]
    fn get_drawing_depth(&self) -> i8 {
        self.texture.get_drawing_depth()
    }
}

impl Node2D for BlackOutTexture {
    impl_node2d_for_wrapped! {texture}
}

impl Transform for BlackOutTexture {
    impl_transform_object_for_wrapped! {texture}
}

pub struct DarkEffectPanel {
    canvas: EffectableWrap<MovableWrap<SubScreen>>,
}

impl DarkEffectPanel {
    pub fn new(ctx: &mut ggez::Context, rect: numeric::Rect, now: Clock) -> Self {
        DarkEffectPanel {
            canvas: EffectableWrap::new(
                MovableWrap::new(
                    Box::new(SubScreen::new(
                        ctx,
                        rect,
                        0,
                        ggraphics::Color::from_rgba_u32(0),
                    )),
                    None,
                    now,
                ),
                vec![],
            ),
        }
    }

    pub fn new_effect(
        &mut self,
        required_time: Clock,
        now: Clock,
        init_dark_alpha: u8,
        fin_dark_alpha: u8,
    ) {
        self.canvas.add_effect(vec![effect::alpha_effect(
            required_time,
            now,
            init_dark_alpha,
            fin_dark_alpha,
        )]);
    }

    ///
    /// # ?????????????????????
    ///
    pub fn run_effect<'a>(&mut self, ctx: &mut SuzuContext<'a>, t: Clock) {
        if !self.canvas.is_empty_effect() {
            ctx.process_utility.redraw();
            self.canvas.effect(ctx.context, t);
        }
    }

    pub fn set_alpha(&mut self, alpha: f32) {
        self.canvas.set_alpha(alpha);
    }
}

impl Drawable for DarkEffectPanel {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.is_visible() {
            sub_screen::stack_screen(ctx, &self.canvas);
            sub_screen::pop_screen(ctx);
            self.canvas.draw(ctx).unwrap();
        }

        Ok(())
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.canvas.hide()
    }

    #[inline(always)]
    fn appear(&mut self) {
        self.canvas.appear()
    }

    #[inline(always)]
    fn is_visible(&self) -> bool {
        self.canvas.is_visible()
    }

    #[inline(always)]
    fn set_drawing_depth(&mut self, depth: i8) {
        self.canvas.set_drawing_depth(depth)
    }

    #[inline(always)]
    fn get_drawing_depth(&self) -> i8 {
        self.canvas.get_drawing_depth()
    }
}
