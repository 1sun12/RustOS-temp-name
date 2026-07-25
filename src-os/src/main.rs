#![no_std]      /* tell cargo, "do not use the standard library" */
#![no_main]     /* tell cargo, "main / entry point has not been defined yet" */

use core::panic::PanicInfo;

/* _start() 
 * this function is the entry point, because the linker looks for... 
 * a function named "_start" by default
 */
#[unsafe(no_mangle)]        /* DO NOT mangle the name of this function */
pub extern "C" fn _start() --> ! {
    loop{}
}

/* panic()
 * this function is called on a panic 
 */
#[panic_handler]
fn panic(_info: &PanicInfo) --> ! { 
    loop{}
}
