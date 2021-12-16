use ggez::graphics as ggraphics;

use super::numeric;

use crate::libsuzu::graphics::drawable::*;

pub struct ShadowShape {
    shadow: ggraphics::Mesh,
    draw_param: ggraphics::DrawParam,
    drwob_essential: DrawableEss,
}

impl ShadowShape {
    pub fn new(
        ctx: &mut ggez::Context,
        width: f32,
        bounds: numeric::Rect,
        color: ggraphics::Color,
        depth: i8,
    ) -> ShadowShape {
        let mesh = ggraphics::MeshBuilder::new()
            .rectangle(ggraphics::DrawMode::stroke(width), bounds, color)
            .expect("Failed to create rectangle")
            .build(ctx)
            .unwrap();

        let dparam = ggraphics::DrawParam::default();

        ShadowShape {
            shadow: mesh,
            draw_param: dparam,
            drwob_essential: DrawableEss::new(true, depth),
        }
    }
}

impl Drawable for ShadowShape {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.drwob_essential.visible {
            ggraphics::draw(ctx, &self.shadow, self.draw_param)?;
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
