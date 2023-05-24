//! io-lifetimes provides two different options for library authors
//! writing APIs which accept untyped I/O resources.
//!
//! The following uses the POSIX-ish `Fd` types; similar considerations
//! apply to the Windows and portable types.

#[cfg(not(windows))]
use io_lifetimes::{AsFd, BorrowedFd, OwnedFd};

/// The simplest way to accept a borrowed I/O resource is to simply use a
/// `BorrwedFd` as an argument. This doesn't require the function to have any
/// type parameters. It also works in FFI signatures, as `BorrowedFd` and (on
/// Rust >= 1.63) `Option<BorrowedFd>` are guaranteed to have the same layout
/// as `RawFd`.
///
/// Callers with an `AsFd`-implementing type would call `.as_fd()` and pass
/// the result.
#[cfg(not(windows))]
fn use_fd_a(fd: BorrowedFd<'_>) {
    let _ = fd;
}

/// Another way to do this is to use an `AsFd` type parameter. This is more
/// verbose at the function definition site, and entails monomorphization, but
/// it has the advantage of allowing users to pass in any type implementing
/// `AsFd` directly, without having to call `.as_fd()` themselves.
#[cfg(not(windows))]
fn use_fd_b<Fd: AsFd>(fd: Fd) {
    let _ = fd.as_fd();
}

/// Another way to do this is to use an `impl AsFd` parameter.
#[cfg(not(windows))]
fn use_fd_c(fd: impl AsFd) {
    let _ = fd.as_fd();
}

/// The simplest way to accept a consumed I/O resource is to simply use an
/// `OwnedFd` as an argument. Similar to `use_fd_a`, this doesn't require the
/// function to have any type parameters, and also works in FFI signatures.
///
/// Callers with an `IntoFd`-implementing type would call `.into_fd()` and pass
/// the result.
#[cfg(not(windows))]
fn consume_fd_a(fd: OwnedFd) {
    let _ = fd;
}

/// Another way to do this is to use an `IntoFd` type parameter. Similar to
/// `use_fd_b`, this is more verbose here and entails monomorphization, but it
/// has the advantage of allowing users to pass in any type implementing
/// `IntoFd` directly.
#[cfg(not(windows))]
fn consume_fd_b<Fd: Into<OwnedFd>>(fd: Fd) {
    let _: OwnedFd = fd.into();
}

/// Another way to do this is to use an `impl IntoFd` parameter.
#[cfg(not(windows))]
fn consume_fd_c(fd: impl Into<OwnedFd>) {
    let _: OwnedFd = fd.into();
}

/// Now let's see how the APIs look for users.
#[cfg(not(windows))]
fn main() {
    let f = std::fs::File::open("Cargo.toml").unwrap();

    // The simple option requires an `.as_fd()` at the callsite.
    use_fd_a(f.as_fd());

    // Another option can take a reference to any owning type directly.
    use_fd_b(&f);

    // Of course, users can still pass in `BorrowedFd` values if they want to.
    use_fd_b(f.as_fd());

    // The other option is `impl AsFd`.
    use_fd_c(&f);

    // Users can still pass in `BorrowedFd` values if they want to here too.
    use_fd_c(f.as_fd());

    let a = std::fs::File::open("Cargo.toml").unwrap();
    let b = std::fs::File::open("Cargo.toml").unwrap();
    let c = std::fs::File::open("Cargo.toml").unwrap();

    // The simple option requires an `.into()` at the callsite.
    consume_fd_a(a.into());

    // Another option can take any `Into<OwnedFd>` type directly.
    consume_fd_b(b);

    // The other option can take any `Into<OwnedFd>` type directly.
    consume_fd_c(c);
}

#[cfg(windows)]
fn main() {
    println!("This example uses non-Windows APIs.");
}
