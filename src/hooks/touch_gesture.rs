//! Touch gesture handling for mobile interactions

use dioxus::prelude::*;

/// Type of gesture detected during touch interaction
#[derive(Clone, Copy, PartialEq)]
pub enum GestureType {
    Horizontal,
    Vertical,
}

/// Direction of a completed swipe gesture
#[derive(Clone, Copy, PartialEq)]
pub enum SwipeDirection {
    Left,
    Right,
    Up,
    Down,
}

/// Touch gesture state and handlers
#[derive(Clone, Copy)]
pub struct TouchGesture {
    touch_start_x: Signal<f64>,
    touch_start_y: Signal<f64>,
    touch_current_x: Signal<f64>,
    touch_current_y: Signal<f64>,
    is_touching: Signal<bool>,
    gesture_locked: Signal<Option<GestureType>>,
}

impl TouchGesture {
    /// Handle touch start event
    pub fn on_touch_start(&mut self, x: f64, y: f64) {
        self.touch_start_x.set(x);
        self.touch_start_y.set(y);
        self.touch_current_x.set(x);
        self.touch_current_y.set(y);
        self.is_touching.set(true);
        self.gesture_locked.set(None);
    }

    /// Handle touch move event
    pub fn on_touch_move(&mut self, x: f64, y: f64, lock_threshold: f64) {
        self.touch_current_x.set(x);
        self.touch_current_y.set(y);

        // Lock gesture orientation early (after threshold movement)
        if (*self.gesture_locked.read()).is_none() {
            let delta_x = ((*self.touch_current_x.read()) - (*self.touch_start_x.read())).abs();
            let delta_y = ((*self.touch_current_y.read()) - (*self.touch_start_y.read())).abs();

            if delta_x > lock_threshold || delta_y > lock_threshold {
                if delta_x > delta_y {
                    self.gesture_locked.set(Some(GestureType::Horizontal));
                } else {
                    self.gesture_locked.set(Some(GestureType::Vertical));
                }
            }
        }
    }

    /// Handle touch end/cancel and return the detected swipe direction if threshold met
    pub fn on_touch_end(&mut self, swipe_threshold: f64) -> Option<SwipeDirection> {
        if !(*self.is_touching.read()) {
            return None;
        }

        let delta_x = *self.touch_current_x.read() - *self.touch_start_x.read();
        let delta_y = *self.touch_current_y.read() - *self.touch_start_y.read();

        let direction = match *self.gesture_locked.read() {
            Some(GestureType::Horizontal) => {
                if delta_x < -swipe_threshold {
                    Some(SwipeDirection::Left)
                } else if delta_x > swipe_threshold {
                    Some(SwipeDirection::Right)
                } else {
                    None
                }
            }
            Some(GestureType::Vertical) => {
                if delta_y < -swipe_threshold {
                    Some(SwipeDirection::Up)
                } else if delta_y > swipe_threshold {
                    Some(SwipeDirection::Down)
                } else {
                    None
                }
            }
            None => {
                // No gesture locked, prioritize vertical
                if delta_y.abs() > delta_x.abs() {
                    if delta_y < -swipe_threshold {
                        Some(SwipeDirection::Up)
                    } else if delta_y > swipe_threshold {
                        Some(SwipeDirection::Down)
                    } else {
                        None
                    }
                } else {
                    None
                }
            }
        };

        self.reset();
        direction
    }

    /// Reset touch state (called on touch end or cancel)
    pub fn reset(&mut self) {
        self.is_touching.set(false);
        self.gesture_locked.set(None);
    }

    /// Check if currently touching
    pub fn is_touching(&self) -> bool {
        *self.is_touching.read()
    }
}

/// Hook to create and manage touch gesture state
pub fn use_touch_gesture() -> TouchGesture {
    let touch_start_x = use_signal(|| 0.0);
    let touch_start_y = use_signal(|| 0.0);
    let touch_current_x = use_signal(|| 0.0);
    let touch_current_y = use_signal(|| 0.0);
    let is_touching = use_signal(|| false);
    let gesture_locked = use_signal(|| Option::<GestureType>::None);

    TouchGesture {
        touch_start_x,
        touch_start_y,
        touch_current_x,
        touch_current_y,
        is_touching,
        gesture_locked,
    }
}
