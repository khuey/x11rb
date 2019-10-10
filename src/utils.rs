use std::ops::{Deref, Index};
use std::slice::{from_raw_parts, SliceIndex};
use std::mem::forget;
use libc::free;

/// Wrapper around a slice that was allocated in C code.
#[derive(Debug)]
pub struct CSlice {
    slice: &'static [u8]
}

impl CSlice {
    /// Constructs a new `CSlice` from the given parts. `libc::free` will be called on the given
    /// pointer when the slice is dropped.
    ///
    /// # Safety
    ///
    /// The same rules as for `std::slice::from_raw_parts` apply. Additionally, the given pointer
    /// must be safe to free with `libc::free`.
    pub unsafe fn new(ptr: *const u8, len: usize) -> CSlice {
        let slice = from_raw_parts(ptr, len);
        CSlice{ slice }
    }

    /// Convert `self` into a raw part.
    ///
    /// Ownership of the returned pointer is given to the caller. Specifically, `libc::free` will
    /// not be called on it by `CSlice`.
    pub fn into_ptr(self) -> *const u8 {
        let ptr = self.slice.as_ptr();
        forget(self);
        ptr
    }
}

impl Drop for CSlice {
    fn drop(&mut self) {
        unsafe { free(self.slice.as_ptr() as _) }
    }
}

impl Deref for CSlice {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        self.slice
    }
}

impl<I> Index<I> for CSlice
where I: SliceIndex<[u8]>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &I::Output {
        self.slice.index(index)
    }
}

/// A wrapper around some piece of raw bytes.
///
/// If trait associated type bounds were stable, the Connection trait could just use an associated
/// type with bound Deref<[u8]>. Since this does not work, we get this enumeration that represents
/// some owned bytes.
#[derive(Debug)]
pub enum Buffer {
    CSlice(CSlice),
    Vec(Vec<u8>)
}

impl Buffer {
    /// Constructs a new buffer from the given parts. `libc::free` will be called on the given
    /// pointer. In other words, this creates a `CSlice` variant of this enumeration.
    ///
    /// # Safety
    ///
    /// The same rules as for `CSlice::new` and `std::slice::from_raw_parts` apply. Additionally,
    /// the given pointer must be safe to free with `libc::free`.
    pub unsafe fn from_raw_parts(ptr: *const u8, len: usize) -> Self {
        Self::CSlice(CSlice::new(ptr, len))
    }

    /// Constructs a new buffer containing the given `Vec`.
    pub fn from_vec(vec: Vec<u8>) -> Self {
        Self::Vec(vec)
    }
}

impl Deref for Buffer {
    type Target = [u8];

    fn deref(&self) -> &[u8] {
        match self {
            Self::CSlice(ref slice) => slice.deref(),
            Self::Vec(ref vec) => vec.deref()
        }
    }
}

impl<I> Index<I> for Buffer
where I: SliceIndex<[u8]>
{
    type Output = I::Output;

    fn index(&self, index: I) -> &I::Output {
        self.deref().index(index)
    }
}