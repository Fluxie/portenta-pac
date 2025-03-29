#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x017c],
    tscdr: Tscdr,
}
impl RegisterBlock {
    ///0x17c - Temperature Sensor Calibration Data Register
    #[inline(always)]
    pub const fn tscdr(&self) -> &Tscdr {
        &self.tscdr
    }
}
/**TSCDR (r) register accessor: Temperature Sensor Calibration Data Register

You can [`read`](crate::Reg::read) this register and get [`tscdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tscdr`] module*/
#[doc(alias = "TSCDR")]
pub type Tscdr = crate::Reg<tscdr::TscdrSpec>;
///Temperature Sensor Calibration Data Register
pub mod tscdr;
