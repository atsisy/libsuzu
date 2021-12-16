use ggez::graphics as ggraphics;

use crate::libsuzu::numeric;

use crate::libsuzu::graphics::drawable::*;

use crate::{mintp, mintv, move_param, set_param_pos, get_param_pos};

///
/// Tile状に画像を切り取って表示する
///
#[derive(Clone)]
pub struct TileBatch {
    sprite_batch: ggraphics::spritebatch::SpriteBatch,
    image_size: numeric::Vector2u,
    tile_size: numeric::Vector2u,
    tile_size_ratio_float: numeric::Vector2f,
    drwob_essential: DrawableEss,
    draw_param: ggraphics::DrawParam,
}

impl TileBatch {
    pub fn new(
        image: ggraphics::Image,
        tile_size: numeric::Vector2u,
        pos: numeric::Point2f,
        draw_depth: i8,
    ) -> Self {
        let param = ggraphics::DrawParam::new().dest(mintp!(pos));

        let tile_size_ratio_float = numeric::Vector2f::new(
            tile_size.x as f32 / image.width() as f32,
            tile_size.y as f32 / image.height() as f32,
        );

	let size = numeric::Vector2u::new(image.width() as u32, image.height() as u32);

	let sprite_batch = ggraphics::spritebatch::SpriteBatch::new(image);
	//sprite_batch.set_filter(ggraphics::FilterMode::Nearest);

        TileBatch {
            image_size: size,
            sprite_batch: sprite_batch,
            tile_size: tile_size,
            tile_size_ratio_float: tile_size_ratio_float,
            drwob_essential: DrawableEss::new(true, draw_depth),
            draw_param: param,
        }
    }

    ///
    /// バッチ処理を追加するメソッド
    /// 位置指定には、比率を用いる
    ///
    pub fn add_batch_ratio_float(
        &mut self,
        tile_pos: numeric::Vector2f,
        dest_pos: numeric::Point2f,
        scale: numeric::Vector2f,
        color: ggraphics::Color,
    ) {
	let draw_param = ggraphics::DrawParam::default()
	    .src(numeric::Rect::new(
		tile_pos.x,
		tile_pos.y,
		self.tile_size_ratio_float.x,
                self.tile_size_ratio_float.y,
            ))
	    .scale(mintv!(scale))
	    .dest(mintp!(dest_pos))
	    .color(color);
        self.sprite_batch.add(draw_param);
    }

    ///
    /// バッチ処理を追加するメソッド
    /// 位置指定には、タイルポジションを用いる
    ///
    pub fn add_batch_tile_position(
        &mut self,
        tile_pos: numeric::Vector2u,
        dest_pos: numeric::Point2f,
        scale: numeric::Vector2f,
        color: ggraphics::Color,
    ) {
        // 比率表示の位置を計算
        let ratio_pos = numeric::Vector2f::new(
            (tile_pos.x * self.tile_size.x) as f32 / self.image_size.x as f32,
            (tile_pos.y * self.tile_size.y) as f32 / self.image_size.y as f32,
        );

        // 比率指定でバッチ処理を追加するメソッドを呼び出す
        self.add_batch_ratio_float(ratio_pos, dest_pos, scale, color);
    }

    ///
    /// 追加したバッチ処理をクリアするメソッド
    ///
    pub fn clear_batch(&mut self) {
        self.sprite_batch.clear()
    }

    pub fn get_tile_size(&self) -> numeric::Vector2u {
        self.tile_size
    }

    pub fn set_filter(&mut self, filter: ggraphics::FilterMode) {
	self.sprite_batch.set_filter(filter);
    }
}

impl Drawable for TileBatch {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.is_visible() {
            ggraphics::draw(ctx, &self.sprite_batch, self.draw_param).unwrap();
        }

        Ok(())
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

    fn set_drawing_depth(&mut self, depth: i8) {
        self.drwob_essential.drawing_depth = depth;
    }

    fn get_drawing_depth(&self) -> i8 {
        self.drwob_essential.drawing_depth
    }
}

impl Node2D for TileBatch {
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
