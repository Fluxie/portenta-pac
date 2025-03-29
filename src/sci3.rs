#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_smr: [u8; 0x01],
    brr: Brr,
    _reserved_2_scr: [u8; 0x01],
    tdr: Tdr,
    _reserved_4_ssr: [u8; 0x01],
    rdr: Rdr,
    scmr: Scmr,
    semr: Semr,
    snfr: Snfr,
    simr1: Simr1,
    simr2: Simr2,
    simr3: Simr3,
    sisr: Sisr,
    spmr: Spmr,
    _reserved_14_ftdrh: [u8; 0x02],
    _reserved_15_frdrh: [u8; 0x02],
    mddr: Mddr,
    dccr: Dccr,
    fcr: Fcr,
    fdr: Fdr,
    lsr: Lsr,
    cdr: Cdr,
    sptr: Sptr,
    actr: Actr,
    _reserved24: [u8; 0x02],
    mmr: Mmr,
    _reserved25: [u8; 0x01],
    tmpr: Tmpr,
    rmpr: Rmpr,
    mesr: Mesr,
    mecr: Mecr,
}
impl RegisterBlock {
    ///0x00 - Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)
    #[inline(always)]
    pub const fn smr_smci(&self) -> &SmrSmci {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)
    #[inline(always)]
    pub const fn smr(&self) -> &Smr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x01 - Bit Rate Register
    #[inline(always)]
    pub const fn brr(&self) -> &Brr {
        &self.brr
    }
    ///0x02 - Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)
    #[inline(always)]
    pub const fn scr_smci(&self) -> &ScrSmci {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x02 - Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)
    #[inline(always)]
    pub const fn scr(&self) -> &Scr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x03 - Transmit Data Register
    #[inline(always)]
    pub const fn tdr(&self) -> &Tdr {
        &self.tdr
    }
    ///0x04 - Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)
    #[inline(always)]
    pub const fn ssr_smci(&self) -> &SsrSmci {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - Serial Status Register for Manchester Mode (SCMR.SMIF = 0, and MMR.MANEN = 1)
    #[inline(always)]
    pub const fn ssr_manc(&self) -> &SsrManc {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - Serial Status Register for Non-Smart Card Interface and FIFO Mode (SCMR.SMIF = 0, FCR.FM = 1, and MMR.MANEN = 0)
    #[inline(always)]
    pub const fn ssr_fifo(&self) -> &SsrFifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)
    #[inline(always)]
    pub const fn ssr(&self) -> &Ssr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x05 - Receive Data Register
    #[inline(always)]
    pub const fn rdr(&self) -> &Rdr {
        &self.rdr
    }
    ///0x06 - Smart Card Mode Register
    #[inline(always)]
    pub const fn scmr(&self) -> &Scmr {
        &self.scmr
    }
    ///0x07 - Serial Extended Mode Register
    #[inline(always)]
    pub const fn semr(&self) -> &Semr {
        &self.semr
    }
    ///0x08 - Noise Filter Setting Register
    #[inline(always)]
    pub const fn snfr(&self) -> &Snfr {
        &self.snfr
    }
    ///0x09 - IIC Mode Register 1
    #[inline(always)]
    pub const fn simr1(&self) -> &Simr1 {
        &self.simr1
    }
    ///0x0a - IIC Mode Register 2
    #[inline(always)]
    pub const fn simr2(&self) -> &Simr2 {
        &self.simr2
    }
    ///0x0b - IIC Mode Register 3
    #[inline(always)]
    pub const fn simr3(&self) -> &Simr3 {
        &self.simr3
    }
    ///0x0c - IIC Status Register
    #[inline(always)]
    pub const fn sisr(&self) -> &Sisr {
        &self.sisr
    }
    ///0x0d - SPI Mode Register
    #[inline(always)]
    pub const fn spmr(&self) -> &Spmr {
        &self.spmr
    }
    ///0x0e - Transmit FIFO Data Register
    #[inline(always)]
    pub const fn ftdrh(&self) -> &Ftdrh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    ///0x0e - Transmit Data Register for Manchester mode (MMR.MANEN = 1)
    #[inline(always)]
    pub const fn tdrhl_man(&self) -> &TdrhlMan {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    ///0x0e - Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)
    #[inline(always)]
    pub const fn tdrhl(&self) -> &Tdrhl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    ///0x0e - Transmit FIFO Data Register
    #[inline(always)]
    pub const fn ftdrhl(&self) -> &Ftdrhl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(14).cast() }
    }
    ///0x0f - Transmit FIFO Data Register
    #[inline(always)]
    pub const fn ftdrl(&self) -> &Ftdrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(15).cast() }
    }
    ///0x10 - Receive FIFO Data Register
    #[inline(always)]
    pub const fn frdrh(&self) -> &Frdrh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - Receive Data Register for Manchester mode (MMR.MANEN = 1)
    #[inline(always)]
    pub const fn rdrhl_man(&self) -> &RdrhlMan {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)
    #[inline(always)]
    pub const fn rdrhl(&self) -> &Rdrhl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x10 - Receive FIFO Data Register
    #[inline(always)]
    pub const fn frdrhl(&self) -> &Frdrhl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(16).cast() }
    }
    ///0x11 - Receive FIFO Data Register
    #[inline(always)]
    pub const fn frdrl(&self) -> &Frdrl {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(17).cast() }
    }
    ///0x12 - Modulation Duty Register
    #[inline(always)]
    pub const fn mddr(&self) -> &Mddr {
        &self.mddr
    }
    ///0x13 - Data Compare Match Control Register
    #[inline(always)]
    pub const fn dccr(&self) -> &Dccr {
        &self.dccr
    }
    ///0x14 - FIFO Control Register
    #[inline(always)]
    pub const fn fcr(&self) -> &Fcr {
        &self.fcr
    }
    ///0x16 - FIFO Data Count Register
    #[inline(always)]
    pub const fn fdr(&self) -> &Fdr {
        &self.fdr
    }
    ///0x18 - Line Status Register
    #[inline(always)]
    pub const fn lsr(&self) -> &Lsr {
        &self.lsr
    }
    ///0x1a - Compare Match Data Register
    #[inline(always)]
    pub const fn cdr(&self) -> &Cdr {
        &self.cdr
    }
    ///0x1c - Serial Port Register
    #[inline(always)]
    pub const fn sptr(&self) -> &Sptr {
        &self.sptr
    }
    ///0x1d - Adjustment Communication Timing Register
    #[inline(always)]
    pub const fn actr(&self) -> &Actr {
        &self.actr
    }
    ///0x20 - Manchester Mode Register
    #[inline(always)]
    pub const fn mmr(&self) -> &Mmr {
        &self.mmr
    }
    ///0x22 - Transmit Manchester Preface Setting Register
    #[inline(always)]
    pub const fn tmpr(&self) -> &Tmpr {
        &self.tmpr
    }
    ///0x23 - Receive Manchester Preface Setting Register
    #[inline(always)]
    pub const fn rmpr(&self) -> &Rmpr {
        &self.rmpr
    }
    ///0x24 - Manchester Extended Error Status Register
    #[inline(always)]
    pub const fn mesr(&self) -> &Mesr {
        &self.mesr
    }
    ///0x25 - Manchester Extended Error Control Register
    #[inline(always)]
    pub const fn mecr(&self) -> &Mecr {
        &self.mecr
    }
}
/**SMR (rw) register accessor: Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)

You can [`read`](crate::Reg::read) this register and get [`smr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smr`] module*/
#[doc(alias = "SMR")]
pub type Smr = crate::Reg<smr::SmrSpec>;
///Serial Mode Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)
pub mod smr;
/**SMR_SMCI (rw) register accessor: Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)

You can [`read`](crate::Reg::read) this register and get [`smr_smci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smr_smci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@smr_smci`] module*/
#[doc(alias = "SMR_SMCI")]
pub type SmrSmci = crate::Reg<smr_smci::SmrSmciSpec>;
///Serial Mode Register for Smart Card Interface Mode (SCMR.SMIF = 1)
pub mod smr_smci;
/**BRR (rw) register accessor: Bit Rate Register

You can [`read`](crate::Reg::read) this register and get [`brr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brr`] module*/
#[doc(alias = "BRR")]
pub type Brr = crate::Reg<brr::BrrSpec>;
///Bit Rate Register
pub mod brr;
/**SCR (rw) register accessor: Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)

You can [`read`](crate::Reg::read) this register and get [`scr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scr`] module*/
#[doc(alias = "SCR")]
pub type Scr = crate::Reg<scr::ScrSpec>;
///Serial Control Register for Non-Smart Card Interface Mode (SCMR.SMIF = 0)
pub mod scr;
/**SCR_SMCI (rw) register accessor: Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)

You can [`read`](crate::Reg::read) this register and get [`scr_smci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scr_smci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scr_smci`] module*/
#[doc(alias = "SCR_SMCI")]
pub type ScrSmci = crate::Reg<scr_smci::ScrSmciSpec>;
///Serial Control Register for Smart Card Interface Mode (SCMR.SMIF = 1)
pub mod scr_smci;
/**TDR (rw) register accessor: Transmit Data Register

You can [`read`](crate::Reg::read) this register and get [`tdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdr`] module*/
#[doc(alias = "TDR")]
pub type Tdr = crate::Reg<tdr::TdrSpec>;
///Transmit Data Register
pub mod tdr;
/**SSR (rw) register accessor: Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`ssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssr`] module*/
#[doc(alias = "SSR")]
pub type Ssr = crate::Reg<ssr::SsrSpec>;
///Serial Status Register for Non-Smart Card Interface and Non-FIFO Mode (SCMR.SMIF = 0, FCR.FM = 0, and MMR.MANEN = 0)
pub mod ssr;
/**SSR_FIFO (rw) register accessor: Serial Status Register for Non-Smart Card Interface and FIFO Mode (SCMR.SMIF = 0, FCR.FM = 1, and MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`ssr_fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssr_fifo`] module*/
#[doc(alias = "SSR_FIFO")]
pub type SsrFifo = crate::Reg<ssr_fifo::SsrFifoSpec>;
///Serial Status Register for Non-Smart Card Interface and FIFO Mode (SCMR.SMIF = 0, FCR.FM = 1, and MMR.MANEN = 0)
pub mod ssr_fifo;
/**SSR_MANC (rw) register accessor: Serial Status Register for Manchester Mode (SCMR.SMIF = 0, and MMR.MANEN = 1)

You can [`read`](crate::Reg::read) this register and get [`ssr_manc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_manc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssr_manc`] module*/
#[doc(alias = "SSR_MANC")]
pub type SsrManc = crate::Reg<ssr_manc::SsrMancSpec>;
///Serial Status Register for Manchester Mode (SCMR.SMIF = 0, and MMR.MANEN = 1)
pub mod ssr_manc;
/**SSR_SMCI (rw) register accessor: Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`ssr_smci::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ssr_smci::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssr_smci`] module*/
#[doc(alias = "SSR_SMCI")]
pub type SsrSmci = crate::Reg<ssr_smci::SsrSmciSpec>;
///Serial Status Register for Smart Card Interface Mode (SCMR.SMIF = 1, and MMR.MANEN = 0)
pub mod ssr_smci;
/**RDR (r) register accessor: Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`rdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdr`] module*/
#[doc(alias = "RDR")]
pub type Rdr = crate::Reg<rdr::RdrSpec>;
///Receive Data Register
pub mod rdr;
/**SCMR (rw) register accessor: Smart Card Mode Register

You can [`read`](crate::Reg::read) this register and get [`scmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scmr`] module*/
#[doc(alias = "SCMR")]
pub type Scmr = crate::Reg<scmr::ScmrSpec>;
///Smart Card Mode Register
pub mod scmr;
/**SEMR (rw) register accessor: Serial Extended Mode Register

You can [`read`](crate::Reg::read) this register and get [`semr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`semr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@semr`] module*/
#[doc(alias = "SEMR")]
pub type Semr = crate::Reg<semr::SemrSpec>;
///Serial Extended Mode Register
pub mod semr;
/**SNFR (rw) register accessor: Noise Filter Setting Register

You can [`read`](crate::Reg::read) this register and get [`snfr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snfr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snfr`] module*/
#[doc(alias = "SNFR")]
pub type Snfr = crate::Reg<snfr::SnfrSpec>;
///Noise Filter Setting Register
pub mod snfr;
/**SIMR1 (rw) register accessor: IIC Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`simr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@simr1`] module*/
#[doc(alias = "SIMR1")]
pub type Simr1 = crate::Reg<simr1::Simr1Spec>;
///IIC Mode Register 1
pub mod simr1;
/**SIMR2 (rw) register accessor: IIC Mode Register 2

You can [`read`](crate::Reg::read) this register and get [`simr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@simr2`] module*/
#[doc(alias = "SIMR2")]
pub type Simr2 = crate::Reg<simr2::Simr2Spec>;
///IIC Mode Register 2
pub mod simr2;
/**SIMR3 (rw) register accessor: IIC Mode Register 3

You can [`read`](crate::Reg::read) this register and get [`simr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`simr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@simr3`] module*/
#[doc(alias = "SIMR3")]
pub type Simr3 = crate::Reg<simr3::Simr3Spec>;
///IIC Mode Register 3
pub mod simr3;
/**SISR (r) register accessor: IIC Status Register

You can [`read`](crate::Reg::read) this register and get [`sisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sisr`] module*/
#[doc(alias = "SISR")]
pub type Sisr = crate::Reg<sisr::SisrSpec>;
///IIC Status Register
pub mod sisr;
/**SPMR (rw) register accessor: SPI Mode Register

You can [`read`](crate::Reg::read) this register and get [`spmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spmr`] module*/
#[doc(alias = "SPMR")]
pub type Spmr = crate::Reg<spmr::SpmrSpec>;
///SPI Mode Register
pub mod spmr;
/**FTDRHL (w) register accessor: Transmit FIFO Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrhl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ftdrhl`] module*/
#[doc(alias = "FTDRHL")]
pub type Ftdrhl = crate::Reg<ftdrhl::FtdrhlSpec>;
///Transmit FIFO Data Register
pub mod ftdrhl;
/**TDRHL (rw) register accessor: Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`tdrhl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdrhl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdrhl`] module*/
#[doc(alias = "TDRHL")]
pub type Tdrhl = crate::Reg<tdrhl::TdrhlSpec>;
///Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)
pub mod tdrhl;
/**TDRHL_MAN (rw) register accessor: Transmit Data Register for Manchester mode (MMR.MANEN = 1)

You can [`read`](crate::Reg::read) this register and get [`tdrhl_man::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdrhl_man::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdrhl_man`] module*/
#[doc(alias = "TDRHL_MAN")]
pub type TdrhlMan = crate::Reg<tdrhl_man::TdrhlManSpec>;
///Transmit Data Register for Manchester mode (MMR.MANEN = 1)
pub mod tdrhl_man;
/**FTDRH (w) register accessor: Transmit FIFO Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrh::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ftdrh`] module*/
#[doc(alias = "FTDRH")]
pub type Ftdrh = crate::Reg<ftdrh::FtdrhSpec>;
///Transmit FIFO Data Register
pub mod ftdrh;
/**FTDRL (w) register accessor: Transmit FIFO Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ftdrl::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ftdrl`] module*/
#[doc(alias = "FTDRL")]
pub type Ftdrl = crate::Reg<ftdrl::FtdrlSpec>;
///Transmit FIFO Data Register
pub mod ftdrl;
/**FRDRHL (r) register accessor: Receive FIFO Data Register

You can [`read`](crate::Reg::read) this register and get [`frdrhl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frdrhl`] module*/
#[doc(alias = "FRDRHL")]
pub type Frdrhl = crate::Reg<frdrhl::FrdrhlSpec>;
///Receive FIFO Data Register
pub mod frdrhl;
/**RDRHL (r) register accessor: Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`rdrhl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdrhl`] module*/
#[doc(alias = "RDRHL")]
pub type Rdrhl = crate::Reg<rdrhl::RdrhlSpec>;
///Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)
pub mod rdrhl;
/**RDRHL_MAN (r) register accessor: Receive Data Register for Manchester mode (MMR.MANEN = 1)

You can [`read`](crate::Reg::read) this register and get [`rdrhl_man::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdrhl_man`] module*/
#[doc(alias = "RDRHL_MAN")]
pub type RdrhlMan = crate::Reg<rdrhl_man::RdrhlManSpec>;
///Receive Data Register for Manchester mode (MMR.MANEN = 1)
pub mod rdrhl_man;
/**FRDRH (r) register accessor: Receive FIFO Data Register

You can [`read`](crate::Reg::read) this register and get [`frdrh::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frdrh`] module*/
#[doc(alias = "FRDRH")]
pub type Frdrh = crate::Reg<frdrh::FrdrhSpec>;
///Receive FIFO Data Register
pub mod frdrh;
/**FRDRL (r) register accessor: Receive FIFO Data Register

You can [`read`](crate::Reg::read) this register and get [`frdrl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frdrl`] module*/
#[doc(alias = "FRDRL")]
pub type Frdrl = crate::Reg<frdrl::FrdrlSpec>;
///Receive FIFO Data Register
pub mod frdrl;
/**MDDR (rw) register accessor: Modulation Duty Register

You can [`read`](crate::Reg::read) this register and get [`mddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mddr`] module*/
#[doc(alias = "MDDR")]
pub type Mddr = crate::Reg<mddr::MddrSpec>;
///Modulation Duty Register
pub mod mddr;
/**DCCR (rw) register accessor: Data Compare Match Control Register

You can [`read`](crate::Reg::read) this register and get [`dccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dccr`] module*/
#[doc(alias = "DCCR")]
pub type Dccr = crate::Reg<dccr::DccrSpec>;
///Data Compare Match Control Register
pub mod dccr;
/**FCR (rw) register accessor: FIFO Control Register

You can [`read`](crate::Reg::read) this register and get [`fcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcr`] module*/
#[doc(alias = "FCR")]
pub type Fcr = crate::Reg<fcr::FcrSpec>;
///FIFO Control Register
pub mod fcr;
/**FDR (r) register accessor: FIFO Data Count Register

You can [`read`](crate::Reg::read) this register and get [`fdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fdr`] module*/
#[doc(alias = "FDR")]
pub type Fdr = crate::Reg<fdr::FdrSpec>;
///FIFO Data Count Register
pub mod fdr;
/**LSR (r) register accessor: Line Status Register

You can [`read`](crate::Reg::read) this register and get [`lsr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lsr`] module*/
#[doc(alias = "LSR")]
pub type Lsr = crate::Reg<lsr::LsrSpec>;
///Line Status Register
pub mod lsr;
/**CDR (rw) register accessor: Compare Match Data Register

You can [`read`](crate::Reg::read) this register and get [`cdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cdr`] module*/
#[doc(alias = "CDR")]
pub type Cdr = crate::Reg<cdr::CdrSpec>;
///Compare Match Data Register
pub mod cdr;
/**SPTR (rw) register accessor: Serial Port Register

You can [`read`](crate::Reg::read) this register and get [`sptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sptr`] module*/
#[doc(alias = "SPTR")]
pub type Sptr = crate::Reg<sptr::SptrSpec>;
///Serial Port Register
pub mod sptr;
/**ACTR (rw) register accessor: Adjustment Communication Timing Register

You can [`read`](crate::Reg::read) this register and get [`actr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@actr`] module*/
#[doc(alias = "ACTR")]
pub type Actr = crate::Reg<actr::ActrSpec>;
///Adjustment Communication Timing Register
pub mod actr;
/**MMR (rw) register accessor: Manchester Mode Register

You can [`read`](crate::Reg::read) this register and get [`mmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmr`] module*/
#[doc(alias = "MMR")]
pub type Mmr = crate::Reg<mmr::MmrSpec>;
///Manchester Mode Register
pub mod mmr;
/**TMPR (rw) register accessor: Transmit Manchester Preface Setting Register

You can [`read`](crate::Reg::read) this register and get [`tmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tmpr`] module*/
#[doc(alias = "TMPR")]
pub type Tmpr = crate::Reg<tmpr::TmprSpec>;
///Transmit Manchester Preface Setting Register
pub mod tmpr;
/**RMPR (rw) register accessor: Receive Manchester Preface Setting Register

You can [`read`](crate::Reg::read) this register and get [`rmpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmpr`] module*/
#[doc(alias = "RMPR")]
pub type Rmpr = crate::Reg<rmpr::RmprSpec>;
///Receive Manchester Preface Setting Register
pub mod rmpr;
/**MESR (rw) register accessor: Manchester Extended Error Status Register

You can [`read`](crate::Reg::read) this register and get [`mesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mesr`] module*/
#[doc(alias = "MESR")]
pub type Mesr = crate::Reg<mesr::MesrSpec>;
///Manchester Extended Error Status Register
pub mod mesr;
/**MECR (rw) register accessor: Manchester Extended Error Control Register

You can [`read`](crate::Reg::read) this register and get [`mecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mecr`] module*/
#[doc(alias = "MECR")]
pub type Mecr = crate::Reg<mecr::MecrSpec>;
///Manchester Extended Error Control Register
pub mod mecr;
