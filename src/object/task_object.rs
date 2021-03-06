pub mod factory;
pub mod tt_main_component;
pub mod tt_menu_component;
pub mod tt_sub_component;

use ggez::graphics as ggraphics;
use ggez::input as ginput;
use ginput::mouse::CursorIcon;

use crate::libsuzu::core::Clock;
use crate::libsuzu::device::VirtualKey;
use crate::libsuzu::graphics::drawable::*;
use crate::libsuzu::graphics::object::sub_screen;
use crate::libsuzu::graphics::object::sub_screen::SubScreen;
use crate::libsuzu::graphics::object::*;
use crate::impl_node2d_for_wrapped;
use crate::impl_transform_object_for_wrapped;
use crate::libsuzu::numeric;

use crate::add_delay_event;
use crate::core::util;
use crate::flush_delay_event;
use crate::flush_delay_event_and_redraw_check;
use crate::object::util_object::*;
use crate::object::{effect, move_fn};
use crate::scene::*;
use tt_main_component::*;
use tt_menu_component::*;
use tt_sub_component::*;

use super::{Clickable, DarkEffectPanel};
use crate::core::{
    BorrowingInformation, GensoDate, RentalLimit, ReturnBookInformation, SuzuContext, TextureID,
    TileBatchTextureID,
};

use number_to_jk::number_to_jk;

pub struct TaskTable {
    canvas: SubScreen,
    info_panel: TaskInfoPanel,
    sight: SuzuMiniSight,
    desk: DeskObjects,
    staging_object: Option<TaskTableStagingObject>,
    kosuzu_memory: KosuzuMemory,
    dark_effect_panel: DarkEffectPanel,
    shelving_box: ShelvingBookBox,
    event_list: DelayEventList<TaskTable>,
    borrowing_record_book: BorrowingRecordBook,
    record_book_is_staged: bool,
    manual_book_is_staged: bool,
    customer_silhouette_menu: CustomerMenuGroup,
    on_desk_menu: OnDeskMenuGroup,
    record_book_menu: RecordBookMenuGroup,
    current_customer_request: Option<CustomerRequest>,
    kosuzu_phrase: KosuzuPhrase,
    today: GensoDate,
    task_is_done: bool,
    return_late_checked: bool,
    appearance_frame: TileBatchFrame,
    current_page_book_condition_report: Option<BookConditionEvalReport>,
}

impl TaskTable {
    pub fn new<'a>(
        ctx: &mut SuzuContext<'a>,
        pos: numeric::Rect,
        info_panel_rect: numeric::Rect,
        sight_rect: numeric::Rect,
        desk_rect: numeric::Rect,
        shelving_box_rect: numeric::Rect,
        record_book_data: BorrowingRecordBookData,
        customer_request: Option<CustomerRequest>,
        t: Clock,
    ) -> Self {
        let sight = SuzuMiniSight::new(ctx, sight_rect, t);

        let mut desk = DeskObjects::new(ctx, desk_rect);

        let texture = Texture::new(
            ctx.ref_texture(TextureID::Chobo1),
            numeric::Point2f::new(0.0, 0.0),
            numeric::Vector2f::new(0.2, 0.2),
            0.0,
            -1,
        );

        let chobo_texture = Texture::new(
            ctx.ref_texture(TextureID::Chobo1),
            numeric::Point2f::new(0.0, 0.0),
            numeric::Vector2f::new(0.5, 0.5),
            0.0,
            -1,
        );
        let mut record_book = TaskTexture::new(
            OnDeskTexture::new(ctx.context, texture, OnDeskType::BorrowingRecordBook),
            OnDeskTexture::new(ctx.context, chobo_texture, OnDeskType::BorrowingRecordBook),
            0,
            true,
            true,
            DeskObjectType::BorrowRecordBook,
            t,
        );
        record_book.enable_large();
        desk.add_object(TaskItem::Texture(record_book));

        let texture = Texture::new(
            ctx.ref_texture(TextureID::Chobo1),
            numeric::Point2f::new(0.0, 0.0),
            numeric::Vector2f::new(0.2, 0.2),
            0.0,
            -1,
        );

        let chobo_texture = Texture::new(
            ctx.ref_texture(TextureID::Chobo1),
            numeric::Point2f::new(0.0, 0.0),
            numeric::Vector2f::new(0.5, 0.5),
            0.0,
            -1,
        );
        let mut manual_book = TaskTexture::new(
            OnDeskTexture::new(ctx.context, texture, OnDeskType::ManualBook),
            OnDeskTexture::new(ctx.context, chobo_texture, OnDeskType::ManualBook),
            0,
            true,
            true,
            DeskObjectType::ManualBook,
            t,
        );
        manual_book.enable_large();
        //desk.add_object(TaskItem::Texture(manual_book));

        let appr_frame = TileBatchFrame::new(
            ctx.resource,
            TileBatchTextureID::BlackFrame,
            numeric::Rect::new(300.0, 0.0, 1066.0, 768.0),
            numeric::Vector2f::new(1.0, 1.0),
            0,
        );

        let shelving_box = ShelvingBookBox::new(ctx, shelving_box_rect);

        let mut record_book = BorrowingRecordBook::new(
            ctx,
            numeric::Rect::new(250.0, -550.0, 1000.0, 550.0),
            0,
            record_book_data,
            t,
        );
        if record_book.pages_length() == 0 {
            record_book.add_empty_page(ctx, 0);
        }
        record_book.hide();

        TaskTable {
            canvas: SubScreen::new(
                ctx.context,
                pos,
                0,
                ggraphics::Color::from_rgba_u32(0x00000000),
            ),
            info_panel: TaskInfoPanel::new(ctx, info_panel_rect, customer_request),
            sight: sight,
            desk: desk,
            staging_object: None,
            kosuzu_memory: KosuzuMemory::new(),
            dark_effect_panel: DarkEffectPanel::new(
                ctx.context,
                numeric::Rect::new(0.0, 0.0, 1366.0, 768.0),
                0,
            ),
            shelving_box: shelving_box,
            event_list: DelayEventList::new(),
            borrowing_record_book: record_book,
            record_book_is_staged: false,
            manual_book_is_staged: false,
            customer_silhouette_menu: CustomerMenuGroup::new(0),
            on_desk_menu: OnDeskMenuGroup::new(0),
            record_book_menu: RecordBookMenuGroup::new(0),
            current_customer_request: None,
            kosuzu_phrase: KosuzuPhrase::new(ctx, 0),
            today: ctx.take_save_data().date,
            task_is_done: false,
            return_late_checked: false,
            appearance_frame: appr_frame,
            current_page_book_condition_report: None,
        }
    }

    fn some_full_screen_object_is_appeared(&self) -> bool {
        self.manual_book_is_staged || self.record_book_is_staged
    }

    pub fn get_kosuzu_memory(&self) -> &KosuzuMemory {
        &self.kosuzu_memory
    }

    fn select_dragging_object<'a>(&mut self, ctx: &mut SuzuContext<'a>, point: numeric::Point2f) {
        let rpoint = self.canvas.relative_point(point);

        // ??????????????????????????????????????????????????????????????????????????????????????????
        // ??????????????????????????????????????????
        if self.some_full_screen_object_is_appeared() {
            return ();
        }

        if self
            .record_book_menu
            .is_contains_any_menus(ctx.context, rpoint)
        {
            return ();
        }

        if self
            .customer_silhouette_menu
            .is_contains_any_menus(ctx.context, rpoint)
        {
            return ();
        }

        if self.on_desk_menu.is_contains_any_menus(ctx.context, rpoint) {
            return ();
        }

        self.desk.select_dragging_object(ctx, rpoint);
    }

    pub fn mouse_motion_handler<'a>(
        &mut self,
        _: &mut SuzuContext<'a>,
        point: numeric::Point2f,
        _: numeric::Vector2f,
    ) {
        self.borrowing_record_book.mouse_motion_handler(point);
    }

    fn slide_appear_record_book(&mut self, t: Clock) {
        self.borrowing_record_book.appear();

        self.borrowing_record_book.override_move_func(
            move_fn::devide_distance(numeric::Point2f::new(250.0, 100.0), 0.2),
            t,
        );
        self.dark_effect_panel.new_effect(8, t, 0, 200);
    }

    fn slide_hide_record_book(&mut self, t: Clock) {
        self.borrowing_record_book.override_move_func(
            move_fn::devide_distance(numeric::Point2f::new(250.0, -550.0), 0.2),
            t,
        );
        self.record_book_is_staged = false;

        self.record_book_menu.close_all(t);

        self.dark_effect_panel.new_effect(8, t, 200, 0);
    }
    
    fn try_open_borrowing_record_book<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        point: numeric::Point2f,
        t: Clock,
    ) -> bool {
        let rpoint = self.canvas.relative_point(point);

        let clicked_object_type = self.desk.check_clicked_desk_object_type(ctx, rpoint);

        if clicked_object_type.is_some() {
            match clicked_object_type.unwrap() {
                OnDeskType::BorrowingRecordBook => {
                    if !self.some_full_screen_object_is_appeared() {
                        self.slide_appear_record_book(t);
                        self.record_book_is_staged = true;
                        return true;
                    }
                }
                OnDeskType::ManualBook => {
                    if !self.some_full_screen_object_is_appeared() {
                        self.manual_book_is_staged = true;
                        return true;
                    }
                }
                _ => (),
            }
        }

        false
    }

    pub fn dragging_handler<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        point: numeric::Point2f,
        last: numeric::Point2f,
    ) {
        let rpoint = self.canvas.relative_point(point);
        let rlast = self.canvas.relative_point(last);

        self.sight.dragging_handler(ctx, rpoint, rlast);
        self.desk.dragging_handler(ctx, rpoint);
        self.shelving_box.dragging_handler(ctx, rpoint, rlast);
    }

    pub fn unselect_dragging_object<'a>(&mut self, ctx: &mut SuzuContext<'a>, t: Clock) {
        self.sight.unselect_dragging_object(ctx.context, t);
        self.desk.unselect_dragging_object(ctx);
        self.shelving_box.unselect_dragging_object(t);
    }

    pub fn hand_over_check<'a>(&mut self, ctx: &mut SuzuContext<'a>, point: numeric::Point2f) {
        let rpoint = self.canvas.relative_point(point);

        self.hand_over_check_d2s(ctx, rpoint);
        self.hand_over_check_s2d(ctx, rpoint);
        self.hand_over_check_desk2box(ctx, rpoint);
        self.hand_over_check_box2desk(ctx, rpoint);
    }

    fn apply_d2s_point_convertion(&mut self, ctx: &mut ggez::Context, obj: &mut TaskItem) {
        // ????????????????????????????????????
        let mut obj_p = obj.get_object().get_position();

        // obj_p???y????????????????????????????????????????????????????????????y????????????????????????
        // ??????????????????????????????????????????????????????????????????????????????????????????????????????????????????
        obj_p.x = obj.get_object().get_center(ctx).x;

        let p = self.desk_edge_to_sight_edge(ctx, obj_p);

        obj.enable_small();

        // Y?????????????????????????????????????????????Y?????????????????????????????????
        // make_center???????????????????????????????????????????????????????????????
        obj.get_object_mut().make_center(ctx, p);
    }

    fn apply_s2d_point_convertion(&mut self, ctx: &mut ggez::Context, obj: &mut TaskItem) {
        let mut obj_p = obj.get_object().get_position();
        obj_p.x = obj.get_object().get_center(ctx).x;
        let p = self.sight_edge_to_desk_edge(obj_p);
        obj.enable_large();
        obj.get_object_mut().make_center(ctx, p);
    }

    fn apply_desk2box_point_convertion(&mut self, ctx: &mut ggez::Context, obj: &mut TaskItem) {
        // ????????????????????????????????????
        let mut obj_p = obj.get_object().get_center(ctx);

        // Y?????????????????????, X?????????Canvas?????????????????????????????????
        obj_p.x = 0.0;

        obj.enable_small();

        obj.get_object_mut().make_center(ctx, obj_p);

        // ????????????????????????
        obj.get_object_mut().make_center(ctx, obj_p);
    }

    fn apply_box2desk_point_convertion(&mut self, ctx: &mut ggez::Context, obj: &mut TaskItem) {
        // ????????????????????????????????????
        let mut obj_p = obj.get_object().get_center(ctx);

        // Y?????????????????????, X?????????Canvas?????????????????????????????????
        obj_p.x = self.desk.canvas.get_drawing_size(ctx).x;

        obj.enable_large();

        obj.get_object_mut().make_center(ctx, obj_p);

        // ????????????????????????
        obj.get_object_mut().make_center(ctx, obj_p);
    }

    ///
    /// Desk??????MiniSight???????????????????????????????????????????????????
    ///
    fn hand_over_check_d2s<'a>(&mut self, ctx: &mut SuzuContext<'a>, rpoint: numeric::Point2f) {
        let border = self.desk_border(ctx.context);

        if self.desk.has_dragging() && border > rpoint.y {
            if let Some(mut dragging) = self.desk.release_dragging() {
		ctx.process_utility.redraw();
                self.apply_d2s_point_convertion(ctx.context, &mut dragging);
                self.sight.insert_dragging(dragging);
            }
        }
    }

    ///
    /// MiniSight??????Desk???????????????????????????????????????????????????
    ///
    fn hand_over_check_s2d<'a>(&mut self, ctx: &mut SuzuContext<'a>, rpoint: numeric::Point2f) {
        let border = self.desk_border(ctx.context);

        if self.sight.has_dragging() && border < rpoint.y {
            if let Some(mut dragging) = self.sight.release_dragging() {
		ctx.process_utility.redraw();
                self.apply_s2d_point_convertion(ctx.context, &mut dragging);
                self.desk.insert_dragging(dragging);
            }
        }
    }

    fn hand_over_check_desk2box<'a>(&mut self, ctx: &mut SuzuContext<'a>, rpoint: numeric::Point2f) {
        let border = self.desk_border_x(ctx.context);

        if self.desk.has_dragging() && border < rpoint.x {
            if self
                .desk
                .ref_dragging()
                .unwrap()
                .is_shelving_box_handover_locked()
            {
                return;
            }

            if let Some(mut dragging) = self.desk.release_dragging() {
		ctx.process_utility.redraw();
                self.apply_desk2box_point_convertion(ctx.context, &mut dragging);
                self.shelving_box.insert_dragging(dragging);
            }
        }
    }

    fn hand_over_check_box2desk<'a>(&mut self, ctx: &mut SuzuContext<'a>, rpoint: numeric::Point2f) {
        let border = self.desk_border_x(ctx.context);

        if self.shelving_box.has_dragging() && border >= rpoint.x {
            if let Some(mut dragging) = self.shelving_box.release_dragging() {
		ctx.process_utility.redraw();
                self.apply_box2desk_point_convertion(ctx.context, &mut dragging);
                self.desk.insert_dragging(dragging);
            }
        }
    }

    fn desk_border(&mut self, ctx: &mut ggez::Context) -> f32 {
        let sight_edge =
            self.sight.canvas.get_position().y + self.sight.canvas.get_texture_size(ctx).y;
        let diff = (sight_edge - self.desk.canvas.get_position().y).abs();
        sight_edge + diff
    }

    fn desk_border_x(&mut self, ctx: &mut ggez::Context) -> f32 {
        let desk_edge =
            self.desk.canvas.get_position().x + self.desk.canvas.get_texture_size(ctx).x;
        let diff = (desk_edge - self.shelving_box.canvas.get_position().x).abs();
        desk_edge + diff
    }

    fn desk_edge_to_sight_edge(
        &mut self,
        ctx: &mut ggez::Context,
        point: numeric::Point2f,
    ) -> numeric::Point2f {
        numeric::Point2f::new(point.x, self.sight.canvas.get_texture_size(ctx).y)
    }

    fn sight_edge_to_desk_edge(&mut self, rpoint: numeric::Point2f) -> numeric::Point2f {
        numeric::Point2f::new(rpoint.x, 0.0)
    }

    ///
    /// # ?????????????????????
    ///
    fn check_sight_drop_to_desk<'a>(&mut self, ctx: &mut SuzuContext<'a>, t: Clock) {
        let converted = self.sight.check_drop_desk();
        if converted.len() == 0 {
            return ();
        }

        // converted?????????????????????????????????????????????????????????
        // converted????????????Desk??????????????????????????????????????????
        ctx.process_utility.redraw();

        let min = self.desk.desk_objects.get_minimum_depth();
        let converted = converted
            .into_iter()
            .map(|mut obj| {
                self.apply_s2d_point_convertion(ctx.context, &mut obj);
                obj.as_effectable_object().clear_effect();
                obj.as_movable_object_mut()
                    .override_move_func(move_fn::gravity_move(1.0, 10.0, 400.0, 0.4), t);
                obj.get_object_mut().set_drawing_depth(min);
                obj.get_object_mut().finish_dragging(ctx);

                obj.as_effectable_object().add_effect(vec![Box::new(
                    |obj: &mut dyn MovableObject, _: &ggez::Context, t: Clock| {
                        if obj.get_position().y > 150.0 {
                            obj.override_move_func(None, t);
                            EffectFnStatus::EffectFinish
                        } else {
                            EffectFnStatus::EffectContinue
                        }
                    },
                )]);
                obj
            })
            .collect::<Vec<_>>();
        self.desk.desk_objects.change_depth_equally(1);
        self.desk.add_customer_object_vec(converted);
    }

    ///
    /// # ?????????????????????
    ///
    pub fn update<'a>(&mut self, ctx: &mut SuzuContext<'a>, t: Clock) {
        flush_delay_event_and_redraw_check!(self, self.event_list, ctx, t, {});

        self.sight.update(ctx, t);
        self.desk.update(ctx, t);
        self.shelving_box.update(ctx, t);
        self.check_sight_drop_to_desk(ctx, t);
        self.borrowing_record_book.update(ctx, t);
        self.record_book_menu.update(ctx, t);
        self.customer_silhouette_menu.update(ctx, t);
        self.on_desk_menu.update(ctx, t);
        self.kosuzu_phrase.update(ctx, t);
        self.info_panel.update(ctx, t);
        self.check_task_is_done(ctx);

        self.dark_effect_panel.run_effect(ctx, t);
    }

    pub fn finish_customer_event(&mut self, now: Clock) {
        self.sight.finish_customer_event(now);
    }

    pub fn task_is_done(&self) -> bool {
        self.task_is_done
    }

    fn start_borrowing_customer_event<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        info: BorrowingInformation,
        t: Clock,
    ) {
        let mut position = numeric::Point2f::new(0.0, 0.0);

        for book_info in &info.borrowing {
            let mut obj = factory::create_dobj_book(
                ctx,
                DeskObjectType::CustomerObject,
                position,
                book_info.clone(),
                t,
            );
            obj.enable_large();
            self.desk.add_customer_object(obj);

            position.x += 20.0;
        }

        let mut new_silhouette = SimpleObject::new(
            MovableTexture::new(
                Box::new(Texture::new(
                    ctx.ref_texture(TextureID::Mob1TachieDefault),
                    numeric::Point2f::new(130.0, 15.0),
                    numeric::Vector2f::new(0.18, 0.18),
                    0.0,
                    0,
                )),
                None,
                t,
            ),
            vec![
                effect::appear_bale_down_from_top(50, t),
                effect::fade_in(50, t),
            ],
        );
        new_silhouette.set_alpha(0.0);
        self.sight.silhouette_new_customer_update(
            ctx.context,
            new_silhouette,
            info.borrower.to_string(),
            CustomerDialogue::new(
                vec!["???????????????".to_string(), "??????????????????????????????".to_string()],
                vec![100, 100],
            ),
            t,
        );
    }

    fn start_returning_customer_event<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        info: ReturnBookInformation,
        t: Clock,
    ) {
        let mut position = numeric::Point2f::new(0.0, 0.0);

        for book_info in &info.returning {
            let mut obj = factory::create_dobj_book(
                ctx,
                DeskObjectType::CustomerObject,
                position,
                book_info.clone(),
                t,
            );
            obj.enable_large();
            self.desk.add_customer_object(obj);

            position.x += 20.0;
        }

        let mut new_silhouette = SimpleObject::new(
            MovableTexture::new(
                Box::new(Texture::new(
                    ctx.ref_texture(TextureID::Mob1TachieDefault),
                    numeric::Point2f::new(100.0, 20.0),
                    numeric::Vector2f::new(0.18, 0.18),
                    0.0,
                    0,
                )),
                None,
                t,
            ),
            vec![
                effect::appear_bale_down_from_top(50, t),
                effect::fade_in(50, t),
            ],
        );
        new_silhouette.set_alpha(0.0);
        self.sight.silhouette_new_customer_update(
            ctx.context,
            new_silhouette,
            info.borrower.to_string(),
            CustomerDialogue::new(
                vec!["???????????????".to_string(), "??????????????????????????????".to_string()],
                vec![100, 100],
            ),
            t,
        );
    }

    pub fn start_customer_event(&mut self, ctx: &mut SuzuContext, info: CustomerRequest, t: Clock) {
        self.current_customer_request = Some(info.clone());

        match info {
            CustomerRequest::Borrowing(info) => self.start_borrowing_customer_event(ctx, info, t),
            CustomerRequest::Returning(info) => self.start_returning_customer_event(ctx, info, t),
        }
    }

    pub fn add_fee_coins<'a>(&mut self, ctx: &mut SuzuContext<'a>, price: u32, t: Clock) {
        let coins = factory::create_coins(ctx, price, t);

        for mut coin in coins {
            coin.get_object_mut()
                .set_position(util::random_point_in_rect(numeric::Rect::new(
                    10.0, 10.0, 100.0, 100.0,
                )));
            self.desk.add_object(coin);
        }
    }

    fn check_borrowing_task_is_done(&self) -> bool {
        let mut item_counts = 0;
        for obj in self.desk.desk_objects.get_raw_container().iter() {
            match obj {
                TaskItem::Book(_) => {
                    item_counts += 1;
                }
                TaskItem::Coin(_) => item_counts += 1,
                _ => (),
            }
        }

        item_counts += self
            .sight
            .count_not_forbidden_book_items(&self.kosuzu_memory);

        if let Some(dragging) = self.desk.dragging.as_ref() {
            match dragging {
                TaskItem::Book(_) => {
                    item_counts += 1;
                }
                _ => (),
            }
        }

        item_counts == 0
    }

    fn check_returning_task_is_done(&self) -> bool {
        let mut book_count = 0;
        for obj in self.desk.desk_objects.get_raw_container().iter() {
            match obj {
                TaskItem::Book(_) => {
                    book_count += 1;
                }
                _ => (),
            }
        }

        book_count += self
            .sight
            .count_not_forbidden_book_items(&self.kosuzu_memory);

        if let Some(dragging) = self.desk.dragging.as_ref() {
            match dragging {
                TaskItem::Book(_) => {
                    book_count += 1;
                }
                _ => (),
            }
        }

        book_count == 0
    }

    ///
    /// # ?????????????????????
    ///
    fn check_task_is_done<'a>(&mut self, ctx: &mut SuzuContext<'a>) {
        if self.current_customer_request.is_none() {
            self.task_is_done = false;
            return;
        }

        self.task_is_done = match self.current_customer_request.as_ref().unwrap() {
            CustomerRequest::Borrowing(_) => self.check_borrowing_task_is_done(),
            CustomerRequest::Returning(_) => self.check_returning_task_is_done(),
        };

        if self.task_is_done {
            ctx.process_utility.redraw();
        }
    }

    pub fn get_shelving_box(&self) -> &ShelvingBookBox {
        &self.shelving_box
    }

    pub fn get_shelving_box_mut(&mut self) -> &mut ShelvingBookBox {
        &mut self.shelving_box
    }

    /// ??????????????????
    pub fn key_event_handler<'a>(
        &mut self,
        _ctx: &mut SuzuContext<'a>,
        vkey: VirtualKey,
        t: Clock,
    ) {
        match vkey {
            VirtualKey::Action3 => {
                if self.staging_object.is_some() {
                    self.event_list.add_event(
                        Box::new(|tt: &mut Self, _, _| tt.staging_object = None),
                        t + 100,
                    );
                }

                self.slide_hide_record_book(t);
            }
            _ => (),
        }
    }

    pub fn export_borrowing_record_book_data(&self) -> BorrowingRecordBookData {
        self.borrowing_record_book.export_book_data()
    }

    pub fn signing_borrowing_handler<'a>(&mut self, ctx: &mut SuzuContext<'a>, t: Clock) {
        let price = self.borrowing_record_book.get_calculated_price().unwrap();
        self.event_list.add_event(
            Box::new(move |slf: &mut Self, _, t| {
                slf.slide_hide_record_book(t);
            }),
            t + 30,
        );
        self.show_kosuzu_payment_message(ctx, price, t);
        self.add_fee_coins(ctx, price, t);

        // ???????????????????????????????????????????????????
        // ???????????????????????????????????????????????????????????????
        let written_books = self
            .borrowing_record_book
            .get_current_page_written_books()
            .unwrap();

        for item in self.desk.desk_objects.get_raw_container_mut().iter_mut() {
            match item {
                TaskItem::Book(book) => {
                    let info = book.get_large_object_mut().get_book_info();

                    if written_books.contains(&info) {
                        book.unlock_handover();
                    }
                }
                _ => (),
            }
        }

        self.sight.silhouette.insert_new_balloon_phrase(
            "?????????".to_string(),
            TextBalloonPhraseType::SimplePhrase,
            20,
            t,
        );

        for book in written_books {
            self.kosuzu_memory.add_book_to_written_list(book);
        }
    }

    pub fn signing_returning_handler<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        t: Clock,
    ) -> Option<BookConditionEvalReport> {
        self.event_list.add_event(
            Box::new(move |slf: &mut Self, _, t| {
                slf.slide_hide_record_book(t);
            }),
            t + 30,
        );

        self.show_kosuzu_returning_is_done_message(ctx, t);

        // ???????????????????????????????????????????????????
        // ???????????????????????????????????????????????????????????????
        let written_books = self
            .borrowing_record_book
            .get_current_page_written_books()
            .unwrap();
        for item in self.desk.desk_objects.get_raw_container_mut().iter_mut() {
            match item {
                TaskItem::Book(book) => {
                    let info = book.get_large_object_mut().get_book_info();

                    if written_books.contains(&info) {
                        book.unlock_shelving_box_handover();
                    }
                }
                _ => (),
            }
        }

        self.borrowing_record_book
            .get_current_page_condition_eval_report()
    }

    ///
    /// ?????????????????????????????????????????????????????????true?????????????????????????????????false?????????
    ///
    fn click_record_book_menu<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        button: ggez::input::mouse::MouseButton,
        point: numeric::Point2f,
        t: Clock,
    ) -> bool {
        if !self
            .record_book_menu
            .click_book_status_menu(ctx, button, point, t)
            && !self
                .record_book_menu
                .click_book_title_menu(ctx, button, point, t)
            && !self.record_book_menu.click_date_menu(ctx, button, point, t)
            && !self
                .record_book_menu
                .click_customer_name_menu(ctx, button, point, t)
            && !self
                .record_book_menu
                .click_date_check_menu(ctx, button, point, t)
        {
            // ???????????????????????????????????????????????????false???????????????????????????
            return false;
        }

        if self.record_book_menu.date_check_menu_check_button_clicked() {
            if let Some(return_date) = self.borrowing_record_book.get_current_page_return_date() {
                if self.today.is_past(&return_date) {
                    if self.return_late_checked {
                        self.insert_kosuzu_message_set(ctx, "????????????????????????????????????", t);
                    } else {
                        self.add_fee_coins(ctx, 300, t);
                        self.slide_hide_record_book(t);
                        self.return_late_checked = true;
                        self.insert_kosuzu_message_set(
                            ctx,
                            "??????????????????\n??????????????????????????????",
                            t,
                        );
			ctx.take_save_data_mut().task_result.total_money += 300;

                    }
                } else {
                    self.insert_kosuzu_message_set(ctx, "????????????????????????", t);
                }
            }
        }

        if let Some(index) = self.record_book_menu.book_status_menu_last_clicked() {
            let menu_position = self
                .record_book_menu
                .get_book_status_menu_position()
                .unwrap();
            if index <= 2 {
                // ???, ???, ???
                self.borrowing_record_book.insert_book_status_via_choice(
                    ctx.context,
                    index,
                    menu_position,
                );
            } else {
                // ??????
                self.borrowing_record_book
                    .remove_book_status_at(ctx.context, menu_position);
            }

            return true;
        }

        if let Some((index, book_info)) = self.record_book_menu.book_title_menu_last_clicked() {
            let menu_position = self
                .record_book_menu
                .get_book_title_menu_position()
                .unwrap();
            self.borrowing_record_book.insert_book_title_to_books_frame(
                ctx,
                menu_position,
                book_info,
            );

            self.kosuzu_memory.remove_book_info_at(index);

            return true;
        }

        if let Some((_, date)) = self.record_book_menu.date_menu_last_clicked() {
            let menu_position = self.record_book_menu.get_date_menu_position().unwrap();
            let maybe_rental_limit = self.today.rental_limit_type(&date);
            if let Some(rental_limit) = maybe_rental_limit {
                self.borrowing_record_book
                    .insert_date_data_to_customer_info(ctx, menu_position, date, rental_limit);
            }

            return true;
        }

        if let Some((_, name)) = self.record_book_menu.customer_name_menu_last_clicked() {
            let menu_position = self
                .record_book_menu
                .get_customer_name_menu_position()
                .unwrap();
            self.borrowing_record_book
                .insert_customer_name_data_to_customer_info(ctx.context, menu_position, name);

            return true;
        }

        false
    }

    ///
    /// ???????????????????????????sight???text_balloon??????????????????
    ///
    fn insert_custmer_name_phrase<'a>(&mut self, ctx: &mut SuzuContext<'a>, t: Clock) {
        if let Some(customer_request) = self.current_customer_request.as_ref() {
            let customer_name = customer_request.get_customer_name();
            let phrase_text = format!("{}??????", customer_name.to_string());
            self.sight
                .silhouette
                .insert_kosuzu_message_in_chatbox(ctx, "???????????????".to_string());

            self.sight.silhouette.insert_new_balloon_phrase(
                phrase_text.clone(),
                TextBalloonPhraseType::CustomerName(customer_request.get_customer_name().clone()),
                20,
                t,
            );

            add_delay_event!(
                self.event_list,
                move |slf, ctx, _t| {
                    slf.sight
                        .silhouette
                        .set_partner_name_to_chatbox(customer_name);
                    slf.sight
                        .silhouette
                        .insert_customer_message_in_chatbox(ctx, phrase_text);
                },
                30
            );
        }
    }

    ///
    /// ???????????????????????????sight???text_balloon??????????????????
    ///
    fn insert_rental_limit_phrase<'a>(&mut self, ctx: &mut SuzuContext<'a>, t: Clock) {
        if let Some(customer_request) = self.current_customer_request.as_ref() {
            self.sight
                .silhouette
                .insert_kosuzu_message_in_chatbox(ctx, "???????????????????????????????????????".to_string());
            match customer_request {
                CustomerRequest::Borrowing(info) => {
                    let phrase_text = match info.rental_limit {
                        RentalLimit::ShortTerm => "?????????????????????????????????",
                        RentalLimit::LongTerm => "?????????????????????????????????",
                        _ => "",
                    }
                    .to_string();

                    self.sight.silhouette.insert_new_balloon_phrase(
                        phrase_text.clone(),
                        TextBalloonPhraseType::RentalLimit(info.rental_limit.clone()),
                        20,
                        t,
                    );

                    self.sight
                        .silhouette
                        .insert_customer_message_in_chatbox(ctx, phrase_text);
                    self.info_panel
                        .set_rental_limit(ctx, info.rental_limit.clone());
                }
                CustomerRequest::Returning(info) => {
                    let rental_limit = info.get_rental_limit();
                    let phrase_text = match &rental_limit {
                        RentalLimit::ShortTerm => {
                            format!("{}???\n????????????????????????", info.borrow_date.to_short_string())
                        }
                        RentalLimit::LongTerm => {
                            format!("{}???\n????????????????????????", info.borrow_date.to_short_string())
                        }
                        _ => "".to_string(),
                    };

                    self.sight.silhouette.insert_new_balloon_phrase(
                        phrase_text.clone(),
                        TextBalloonPhraseType::RentalLimit(rental_limit.clone()),
                        20,
                        t,
                    );

                    self.sight
                        .silhouette
                        .insert_customer_message_in_chatbox(ctx, phrase_text);
                    self.info_panel.set_rental_limit(ctx, rental_limit);
                }
            }
        }
    }

    fn show_kosuzu_payment_message<'a>(&mut self, ctx: &mut SuzuContext<'a>, price: u32, t: Clock) {
        let msg = format!("??????{}??????????????????", number_to_jk(price as u64));
        self.kosuzu_phrase.insert_new_phrase(ctx, &msg, t);

        self.sight
            .silhouette
            .insert_kosuzu_message_in_chatbox(ctx, msg);
    }

    fn show_kosuzu_returning_is_done_message<'a>(&mut self, ctx: &mut SuzuContext<'a>, t: Clock) {
        let msg = "?????????????????? ???????????????????????????";
        self.kosuzu_phrase.insert_new_phrase(ctx, msg, t);

        self.sight
            .silhouette
            .insert_kosuzu_message_in_chatbox(ctx, msg.to_string());
    }

    fn insert_kosuzu_message_set<'a>(&mut self, ctx: &mut SuzuContext<'a>, msg: &str, t: Clock) {
        self.kosuzu_phrase.insert_new_phrase(ctx, msg, t);

        self.sight
            .silhouette
            .insert_kosuzu_message_in_chatbox(ctx, msg.to_string());
    }

    ///
    /// ?????????????????????????????????????????????????????????true?????????????????????????????????false?????????
    ///
    fn click_customer_silhouette_menu<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        button: ggez::input::mouse::MouseButton,
        point: numeric::Point2f,
        t: Clock,
    ) -> bool {
        if !self
            .customer_silhouette_menu
            .click_customer_question_menu(ctx, button, point, t)
        {
            // ???????????????????????????????????????????????????false???????????????????????????
            return false;
        }

        if let Some(index) = self
            .customer_silhouette_menu
            .question_menu_last_clicked_index()
        {
            match index {
                0 => {
                    if let Some(customer_request) = self.current_customer_request.as_ref() {
                        let name = customer_request.get_customer_name();
                        self.kosuzu_memory.add_customer_name(name.clone());
                        self.info_panel.set_customer_name(ctx, name);
                        self.insert_custmer_name_phrase(ctx, t);
                    }
                }
                1 => self.insert_rental_limit_phrase(ctx, t),
                _ => panic!("Exception"),
            }

            return true;
        }

        false
    }

    ///
    /// ?????????????????????????????????????????????????????????true?????????????????????????????????false?????????
    ///
    fn click_desk_book_menu<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        button: ggez::input::mouse::MouseButton,
        point: numeric::Point2f,
        t: Clock,
    ) -> bool {
        if !self
            .on_desk_menu
            .click_desk_book_menu(ctx, button, point, t)
        {
            // ???????????????????????????????????????????????????false???????????????????????????
            return false;
        }

        if let Some(index) = self.on_desk_menu.desk_book_menu_last_clicked() {
            if let Some(book_info) = self.on_desk_menu.get_desk_menu_target_book_info() {
                return match index {
                    //
                    // ?????????????????????
                    //
                    0 => {
                        if self.kosuzu_memory.is_written_in_record(&book_info) {
                            self.kosuzu_phrase.insert_new_phrase(
                                ctx,
                                "????????????????????????????????????",
                                t,
                            );
                        } else {
                            // info panel
                            self.info_panel
                                .add_book_info(ctx, book_info.clone(), point, t);

                            // internal memory
                            self.kosuzu_memory.add_book_info(book_info);
                        }
                        true
                    }
                    _ => false,
                };
            }
        }

        false
    }

    ///
    /// book_info_frame???????????????????????????????????????
    ///
    /// book_info_frame??????????????????????????????true, ????????????????????????false
    ///
    fn try_show_menus_regarding_book_info<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        click_point: numeric::Point2f,
        lock_status: &RecordBookLockStatus,
        t: Clock,
    ) -> bool {
        let grid_pos = self
            .borrowing_record_book
            .get_book_info_frame_grid_position(ctx.context, click_point);

        let books_table_rows = self.borrowing_record_book.get_books_table_rows();
        let books_table_rows = match books_table_rows {
            Some(it) => it,
            None => return false,
        };

        if grid_pos.is_some() {
            if grid_pos.unwrap().x == books_table_rows as u32 - 1 {
                return false;
            }

            match grid_pos.unwrap().y {
                0 => {
                    if let Some(request) = self.current_customer_request.as_ref() {
                        match request {
                            CustomerRequest::Borrowing(_) => match lock_status {
                                RecordBookLockStatus::BorrowingOk => {
                                    self.record_book_menu.show_book_title_menu(
                                        ctx,
                                        click_point,
                                        &self.kosuzu_memory,
                                        t,
                                    );
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                }
                1 => {
                    if let Some(request) = self.current_customer_request.as_ref() {
                        match request {
                            CustomerRequest::Returning(_) => match lock_status {
                                RecordBookLockStatus::ReturningMatched => {
                                    if self
                                        .borrowing_record_book
                                        .is_current_books_table_some_data_at(
                                            numeric::Vector2u::new(grid_pos.unwrap().x, 0),
                                        )
                                    {
                                        self.record_book_menu.show_book_status_menu(
                                            ctx,
                                            click_point,
                                            t,
                                        );
                                    }
                                }
                                RecordBookLockStatus::ReturningVary => {
                                    self.record_book_menu.show_locked_menu(ctx, click_point, t);
                                }
                                _ => (),
                            },
                            _ => (),
                        }
                    }
                }
                _ => (),
            }

            true
        } else {
            false
        }
    }

    ///
    /// customer_info_frame???????????????????????????????????????
    ///
    /// customer_info_frame??????????????????????????????true, ????????????????????????false
    ///
    fn try_show_menus_regarding_customer_info<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        click_point: numeric::Point2f,
        lock_status: &RecordBookLockStatus,
        t: Clock,
    ) -> bool {
        let maybe_grid_pos = self
            .borrowing_record_book
            .get_customer_info_frame_grid_position(ctx.context, click_point);

        if let Some(grid_pos) = maybe_grid_pos {
            if grid_pos == numeric::Vector2u::new(2, 1) {
                if let Some(request) = self.current_customer_request.as_ref() {
                    match request {
                        CustomerRequest::Borrowing(_) => {
                            if self
                                .borrowing_record_book
                                .current_page_is_borrowing_signed()
                                != RecordBookLockStatus::BorrowingLocked
                            {
                                self.record_book_menu.show_customer_name_menu(
                                    ctx,
                                    click_point,
                                    &self.kosuzu_memory,
                                    t,
                                );
                            }
                        }
                        _ => (),
                    }
                }
            } else if grid_pos == numeric::Vector2u::new(1, 1)
                || grid_pos == numeric::Vector2u::new(0, 1)
            {
                if let Some(request) = self.current_customer_request.as_ref() {
                    match request {
                        CustomerRequest::Borrowing(_) => {
                            if self
                                .borrowing_record_book
                                .current_page_is_borrowing_signed()
                                != RecordBookLockStatus::BorrowingLocked
                            {
                                self.record_book_menu.show_date_menu(
                                    ctx,
                                    click_point,
                                    self.today.clone(),
                                    t,
                                );
                            }
                        }
                        CustomerRequest::Returning(_) => match lock_status {
                            RecordBookLockStatus::ReturningMatched => {
                                if grid_pos.x == 0 {
                                    let return_date =
                                        self.borrowing_record_book.get_current_page_return_date();
                                    if let Some(return_date) = return_date {
                                        self.record_book_menu.show_date_check_menu(
                                            ctx,
                                            click_point,
                                            self.today.clone(),
                                            return_date,
                                            t,
                                        );
                                    }
                                }
                            }
                            _ => (),
                        },
                    }
                }
            }

            true
        } else {
            false
        }
    }

    ///
    /// ??????????????????????????????????????????????????????
    ///
    /// customer_menu??????????????????????????????true, ????????????????????????false
    ///
    fn try_show_menus_regarding_customer_silhoutte(
        &mut self,
        ctx: &mut SuzuContext,
        click_point: numeric::Point2f,
        t: Clock,
    ) -> bool {
        let rpoint = self.sight.canvas.relative_point(click_point);

        if self
            .sight
            .silhouette
            .contains_character_silhouette(ctx.context, rpoint)
        {
            self.customer_silhouette_menu
                .show_customer_question_menu(ctx, click_point, t);
            true
        } else {
            false
        }
    }

    ///
    /// ??????????????????????????????????????????????????????
    ///
    /// customer_menu??????????????????????????????true, ????????????????????????false
    ///
    fn try_show_menus_regarding_ondesk_book_info<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        click_point: numeric::Point2f,
        t: Clock,
    ) -> bool {
        let rpoint = self.desk.canvas.relative_point(click_point);

        for dobj in self.desk.get_desk_objects_list().iter().rev() {
            if dobj.get_object().contains(ctx.context, rpoint) {
                let dobj_ref = &dobj.get_object();
                let obj_type = dobj_ref.get_type();
                let hold_data = dobj_ref.click_hold_data(ctx.context, rpoint);

                match obj_type {
                    OnDeskType::Book => match hold_data {
                        HoldData::BookName(info) => {
                            self.on_desk_menu.show_desk_book_menu(
                                ctx,
                                click_point,
                                info.clone(),
                                t,
                            );
                        }
                        _ => panic!(""),
                    },
                    _ => (),
                }

                return true;
            }
        }

        false
    }

    pub fn get_target_page_book_condition_eval_report(&self) -> Option<&BookConditionEvalReport> {
        self.current_page_book_condition_report.as_ref()
    }

    fn try_show_menus<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        click_point: numeric::Point2f,
        t: Clock,
    ) {
        // ?????????????????????????????????????????????????????????????????????
        if self.record_book_menu.is_some_menu_opened() {
            self.record_book_menu.close_all(t);
            return ();
        }

        // ?????????????????????????????????????????????????????????????????????
        if self.customer_silhouette_menu.is_some_menu_opened() {
            self.customer_silhouette_menu.close_all(t);
            return ();
        }

        if self.on_desk_menu.is_some_menu_opened() {
            self.on_desk_menu.close_all(t);
            return ();
        }

        if self.current_customer_request.is_some() {
            let record_book_lock_status = match self.current_customer_request.as_ref().unwrap() {
                CustomerRequest::Borrowing(_) => self
                    .borrowing_record_book
                    .current_page_is_borrowing_signed(),
                CustomerRequest::Returning(_) => self
                    .borrowing_record_book
                    .check_current_page_is_matched_with(self.current_customer_request.as_ref()),
            };

            if self.try_show_menus_regarding_book_info(
                ctx,
                click_point,
                &record_book_lock_status,
                t,
            ) {
                return;
            }

            if self.try_show_menus_regarding_customer_info(
                ctx,
                click_point,
                &record_book_lock_status,
                t,
            ) {
                return;
            }
        }

        if self
            .borrowing_record_book
            .contains(ctx.context, click_point)
        {
            return;
        } else {
            if self.record_book_is_staged {
                self.slide_hide_record_book(t);
                return;
            }
        }

        // BorrowingRecordBook????????????????????????????????????
        // OnDeskBook??????????????????????????????????????????????????????
        if !self.try_show_menus_regarding_customer_silhoutte(ctx, click_point, t) {
            self.try_show_menus_regarding_ondesk_book_info(ctx, click_point, t);
        }
    }

    pub fn mouse_wheel_event<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        point: numeric::Point2f,
        x: f32,
        y: f32,
    ) {
        let rpoint = self.canvas.relative_point(point);
        self.sight.mouse_wheel_event(ctx, rpoint, x, y);
    }
}

impl Drawable for TaskTable {
    fn draw(&mut self, ctx: &mut ggez::Context) -> ggez::GameResult<()> {
        if self.is_visible() {
            sub_screen::stack_screen(ctx, &self.canvas);

            self.info_panel.draw(ctx).unwrap();
            self.sight.draw(ctx).unwrap();
            self.desk.draw(ctx).unwrap();
            self.shelving_box.draw(ctx).unwrap();

            if let Some(staging_object) = self.staging_object.as_mut() {
                staging_object.draw(ctx)?;
            }

            self.dark_effect_panel.draw(ctx).unwrap();

            //self.manual_book.draw(ctx)?;

            self.appearance_frame.draw(ctx)?;

            self.borrowing_record_book.draw(ctx)?;
            self.kosuzu_phrase.draw(ctx)?;
            self.customer_silhouette_menu.draw(ctx)?;
            self.record_book_menu.draw(ctx)?;
            self.on_desk_menu.draw(ctx)?;

            sub_screen::pop_screen(ctx);
            self.canvas.draw(ctx).unwrap();
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

    /// ???????????????????????????
    fn set_drawing_depth(&mut self, depth: i8) {
        self.canvas.set_drawing_depth(depth);
    }

    /// ?????????????????????
    fn get_drawing_depth(&self) -> i8 {
        self.canvas.get_drawing_depth()
    }
}

impl Node2D for TaskTable {
    impl_node2d_for_wrapped! {canvas}
}

impl Transform for TaskTable {
    impl_transform_object_for_wrapped! {canvas}
}

impl Clickable for TaskTable {
    fn button_down<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        _: Clock,
        _: ggez::input::mouse::MouseButton,
        point: numeric::Point2f,
    ) {
        self.select_dragging_object(ctx, point);
    }

    fn button_up<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        t: Clock,
        _: ggez::input::mouse::MouseButton,
        point: numeric::Point2f,
    ) {
        let rpoint = self.canvas.relative_point(point);

        // ??????????????????????????????????????????????????????????????????????????????????????????????????????
        if !self
            .record_book_menu
            .is_contains_any_menus(ctx.context, rpoint)
        {
            self.record_book_menu.close_all(t);
        }

        if !self
            .customer_silhouette_menu
            .is_contains_any_menus(ctx.context, rpoint)
        {
            self.customer_silhouette_menu.close_all(t);
        }

        if !self.on_desk_menu.is_contains_any_menus(ctx.context, rpoint) {
            self.on_desk_menu.close_all(t);
        }
    }

    fn on_click<'a>(
        &mut self,
        ctx: &mut SuzuContext<'a>,
        t: Clock,
        button: ggez::input::mouse::MouseButton,
        point: numeric::Point2f,
    ) {
        let rpoint = self.canvas.relative_point(point);

        if self.click_record_book_menu(ctx, button, rpoint, t) {
            self.record_book_menu.close_all(t);
            return;
        }

        if self.borrowing_record_book.click_handler(ctx, t, rpoint) {
            // ?????????????????????????????????????????????????????????
            return;
        }

        if let Some(sign_entry) = self
            .borrowing_record_book
            .sign_with_mouse_click(ctx, rpoint, self.current_customer_request.as_ref())
        {
            match sign_entry {
                SignFrameEntry::BorrowingSign => self.signing_borrowing_handler(ctx, t),
                SignFrameEntry::ReturningSign => {
                    self.current_page_book_condition_report =
                        self.signing_returning_handler(ctx, t);
                }
            }
            return;
        }

        if self.click_customer_silhouette_menu(ctx, button, rpoint, t) {
            self.customer_silhouette_menu.close_all(t);
            return;
        }

        if self.click_desk_book_menu(ctx, button, point, t) {
            self.on_desk_menu.close_all(t);
            return;
        }

        // ??????????????????????????????????????????????????????????????????????????????????????????????????????
        self.try_show_menus(ctx, rpoint, t);

        if self.try_open_borrowing_record_book(ctx, rpoint, t) {
            return;
        }

        if self.desk.click_handler(ctx, t, button, rpoint) {
            // ?????????????????????????????????????????????????????????
            return;
        }
    }

    fn clickable_status(
        &mut self,
        ctx: &mut ggez::Context,
        point: numeric::Point2f,
    ) -> ggez::input::mouse::CursorIcon {
        let rpoint = self.canvas.relative_point(point);

        let mut cursor_status = self.desk.check_mouse_cursor_status(ctx, rpoint);

        if cursor_status != CursorIcon::Default {
            return cursor_status;
        }

        cursor_status = self.sight.check_mouse_cursor_status(ctx, rpoint);
        if cursor_status != CursorIcon::Default {
            return cursor_status;
        }

        cursor_status
    }
}
