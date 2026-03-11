use alloc::string::String;

use zeroize::{Zeroize, Zeroizing};

/// A wrapper that ensures secrets are zeroed out after use
pub struct SafeSecret {
    inner: String,
}

impl SafeSecret {
    #[inline(always)]
    pub fn new(s: String) -> Self {
        Self { inner: s }
    }

    #[inline(always)]
    pub fn reveal(&self) -> Zeroizing<String> {
        Zeroizing::new(self.inner.clone())
    }
}

impl Zeroize for SafeSecret {
    fn zeroize(&mut self) {
        self.inner.zeroize();
    }
}
