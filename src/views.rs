//! Typed views using temporary objects.
//!
//! This module defines the return types for [`AsFilelike::as_filelike_view`]
//! and [`AsSocketlike::as_socketlike_view`].
//!
//! [`AsSocketlike::as_socketlike_view`]: crate::AsSocketlike::as_socketlike_view

use crate::portability::{AsRawFilelike, FromRawFilelike, IntoRawFilelike};
#[cfg(windows)]
use crate::{
    portability::{AsRawSocketlike, FromRawSocketlike, IntoRawSocketlike},
    AsSocketlike, FromSocketlike, IntoSocketlike, OwnedSocketlike,
};
use crate::{AsFilelike, FromFilelike, IntoFilelike, OwnedFilelike};
use core::fmt;
use core::marker::PhantomData;
use core::ops::{Deref, DerefMut};

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. These are returned by [`AsFilelike::as_filelike_view`].
pub struct FilelikeView<'filelike, Target: FromFilelike + IntoFilelike> {
    /// The value to dereference to. This is an `Option` so that we can consume
    /// it in our `Drop` impl.
    target: Option<Target>,

    /// This field exists because we don't otherwise explicitly use
    /// `'filelike`.
    _phantom: PhantomData<&'filelike OwnedFilelike>,
}

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. These are returned by [`AsSocketlike::as_socketlike_view`].
///
/// [`AsSocketlike::as_socketlike_view`]: crate::AsSocketlike::as_socketlike_view
#[cfg(any(unix, target_os = "wasi"))]
pub type SocketlikeView<'socketlike, Target> = FilelikeView<'socketlike, Target>;

/// A non-owning view of a resource which dereferences to a `&Target` or
/// `&mut Target`. These are returned by [`AsSocketlike::as_socketlike_view`].
#[cfg(windows)]
pub struct SocketlikeView<'socketlike, Target: FromSocketlike + IntoSocketlike> {
    /// The value to dereference to. This is an `Option` so that we can consume
    /// it in our `Drop` impl.
    target: Option<Target>,

    /// This field exists because we don't otherwise explicitly use
    /// `'socketlike`.
    _phantom: PhantomData<&'socketlike OwnedSocketlike>,
}

impl<Target: FromFilelike + IntoFilelike> FilelikeView<'_, Target> {
    /// Construct a temporary `Target` and wrap it in a `FilelikeView` object.
    #[inline]
    pub(crate) fn new<T: AsFilelike>(filelike: &T) -> Self {
        // Safety: The returned `FilelikeView` is scoped to the lifetime of
        // `filelike`, which we've borrowed here, so the view won't outlive
        // the object it's borrowed from.
        let owned =
            unsafe { OwnedFilelike::from_raw_filelike(filelike.as_filelike().as_raw_filelike()) };
        Self {
            target: Some(Target::from_filelike(owned)),
            _phantom: PhantomData,
        }
    }
}

#[cfg(windows)]
impl<Target: FromSocketlike + IntoSocketlike> SocketlikeView<'_, Target> {
    /// Construct a temporary `Target` and wrap it in a `SocketlikeView`
    /// object.
    #[inline]
    pub(crate) fn new<T: AsSocketlike>(socketlike: &T) -> Self {
        // Safety: The returned `SocketlikeView` is scoped to the lifetime of
        // `socketlike`, which we've borrowed here, so the view won't outlive
        // the object it's borrowed from.
        let owned = unsafe {
            OwnedSocketlike::from_raw_socketlike(socketlike.as_socketlike().as_raw_socketlike())
        };
        Self {
            target: Some(Target::from_socketlike(owned)),
            _phantom: PhantomData,
        }
    }
}

impl<Target: FromFilelike + IntoFilelike> Deref for FilelikeView<'_, Target> {
    type Target = Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.target.as_ref().unwrap()
    }
}

#[cfg(windows)]
impl<Target: FromSocketlike + IntoSocketlike> Deref for SocketlikeView<'_, Target> {
    type Target = Target;

    #[inline]
    fn deref(&self) -> &Self::Target {
        self.target.as_ref().unwrap()
    }
}

impl<Target: FromFilelike + IntoFilelike> DerefMut for FilelikeView<'_, Target> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.target.as_mut().unwrap()
    }
}

#[cfg(windows)]
impl<Target: FromSocketlike + IntoSocketlike> DerefMut for SocketlikeView<'_, Target> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.target.as_mut().unwrap()
    }
}

impl<Target: FromFilelike + IntoFilelike> Drop for FilelikeView<'_, Target> {
    fn drop(&mut self) {
        // Use `Into*` to consume `self.target` without freeing its resource.
        let _ = self
            .target
            .take()
            .unwrap()
            .into_filelike()
            .into_raw_filelike();
    }
}

#[cfg(windows)]
impl<Target: FromSocketlike + IntoSocketlike> Drop for SocketlikeView<'_, Target> {
    fn drop(&mut self) {
        // Use `Into*` to consume `self.target` without freeing its resource.
        let _ = self
            .target
            .take()
            .unwrap()
            .into_socketlike()
            .into_raw_socketlike();
    }
}

impl<Target: FromFilelike + IntoFilelike + fmt::Debug> fmt::Debug for FilelikeView<'_, Target> {
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FilelikeView")
            .field("target", &*self)
            .finish()
    }
}

#[cfg(windows)]
impl<Target: FromSocketlike + IntoSocketlike + fmt::Debug> fmt::Debug
    for SocketlikeView<'_, Target>
{
    #[allow(clippy::missing_inline_in_public_items)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("SocketlikeView")
            .field("target", &*self)
            .finish()
    }
}
