#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ssicr: Ssicr,
    ssisr: Ssisr,
    _reserved2: [u8; 0x08],
    ssifcr: Ssifcr,
    ssifsr: Ssifsr,
    ssiftdr: Ssiftdr,
    ssifrdr: Ssifrdr,
    ssiofr: Ssiofr,
    ssiscr: Ssiscr,
}
impl RegisterBlock {
    ///0x00 - Control Register
    #[inline(always)]
    pub const fn ssicr(&self) -> &Ssicr {
        &self.ssicr
    }
    ///0x04 - Status Register
    #[inline(always)]
    pub const fn ssisr(&self) -> &Ssisr {
        &self.ssisr
    }
    ///0x10 - FIFO Control Register
    #[inline(always)]
    pub const fn ssifcr(&self) -> &Ssifcr {
        &self.ssifcr
    }
    ///0x14 - FIFO Status Register
    #[inline(always)]
    pub const fn ssifsr(&self) -> &Ssifsr {
        &self.ssifsr
    }
    ///0x18 - Transmit FIFO Data Register
    #[inline(always)]
    pub const fn ssiftdr(&self) -> &Ssiftdr {
        &self.ssiftdr
    }
    ///0x1c - Receive FIFO Data Register
    #[inline(always)]
    pub const fn ssifrdr(&self) -> &Ssifrdr {
        &self.ssifrdr
    }
    ///0x20 - Audio Format Register
    #[inline(always)]
    pub const fn ssiofr(&self) -> &Ssiofr {
        &self.ssiofr
    }
    ///0x24 - Status Control Register
    #[inline(always)]
    pub const fn ssiscr(&self) -> &Ssiscr {
        &self.ssiscr
    }
}
/**SSICR (rw) register accessor: Control Register

You can [`read`](crate::Reg::read) this register and get [`ssicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssicr`] module*/
#[doc(alias = "SSICR")]
pub type Ssicr = crate::Reg<ssicr::SsicrSpec>;
///Control Register
pub mod ssicr;
/**SSISR (rw) register accessor: Status Register

You can [`read`](crate::Reg::read) this register and get [`ssisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssisr`] module*/
#[doc(alias = "SSISR")]
pub type Ssisr = crate::Reg<ssisr::SsisrSpec>;
///Status Register
pub mod ssisr;
/**SSIFCR (rw) register accessor: FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`ssifcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssifcr`] module*/
#[doc(alias = "SSIFCR")]
pub type Ssifcr = crate::Reg<ssifcr::SsifcrSpec>;
///FIFO Control Register
pub mod ssifcr;
/**SSIFSR (rw) register accessor: FIFO Status Register

You can [`read`](crate::Reg::read) this register and get [`ssifsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssifsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssifsr`] module*/
#[doc(alias = "SSIFSR")]
pub type Ssifsr = crate::Reg<ssifsr::SsifsrSpec>;
///FIFO Status Register
pub mod ssifsr;
/**SSIFTDR (w) register accessor: Transmit FIFO Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiftdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssiftdr`] module*/
#[doc(alias = "SSIFTDR")]
pub type Ssiftdr = crate::Reg<ssiftdr::SsiftdrSpec>;
///Transmit FIFO Data Register
pub mod ssiftdr;
/**SSIFRDR (r) register accessor: Receive FIFO Data Register

You can [`read`](crate::Reg::read) this register and get [`ssifrdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssifrdr`] module*/
#[doc(alias = "SSIFRDR")]
pub type Ssifrdr = crate::Reg<ssifrdr::SsifrdrSpec>;
///Receive FIFO Data Register
pub mod ssifrdr;
/**SSIOFR (rw) register accessor: Audio Format Register

You can [`read`](crate::Reg::read) this register and get [`ssiofr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiofr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssiofr`] module*/
#[doc(alias = "SSIOFR")]
pub type Ssiofr = crate::Reg<ssiofr::SsiofrSpec>;
///Audio Format Register
pub mod ssiofr;
/**SSISCR (rw) register accessor: Status Control Register

You can [`read`](crate::Reg::read) this register and get [`ssiscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssiscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssiscr`] module*/
#[doc(alias = "SSISCR")]
pub type Ssiscr = crate::Reg<ssiscr::SsiscrSpec>;
///Status Control Register
pub mod ssiscr;
