// use std::cell::RefCell;
// use std::collections::VecDeque;

// use ggez::graphics as ggraphics;

// use crate::graphics::drawable::*;
// use crate::graphics::object::sub_screen;
// use crate::graphics::object::sub_screen::SubScreen;
// use crate::graphics::object::*;
// use crate::numeric;

// struct DebugScreen {
//     font_info: FontInformation,
//     text_limit: usize,
//     text: VecDeque<UniText>,
//     screen: SubScreen,
//     rect: numeric::Rect,
// }

// impl DebugScreen {
//     fn new(ctx: &mut ggez::Context, rect: numeric::Rect, font_info: FontInformation) -> Self {
//         DebugScreen {
//             font_info: font_info,
//             text_limit: (rect.w / font_info.scale.y) as usize,
//             text: VecDeque::new(),
//             screen: SubScreen::new(ctx, rect, 0, ggraphics::Color::from_rgba_u32(0x000000a0)),
//             rect: rect,
//         }
//     }

//     fn set_back_color(&mut self, color: ggraphics::Color) {
//         self.screen.set_color(color);
//     }

//     fn set_text_limit(&mut self, limit: usize) {
//         self.text_limit = limit;
//     }

//     fn push_text(&mut self, text: &str) {
//         let text = UniText::new(
//             text.to_string(),
//             numeric::Point2f::new(10.0, self.rect.h - self.font_info.scale.y - 5.0),
//             numeric::Vector2f::new(1.0, 1.0),
//             0.0,
//             0,
//             self.font_info,
//         );

//         if self.text.len() > self.text_limit {
//             self.text.pop_front();
//         }

//         for text in &mut self.text {
//             text.move_diff(numeric::Vector2f::new(0.0, -self.font_info.scale.y));
//         }

//         self.text.push_back(text);
//     }

//     fn replace_screen(&mut self, ctx: &mut ggez::Context, rect: numeric::Rect) {
//         self.screen = SubScreen::new(ctx, rect, 0, ggraphics::Color::from_rgba_u32(0x000000a0));
//     }
// }

// impl DrawableComponent for DebugScreen {
//     fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
// 	if self.is_visible() {
//             sub_screen::stack_screen(ctx, &self.screen);
	    
//             for text in &mut self.text {
// 		text.draw(ctx)?;
//             }
	    
//             sub_screen::pop_screen(ctx);
//             self.screen.draw(ctx)?;
// 	}

// 	Ok(())
//     }

//     fn hide(&mut self) {
//         self.screen.hide();
//     }

//     fn appear(&mut self) {
//         self.screen.appear();
//     }

//     fn is_visible(&self) -> bool {
//         self.screen.is_visible()
//     }

//     fn set_drawing_depth(&mut self, depth: i8) {
//         self.screen.set_drawing_depth(depth);
//     }

//     fn get_drawing_depth(&self) -> i8 {
//         self.screen.get_drawing_depth()
//     }
// }

// thread_local!(static DEBUG_SCREEN: RefCell<Option<DebugScreen>> = {
//     RefCell::new(None)
// });

// pub fn debug_screen_hide() {
//     DEBUG_SCREEN.with(|screen| {
//         if let Some(screen) = screen.borrow_mut().as_mut() {
//             screen.hide();
//         }
//     });
// }

// pub fn debug_screen_appear() {
//     DEBUG_SCREEN.with(|screen| {
//         if let Some(screen) = screen.borrow_mut().as_mut() {
//             screen.appear();
//         }
//     });
// }

// pub fn debug_screen_init(ctx: &mut ggez::Context, rect: numeric::Rect, font_info: FontInformation) {
//     DEBUG_SCREEN.with(|screen| {
//         screen.replace_with(|_| Some(DebugScreen::new(ctx, rect, font_info)));
//     });
// }

// pub fn debug_screen_change_size(ctx: &mut ggez::Context, rect: numeric::Rect) {
//     DEBUG_SCREEN.with(|screen| {
//         if let Some(screen) = screen.borrow_mut().as_mut() {
//             screen.replace_screen(ctx, rect);
//         }
//     });
// }

// pub fn debug_screen_push_text(text: &str) {
//     DEBUG_SCREEN.with(|screen| {
//         if let Some(screen) = screen.borrow_mut().as_mut() {
//             screen.push_text(text);
//         }
//     });
// }

// pub fn debug_screen_change_color(color: ggraphics::Color) {
//     DEBUG_SCREEN.with(|screen| {
//         if let Some(screen) = screen.borrow_mut().as_mut() {
//             screen.set_back_color(color);
//         }
//     });
// }

// pub fn debug_screen_set_limit(limit: usize) {
//     DEBUG_SCREEN.with(|screen| {
//         if let Some(screen) = screen.borrow_mut().as_mut() {
//             screen.set_text_limit(limit);
//         }
//     });
// }

// pub fn debug_screen_draw(ctx: &mut ggez::Context) {
//     DEBUG_SCREEN.with(|screen| {
//         if let Some(screen) = screen.borrow_mut().as_mut() {
//             screen.draw(ctx).unwrap();
//         }
//     });
// }
