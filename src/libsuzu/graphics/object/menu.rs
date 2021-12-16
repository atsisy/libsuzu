use ggez::graphics as ggraphics;

use crate::libsuzu::graphics::object::*;

use super::sub_screen;
use super::sub_screen::SubScreen;

pub struct VerticalMenu {
    item_text: Vec<VerticalText>,
    canvas: SubScreen,
}

///
/// 縦書きの文字列列挙を行うための構造体
///
impl VerticalMenu {
    pub fn new(
        ctx: &mut ggez::Context,
        position: numeric::Point2f,
        padding: f32,
        item_text: Vec<String>,
        font_info: FontInformation,
    ) -> Self {
        // 縦書きメニューの表示エリア
        let canvas_area = numeric::Rect::new(
            position.x,
            position.y,
            (item_text.len() as f32 * font_info.scale.x) + padding,
            item_text.iter().fold(0.0, |max, text| {
                (text.len() as f32 * font_info.scale.y).max(max)
            }) + padding,
        );

        // StringからVerticalTextを生成
        let mut text_position = numeric::Point2f::new(
            canvas_area.w - (padding / 2.0) - font_info.scale.x,
            padding / 2.0,
        );
        let vertical_item_text = item_text
            .iter()
            .map(|raw_string| {
                let vtext = VerticalText::new(
                    raw_string.to_string(),
                    text_position,
                    numeric::Vector2f::new(1.0, 1.0),
                    0.0,
                    0,
                    font_info.clone(),
                );
                text_position.x -= font_info.scale.x;
                vtext
            })
            .collect();

        VerticalMenu {
            item_text: vertical_item_text,
            canvas: SubScreen::new(ctx, canvas_area, 0, ggraphics::Color::from_rgba_u32(0xff)),
        }
    }
}

impl Drawable for VerticalMenu {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.is_visible() {
            sub_screen::stack_screen(ctx, &self.canvas);

            for vtext in &mut self.item_text {
                vtext.draw(ctx)?;
            }

            sub_screen::pop_screen(ctx);
            self.canvas.draw(ctx)?;
        }

        Ok(())
    }

    fn hide(&mut self) {
        self.canvas.hide();
    }

    fn appear(&mut self) {
        self.canvas.appear();
    }

    fn is_visible(&self) -> bool {
        self.canvas.is_visible()
    }

    fn set_drawing_depth(&mut self, depth: i8) {
        self.canvas.set_drawing_depth(depth);
    }

    fn get_drawing_depth(&self) -> i8 {
        self.canvas.get_drawing_depth()
    }
}
