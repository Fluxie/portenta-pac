#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dmast: Dmast,
    _reserved1: [u8; 0x3f],
    dmechr: Dmechr,
}
impl RegisterBlock {
    ///0x00 - DMA Module Activation Register
    #[inline(always)]
    pub const fn dmast(&self) -> &Dmast {
        &self.dmast
    }
    ///0x40 - DMAC Error Channel Register
    #[inline(always)]
    pub const fn dmechr(&self) -> &Dmechr {
        &self.dmechr
    }
}
/**DMAST (rw) register accessor: DMA Module Activation Register

You can [`read`](crate::Reg::read) this register and get [`dmast::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmast::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmast`] module*/
#[doc(alias = "DMAST")]
pub type Dmast = crate::Reg<dmast::DmastSpec>;
///DMA Module Activation Register
pub mod dmast;
/**DMECHR (rw) register accessor: DMAC Error Channel Register

You can [`read`](crate::Reg::read) this register and get [`dmechr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmechr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmechr`] module*/
#[doc(alias = "DMECHR")]
pub type Dmechr = crate::Reg<dmechr::DmechrSpec>;
///DMAC Error Channel Register
pub mod dmechr;
