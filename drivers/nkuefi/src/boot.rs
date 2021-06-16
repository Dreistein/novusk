use core::fmt::Write;
use crate::kernel;
use uefi::{Handle, ResultExt};
use uefi::table::{Boot, SystemTable};

#[no_mangle]
pub unsafe extern "C" fn efi_main(image: Handle, system_table: SystemTable<Boot>) -> ! {
    uefi_services::init(&system_table);

    system_table.stdout().reset(false)
        .expect_success("Couldn't reset display");

    writeln!(system_table.stdout(), "{}", "Starting kernel...");

    kernel::start_novusk();
}
