use std::cmp::Ordering;

use ggez::input::mouse::MouseButton;

use crate::libsuzu::device::*;
use crate::libsuzu::numeric;

pub trait Drawable {
    /// このトレイトを実装する場合、このメソッドには描画を行う処理を記述する
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()>;

    /// このメソッドを呼び出した後は、
    /// drawメソッドを呼び出しても何も描画されなくなることを保証しなければならない
    /// appearメソッドが呼び出されれば、この効果は無効化されなければならない
    fn hide(&mut self);

    /// このメソッドを呼び出した後は、
    /// hideメソッドを呼び出していた場合でも、drawメソッドで描画できることを保証しなければならない
    /// hideメソッドが呼び出されれば、この効果は無効化されなければならない
    fn appear(&mut self);

    /// drawが有効ならtrue, そうでない場合はfalse
    fn is_visible(&self) -> bool;

    /// 描画順序を設定する
    fn set_drawing_depth(&mut self, depth: i8);

    /// 描画順序を返す
    fn get_drawing_depth(&self) -> i8;

    /// キー入力時の動作
    fn virtual_key_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _event_type: KeyboardEvent,
        _vkey: VirtualKey,
    ) {
        // Nothing
    }

    /// マウスイベント時の動作
    fn mouse_button_event(
        &mut self,
        _ctx: &mut ggez::Context,
        _event_type: MouseButtonEvent,
        _button: MouseButton,
        _point: numeric::Point2f,
    ) {
        // Nothing
    }
}

///
/// # 基本的な描画処理を保証させるトレイト
/// 汎用的なdrawインターフェイスを提供する
///
pub trait Node2D: Drawable {
    /// 描画開始地点を設定する
    fn set_position(&mut self, _pos: numeric::Point2f) {}

    /// 描画開始地点を返す
    fn get_position(&self) -> numeric::Point2f {
        numeric::Point2f::new(0.0, 0.0)
    }

    /// offsetで指定しただけ描画位置を動かす
    fn move_diff(&mut self, _offset: numeric::Vector2f) {}
}

///
/// DrawableObjectを深度（Z軸）でソートするための関数
///
/// この関数でソートすると、深度が深いものが先頭に来るようにソートされる
///
pub fn drawable_object_sort_with_depth<T, U>(a: &T, b: &U) -> Ordering
where
    T: Node2D,
    U: Node2D,
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
/// DrawableObjectを深度（Z軸）でソートするための関数
///
/// この関数でソートすると、深度が深いものが先頭に来るようにソートされる
///
pub fn boxed_drawable_object_sort_with_depth<T, U>(a: &Box<T>, b: &Box<U>) -> Ordering
where
    T: Node2D,
    U: Node2D,
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
/// # Trait DrawableObjectを実装するために必要なフィールド群
/// Trait DrawableObjectを実装する場合に便利な構造体
///
/// ## フィールド
/// ### visible
/// bool型
///
/// ### drawing_depth
/// i8型
///
#[derive(Clone)]
pub struct DrawableEss {
    pub visible: bool,
    pub drawing_depth: i8,
}

impl DrawableEss {
    // DrawableObjectEssentialの生成関数
    pub fn new(visible: bool, depth: i8) -> DrawableEss {
        DrawableEss {
            visible: visible,
            drawing_depth: depth,
        }
    }
}
