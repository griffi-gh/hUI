//! state managment for stateful elements

use alloc::{boxed::Box, vec::Vec};
use hashbrown::{HashMap, HashSet};
use nohash_hasher::BuildNoHashHasher;
use core::{any::Any, hash::{Hash, Hasher}};
use rustc_hash::FxHasher;

//TODO impl StateRepo functions and automatic cleanup of inactive ids

#[cfg(feature = "derive")]
pub use hui_derive::State;

/// Marker trait for state objects
pub trait State: Any {}

/// Integer type used to identify a state object
type StateId = u64;

fn hash_local(x: impl Hash, g: &[StateId]) -> StateId {
  let mut hasher = FxHasher::default();
  0xdeadbeefu64.hash(&mut hasher);
  for x in g {
    x.hash(&mut hasher);
  }
  x.hash(&mut hasher);
  hasher.finish()
}

fn hash_global(x: impl Hash) -> StateId {
  let mut hasher = FxHasher::default();
  0xcafebabeu64.hash(&mut hasher);
  x.hash(&mut hasher);
  hasher.finish()
}

#[derive(Default)]
pub struct StateRepo {
  /// Stack of ids used to identify state objects
  id_stack: Vec<StateId>,

  /// Implementation detail: used to prevent needlessly reallocating the id stack if the `global`` function is used
  standby: Vec<StateId>,

  /// Actual state objects
  state: HashMap<StateId, Box<dyn Any>, BuildNoHashHasher<StateId>>,

  /// IDs that were accessed during the current frame, everything else is considered inactive and can be cleaned up
  active_ids: HashSet<StateId, BuildNoHashHasher<StateId>>
}

impl StateRepo {
  /// Push an id to the stack
  pub fn push(&mut self, id: impl Hash) {
    self.id_stack.push(hash_global(id));
  }

  /// Pop the last id from the stack
  ///
  /// ## Panics:
  /// Panics if the stack is empty
  pub fn pop(&mut self) {
    self.id_stack.pop().expect("stack is empty");
  }

  /// Create a new [`StateRepo`]
  pub(crate) fn new() -> Self {
    Self::default()
  }

  /// Get a reference to a state object by its id
  pub fn acquire<T: State>(&mut self, id: impl Hash) -> Option<&T> {
    let id = hash_local(id, &self.id_stack);
    self.active_ids.insert(id);
    self.state.get(&id).unwrap().downcast_ref::<T>()
  }

  /// Get a reference to a state object by its id or insert a new one
  pub fn acquire_or_insert<T: State>(&mut self, id: impl Hash, state: T) -> &T {
    let id = hash_local(id, &self.id_stack);
    self.state.entry(id)
      .or_insert_with(|| Box::new(state) as Box<dyn Any>)
      .downcast_ref::<T>().unwrap()
  }

  /// Get a reference to a state object by its id or insert a new default one
  pub fn acquire_or_default<T: State + Default>(&mut self, id: impl Hash) -> &T {
    let id = hash_local(id, &self.id_stack);
    self.state.entry(id)
      .or_insert_with(|| Box::<T>::default() as Box<dyn Any>)
      .downcast_ref::<T>().unwrap()
  }

  /// Get a mutable reference to a state object by its id
  pub fn acquire_mut<T: State>(&mut self, id: impl Hash) -> Option<&mut T> {
    let id = hash_local(id, &self.id_stack);
    self.active_ids.insert(id);
    self.state.get_mut(&id).unwrap().downcast_mut::<T>()
  }

  /// Get a mutable reference to a state object by its id or insert a new one
  pub fn acquire_mut_or_insert<T: State>(&mut self, id: impl Hash, state: T) -> &mut T {
    let id = hash_local(id, &self.id_stack);
    self.state.entry(id)
      .or_insert_with(|| Box::new(state) as Box<dyn Any>)
      .downcast_mut::<T>().unwrap()
  }

  /// Get a mutable reference to a state object by its id or insert a new default one
  pub fn acquire_mut_or_default<T: State + Default>(&mut self, id: impl Hash) -> &mut T {
    let id = hash_local(id, &self.id_stack);
    self.state.entry(id)
      .or_insert_with(|| Box::<T>::default() as Box<dyn Any>)
      .downcast_mut::<T>().unwrap()
  }

  /// Temporarily forget about current id stack, and use an empty one (within the context of the closure)
  ///
  /// Can be useful for state management of non-hierarchical objects, e.g. popups
  pub fn global<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
    self.standby.clear();
    core::mem::swap(&mut self.id_stack, &mut self.standby);
    let ret = f(self);
    core::mem::swap(&mut self.id_stack, &mut self.standby);
    ret
  }

  /// Scope the state repo
  ///
  /// Anything pushed or popped will be discarded after the closure,
  /// and the stack will be restored to its previous state
  pub fn scope<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
    self.standby.clear();
    self.standby.extend(self.id_stack.iter().copied());
    let ret = f(self);
    core::mem::swap(&mut self.id_stack, &mut self.standby);
    ret
    //XXX: this is super efficient, but works only for pushes, if anything is popped, it will be lost
    // let len = self.id_stack.len();
    // let ret = f(self);
    // self.id_stack.truncate(len);
  }
}
