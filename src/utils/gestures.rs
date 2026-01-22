use crate::constants::{GESTURE_LOCK_THRESHOLD, SWIPE_THRESHOLD};

#[derive(Default, Clone, Copy, PartialEq)]
pub struct GestureState {
    pub start_x: f64,
    pub start_y: f64,
    pub current_x: f64,
    pub current_y: f64,
    pub locked_direction: Option<GestureDirection>,
    pub is_tracking: bool,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum GestureDirection {
    Horizontal,
    Vertical,
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum CompletedGesture {
    SwipeLeft,
    SwipeRight,
    SwipeUp,
    SwipeDown,
    Cancel,
}

impl GestureState {
    pub fn start(&mut self, x: f64, y: f64) {
        self.start_x = x;
        self.start_y = y;
        self.current_x = x;
        self.current_y = y;
        self.locked_direction = None;
        self.is_tracking = true;
    }

    pub fn update(&mut self, x: f64, y: f64) {
        if !self.is_tracking {
            return;
        }

        self.current_x = x;
        self.current_y = y;

        let dx = (self.current_x - self.start_x).abs();
        let dy = (self.current_y - self.start_y).abs();

        if self.locked_direction.is_none()
            && (dx > GESTURE_LOCK_THRESHOLD || dy > GESTURE_LOCK_THRESHOLD)
        {
            self.locked_direction = Some(if dx > dy * 1.5 {
                GestureDirection::Horizontal
            } else if dy > dx * 1.5 {
                GestureDirection::Vertical
            } else {
                GestureDirection::Vertical
            });
        }
    }

    pub fn complete(&self) -> CompletedGesture {
        if !self.is_tracking {
            return CompletedGesture::Cancel;
        }

        let dx = self.current_x - self.start_x;
        let dy = self.current_y - self.start_y;

        match self.locked_direction {
            Some(GestureDirection::Horizontal) => {
                if dx < -SWIPE_THRESHOLD {
                    CompletedGesture::SwipeLeft
                } else if dx > SWIPE_THRESHOLD {
                    CompletedGesture::SwipeRight
                } else {
                    CompletedGesture::Cancel
                }
            }
            Some(GestureDirection::Vertical) => {
                if dy < -SWIPE_THRESHOLD {
                    CompletedGesture::SwipeUp
                } else if dy > SWIPE_THRESHOLD {
                    CompletedGesture::SwipeDown
                } else {
                    CompletedGesture::Cancel
                }
            }
            None => CompletedGesture::Cancel,
        }
    }

    pub fn reset(&mut self) {
        self.is_tracking = false;
        self.locked_direction = None;
    }

    pub fn get_delta(&self) -> (f64, f64) {
        (self.current_x - self.start_x, self.current_y - self.start_y)
    }
}
