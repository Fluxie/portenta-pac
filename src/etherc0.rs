#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ecmr: Ecmr,
    _reserved1: [u8; 0x04],
    rflr: Rflr,
    _reserved2: [u8; 0x04],
    ecsr: Ecsr,
    _reserved3: [u8; 0x04],
    ecsipr: Ecsipr,
    _reserved4: [u8; 0x04],
    pir: Pir,
    _reserved5: [u8; 0x04],
    psr: Psr,
    _reserved6: [u8; 0x14],
    rdmlr: Rdmlr,
    _reserved7: [u8; 0x0c],
    ipgr: Ipgr,
    apr: Apr,
    mpr: Mpr,
    _reserved10: [u8; 0x04],
    rfcf: Rfcf,
    tpauser: Tpauser,
    tpausecr: Tpausecr,
    bcfrr: Bcfrr,
    _reserved14: [u8; 0x50],
    mahr: Mahr,
    _reserved15: [u8; 0x04],
    malr: Malr,
    _reserved16: [u8; 0x04],
    trocr: Trocr,
    cdcr: Cdcr,
    lccr: Lccr,
    cndcr: Cndcr,
    _reserved20: [u8; 0x04],
    cefcr: Cefcr,
    frecr: Frecr,
    tsfrcr: Tsfrcr,
    tlfrcr: Tlfrcr,
    rfcr: Rfcr,
    mafcr: Mafcr,
}
impl RegisterBlock {
    ///0x00 - ETHERC Mode Register
    #[inline(always)]
    pub const fn ecmr(&self) -> &Ecmr {
        &self.ecmr
    }
    ///0x08 - Receive Frame Maximum Length Register
    #[inline(always)]
    pub const fn rflr(&self) -> &Rflr {
        &self.rflr
    }
    ///0x10 - ETHERC Status Register
    #[inline(always)]
    pub const fn ecsr(&self) -> &Ecsr {
        &self.ecsr
    }
    ///0x18 - ETHERC Interrupt Enable Register
    #[inline(always)]
    pub const fn ecsipr(&self) -> &Ecsipr {
        &self.ecsipr
    }
    ///0x20 - PHY Interface Register
    #[inline(always)]
    pub const fn pir(&self) -> &Pir {
        &self.pir
    }
    ///0x28 - PHY Status Register
    #[inline(always)]
    pub const fn psr(&self) -> &Psr {
        &self.psr
    }
    ///0x40 - Random Number Generation Counter Upper Limit Setting Register
    #[inline(always)]
    pub const fn rdmlr(&self) -> &Rdmlr {
        &self.rdmlr
    }
    ///0x50 - Interpacket Gap Register
    #[inline(always)]
    pub const fn ipgr(&self) -> &Ipgr {
        &self.ipgr
    }
    ///0x54 - Automatic PAUSE Frame Register
    #[inline(always)]
    pub const fn apr(&self) -> &Apr {
        &self.apr
    }
    ///0x58 - Manual PAUSE Frame Register
    #[inline(always)]
    pub const fn mpr(&self) -> &Mpr {
        &self.mpr
    }
    ///0x60 - Received PAUSE Frame Counter
    #[inline(always)]
    pub const fn rfcf(&self) -> &Rfcf {
        &self.rfcf
    }
    ///0x64 - PAUSE Frame Retransmit Count Setting Register
    #[inline(always)]
    pub const fn tpauser(&self) -> &Tpauser {
        &self.tpauser
    }
    ///0x68 - PAUSE Frame Retransmit Counter
    #[inline(always)]
    pub const fn tpausecr(&self) -> &Tpausecr {
        &self.tpausecr
    }
    ///0x6c - Broadcast Frame Receive Count Setting Register
    #[inline(always)]
    pub const fn bcfrr(&self) -> &Bcfrr {
        &self.bcfrr
    }
    ///0xc0 - MAC Address Upper Bit Register
    #[inline(always)]
    pub const fn mahr(&self) -> &Mahr {
        &self.mahr
    }
    ///0xc8 - MAC Address Lower Bit Register
    #[inline(always)]
    pub const fn malr(&self) -> &Malr {
        &self.malr
    }
    ///0xd0 - Transmit Retry Over Counter Register
    #[inline(always)]
    pub const fn trocr(&self) -> &Trocr {
        &self.trocr
    }
    ///0xd4 - Late Collision Detect Counter Register
    #[inline(always)]
    pub const fn cdcr(&self) -> &Cdcr {
        &self.cdcr
    }
    ///0xd8 - Lost Carrier Counter Register
    #[inline(always)]
    pub const fn lccr(&self) -> &Lccr {
        &self.lccr
    }
    ///0xdc - Carrier Not Detect Counter Register
    #[inline(always)]
    pub const fn cndcr(&self) -> &Cndcr {
        &self.cndcr
    }
    ///0xe4 - CRC Error Frame Receive Counter Register
    #[inline(always)]
    pub const fn cefcr(&self) -> &Cefcr {
        &self.cefcr
    }
    ///0xe8 - Frame Receive Error Counter Register
    #[inline(always)]
    pub const fn frecr(&self) -> &Frecr {
        &self.frecr
    }
    ///0xec - Too-Short Frame Receive Counter Register
    #[inline(always)]
    pub const fn tsfrcr(&self) -> &Tsfrcr {
        &self.tsfrcr
    }
    ///0xf0 - Too-Long Frame Receive Counter Register
    #[inline(always)]
    pub const fn tlfrcr(&self) -> &Tlfrcr {
        &self.tlfrcr
    }
    ///0xf4 - Received Alignment Error Frame Counter Register
    #[inline(always)]
    pub const fn rfcr(&self) -> &Rfcr {
        &self.rfcr
    }
    ///0xf8 - Multicast Address Frame Receive Counter Register
    #[inline(always)]
    pub const fn mafcr(&self) -> &Mafcr {
        &self.mafcr
    }
}
/**ECMR (rw) register accessor: ETHERC Mode Register

You can [`read`](crate::Reg::read) this register and get [`ecmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecmr`] module*/
#[doc(alias = "ECMR")]
pub type Ecmr = crate::Reg<ecmr::EcmrSpec>;
///ETHERC Mode Register
pub mod ecmr;
/**RFLR (rw) register accessor: Receive Frame Maximum Length Register

You can [`read`](crate::Reg::read) this register and get [`rflr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rflr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rflr`] module*/
#[doc(alias = "RFLR")]
pub type Rflr = crate::Reg<rflr::RflrSpec>;
///Receive Frame Maximum Length Register
pub mod rflr;
/**ECSR (rw) register accessor: ETHERC Status Register

You can [`read`](crate::Reg::read) this register and get [`ecsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecsr`] module*/
#[doc(alias = "ECSR")]
pub type Ecsr = crate::Reg<ecsr::EcsrSpec>;
///ETHERC Status Register
pub mod ecsr;
/**ECSIPR (rw) register accessor: ETHERC Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`ecsipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ecsipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ecsipr`] module*/
#[doc(alias = "ECSIPR")]
pub type Ecsipr = crate::Reg<ecsipr::EcsiprSpec>;
///ETHERC Interrupt Enable Register
pub mod ecsipr;
/**PIR (rw) register accessor: PHY Interface Register

You can [`read`](crate::Reg::read) this register and get [`pir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pir`] module*/
#[doc(alias = "PIR")]
pub type Pir = crate::Reg<pir::PirSpec>;
///PHY Interface Register
pub mod pir;
/**PSR (r) register accessor: PHY Status Register

You can [`read`](crate::Reg::read) this register and get [`psr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@psr`] module*/
#[doc(alias = "PSR")]
pub type Psr = crate::Reg<psr::PsrSpec>;
///PHY Status Register
pub mod psr;
/**RDMLR (rw) register accessor: Random Number Generation Counter Upper Limit Setting Register

You can [`read`](crate::Reg::read) this register and get [`rdmlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdmlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdmlr`] module*/
#[doc(alias = "RDMLR")]
pub type Rdmlr = crate::Reg<rdmlr::RdmlrSpec>;
///Random Number Generation Counter Upper Limit Setting Register
pub mod rdmlr;
/**IPGR (rw) register accessor: Interpacket Gap Register

You can [`read`](crate::Reg::read) this register and get [`ipgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ipgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ipgr`] module*/
#[doc(alias = "IPGR")]
pub type Ipgr = crate::Reg<ipgr::IpgrSpec>;
///Interpacket Gap Register
pub mod ipgr;
/**APR (rw) register accessor: Automatic PAUSE Frame Register

You can [`read`](crate::Reg::read) this register and get [`apr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`apr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@apr`] module*/
#[doc(alias = "APR")]
pub type Apr = crate::Reg<apr::AprSpec>;
///Automatic PAUSE Frame Register
pub mod apr;
/**MPR (rw) register accessor: Manual PAUSE Frame Register

You can [`read`](crate::Reg::read) this register and get [`mpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mpr`] module*/
#[doc(alias = "MPR")]
pub type Mpr = crate::Reg<mpr::MprSpec>;
///Manual PAUSE Frame Register
pub mod mpr;
/**RFCF (r) register accessor: Received PAUSE Frame Counter

You can [`read`](crate::Reg::read) this register and get [`rfcf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfcf`] module*/
#[doc(alias = "RFCF")]
pub type Rfcf = crate::Reg<rfcf::RfcfSpec>;
///Received PAUSE Frame Counter
pub mod rfcf;
/**TPAUSER (rw) register accessor: PAUSE Frame Retransmit Count Setting Register

You can [`read`](crate::Reg::read) this register and get [`tpauser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tpauser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tpauser`] module*/
#[doc(alias = "TPAUSER")]
pub type Tpauser = crate::Reg<tpauser::TpauserSpec>;
///PAUSE Frame Retransmit Count Setting Register
pub mod tpauser;
/**TPAUSECR (r) register accessor: PAUSE Frame Retransmit Counter

You can [`read`](crate::Reg::read) this register and get [`tpausecr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tpausecr`] module*/
#[doc(alias = "TPAUSECR")]
pub type Tpausecr = crate::Reg<tpausecr::TpausecrSpec>;
///PAUSE Frame Retransmit Counter
pub mod tpausecr;
/**BCFRR (rw) register accessor: Broadcast Frame Receive Count Setting Register

You can [`read`](crate::Reg::read) this register and get [`bcfrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcfrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcfrr`] module*/
#[doc(alias = "BCFRR")]
pub type Bcfrr = crate::Reg<bcfrr::BcfrrSpec>;
///Broadcast Frame Receive Count Setting Register
pub mod bcfrr;
/**MAHR (rw) register accessor: MAC Address Upper Bit Register

You can [`read`](crate::Reg::read) this register and get [`mahr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mahr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mahr`] module*/
#[doc(alias = "MAHR")]
pub type Mahr = crate::Reg<mahr::MahrSpec>;
///MAC Address Upper Bit Register
pub mod mahr;
/**MALR (rw) register accessor: MAC Address Lower Bit Register

You can [`read`](crate::Reg::read) this register and get [`malr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`malr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@malr`] module*/
#[doc(alias = "MALR")]
pub type Malr = crate::Reg<malr::MalrSpec>;
///MAC Address Lower Bit Register
pub mod malr;
/**TROCR (rw) register accessor: Transmit Retry Over Counter Register

You can [`read`](crate::Reg::read) this register and get [`trocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trocr`] module*/
#[doc(alias = "TROCR")]
pub type Trocr = crate::Reg<trocr::TrocrSpec>;
///Transmit Retry Over Counter Register
pub mod trocr;
/**CDCR (rw) register accessor: Late Collision Detect Counter Register

You can [`read`](crate::Reg::read) this register and get [`cdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cdcr`] module*/
#[doc(alias = "CDCR")]
pub type Cdcr = crate::Reg<cdcr::CdcrSpec>;
///Late Collision Detect Counter Register
pub mod cdcr;
/**LCCR (rw) register accessor: Lost Carrier Counter Register

You can [`read`](crate::Reg::read) this register and get [`lccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lccr`] module*/
#[doc(alias = "LCCR")]
pub type Lccr = crate::Reg<lccr::LccrSpec>;
///Lost Carrier Counter Register
pub mod lccr;
/**CNDCR (rw) register accessor: Carrier Not Detect Counter Register

You can [`read`](crate::Reg::read) this register and get [`cndcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cndcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cndcr`] module*/
#[doc(alias = "CNDCR")]
pub type Cndcr = crate::Reg<cndcr::CndcrSpec>;
///Carrier Not Detect Counter Register
pub mod cndcr;
/**CEFCR (rw) register accessor: CRC Error Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`cefcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cefcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cefcr`] module*/
#[doc(alias = "CEFCR")]
pub type Cefcr = crate::Reg<cefcr::CefcrSpec>;
///CRC Error Frame Receive Counter Register
pub mod cefcr;
/**FRECR (rw) register accessor: Frame Receive Error Counter Register

You can [`read`](crate::Reg::read) this register and get [`frecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frecr`] module*/
#[doc(alias = "FRECR")]
pub type Frecr = crate::Reg<frecr::FrecrSpec>;
///Frame Receive Error Counter Register
pub mod frecr;
/**TSFRCR (rw) register accessor: Too-Short Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`tsfrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsfrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tsfrcr`] module*/
#[doc(alias = "TSFRCR")]
pub type Tsfrcr = crate::Reg<tsfrcr::TsfrcrSpec>;
///Too-Short Frame Receive Counter Register
pub mod tsfrcr;
/**TLFRCR (rw) register accessor: Too-Long Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`tlfrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tlfrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tlfrcr`] module*/
#[doc(alias = "TLFRCR")]
pub type Tlfrcr = crate::Reg<tlfrcr::TlfrcrSpec>;
///Too-Long Frame Receive Counter Register
pub mod tlfrcr;
/**RFCR (rw) register accessor: Received Alignment Error Frame Counter Register

You can [`read`](crate::Reg::read) this register and get [`rfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfcr`] module*/
#[doc(alias = "RFCR")]
pub type Rfcr = crate::Reg<rfcr::RfcrSpec>;
///Received Alignment Error Frame Counter Register
pub mod rfcr;
/**MAFCR (rw) register accessor: Multicast Address Frame Receive Counter Register

You can [`read`](crate::Reg::read) this register and get [`mafcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mafcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mafcr`] module*/
#[doc(alias = "MAFCR")]
pub type Mafcr = crate::Reg<mafcr::MafcrSpec>;
///Multicast Address Frame Receive Counter Register
pub mod mafcr;
