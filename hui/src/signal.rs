//! signal handling for UI events

use std::any::{Any, TypeId};
use hashbrown::HashMap;
use nohash_hasher::BuildNoHashHasher;

pub mod trigger;

#[cfg(feature = "derive")]
pub use hui_derive::Signal;

/// A marker trait for UI Signals
pub trait Signal: Any {}

// #[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, Default)]
// pub(crate) struct DummySignal;
// impl UiSignal for DummySignal {}

/// Internal storage for signals
pub struct SignalStore {
  //TODO use a multithreaded queue instead, to allow easily offloading ui processing to a different thread
  ///XXX: is this truly the most efficient structure?
  sig: HashMap<TypeId, Vec<Box<dyn Any>>, BuildNoHashHasher<u64>>
}

impl SignalStore {
  /// Create a new [`SigIntStore`]
  pub(crate) fn new() -> Self {
    Self {
      sig: Default::default(),
    }
  }

  /// Ensure that store for given signal type exists and return a mutable reference to it
  fn internal_store<T: Signal + 'static>(&mut self) -> &mut Vec<Box<dyn Any>> {
    let type_id = TypeId::of::<T>();
    self.sig.entry(type_id).or_default()
  }

  /// Add a signal to the store
  ///
  /// Signals are stored in the order they are added
  pub fn add<T: Signal + 'static>(&mut self, sig: T) {
    let type_id = TypeId::of::<T>();
    if let Some(v) = self.sig.get_mut(&type_id) {
      v.push(Box::new(sig));
    } else {
      self.sig.insert(type_id, vec![Box::new(sig)]);
    }
  }

  /// Drain all signals of a given type
  pub(crate) fn drain<T: Signal + 'static>(&mut self) -> impl Iterator<Item = T> + '_ {
    self.internal_store::<T>()
      .drain(..)
      .map(|x| *x.downcast::<T>().unwrap()) //unchecked?
  }

  /// Clear all signals
  pub(crate) fn clear(&mut self) {
    //XXX: should we clear the vecs instead?
    self.sig.clear();
  }
}


// pub trait Signal {
//   type Arg;
//   type Output;
//   fn call(&self, arg: Self::Arg) -> Self::Output;
// }

// impl<F: Fn() -> T, T> Signal for F {
//   type Arg = ();
//   type Output = T;
//   fn call(&self, _: Self::Arg) -> Self::Output {
//     self()
//   }
// }

// // impl<F: Fn(A) -> T, A, T> Signal for F {
// //   type Arg = A;
// //   type Output = T;
// //   fn call(&self, a: Self::Arg) -> Self::Output {
// //     self(a)
// //   }
// // }

// pub struct SignalTrigger<R: UiSignal + 'static, A = ()>(pub(crate) Box<dyn Fn(A) -> R + 'static>);

// impl<R: UiSignal + 'static, A> SignalTrigger<R, A> {
//   pub fn new<F: Fn(A) -> R + 'static>(f: F) -> Self {
//     Self(Box::new(f))
//   }

//   pub fn call(&self, a: A) -> R {
//     (self.0)(a)
//   }
// }

// impl<R: UiSignal + 'static, A, T: Fn(A) -> R + 'static> From<T> for SignalTrigger<R, A> {
//   fn from(f: T) -> Self {
//     Self(Box::new(f))
//   }
// }
