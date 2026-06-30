use gpui::{MouseMoveEvent, Pixels, Point, point, px};

use crate::ScrollableHandle;

pub struct MiddleClickAutoscroll<Handle> {
    origin: Point<Pixels>,
    current: Point<Pixels>,
    scroll_handle: Handle,
}

impl<Handle> MiddleClickAutoscroll<Handle>
where
    Handle: ScrollableHandle,
{
    pub fn new(origin: Point<Pixels>, scroll_handle: Handle) -> Self {
        Self {
            origin,
            current: origin,
            scroll_handle,
        }
    }

    pub fn update(&mut self, event: &MouseMoveEvent) {
        self.current = event.position;
    }

    pub fn tick(&self) -> bool {
        let dx = Self::velocity(self.current.x - self.origin.x);
        let dy = Self::velocity(self.current.y - self.origin.y);

        if dx == Pixels::ZERO && dy == Pixels::ZERO {
            return false;
        }

        let max_offset = self.scroll_handle.max_offset();
        let current_offset = self.scroll_handle.offset();
        self.scroll_handle.set_offset(point(
            (current_offset.x - dx).clamp(-max_offset.x, px(0.)),
            (current_offset.y - dy).clamp(-max_offset.y, px(0.)),
        ));
        true
    }

    fn velocity(delta: Pixels) -> Pixels {
        const DEAD_ZONE: Pixels = px(8.);
        const LINEAR_SCALE: f32 = 0.16;
        const QUADRATIC_SCALE: f32 = 0.022;
        const MAX_PIXELS_PER_TICK: Pixels = px(180.);

        let distance = delta.abs();
        if distance <= DEAD_ZONE {
            return Pixels::ZERO;
        }

        let distance = (distance - DEAD_ZONE).as_f32();
        px(
            (distance * LINEAR_SCALE + distance * distance * QUADRATIC_SCALE)
                .min(MAX_PIXELS_PER_TICK.as_f32()),
        ) * delta.signum()
    }
}
