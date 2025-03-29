#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dmsar: Dmsar,
    dmdar: Dmdar,
    dmcra: Dmcra,
    dmcrb: Dmcrb,
    dmtmd: Dmtmd,
    _reserved5: [u8; 0x01],
    dmint: Dmint,
    dmamd: Dmamd,
    _reserved7: [u8; 0x02],
    dmofr: Dmofr,
    dmcnt: Dmcnt,
    dmreq: Dmreq,
    dmsts: Dmsts,
    _reserved11: [u8; 0x01],
    dmsrr: Dmsrr,
    dmdrr: Dmdrr,
    dmsbs: Dmsbs,
    dmdbs: Dmdbs,
    dmbwr: Dmbwr,
}
impl RegisterBlock {
    ///0x00 - DMA Source Address Register
    #[inline(always)]
    pub const fn dmsar(&self) -> &Dmsar {
        &self.dmsar
    }
    ///0x04 - DMA Destination Address Register
    #[inline(always)]
    pub const fn dmdar(&self) -> &Dmdar {
        &self.dmdar
    }
    ///0x08 - DMA Transfer Count Register
    #[inline(always)]
    pub const fn dmcra(&self) -> &Dmcra {
        &self.dmcra
    }
    ///0x0c - DMA Block Transfer Count Register
    #[inline(always)]
    pub const fn dmcrb(&self) -> &Dmcrb {
        &self.dmcrb
    }
    ///0x10 - DMA Transfer Mode Register
    #[inline(always)]
    pub const fn dmtmd(&self) -> &Dmtmd {
        &self.dmtmd
    }
    ///0x13 - DMA Interrupt Setting Register
    #[inline(always)]
    pub const fn dmint(&self) -> &Dmint {
        &self.dmint
    }
    ///0x14 - DMA Address Mode Register
    #[inline(always)]
    pub const fn dmamd(&self) -> &Dmamd {
        &self.dmamd
    }
    ///0x18 - DMA Offset Register
    #[inline(always)]
    pub const fn dmofr(&self) -> &Dmofr {
        &self.dmofr
    }
    ///0x1c - DMA Transfer Enable Register
    #[inline(always)]
    pub const fn dmcnt(&self) -> &Dmcnt {
        &self.dmcnt
    }
    ///0x1d - DMA Software Start Register
    #[inline(always)]
    pub const fn dmreq(&self) -> &Dmreq {
        &self.dmreq
    }
    ///0x1e - DMA Status Register
    #[inline(always)]
    pub const fn dmsts(&self) -> &Dmsts {
        &self.dmsts
    }
    ///0x20 - DMA Source Reload Address Register
    #[inline(always)]
    pub const fn dmsrr(&self) -> &Dmsrr {
        &self.dmsrr
    }
    ///0x24 - DMA Destination Reload Address Register
    #[inline(always)]
    pub const fn dmdrr(&self) -> &Dmdrr {
        &self.dmdrr
    }
    ///0x28 - DMA Source Buffer Size Register
    #[inline(always)]
    pub const fn dmsbs(&self) -> &Dmsbs {
        &self.dmsbs
    }
    ///0x2c - DMA Destination Buffer Size Register
    #[inline(always)]
    pub const fn dmdbs(&self) -> &Dmdbs {
        &self.dmdbs
    }
    ///0x30 - DMA Bufferable Write Enable Register
    #[inline(always)]
    pub const fn dmbwr(&self) -> &Dmbwr {
        &self.dmbwr
    }
}
/**DMSAR (rw) register accessor: DMA Source Address Register

You can [`read`](crate::Reg::read) this register and get [`dmsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmsar`] module*/
#[doc(alias = "DMSAR")]
pub type Dmsar = crate::Reg<dmsar::DmsarSpec>;
///DMA Source Address Register
pub mod dmsar;
/**DMDAR (rw) register accessor: DMA Destination Address Register

You can [`read`](crate::Reg::read) this register and get [`dmdar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmdar`] module*/
#[doc(alias = "DMDAR")]
pub type Dmdar = crate::Reg<dmdar::DmdarSpec>;
///DMA Destination Address Register
pub mod dmdar;
/**DMCRA (rw) register accessor: DMA Transfer Count Register

You can [`read`](crate::Reg::read) this register and get [`dmcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmcra`] module*/
#[doc(alias = "DMCRA")]
pub type Dmcra = crate::Reg<dmcra::DmcraSpec>;
///DMA Transfer Count Register
pub mod dmcra;
/**DMCRB (rw) register accessor: DMA Block Transfer Count Register

You can [`read`](crate::Reg::read) this register and get [`dmcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmcrb`] module*/
#[doc(alias = "DMCRB")]
pub type Dmcrb = crate::Reg<dmcrb::DmcrbSpec>;
///DMA Block Transfer Count Register
pub mod dmcrb;
/**DMTMD (rw) register accessor: DMA Transfer Mode Register

You can [`read`](crate::Reg::read) this register and get [`dmtmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmtmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmtmd`] module*/
#[doc(alias = "DMTMD")]
pub type Dmtmd = crate::Reg<dmtmd::DmtmdSpec>;
///DMA Transfer Mode Register
pub mod dmtmd;
/**DMINT (rw) register accessor: DMA Interrupt Setting Register

You can [`read`](crate::Reg::read) this register and get [`dmint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmint`] module*/
#[doc(alias = "DMINT")]
pub type Dmint = crate::Reg<dmint::DmintSpec>;
///DMA Interrupt Setting Register
pub mod dmint;
/**DMAMD (rw) register accessor: DMA Address Mode Register

You can [`read`](crate::Reg::read) this register and get [`dmamd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmamd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmamd`] module*/
#[doc(alias = "DMAMD")]
pub type Dmamd = crate::Reg<dmamd::DmamdSpec>;
///DMA Address Mode Register
pub mod dmamd;
/**DMOFR (rw) register accessor: DMA Offset Register

You can [`read`](crate::Reg::read) this register and get [`dmofr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmofr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmofr`] module*/
#[doc(alias = "DMOFR")]
pub type Dmofr = crate::Reg<dmofr::DmofrSpec>;
///DMA Offset Register
pub mod dmofr;
/**DMCNT (rw) register accessor: DMA Transfer Enable Register

You can [`read`](crate::Reg::read) this register and get [`dmcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmcnt`] module*/
#[doc(alias = "DMCNT")]
pub type Dmcnt = crate::Reg<dmcnt::DmcntSpec>;
///DMA Transfer Enable Register
pub mod dmcnt;
/**DMREQ (rw) register accessor: DMA Software Start Register

You can [`read`](crate::Reg::read) this register and get [`dmreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmreq`] module*/
#[doc(alias = "DMREQ")]
pub type Dmreq = crate::Reg<dmreq::DmreqSpec>;
///DMA Software Start Register
pub mod dmreq;
/**DMSTS (rw) register accessor: DMA Status Register

You can [`read`](crate::Reg::read) this register and get [`dmsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmsts`] module*/
#[doc(alias = "DMSTS")]
pub type Dmsts = crate::Reg<dmsts::DmstsSpec>;
///DMA Status Register
pub mod dmsts;
/**DMSRR (rw) register accessor: DMA Source Reload Address Register

You can [`read`](crate::Reg::read) this register and get [`dmsrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmsrr`] module*/
#[doc(alias = "DMSRR")]
pub type Dmsrr = crate::Reg<dmsrr::DmsrrSpec>;
///DMA Source Reload Address Register
pub mod dmsrr;
/**DMDRR (rw) register accessor: DMA Destination Reload Address Register

You can [`read`](crate::Reg::read) this register and get [`dmdrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmdrr`] module*/
#[doc(alias = "DMDRR")]
pub type Dmdrr = crate::Reg<dmdrr::DmdrrSpec>;
///DMA Destination Reload Address Register
pub mod dmdrr;
/**DMSBS (rw) register accessor: DMA Source Buffer Size Register

You can [`read`](crate::Reg::read) this register and get [`dmsbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmsbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmsbs`] module*/
#[doc(alias = "DMSBS")]
pub type Dmsbs = crate::Reg<dmsbs::DmsbsSpec>;
///DMA Source Buffer Size Register
pub mod dmsbs;
/**DMDBS (rw) register accessor: DMA Destination Buffer Size Register

You can [`read`](crate::Reg::read) this register and get [`dmdbs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmdbs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmdbs`] module*/
#[doc(alias = "DMDBS")]
pub type Dmdbs = crate::Reg<dmdbs::DmdbsSpec>;
///DMA Destination Buffer Size Register
pub mod dmdbs;
/**DMBWR (rw) register accessor: DMA Bufferable Write Enable Register

You can [`read`](crate::Reg::read) this register and get [`dmbwr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmbwr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmbwr`] module*/
#[doc(alias = "DMBWR")]
pub type Dmbwr = crate::Reg<dmbwr::DmbwrSpec>;
///DMA Bufferable Write Enable Register
pub mod dmbwr;
