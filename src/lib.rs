#![no_std]

pub use self::allocator::*;
pub use self::boot::*;
pub use self::console::*;
pub use self::debug::*;
pub use self::device::*;
pub use self::filesystem::*;
pub use self::guid::*;
pub use self::header::*;
pub use self::image::*;
pub use self::memory::*;
pub use self::path::*;
pub use self::pointer::*;
pub use self::proto::*;
pub use self::runtime::*;
pub use self::status::*;
pub use self::string::*;
pub use self::system::*;
pub use self::time::*;

use alloc::boxed::Box;
use core::cell::RefCell;
use core::fmt::Write;

mod allocator;
mod boot;
mod console;
mod debug;
mod device;
mod event;
mod filesystem;
mod guid;
mod header;
mod image;
mod memory;
mod path;
mod pointer;
mod proto;
mod runtime;
mod status;
mod string;
mod system;
mod time;
mod entry_macro;

extern crate alloc;

pub(crate) static mut ST: Option<&SystemTable> = None;
pub(crate) static mut IMAGE: Option<&Image> = None;
pub(crate) static mut DEBUG_WRITER: Option<RefCell<Box<dyn Write>>> = None;

/// Initializes the ZFI.
///
/// This must be called before using any ZFI API. Usually you should call this right away as the
/// first thing in the `efi_main`. See project README for an example.
///
/// # Safety
/// Calling this function more than once is undefined behavior.
pub unsafe fn init(
    im: &'static Image,
    st: &'static SystemTable,
    debug_writer: Option<fn() -> Box<dyn Write>>,
) {
    // Initialize foundation.
    ST = Some(st);
    IMAGE = Some(im);

    // Check EFI version.
    if st.hdr().revision() < TableRevision::new(1, 1) {
        panic!("UEFI version is too old to run {}", im.proto().file_path());
    }

    // Initialize debug log.
    if let Some(f) = debug_writer {
        DEBUG_WRITER = Some(RefCell::new(f()));
    }
}
