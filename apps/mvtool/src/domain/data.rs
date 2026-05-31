// apps/mvtool/src/domain/data.rs

use std::sync::Arc;
use std::sync::RwLock;

use mvcore::io::Root;

#[derive(Clone)]
pub struct SharedData {
    inner: Arc<RwLock<Root>>,
}

impl SharedData {
    pub fn new(root: Root) -> Self {
        Self {
            inner: Arc::new(RwLock::new(root)),
        }
    }

    #[allow(dead_code)]
    pub fn read(&self) -> std::sync::RwLockReadGuard<'_, Root> {
        self.inner.read().expect("Root lock poisoned")
    }

    pub fn write(&self) -> std::sync::RwLockWriteGuard<'_, Root> {
        self.inner.write().expect("Root lock poisoned")
    }
}
