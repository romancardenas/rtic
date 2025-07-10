pub use riscv_slic::{InterruptNumber, lock, pend, run};

#[cfg(all(
    feature = "riscv-slic",
    not(any(feature = "riscv-clint-backend", feature = "riscv-mecall-backend"))
))]
compile_error!("Building for the riscv-slic, but no compatible backend selected");

/// USE CASE RE-EXPORTS: needed for SLIC-only
pub use riscv_slic::{self, interrupt as int_macro, set_priority, swi};

pub mod interrupt {
    #[inline]
    pub fn disable() {
        riscv_slic::disable();
    }

    #[inline]
    pub unsafe fn enable() {
        unsafe { riscv_slic::enable() };
    }
}
