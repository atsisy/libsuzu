pub type Clock = u64;

#[macro_export]
macro_rules! hash {
    ( $( $t:expr),* ) => {
        {
            let mut temp_hash = HashMap::new();
            $(
                temp_hash.insert($t.0, $t.1);
            )*
                temp_hash
        }
    };
}

pub struct ScheduledEvent<Args> {
    run_time: Clock,
    func: Box<dyn Fn(Args) -> ()>,
}

impl<Args> ScheduledEvent<Args> {
    /// ScheduledEvent構造体の生成メソッド
    ///
    /// # Example
    /// ```
    ///     //use crate::libsuzu::core::*;
    ///
    ///     //let event = ScheduledEvent::new(&move |x: i32| {
    ///         //println!("Event is Called {}", x);
    ///         //Ok(())
    ///     //}, 10);
    /// ```
    pub fn new(func: Box<dyn Fn(Args) -> ()>, call_abs: Clock) -> ScheduledEvent<Args> {
        ScheduledEvent {
            run_time: call_abs,
            func: func,
        }
    }

    pub fn call_event(&self, args: Args) {
        (self.func)(args)
    }

    pub fn get_scheduled(&self) -> Clock {
        self.run_time
    }
}

pub trait Updatable {
    fn update(&mut self, _ctx: &mut ggez::Context, _t: Clock) {}
}

