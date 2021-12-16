pub mod menu;
pub mod shadow;
pub mod shape;
pub mod sub_screen;
pub mod tile_batch;

use super::super::numeric;
use super::drawable::{Drawable, Node2D, DrawableEss};
use crate::libsuzu::core::Clock;
use ggez::graphics as ggraphics;
use ggez::*;
use std::cmp::Ordering;
use std::ops::{Deref, DerefMut};
use crate::{mintp, mintv};

#[macro_export]
macro_rules! set_param_scale {
    ($v1: expr, $scale: expr) => {
	$v1 = $v1.scale(mintv!($scale));
    };
}

#[macro_export]
macro_rules! set_param_pos {
    ($v1: expr, $pos: expr) => {
	$v1 = $v1.dest(mintp!($pos));
    };
}

#[macro_export]
macro_rules! set_param_rotation {
    ($v1: expr, $r: expr) => {
	$v1 = $v1.rotation($r);
    };
}

#[macro_export]
macro_rules! set_param_offset {
    ($v1: expr, $offset: expr) => {
	$v1 = $v1.offset(mintp!($offset));
    };
}

#[macro_export]
macro_rules! get_param_pos {
    ($v1: expr) => {
	match $v1.trans {
	    ggraphics::Transform::Values {
                dest,
		..
            } => {
		dest
	    },
	    _ => panic!("Not a ggraphics::Transform::Values"),
	}
    };
}

#[macro_export]
macro_rules! get_param_rotation {
    ($v1: expr) => {
	match $v1.trans {
	    ggraphics::Transform::Values {
                rotation,
		..
            } => {
		rotation
	    },
	    _ => panic!("Not a ggraphics::Transform::Values"),
	}
    };
}

#[macro_export]
macro_rules! get_param_scale {
    ($v1: expr) => {
	match $v1.trans {
	    ggraphics::Transform::Values {
                scale,
		..
            } => {
		scale
	    },
	    _ => panic!("Not a ggraphics::Transform::Values"),
	}
    };
}

#[macro_export]
macro_rules! get_param_offset {
    ($v1: expr) => {
	match $v1.trans {
	    ggraphics::Transform::Values {
                offset,
		..
            } => {
		offset
	    },
	    _ => panic!("Not a ggraphics::Transform::Values"),
	}
    };
}

#[macro_export]
macro_rules! move_param {
    ($v1: expr, $diff: expr) => {
	let mut now = get_param_pos!($v1);
	now.x += $diff.x;
	now.y += $diff.y;
	set_param_pos!($v1, now);
    };
}

///
/// # テクスチャを利用して描画を行うために必要なインターフェイスを保証させるトレイト
/// テクスチャを利用して描画を行う場合は、このトレイトを実装し、動作を保証しなければならない
///
pub trait Transform: Node2D {
    /// テクスチャのスケールを設定する
    fn set_scale(&mut self, scale: numeric::Vector2f);

    /// テクスチャのスケールを返す
    fn get_scale(&self) -> numeric::Vector2f;

    /// テクスチャの回転状態を設定する
    fn set_rotation(&mut self, rad: f32);

    /// テクスチャの回転状態を返す
    fn get_rotation(&self) -> f32;

    /// テクスチャの切り抜きを設定する。範囲は倍率（f32）で設定する
    fn set_crop(&mut self, crop: ggraphics::Rect);

    /// テクスチャの切り抜き情報を返す
    fn get_crop(&self) -> ggraphics::Rect;

    /// テクスチャを描画する際の色情報を設定する
    fn set_drawing_color(&mut self, color: ggraphics::Color);

    /// テクスチャを描画する際の色情報を返す
    fn get_drawing_color(&self) -> ggraphics::Color;

    /// テクスチャのalpha値を設定する
    fn set_alpha(&mut self, alpha: f32);

    /// テクスチャのalpha値を返す
    fn get_alpha(&self) -> f32;

    /// テクスチャに対するエフェクトの始点を設定する
    fn set_transform_offset(&mut self, offset: numeric::Point2f);

    /// テクスチャに対するエフェクトの始点を返す
    fn get_transform_offset(&self) -> numeric::Point2f;

    /// 実際に描画が行われるエリアをRectで返す
    fn get_drawing_area(&self, ctx: &mut ggez::Context) -> ggraphics::Rect {
        let point = self.get_position();
        let size = self.get_drawing_size(ctx);
        ggraphics::Rect::new(point.x, point.y, size.x, size.y)
    }

    /// 実際に描画が行われる幅と高さを返す
    fn get_drawing_size(&self, ctx: &mut ggez::Context) -> numeric::Vector2f {
        let scale = self.get_scale();
        let size = self.get_texture_size(ctx);
        numeric::Vector2f::new(size.x * scale.x, size.y * scale.y)
    }

    /// テクスチャのサイズを返す
    fn get_texture_size(&self, ctx: &mut ggez::Context) -> numeric::Vector2f;
    
    // 色指定
    fn set_color(&mut self, color: ggraphics::Color);

    // 指定した色を取得
    fn get_color(&mut self) -> ggraphics::Color;

    /// 描画しているテクスチャの中央座標を返すメソッド
    fn get_center(&self, ctx: &mut ggez::Context) -> numeric::Point2f {
        let p = self.get_drawing_area(ctx);
        numeric::Point2f::new(p.x + (p.w / 2.0), p.y + (p.h / 2.0))
    }

    /// 描画開始位置から、中心までのオフセットを返すメソッド
    fn get_center_offset(&self, ctx: &mut ggez::Context) -> numeric::Vector2f {
        let size = self.get_drawing_size(ctx);
        numeric::Vector2f::new(size.x / 2.0, size.y / 2.0)
    }

    #[inline(always)]
    fn fit_scale(&mut self, ctx: &mut ggez::Context, size: numeric::Vector2f) {
        let current_size = self.get_texture_size(ctx);
        self.set_scale(numeric::Vector2f::new(
            size.x / current_size.x,
            size.y / current_size.y,
        ));
    }

    #[inline(always)]
    fn fit_crop(&mut self, ctx: &mut ggez::Context, size: numeric::Vector2f) {
        let current_size = self.get_drawing_size(ctx);
        self.set_crop(numeric::Rect::new(
	    0.0,
	    0.0,
	    size.x / current_size.x,
	    size.y / current_size.y
	));
    }

    #[inline(always)]
    fn make_center(&mut self, ctx: &mut ggez::Context, center: numeric::Point2f) {
        self.move_diff(center - self.get_center(ctx));
    }

    #[inline(always)]
    fn contains(&self, ctx: &mut ggez::Context, point: numeric::Point2f) -> bool {
        self.get_drawing_area(ctx).contains(mintp!(point))
    }
}

#[macro_export]
macro_rules! impl_node2d_for_wrapped {
    ( $( $texture: tt ),* ) => {
	#[inline(always)]
	fn set_position(&mut self, pos: numeric::Point2f) {
            self.$($texture)*.set_position(pos);
	}

	#[inline(always)]
	fn get_position(&self) -> numeric::Point2f {
            self.$($texture)*.get_position()
	}

	#[inline(always)]
	fn move_diff(&mut self, offset: numeric::Vector2f) {
            self.$($texture)*.move_diff(offset)
	}
    };
}

#[macro_export]
macro_rules! impl_transform_object_for_wrapped {
    ( $( $texture: tt ),* ) => {
	#[inline(always)]
	fn set_scale(&mut self, scale: numeric::Vector2f) {
	    self.$($texture)*.set_scale(scale)
	}

	#[inline(always)]
	fn get_scale(&self) -> numeric::Vector2f {
	    self.$($texture)*.get_scale()
	}

	#[inline(always)]
	fn set_rotation(&mut self, rad: f32) {
	    self.$($texture)*.set_rotation(rad)
	}

	#[inline(always)]
	fn get_rotation(&self) -> f32 {
	    self.$($texture)*.get_rotation()
	}

	#[inline(always)]
	fn set_crop(&mut self, crop: ggraphics::Rect) {
	    self.$($texture)*.set_crop(crop);
	}

	#[inline(always)]
	fn get_crop(&self) -> ggraphics::Rect {
	    self.$($texture)*.get_crop()
	}

	#[inline(always)]
	fn set_drawing_color(&mut self, color: ggraphics::Color) {
	    self.$($texture)*.set_drawing_color(color)
	}

	#[inline(always)]
	fn get_drawing_color(&self) -> ggraphics::Color {
	    self.$($texture)*.get_drawing_color()
	}

	#[inline(always)]
	fn set_alpha(&mut self, alpha: f32) {
	    self.$($texture)*.set_alpha(alpha);
	}

	#[inline(always)]
	fn get_alpha(&self) -> f32 {
	    self.$($texture)*.get_alpha()
	}

	#[inline(always)]
	fn set_transform_offset(&mut self, offset: numeric::Point2f) {
	    self.$($texture)*.set_transform_offset(offset);
	}

	#[inline(always)]
	fn get_transform_offset(&self) -> numeric::Point2f {
	    self.$($texture)*.get_transform_offset()
	}

	#[inline(always)]
	fn get_drawing_area(&self, ctx: &mut ggez::Context) -> ggraphics::Rect {
	    self.$($texture)*.get_drawing_area(ctx)
	}

	#[inline(always)]
	fn get_drawing_size(&self, ctx: &mut ggez::Context) -> numeric::Vector2f {
	    self.$($texture)*.get_drawing_size(ctx)
	}

	#[inline(always)]
	fn get_texture_size(&self, ctx: &mut ggez::Context) -> numeric::Vector2f {
	    self.$($texture)*.get_texture_size(ctx)
	}

	#[inline(always)]
	fn set_color(&mut self, color: ggraphics::Color) {
	    self.$($texture)*.set_color(color);
	}

	#[inline(always)]
	fn get_color(&mut self) -> ggraphics::Color {
	    self.$($texture)*.get_color()
        }
    };
}


pub type GenericMoveFn =
    Box<dyn Fn(&dyn MovableObject, Clock) -> Option<numeric::Point2f>>;

///
/// # 任意の関数に従って座標を動かすことができることを保証するトレイト
///
pub trait MovableObject: Transform {
    // 任意の関数に従って、座標を動かす
    fn move_with_func(&mut self, t: Clock);

    // 従う関数を変更する
    fn override_move_func(
        &mut self,
        move_fn: Option<GenericMoveFn>,
        now: Clock,
    );

    // 動作する関数が設定された時間を返す
    fn mf_start_timing(&self) -> Clock;

    // 現在停止しているかを返す
    fn is_stop(&self) -> bool;
}

///
/// MovableObjectを深度（Z軸）でソートするための関数
///
/// この関数でソートすると、深度が深いものが先頭に来るようにソートされる
///
pub fn boxed_movable_object_sort_with_depth<T, U>(a: &Box<T>, b: &Box<U>) -> Ordering
where
    T: MovableObject,
    U: MovableObject,
{
    let (ad, bd) = (a.get_drawing_depth(), b.get_drawing_depth());
    if ad > bd {
        Ordering::Less
    } else if ad < bd {
        Ordering::Greater
    } else {
        Ordering::Equal
    }
}

///
/// # エフェクトに対応していることを保証させるトレイト
///
pub trait Effectable {
    // エフェクト処理を行う
    fn effect(&mut self, ctx: &mut ggez::Context, t: Clock);
}

#[derive(PartialEq, Clone, Copy)]
pub enum EffectFnStatus {
    EffectContinue,
    EffectFinish,
}

pub type GenericEffectFn =
    Box<dyn Fn(&mut dyn MovableObject, &ggez::Context, Clock) -> EffectFnStatus>;

///
/// # クロージャによって実装されるエフェクトに対応していることを保証させるトレイト
/// Effectableを実装している必要がある
///
pub trait HasGenericEffect: Effectable {
    // 新しくエフェクトを追加するメソッド
    fn add_effect(&mut self, effect: Vec<GenericEffectFn>);

    fn clear_effect(&mut self);

    fn is_empty_effect(&self) -> bool;
}

///
/// # Trait MovableObjectを実装するために必要なフィールド群
/// Trait MovableObjectを実装する場合に便利な構造体
///
/// ## フィールド
/// ### move_func
/// 従う関数をクロージャで表現したもの。MovableObjectを実装する場合は、これに従うように実装するべき
///
/// ### mf_set_time
/// 最後にmove_funcが変更された時の時刻
///
/// ### init_position
/// 生成された時の初期位置
///
pub struct MovableEss {
    move_func: Option<GenericMoveFn>,
    mf_set_time: Clock,
    init_position: numeric::Point2f,
}

impl MovableEss {
    // MovableEssを生成する関数
    pub fn new(
        f: Option<GenericMoveFn>,
        t: Clock,
        init_pos: numeric::Point2f,
    ) -> MovableEss {
        MovableEss {
            move_func: f,
            mf_set_time: t,
            init_position: init_pos,
        }
    }

    pub fn clone(
        &self,
        f: Option<GenericMoveFn>,
    ) -> Self {
        MovableEss {
            move_func: f,
            mf_set_time: self.mf_set_time,
            init_position: self.init_position,
        }
    }
}

///
/// # Trait Effectableを実装するために必要なフィールド群
/// Trait Effectableを実装する場合に便利な構造体
///
///
pub struct HasGenericEffectEssential {
    effects_list: Vec<GenericEffectFn>,
}

impl HasGenericEffectEssential {
    fn new(list: Vec<GenericEffectFn>) -> HasGenericEffectEssential {
        HasGenericEffectEssential { effects_list: list }
    }
}

pub struct Texture {
    drwob_essential: DrawableEss,
    texture: ggraphics::Image,
    draw_param: ggraphics::DrawParam,
}

impl Texture {
    pub fn new(
        texture: ggraphics::Image,
        pos: numeric::Point2f,
        scale: numeric::Vector2f,
        rotation: f32,
        drawing_depth: i8,
    ) -> Texture {
        let param = ggraphics::DrawParam::new()
	    .dest(mintp!(pos))
            .scale(mintv!(scale))
            .rotation(rotation);

        Texture {
            drwob_essential: DrawableEss::new(true, drawing_depth),
            texture: texture,
            draw_param: param,
        }
    }

    pub fn replace_texture(&mut self, texture: ggraphics::Image) {
        self.texture = texture;
    }
    
    pub fn set_filter(&mut self, filter: ggraphics::FilterMode) {
	self.texture.set_filter(filter);
    }

    pub fn get_filter(&mut self) -> ggraphics::FilterMode {
	self.texture.filter()
    }
}

impl Drawable for Texture {
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.drwob_essential.visible {
            ggraphics::draw(ctx, &self.texture, self.draw_param)
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.drwob_essential.visible = false;
    }

    #[inline(always)]
    fn appear(&mut self) {
        self.drwob_essential.visible = true;
    }

    #[inline(always)]
    fn is_visible(&self) -> bool {
        self.drwob_essential.visible
    }

    #[inline(always)]
    fn set_drawing_depth(&mut self, depth: i8) {
        self.drwob_essential.drawing_depth = depth;
    }

    #[inline(always)]
    fn get_drawing_depth(&self) -> i8 {
        self.drwob_essential.drawing_depth
    }
}

impl Node2D for Texture {
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

impl Transform for Texture {
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
    }

    #[inline(always)]
    fn get_drawing_color(&self) -> ggraphics::Color {
        self.draw_param.color
    }

    #[inline(always)]
    fn set_alpha(&mut self, alpha: f32) {
        self.draw_param.color.a = alpha;
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
        numeric::Vector2f::new(self.texture.width() as f32, self.texture.height() as f32)
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

///
/// # フォントの情報を持つ構造体
///
/// ## フィールド
/// ### font
/// フォント
///
/// ### scale
/// フォントのスケール
///
#[derive(Debug, Clone, Copy)]
pub struct FontInformation {
    pub font: ggraphics::Font,
    pub scale: numeric::Vector2f,
    pub color: ggraphics::Color,
}

impl FontInformation {
    pub fn new(
        font: ggraphics::Font,
        scale: numeric::Vector2f,
        color: ggraphics::Color,
    ) -> FontInformation {
        FontInformation {
            font: font,
            scale: scale,
            color: color,
        }
    }
}

///
/// # Move可能で描画可能なテキスト
///
/// ## フィールド
/// ### drwob_essential
/// DrawableObjectを実装するために持つ構造体
///
/// ### text
/// 文字列の実態
///
/// ### draw_param
/// 主に、Trait TextureObjectをを実装するために持つ構造体
/// 描画位置, スケールなどの情報を保持している
///
/// ### mv_essential
/// MovableObjectを実装するために必要なフィールド
///
/// ### font_info
/// フォントの種類やスケールが保持されている
///
pub struct UniText {
    drwob_essential: DrawableEss,
    text: graphics::Text,
    font_info: FontInformation,
    draw_param: ggraphics::DrawParam,
}

impl UniText {
    // 生成関数
    pub fn new(
        text: String,
        pos: numeric::Point2f,
        scale: numeric::Vector2f,
        rotation: f32,
        drawing_depth: i8,
        font_info: FontInformation,
    ) -> UniText {
        let param = ggraphics::DrawParam::new()
            .dest(mintp!(pos))
            .scale(mintv!(scale))
            .rotation(rotation);

        let mut ret_text = UniText {
            drwob_essential: DrawableEss::new(true, drawing_depth),
            text: ggraphics::Text::new(text),
            font_info: font_info,
            draw_param: param,
        };

        ret_text.apply_font_information();
        ret_text
    }

    fn apply_font_information(&mut self) {
        self.text.set_font(
            self.font_info.font,
            ggraphics::PxScale {
                x: self.font_info.scale.x,
                y: self.font_info.scale.y,
            },
        );
        self.draw_param.color = self.font_info.color;
    }

    pub fn get_font_scale(&self) -> numeric::Vector2f {
        self.font_info.scale
    }

    pub fn set_font_scale(&mut self, scale: numeric::Vector2f) {
        self.font_info.scale = scale;
        self.apply_font_information();
    }

    pub fn change_font(&mut self, font: ggraphics::Font) {
        self.font_info.font = font;
        self.apply_font_information();
    }

    pub fn get_text(&self) -> String {
        self.text.contents()
    }

    pub fn replace_text(&mut self, text: String) {
        self.text = ggraphics::Text::new(text);
        self.apply_font_information();
    }
}

impl Drawable for UniText {
    #[inline(always)]
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        if self.drwob_essential.visible {
            // textを描画する
            ggraphics::draw(ctx, &self.text, self.draw_param)
        } else {
            Ok(())
        }
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.drwob_essential.visible = false;
    }

    #[inline(always)]
    fn appear(&mut self) {
        self.drwob_essential.visible = true;
    }

    #[inline(always)]
    fn is_visible(&self) -> bool {
        self.drwob_essential.visible
    }

    #[inline(always)]
    fn set_drawing_depth(&mut self, depth: i8) {
        self.drwob_essential.drawing_depth = depth;
    }

    #[inline(always)]
    fn get_drawing_depth(&self) -> i8 {
        self.drwob_essential.drawing_depth
    }
}

impl Node2D for UniText {
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

impl Transform for UniText {
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
    }

    #[inline(always)]
    fn get_drawing_color(&self) -> ggraphics::Color {
        self.draw_param.color
    }

    #[inline(always)]
    fn set_alpha(&mut self, alpha: f32) {
        self.draw_param.color.a = alpha;
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
    fn get_texture_size(&self, ctx: &mut ggez::Context) -> numeric::Vector2f {
        numeric::Vector2f::new(self.text.width(ctx) as f32, self.text.height(ctx) as f32)
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

pub struct MovableWrap<T: ?Sized + Transform> {
    texture_object: Box<T>,
    mv_essential: MovableEss,
}

impl<T: ?Sized + Transform> MovableWrap<T> {
    // 生成関数
    pub fn new(
        texture_object: Box<T>,
        mf: Option<GenericMoveFn>,
        t: Clock,
    ) -> MovableWrap<T> {
        let pos = texture_object.get_position();
        MovableWrap::<T> {
            texture_object: texture_object,
            mv_essential: MovableEss::new(mf, t, pos),
        }
    }

    pub fn ref_wrapped_object(&self) -> &Box<T> {
        &self.texture_object
    }

    pub fn ref_wrapped_object_mut(&mut self) -> &mut Box<T> {
        &mut self.texture_object
    }

    pub fn move_wrapped_object(self) -> Box<T> {
        self.texture_object
    }
}

impl<T: ?Sized + Transform> Deref for MovableWrap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.ref_wrapped_object()
    }
}

impl<T: ?Sized + Transform> DerefMut for MovableWrap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ref_wrapped_object_mut()
    }
}

impl<T: ?Sized + Transform> Drawable for MovableWrap<T> {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        self.texture_object.draw(ctx)
    }

    fn hide(&mut self) {
        self.texture_object.hide();
    }

    fn appear(&mut self) {
        self.texture_object.appear();
    }

    fn is_visible(&self) -> bool {
        self.texture_object.is_visible()
    }

    fn set_drawing_depth(&mut self, depth: i8) {
        self.texture_object.set_drawing_depth(depth);
    }

    fn get_drawing_depth(&self) -> i8 {
        self.texture_object.get_drawing_depth()
    }
}

impl<T: ?Sized + Transform> Node2D for MovableWrap<T> {
    fn set_position(&mut self, pos: numeric::Point2f) {
        self.texture_object.set_position(pos);
    }

    fn get_position(&self) -> numeric::Point2f {
        self.texture_object.get_position()
    }

    fn move_diff(&mut self, offset: numeric::Vector2f) {
        self.texture_object.move_diff(offset);
    }
}

impl<T: ?Sized + Transform> Transform for MovableWrap<T> {
    impl_transform_object_for_wrapped! {texture_object}
}

impl<T: ?Sized + Transform> MovableObject for MovableWrap<T> {
    fn move_with_func(&mut self, t: Clock) {
        let not_stop = self.mv_essential.move_func.is_some();
        if !not_stop {
            return;
        }

	if let Some(pos) = (self.mv_essential.move_func.as_ref().unwrap())(
            self,
            t - self.mv_essential.mf_set_time,
        ) {
	    self.set_position(pos);
	} else {
	    self.mv_essential.move_func = None;
	}
    }

    // 従う関数を変更する
    fn override_move_func(
        &mut self,
        move_fn: Option<GenericMoveFn>,
        now: Clock,
    ) {
        self.mv_essential.move_func = move_fn;
        self.mv_essential.mf_set_time = now;
    }

    fn mf_start_timing(&self) -> Clock {
        self.mv_essential.mf_set_time
    }

    fn is_stop(&self) -> bool {
        self.mv_essential.move_func.is_none()
    }
}

///
/// # エフェクトを掛けるためのジェネリック構造体
/// この構造体で包まれたオブジェクトはエフェクトの効果を受ける
///
/// ## フィールド
/// ### movable_object
/// MovableObject, TextureObjectトレイトを実装していなければならない。
/// エフェクトはこのオブジェクトに対して行われる。
///
/// ### geffect_essential
/// HasGenericEffectEssentialを実装するために必要なフィールド
/// エフェクトのクロージャが含まれる
///
pub struct EffectableWrap<T: MovableObject + Transform> {
    movable_object: T,
    geffect_essential: HasGenericEffectEssential,
}

impl<T: MovableObject + Transform> EffectableWrap<T> {
    // 生成関数
    pub fn new(movable_object: T, effects: Vec<GenericEffectFn>) -> EffectableWrap<T> {
        EffectableWrap::<T> {
            movable_object: movable_object,
            geffect_essential: HasGenericEffectEssential::new(effects),
        }
    }

    pub fn ref_wrapped_object(&self) -> &T {
        &self.movable_object
    }

    pub fn ref_wrapped_object_mut(&mut self) -> &mut T {
        &mut self.movable_object
    }

    pub fn move_wrapped_object(self) -> Box<T> {
        Box::new(self.movable_object)
    }
}

impl<T: MovableObject + Transform> Deref for EffectableWrap<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.ref_wrapped_object()
    }
}

impl<T: MovableObject + Transform> DerefMut for EffectableWrap<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.ref_wrapped_object_mut()
    }
}

impl<T: MovableObject + Transform> Drawable for EffectableWrap<T> {
    #[inline(always)]
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.movable_object.draw(ctx)
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.movable_object.hide()
    }

    #[inline(always)]
    fn appear(&mut self) {
        self.movable_object.appear()
    }

    #[inline(always)]
    fn is_visible(&self) -> bool {
        self.movable_object.is_visible()
    }

    #[inline(always)]
    fn set_drawing_depth(&mut self, depth: i8) {
        self.movable_object.set_drawing_depth(depth)
    }

    #[inline(always)]
    fn get_drawing_depth(&self) -> i8 {
        self.movable_object.get_drawing_depth()
    }
}

impl<T: MovableObject + Transform> Node2D for EffectableWrap<T> {
    #[inline(always)]
    fn set_position(&mut self, pos: numeric::Point2f) {
        self.movable_object.set_position(pos)
    }

    #[inline(always)]
    fn get_position(&self) -> numeric::Point2f {
        self.movable_object.get_position()
    }

    #[inline(always)]
    fn move_diff(&mut self, offset: numeric::Vector2f) {
        self.movable_object.move_diff(offset);
    }
}

impl<T: MovableObject + Transform> Transform for EffectableWrap<T> {
    impl_transform_object_for_wrapped! {movable_object}
}

impl<T: MovableObject + Transform> MovableObject for EffectableWrap<T> {
    #[inline(always)]
    fn move_with_func(&mut self, t: Clock) {
        // クロージャにはselfと経過時間を与える
        self.movable_object.move_with_func(t)
    }

    // 従う関数を変更する
    fn override_move_func(
        &mut self,
        move_fn: Option<GenericMoveFn>,
        now: Clock,
    ) {
        self.movable_object.override_move_func(move_fn, now)
    }

    fn mf_start_timing(&self) -> Clock {
        self.movable_object.mf_start_timing()
    }

    fn is_stop(&self) -> bool {
        self.movable_object.is_stop()
    }
}

impl<T: MovableObject + Transform> HasGenericEffect for EffectableWrap<T> {
    // 新しくエフェクトを追加するメソッド
    fn add_effect(&mut self, effect: Vec<GenericEffectFn>) {
        self.geffect_essential.effects_list.extend(effect)
    }

    fn clear_effect(&mut self) {
        self.geffect_essential.effects_list.clear();
    }

    fn is_empty_effect(&self) -> bool {
	self.geffect_essential.effects_list.is_empty()
    }
}

impl<T: MovableObject + Transform> Effectable for EffectableWrap<T> {
    // 新しくエフェクトを追加するメソッド
    fn effect(&mut self, ctx: &mut ggez::Context, t: Clock) {
        let borrowed_movable = &mut self.movable_object;
        self.geffect_essential
            .effects_list
            .retain(|f| (f)(borrowed_movable, ctx, t) != EffectFnStatus::EffectFinish);
    }
}

pub type MovableTexture = MovableWrap<Texture>;
pub type SimpleObject = EffectableWrap<MovableTexture>;
pub type MovableText = MovableWrap<UniText>;
pub type SimpleText = EffectableWrap<MovableText>;

pub struct VerticalText {
    drwob_essential: DrawableEss,
    text: Vec<graphics::Text>,
    font_info: FontInformation,
    draw_param: ggraphics::DrawParam,
    raw_text: String,
}

impl VerticalText {
    pub fn new(
        text: String,
        pos: numeric::Point2f,
        scale: numeric::Vector2f,
        rotation: f32,
        drawing_depth: i8,
        font_info: FontInformation,
    ) -> Self {
	let param = ggraphics::DrawParam::new()
	    .dest(mintp!(pos))
            .scale(mintv!(scale))
            .rotation(rotation)
	    .color(font_info.color);

        let mut text_vec = Vec::new();
        for ch in text.chars() {
            let mut text_fragment = ggraphics::Text::new(ch);
            text_fragment.set_font(
                font_info.font,
                ggraphics::PxScale {
                    x: font_info.scale.x,
                    y: font_info.scale.y,
                },
            );
            text_vec.push(text_fragment);
        }

        VerticalText {
            drwob_essential: DrawableEss::new(true, drawing_depth),
            text: text_vec,
            font_info: font_info,
            draw_param: param,
            raw_text: text.to_string(),
        }
    }

    pub fn get_text(&self) -> &str {
        &self.raw_text
    }

    pub fn len(&self) -> usize {
        self.text.len()
    }

    pub fn replace_text(&mut self, text: String) {
        let mut text_vec = Vec::new();
        for ch in text.chars() {
            let mut text_fragment = ggraphics::Text::new(ch);
            text_fragment.set_font(
                self.font_info.font,
                ggraphics::PxScale {
                    x: self.font_info.scale.x,
                    y: self.font_info.scale.y,
                },
            );
            text_vec.push(text_fragment);
        }
        self.text = text_vec;
        self.raw_text = text;
    }

    pub fn get_indent_num(&self) -> usize {
        self.text.iter().fold(0, |sum, text| {
            sum + if text.contents().eq("\n") { 1 } else { 0 }
        })
    }
}

impl Drawable for VerticalText {
    #[inline(always)]
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        let mut height =
            self.raw_text.len() as f32 * self.font_info.scale.y * self.draw_param.src.h;
        let color = Some(self.draw_param.color);
        let mut pos = ggez::mint::Point2 { x: self.get_indent_num() as f32 * self.font_info.scale.x, y: 0.0 };

        if self.drwob_essential.visible {
            for fragment in &self.text {
                if fragment.contents().eq("\n") {
                    pos.x -= self.font_info.scale.x;
                    pos.y = 0.0;
                    continue;
                }

                if height < self.font_info.scale.y {
                    break;
                }

                ggraphics::queue_text(ctx, fragment, pos, color);

                height -= self.font_info.scale.y;
                pos.y += self.font_info.scale.y;
            }

            ggraphics::draw_queued_text(ctx, self.draw_param, None, ggraphics::FilterMode::Linear)?;
        }

        Ok(())
    }

    #[inline(always)]
    fn hide(&mut self) {
        self.drwob_essential.visible = false;
    }

    #[inline(always)]
    fn appear(&mut self) {
        self.drwob_essential.visible = true;
    }

    #[inline(always)]
    fn is_visible(&self) -> bool {
        self.drwob_essential.visible
    }

    #[inline(always)]
    fn set_drawing_depth(&mut self, depth: i8) {
        self.drwob_essential.drawing_depth = depth;
    }

    #[inline(always)]
    fn get_drawing_depth(&self) -> i8 {
        self.drwob_essential.drawing_depth
    }
}

impl Node2D for VerticalText {
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

impl Transform for VerticalText {
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
    }

    #[inline(always)]
    fn get_drawing_color(&self) -> ggraphics::Color {
        self.draw_param.color
    }

    #[inline(always)]
    fn set_alpha(&mut self, alpha: f32) {
        self.draw_param.color.a = alpha;
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
    fn get_texture_size(&self, _: &mut ggez::Context) -> numeric::Vector2f {
        let width = self.font_info.scale.x * ((self.get_indent_num() + 1) as f32);

        let mut max_chars_num = 0;
        let mut tmp_chars_num = 0;
        for text in &self.text {
            if text.contents().eq("\n") {
                if max_chars_num < tmp_chars_num {
                    max_chars_num = tmp_chars_num;
                }

                tmp_chars_num = 0;
                continue;
            }

            tmp_chars_num += 1;
        }

        if max_chars_num < tmp_chars_num {
            max_chars_num = tmp_chars_num;
        }

        let height = self.font_info.scale.y * (max_chars_num as f32);

        numeric::Vector2f::new(width as f32, height as f32)
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
