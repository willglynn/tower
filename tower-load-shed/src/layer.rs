use std::fmt;
use tower_layer::Layer;

use crate::LoadShed;

/// A `tower-layer` to wrap services in `LoadShed` middleware.
#[derive(Clone)]
pub struct LoadShedLayer {
    _p: (),
}

impl LoadShedLayer {
    /// Creates a new layer.
    pub fn new() -> Self {
        LoadShedLayer { _p: () }
    }
}

impl<S> Layer<S> for LoadShedLayer {
    type Service = LoadShed<S>;

    fn layer(&self, service: S) -> Self::Service {
        LoadShed::new(service)
    }
}

impl fmt::Debug for LoadShedLayer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("LoadShedLayer").finish()
    }
}
