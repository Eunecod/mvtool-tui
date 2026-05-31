// libs/mvcore/src/session/update.rs

use std::any::Any;
use std::fmt;

pub struct UpdateSession(Box<dyn Any + Send + Sync>);

impl UpdateSession {
    pub fn new<T: Any + Send + Sync>(updater: T) -> Self {
        Self(Box::new(updater))
    }

    pub fn extract<T: Any + Send + Sync>(self) -> Result<T, Self> {
        match self.0.downcast::<T>() {
            Ok(boxed_type) => Ok(*boxed_type),
            Err(raw_box) => Err(Self(raw_box)),
        }
    }

    pub fn as_ref_concrete<T: Any + Send + Sync>(&self) -> Option<&T> {
        self.0.downcast_ref::<T>()
    }
}

impl fmt::Debug for UpdateSession {
    fn fmt(&self, format: &mut fmt::Formatter<'_>) -> fmt::Result {
        format.debug_struct("UpdateSession").finish_non_exhaustive()
    }
}
