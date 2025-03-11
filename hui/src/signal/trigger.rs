//! Contains the implementation of signal triggers, which simplify creation of custom elements

use alloc::boxed::Box;
use super::{Signal, SignalStore};

/// Signal trigger that does not take any arguments
#[allow(clippy::complexity)]
pub struct SignalTrigger(Box<dyn Fn(&mut SignalStore)>);

impl SignalTrigger {
  /// Create a new signal trigger from a function or a constructor
  pub fn new<S: Signal + 'static, F: Fn() -> S + 'static>(f: F) -> Self {
    Self(Box::new(move |s: &mut SignalStore| {
      s.add::<S>(f());
    }))
  }

  /// Fire the signal
  pub fn fire(&self, s: &mut SignalStore) {
    (self.0)(s);
  }
}

/// Signal trigger that takes a single argument and passes it to the signal
#[allow(clippy::complexity)]
pub struct SignalTriggerArg<T>(Box<dyn Fn(&mut SignalStore, T)>);

impl<T> SignalTriggerArg<T> {
  /// Create a new signal trigger from a function or a constructor
  pub fn new<S: Signal, F: Fn(T) -> S + 'static>(f: F) -> Self {
    Self(Box::new(move |s: &mut SignalStore, x| {
      s.add::<S>(f(x));
    }))
  }

  /// Fire the signal with the given argument
  pub fn fire(&self, s: &mut SignalStore, x: T) {
    (self.0)(s, x);
  }
}


// #[allow(clippy::complexity)]
// pub struct SignalTriggerElement<E: UiElement>(Box<dyn Fn(&mut SignalStore, &mut E)>);

// impl<E: UiElement> SignalTriggerElement<E> {
//   pub fn new<S: Signal, F: Fn(&mut E) -> S + 'static>(f: F) -> Self {
//     Self(Box::new(move |s: &mut SignalStore, e: &mut E| {
//       s.add::<S>(f(e));
//     }))
//   }

//   pub fn fire(&self, s: &mut SignalStore, e: &mut E) {
//     (self.0)(s, e);
//   }
// }

// #[allow(clippy::complexity)]
// pub struct SignalTriggerElementArg<E: UiElement, T>(Box<dyn Fn(&mut SignalStore, &mut E, T)>);

// impl<E: UiElement, T> SignalTriggerElementArg<E, T> {
//   pub fn new<S: Signal, F: Fn(T, &mut E) -> S + 'static>(f: F) -> Self {
//     Self(Box::new(move |s: &mut SignalStore, e: &mut E, x| {
//       s.add::<S>(f(x, e));
//     }))
//   }

//   pub fn fire(&self, s: &mut SignalStore, e: &mut E, x: T) {
//     (self.0)(s, e, x);
//   }
// }
