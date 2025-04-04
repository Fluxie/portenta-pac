#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    iwdtrr: Iwdtrr,
    _reserved1: [u8; 0x03],
    iwdtsr: Iwdtsr,
}
impl RegisterBlock {
    ///0x00 - IWDT Refresh Register
    #[inline(always)]
    pub const fn iwdtrr(&self) -> &Iwdtrr {
        &self.iwdtrr
    }
    ///0x04 - IWDT Status Register
    #[inline(always)]
    pub const fn iwdtsr(&self) -> &Iwdtsr {
        &self.iwdtsr
    }
}
/**IWDTRR (rw) register accessor: IWDT Refresh Register

You can [`read`](crate::Reg::read) this register and get [`iwdtrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdtrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iwdtrr`] module*/
#[doc(alias = "IWDTRR")]
pub type Iwdtrr = crate::Reg<iwdtrr::IwdtrrSpec>;
///IWDT Refresh Register
pub mod iwdtrr;
/**IWDTSR (rw) register accessor: IWDT Status Register

You can [`read`](crate::Reg::read) this register and get [`iwdtsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iwdtsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iwdtsr`] module*/
#[doc(alias = "IWDTSR")]
pub type Iwdtsr = crate::Reg<iwdtsr::IwdtsrSpec>;
///IWDT Status Register
pub mod iwdtsr;
