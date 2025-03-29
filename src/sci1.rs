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
    tdrhl: Tdrhl,
    rdrhl: Rdrhl,
    mddr: Mddr,
    _reserved17: [u8; 0x0d],
    esmer: Esmer,
    cr0: Cr0,
    cr1: Cr1,
    cr2: Cr2,
    cr3: Cr3,
    pcr: Pcr,
    icr: Icr,
    str: Str,
    stcr: Stcr,
    cf0dr: Cf0dr,
    cf0cr: Cf0cr,
    cf0rr: Cf0rr,
    pcf1dr: Pcf1dr,
    scf1dr: Scf1dr,
    cf1cr: Cf1cr,
    cf1rr: Cf1rr,
    tcr: Tcr,
    tmr: Tmr,
    tpre: Tpre,
    tcnt: Tcnt,
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
    ///0x0e - Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)
    #[inline(always)]
    pub const fn tdrhl(&self) -> &Tdrhl {
        &self.tdrhl
    }
    ///0x10 - Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)
    #[inline(always)]
    pub const fn rdrhl(&self) -> &Rdrhl {
        &self.rdrhl
    }
    ///0x12 - Modulation Duty Register
    #[inline(always)]
    pub const fn mddr(&self) -> &Mddr {
        &self.mddr
    }
    ///0x20 - Extended Serial Module Enable Register
    #[inline(always)]
    pub const fn esmer(&self) -> &Esmer {
        &self.esmer
    }
    ///0x21 - Control Register 0
    #[inline(always)]
    pub const fn cr0(&self) -> &Cr0 {
        &self.cr0
    }
    ///0x22 - Control Register 1
    #[inline(always)]
    pub const fn cr1(&self) -> &Cr1 {
        &self.cr1
    }
    ///0x23 - Control Register 2
    #[inline(always)]
    pub const fn cr2(&self) -> &Cr2 {
        &self.cr2
    }
    ///0x24 - Control Register 3
    #[inline(always)]
    pub const fn cr3(&self) -> &Cr3 {
        &self.cr3
    }
    ///0x25 - Port Control Register
    #[inline(always)]
    pub const fn pcr(&self) -> &Pcr {
        &self.pcr
    }
    ///0x26 - Interrupt Control Register
    #[inline(always)]
    pub const fn icr(&self) -> &Icr {
        &self.icr
    }
    ///0x27 - Status Register
    #[inline(always)]
    pub const fn str(&self) -> &Str {
        &self.str
    }
    ///0x28 - Status Clear Register
    #[inline(always)]
    pub const fn stcr(&self) -> &Stcr {
        &self.stcr
    }
    ///0x29 - Control Field 0 Data Register
    #[inline(always)]
    pub const fn cf0dr(&self) -> &Cf0dr {
        &self.cf0dr
    }
    ///0x2a - Control Field 0 Compare Enable Register
    #[inline(always)]
    pub const fn cf0cr(&self) -> &Cf0cr {
        &self.cf0cr
    }
    ///0x2b - Control Field 0 Receive Data Register
    #[inline(always)]
    pub const fn cf0rr(&self) -> &Cf0rr {
        &self.cf0rr
    }
    ///0x2c - Primary Control Field 1 Data Register
    #[inline(always)]
    pub const fn pcf1dr(&self) -> &Pcf1dr {
        &self.pcf1dr
    }
    ///0x2d - Secondary Control Field 1 Data Register
    #[inline(always)]
    pub const fn scf1dr(&self) -> &Scf1dr {
        &self.scf1dr
    }
    ///0x2e - Control Field 1 Compare Enable Register
    #[inline(always)]
    pub const fn cf1cr(&self) -> &Cf1cr {
        &self.cf1cr
    }
    ///0x2f - Control Field 1 Receive Data Register
    #[inline(always)]
    pub const fn cf1rr(&self) -> &Cf1rr {
        &self.cf1rr
    }
    ///0x30 - Timer Control Register
    #[inline(always)]
    pub const fn tcr(&self) -> &Tcr {
        &self.tcr
    }
    ///0x31 - Timer Mode Register
    #[inline(always)]
    pub const fn tmr(&self) -> &Tmr {
        &self.tmr
    }
    ///0x32 - Timer Prescaler Register
    #[inline(always)]
    pub const fn tpre(&self) -> &Tpre {
        &self.tpre
    }
    ///0x33 - Timer Count Register
    #[inline(always)]
    pub const fn tcnt(&self) -> &Tcnt {
        &self.tcnt
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
/**TDRHL (rw) register accessor: Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`tdrhl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdrhl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdrhl`] module*/
#[doc(alias = "TDRHL")]
pub type Tdrhl = crate::Reg<tdrhl::TdrhlSpec>;
///Transmit Data Register for Non-Manchester mode (MMR.MANEN = 0)
pub mod tdrhl;
/**RDRHL (r) register accessor: Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)

You can [`read`](crate::Reg::read) this register and get [`rdrhl::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdrhl`] module*/
#[doc(alias = "RDRHL")]
pub type Rdrhl = crate::Reg<rdrhl::RdrhlSpec>;
///Receive Data Register for Non-Manchester mode (MMR.MANEN = 0)
pub mod rdrhl;
/**MDDR (rw) register accessor: Modulation Duty Register

You can [`read`](crate::Reg::read) this register and get [`mddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mddr`] module*/
#[doc(alias = "MDDR")]
pub type Mddr = crate::Reg<mddr::MddrSpec>;
///Modulation Duty Register
pub mod mddr;
/**ESMER (rw) register accessor: Extended Serial Module Enable Register

You can [`read`](crate::Reg::read) this register and get [`esmer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esmer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@esmer`] module*/
#[doc(alias = "ESMER")]
pub type Esmer = crate::Reg<esmer::EsmerSpec>;
///Extended Serial Module Enable Register
pub mod esmer;
/**CR0 (rw) register accessor: Control Register 0

You can [`read`](crate::Reg::read) this register and get [`cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr0`] module*/
#[doc(alias = "CR0")]
pub type Cr0 = crate::Reg<cr0::Cr0Spec>;
///Control Register 0
pub mod cr0;
/**CR1 (rw) register accessor: Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr1`] module*/
#[doc(alias = "CR1")]
pub type Cr1 = crate::Reg<cr1::Cr1Spec>;
///Control Register 1
pub mod cr1;
/**CR2 (rw) register accessor: Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr2`] module*/
#[doc(alias = "CR2")]
pub type Cr2 = crate::Reg<cr2::Cr2Spec>;
///Control Register 2
pub mod cr2;
/**CR3 (rw) register accessor: Control Register 3

You can [`read`](crate::Reg::read) this register and get [`cr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cr3`] module*/
#[doc(alias = "CR3")]
pub type Cr3 = crate::Reg<cr3::Cr3Spec>;
///Control Register 3
pub mod cr3;
/**PCR (rw) register accessor: Port Control Register

You can [`read`](crate::Reg::read) this register and get [`pcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcr`] module*/
#[doc(alias = "PCR")]
pub type Pcr = crate::Reg<pcr::PcrSpec>;
///Port Control Register
pub mod pcr;
/**ICR (rw) register accessor: Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`icr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icr`] module*/
#[doc(alias = "ICR")]
pub type Icr = crate::Reg<icr::IcrSpec>;
///Interrupt Control Register
pub mod icr;
/**STR (r) register accessor: Status Register

You can [`read`](crate::Reg::read) this register and get [`str::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@str`] module*/
#[doc(alias = "STR")]
pub type Str = crate::Reg<str::StrSpec>;
///Status Register
pub mod str;
/**STCR (rw) register accessor: Status Clear Register

You can [`read`](crate::Reg::read) this register and get [`stcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stcr`] module*/
#[doc(alias = "STCR")]
pub type Stcr = crate::Reg<stcr::StcrSpec>;
///Status Clear Register
pub mod stcr;
/**CF0DR (rw) register accessor: Control Field 0 Data Register

You can [`read`](crate::Reg::read) this register and get [`cf0dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf0dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cf0dr`] module*/
#[doc(alias = "CF0DR")]
pub type Cf0dr = crate::Reg<cf0dr::Cf0drSpec>;
///Control Field 0 Data Register
pub mod cf0dr;
/**CF0CR (rw) register accessor: Control Field 0 Compare Enable Register

You can [`read`](crate::Reg::read) this register and get [`cf0cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf0cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cf0cr`] module*/
#[doc(alias = "CF0CR")]
pub type Cf0cr = crate::Reg<cf0cr::Cf0crSpec>;
///Control Field 0 Compare Enable Register
pub mod cf0cr;
/**CF0RR (rw) register accessor: Control Field 0 Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`cf0rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf0rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cf0rr`] module*/
#[doc(alias = "CF0RR")]
pub type Cf0rr = crate::Reg<cf0rr::Cf0rrSpec>;
///Control Field 0 Receive Data Register
pub mod cf0rr;
/**PCF1DR (rw) register accessor: Primary Control Field 1 Data Register

You can [`read`](crate::Reg::read) this register and get [`pcf1dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcf1dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcf1dr`] module*/
#[doc(alias = "PCF1DR")]
pub type Pcf1dr = crate::Reg<pcf1dr::Pcf1drSpec>;
///Primary Control Field 1 Data Register
pub mod pcf1dr;
/**SCF1DR (rw) register accessor: Secondary Control Field 1 Data Register

You can [`read`](crate::Reg::read) this register and get [`scf1dr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scf1dr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scf1dr`] module*/
#[doc(alias = "SCF1DR")]
pub type Scf1dr = crate::Reg<scf1dr::Scf1drSpec>;
///Secondary Control Field 1 Data Register
pub mod scf1dr;
/**CF1CR (rw) register accessor: Control Field 1 Compare Enable Register

You can [`read`](crate::Reg::read) this register and get [`cf1cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf1cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cf1cr`] module*/
#[doc(alias = "CF1CR")]
pub type Cf1cr = crate::Reg<cf1cr::Cf1crSpec>;
///Control Field 1 Compare Enable Register
pub mod cf1cr;
/**CF1RR (rw) register accessor: Control Field 1 Receive Data Register

You can [`read`](crate::Reg::read) this register and get [`cf1rr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cf1rr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cf1rr`] module*/
#[doc(alias = "CF1RR")]
pub type Cf1rr = crate::Reg<cf1rr::Cf1rrSpec>;
///Control Field 1 Receive Data Register
pub mod cf1rr;
/**TCR (rw) register accessor: Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`tcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcr`] module*/
#[doc(alias = "TCR")]
pub type Tcr = crate::Reg<tcr::TcrSpec>;
///Timer Control Register
pub mod tcr;
/**TMR (rw) register accessor: Timer Mode Register

You can [`read`](crate::Reg::read) this register and get [`tmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tmr`] module*/
#[doc(alias = "TMR")]
pub type Tmr = crate::Reg<tmr::TmrSpec>;
///Timer Mode Register
pub mod tmr;
/**TPRE (rw) register accessor: Timer Prescaler Register

You can [`read`](crate::Reg::read) this register and get [`tpre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tpre`] module*/
#[doc(alias = "TPRE")]
pub type Tpre = crate::Reg<tpre::TpreSpec>;
///Timer Prescaler Register
pub mod tpre;
/**TCNT (rw) register accessor: Timer Count Register

You can [`read`](crate::Reg::read) this register and get [`tcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tcnt`] module*/
#[doc(alias = "TCNT")]
pub type Tcnt = crate::Reg<tcnt::TcntSpec>;
///Timer Count Register
pub mod tcnt;
