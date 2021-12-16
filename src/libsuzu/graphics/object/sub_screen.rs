use std::cell::RefCell;
use std::rc::Rc;

use ggez::graphics as ggraphics;

use crate::libsuzu::graphics::object::*;

use crate::{
    get_param_pos, get_param_scale,
    get_param_rotation, get_param_offset,
    set_param_offset, set_param_scale,
    set_param_pos, set_param_rotation,
    move_param
};

use crate::libsuzu::numeric;

///
/// 描画対象のスタッキングを行うための構造体
///
/// Example
///
/// ```
/// // sub_screen::stack_screen(ctx, &sub_screen);
///
/// // SubScreenへ描画処理
///
/// // sub_screen::pop_screen(ctx);
/// // self.canvas.draw(ctx).unwrap();
/// ```
#[derive(Clone)]
pub struct SubScreen {
    canvas: Rc<ggraphics::Canvas>,
    drwob_essential: DrawableEss,
    draw_param: ggraphics::DrawParam,
    size: numeric::Vector2f,
    back_color: ggraphics::Color,
}

impl SubScreen {
    pub fn new(
        ctx: &mut ggez::Context,
        pos: ggraphics::Rect,
        depth: i8,
        back_color: ggraphics::Color,
    ) -> SubScreen {
        let dparam = ggraphics::DrawParam::default()
            .dest(ggez::mint::Point2 { x: pos.x, y: pos.y });

        SubScreen {
            canvas: Rc::new(
                ggraphics::Canvas::new(
                    ctx,
                    pos.w as u16,
                    pos.h as u16,
                    ggez::conf::NumSamples::Four,
		    ggraphics::get_window_color_format(ctx),
                )
                .unwrap(),
            ),
            drwob_essential: DrawableEss::new(true, depth),
            draw_param: dparam,
            size: numeric::Vector2f::new(pos.w, pos.h),
            back_color: back_color,
        }
    }

    pub fn relative_point(&self, abs_pos: numeric::Point2f) -> numeric::Point2f {
	let dest = get_param_pos!(self.draw_param);
        numeric::Point2f::new(
            abs_pos.x - dest.x,
            abs_pos.y - dest.y,
        )
    }

    pub fn contains(&self, point: numeric::Point2f) -> bool {
	let dest = self.get_position();
	
        let rect = numeric::Rect::new(
	    dest.x,
	    dest.y,
            self.canvas.width() as f32,
            self.canvas.height() as f32,
        );
        rect.contains(mintp!(point))
    }

    pub fn set_filter(&mut self, mode: ggraphics::FilterMode) {
        Rc::get_mut(&mut self.canvas).unwrap().set_filter(mode);
    }
}

impl Drawable for SubScreen {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        ggraphics::draw(ctx, self.canvas.as_ref(), self.draw_param.clone())
    }

    fn hide(&mut self) {
        self.drwob_essential.visible = false;
    }

    fn appear(&mut self) {
        self.drwob_essential.visible = true;
    }

    fn is_visible(&self) -> bool {
        self.drwob_essential.visible
    }

    /// 描画順序を設定する
    fn set_drawing_depth(&mut self, depth: i8) {
        self.drwob_essential.drawing_depth = depth;
    }

    /// 描画順序を返す
    fn get_drawing_depth(&self) -> i8 {
        self.drwob_essential.drawing_depth
    }
}

impl Node2D for SubScreen {
    #[inline(always)]
    fn set_position(&mut self, pos: numeric::Point2f) {
	set_param_pos!(self.draw_param, pos);
    }

    #[inline(always)]
    fn get_position(&self) -> numeric::Point2f {
	let p = get_param_pos!(self.draw_param);
	numeric::Point2f::new(p.x, p.y)
    }

    #[inline(always)]
    fn move_diff(&mut self, offset: numeric::Vector2f) {
	move_param!(self.draw_param, offset);
    }
}

impl Transform for SubScreen {
    #[inline(always)]
    fn set_scale(&mut self, scale: numeric::Vector2f) {
	set_param_scale!(self.draw_param, scale);
    }

    #[inline(always)]
    fn get_scale(&self) -> numeric::Vector2f {
	let scale = get_param_scale!(self.draw_param);
	numeric::Vector2f::new(scale.x, scale.y)
    }

    #[inline(always)]
    fn set_rotation(&mut self, rad: f32) {
	set_param_rotation!(self.draw_param, rad);
    }

    #[inline(always)]
    fn get_rotation(&self) -> f32 {
	get_param_rotation!(self.draw_param)
    }

    #[inline(always)]
    fn set_crop(&mut self, crop: ggraphics::Rect) {
        self.draw_param.src = crop;
    }

    #[inline(always)]
    fn get_crop(&self) -> ggraphics::Rect {
        self.draw_param.src
    }

    #[inline(always)]
    fn set_drawing_color(&mut self, color: ggraphics::Color) {
        self.draw_param.color = color;
        self.back_color = color;
    }

    #[inline(always)]
    fn get_drawing_color(&self) -> ggraphics::Color {
        self.back_color
    }

    #[inline(always)]
    fn set_alpha(&mut self, alpha: f32) {
        self.draw_param.color.a = alpha;
        self.back_color.a = alpha;
    }

    #[inline(always)]
    fn get_alpha(&self) -> f32 {
        self.draw_param.color.a
    }

    #[inline(always)]
    fn set_transform_offset(&mut self, offset: numeric::Point2f) {
	set_param_offset!(self.draw_param, offset);
    }

    #[inline(always)]
    fn get_transform_offset(&self) -> numeric::Point2f {
	let p = get_param_offset!(self.draw_param);
	numeric::Point2f::new(p.x, p.y)
    }

    #[inline(always)]
    fn get_texture_size(&self, _ctx: &mut ggez::Context) -> numeric::Vector2f {
        numeric::Vector2f::new(
            self.canvas.width() as f32,
            self.canvas.height() as f32,
        )
    }

    #[inline(always)]
    fn set_color(&mut self, color: ggraphics::Color) {
        self.draw_param.color = color;
    }

    #[inline(always)]
    fn get_color(&mut self) -> ggraphics::Color {
        self.draw_param.color
    }
}

thread_local!(static SCREEN_STACK: RefCell<Vec<SubScreen>> = {
    RefCell::new(Vec::new())
});

thread_local!(static TARGET_SCREEN: RefCell<Option<SubScreen>> = {
    RefCell::new(None)
});

fn setup_new_drawing_target(ctx: &mut ggez::Context, screen: &SubScreen) {
    ggraphics::set_canvas(ctx, Some(&screen.canvas));
    ggraphics::clear(ctx, screen.back_color);
    ggraphics::set_screen_coordinates(
        ctx,
        ggraphics::Rect::new(0.0, 0.0, screen.size.x, screen.size.y),
    )
    .unwrap();
}

fn setup_poped_drawing_target(ctx: &mut ggez::Context, screen: &SubScreen) {
    ggraphics::set_canvas(ctx, Some(&screen.canvas));
    ggraphics::set_screen_coordinates(
        ctx,
        ggraphics::Rect::new(0.0, 0.0, screen.size.x, screen.size.y),
    )
    .unwrap();
}

fn make_none_draw_target(ctx: &mut ggez::Context) {
    let window_size = ggraphics::drawable_size(ctx);
    reset_stacking_screen(None);
    ggraphics::set_canvas(ctx, None);
    ggraphics::set_screen_coordinates(
        ctx,
        ggraphics::Rect::new(0.0, 0.0, window_size.0, window_size.1),
    )
    .unwrap();
}

///
/// 現在の描画対象を変更するメソッド
/// Noneを渡すと、ウィンドウが描画対象になる
///
pub fn reset_stacking_screen(new_screen: Option<&SubScreen>) -> Option<SubScreen> {
    TARGET_SCREEN.with(|target_screen| {
        if let Some(new_screen) = new_screen {
            target_screen.replace_with(|_| Some(new_screen.clone()))
        } else {
            target_screen.replace_with(|_| None)
        }
    })
}

///
/// 現在の描画対象を変更し、変更前の描画対象を内部スタックに積む
///
pub fn stack_screen(ctx: &mut ggez::Context, new_screen: &SubScreen) -> usize {
    // 描画対象変更の準備
    setup_new_drawing_target(ctx, &new_screen);

    // 描画対象を変更し、最後に描画対象だったスクリーンを取り出す
    let last_screen = reset_stacking_screen(Some(&new_screen));

    // 最後の描画対象をスタックに積む
    SCREEN_STACK.with(|stack| {
        if let Some(last_screen) = last_screen {
            stack.borrow_mut().push(last_screen);
        }

        stack.borrow().len()
    })
}

///
/// 内部スタックから描画対象を取り出し、現在の描画対象を変更する
/// スタックが空の場合、描画対象がウィンドウになる
///
pub fn pop_screen(ctx: &mut ggez::Context) -> (usize, Option<SubScreen>) {
    // スタックから描画対象を取り出す
    let (stack_len, last_cur_screen) = SCREEN_STACK.with(|stack| {
        let stack_len = stack.borrow().len();
        (stack_len, stack.borrow_mut().pop())
    });

    // 取り出した描画対象がNoneでなければ、それを描画対象とし、
    // Noneならウィンドウを描画対象にする
    (
        stack_len,
        if last_cur_screen.is_some() {
            setup_poped_drawing_target(ctx, last_cur_screen.as_ref().unwrap());
            reset_stacking_screen(last_cur_screen.as_ref())
        } else {
            make_none_draw_target(ctx);
            None
        },
    )
}
