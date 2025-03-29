#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    csar: Csar,
    _reserved1: [u8; 0x0c],
    sramsar: Sramsar,
    stbramsar: Stbramsar,
    _reserved3: [u8; 0x18],
    dtcsar: Dtcsar,
    dmacsar: Dmacsar,
    _reserved5: [u8; 0x08],
    icusara: Icusara,
    icusarb: Icusarb,
    icusarc: Icusarc,
    icusard: Icusard,
    _reserved9: [u8; 0x04],
    icusarf: Icusarf,
    _reserved10: [u8; 0x18],
    icusarg: Icusarg,
    icusarh: Icusarh,
    icusari: Icusari,
    _reserved13: [u8; 0x84],
    bussara: Bussara,
    bussarb: Bussarb,
    _reserved15: [u8; 0x28],
    mmpusara: Mmpusara,
    mmpusarb: Mmpusarb,
    _reserved17: [u8; 0x48],
    tzfsar: Tzfsar,
    _reserved18: [u8; 0x2c],
    cpudsar: Cpudsar,
}
impl RegisterBlock {
    ///0x00 - Cache Security Attribution Register
    #[inline(always)]
    pub const fn csar(&self) -> &Csar {
        &self.csar
    }
    ///0x10 - SRAM Security Attribution Register
    #[inline(always)]
    pub const fn sramsar(&self) -> &Sramsar {
        &self.sramsar
    }
    ///0x14 - Standby RAM memory Security Attribution Register
    #[inline(always)]
    pub const fn stbramsar(&self) -> &Stbramsar {
        &self.stbramsar
    }
    ///0x30 - DTC Controller Security Attribution Register
    #[inline(always)]
    pub const fn dtcsar(&self) -> &Dtcsar {
        &self.dtcsar
    }
    ///0x34 - DMAC Controller Security Attribution Register
    #[inline(always)]
    pub const fn dmacsar(&self) -> &Dmacsar {
        &self.dmacsar
    }
    ///0x40 - Interrupt Controller Unit Security Attribution Register A
    #[inline(always)]
    pub const fn icusara(&self) -> &Icusara {
        &self.icusara
    }
    ///0x44 - Interrupt Controller Unit Security Attribution Register B
    #[inline(always)]
    pub const fn icusarb(&self) -> &Icusarb {
        &self.icusarb
    }
    ///0x48 - Interrupt Controller Unit Security Attribution Register C
    #[inline(always)]
    pub const fn icusarc(&self) -> &Icusarc {
        &self.icusarc
    }
    ///0x4c - Interrupt Controller Unit Security Attribution Register D
    #[inline(always)]
    pub const fn icusard(&self) -> &Icusard {
        &self.icusard
    }
    ///0x54 - Interrupt Controller Unit Security Attribution Register F
    #[inline(always)]
    pub const fn icusarf(&self) -> &Icusarf {
        &self.icusarf
    }
    ///0x70 - Interrupt Controller Unit Security Attribution Register G
    #[inline(always)]
    pub const fn icusarg(&self) -> &Icusarg {
        &self.icusarg
    }
    ///0x74 - Interrupt Controller Unit Security Attribution Register H
    #[inline(always)]
    pub const fn icusarh(&self) -> &Icusarh {
        &self.icusarh
    }
    ///0x78 - Interrupt Controller Unit Security Attribution Register I
    #[inline(always)]
    pub const fn icusari(&self) -> &Icusari {
        &self.icusari
    }
    ///0x100 - BUS Security Attribution Register A
    #[inline(always)]
    pub const fn bussara(&self) -> &Bussara {
        &self.bussara
    }
    ///0x104 - BUS Security Attribution Register B
    #[inline(always)]
    pub const fn bussarb(&self) -> &Bussarb {
        &self.bussarb
    }
    ///0x130 - Master Memory Protection Unit Security Attribution Register A
    #[inline(always)]
    pub const fn mmpusara(&self) -> &Mmpusara {
        &self.mmpusara
    }
    ///0x134 - Master Memory Protection Unit Security Attribution Register B
    #[inline(always)]
    pub const fn mmpusarb(&self) -> &Mmpusarb {
        &self.mmpusarb
    }
    ///0x180 - TrustZone Filter Security Attribution Register
    #[inline(always)]
    pub const fn tzfsar(&self) -> &Tzfsar {
        &self.tzfsar
    }
    ///0x1b0 - CPU Debug Security Attribution Register
    #[inline(always)]
    pub const fn cpudsar(&self) -> &Cpudsar {
        &self.cpudsar
    }
}
/**CSAR (rw) register accessor: Cache Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`csar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@csar`] module*/
#[doc(alias = "CSAR")]
pub type Csar = crate::Reg<csar::CsarSpec>;
///Cache Security Attribution Register
pub mod csar;
/**SRAMSAR (rw) register accessor: SRAM Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`sramsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sramsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sramsar`] module*/
#[doc(alias = "SRAMSAR")]
pub type Sramsar = crate::Reg<sramsar::SramsarSpec>;
///SRAM Security Attribution Register
pub mod sramsar;
/**STBRAMSAR (rw) register accessor: Standby RAM memory Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`stbramsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`stbramsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@stbramsar`] module*/
#[doc(alias = "STBRAMSAR")]
pub type Stbramsar = crate::Reg<stbramsar::StbramsarSpec>;
///Standby RAM memory Security Attribution Register
pub mod stbramsar;
/**DTCSAR (rw) register accessor: DTC Controller Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`dtcsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtcsar`] module*/
#[doc(alias = "DTCSAR")]
pub type Dtcsar = crate::Reg<dtcsar::DtcsarSpec>;
///DTC Controller Security Attribution Register
pub mod dtcsar;
/**DMACSAR (rw) register accessor: DMAC Controller Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`dmacsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacsar`] module*/
#[doc(alias = "DMACSAR")]
pub type Dmacsar = crate::Reg<dmacsar::DmacsarSpec>;
///DMAC Controller Security Attribution Register
pub mod dmacsar;
/**ICUSARA (rw) register accessor: Interrupt Controller Unit Security Attribution Register A

You can [`read`](crate::Reg::read) this register and get [`icusara::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusara::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icusara`] module*/
#[doc(alias = "ICUSARA")]
pub type Icusara = crate::Reg<icusara::IcusaraSpec>;
///Interrupt Controller Unit Security Attribution Register A
pub mod icusara;
/**ICUSARB (rw) register accessor: Interrupt Controller Unit Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`icusarb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icusarb`] module*/
#[doc(alias = "ICUSARB")]
pub type Icusarb = crate::Reg<icusarb::IcusarbSpec>;
///Interrupt Controller Unit Security Attribution Register B
pub mod icusarb;
/**ICUSARC (rw) register accessor: Interrupt Controller Unit Security Attribution Register C

You can [`read`](crate::Reg::read) this register and get [`icusarc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icusarc`] module*/
#[doc(alias = "ICUSARC")]
pub type Icusarc = crate::Reg<icusarc::IcusarcSpec>;
///Interrupt Controller Unit Security Attribution Register C
pub mod icusarc;
/**ICUSARD (rw) register accessor: Interrupt Controller Unit Security Attribution Register D

You can [`read`](crate::Reg::read) this register and get [`icusard::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusard::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icusard`] module*/
#[doc(alias = "ICUSARD")]
pub type Icusard = crate::Reg<icusard::IcusardSpec>;
///Interrupt Controller Unit Security Attribution Register D
pub mod icusard;
/**ICUSARF (rw) register accessor: Interrupt Controller Unit Security Attribution Register F

You can [`read`](crate::Reg::read) this register and get [`icusarf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icusarf`] module*/
#[doc(alias = "ICUSARF")]
pub type Icusarf = crate::Reg<icusarf::IcusarfSpec>;
///Interrupt Controller Unit Security Attribution Register F
pub mod icusarf;
/**ICUSARG (rw) register accessor: Interrupt Controller Unit Security Attribution Register G

You can [`read`](crate::Reg::read) this register and get [`icusarg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icusarg`] module*/
#[doc(alias = "ICUSARG")]
pub type Icusarg = crate::Reg<icusarg::IcusargSpec>;
///Interrupt Controller Unit Security Attribution Register G
pub mod icusarg;
/**ICUSARH (rw) register accessor: Interrupt Controller Unit Security Attribution Register H

You can [`read`](crate::Reg::read) this register and get [`icusarh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusarh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icusarh`] module*/
#[doc(alias = "ICUSARH")]
pub type Icusarh = crate::Reg<icusarh::IcusarhSpec>;
///Interrupt Controller Unit Security Attribution Register H
pub mod icusarh;
/**ICUSARI (rw) register accessor: Interrupt Controller Unit Security Attribution Register I

You can [`read`](crate::Reg::read) this register and get [`icusari::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icusari::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icusari`] module*/
#[doc(alias = "ICUSARI")]
pub type Icusari = crate::Reg<icusari::IcusariSpec>;
///Interrupt Controller Unit Security Attribution Register I
pub mod icusari;
/**BUSSARA (rw) register accessor: BUS Security Attribution Register A

You can [`read`](crate::Reg::read) this register and get [`bussara::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bussara::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bussara`] module*/
#[doc(alias = "BUSSARA")]
pub type Bussara = crate::Reg<bussara::BussaraSpec>;
///BUS Security Attribution Register A
pub mod bussara;
/**BUSSARB (rw) register accessor: BUS Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`bussarb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bussarb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bussarb`] module*/
#[doc(alias = "BUSSARB")]
pub type Bussarb = crate::Reg<bussarb::BussarbSpec>;
///BUS Security Attribution Register B
pub mod bussarb;
/**MMPUSARA (rw) register accessor: Master Memory Protection Unit Security Attribution Register A

You can [`read`](crate::Reg::read) this register and get [`mmpusara::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusara::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpusara`] module*/
#[doc(alias = "MMPUSARA")]
pub type Mmpusara = crate::Reg<mmpusara::MmpusaraSpec>;
///Master Memory Protection Unit Security Attribution Register A
pub mod mmpusara;
/**MMPUSARB (rw) register accessor: Master Memory Protection Unit Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`mmpusarb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusarb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpusarb`] module*/
#[doc(alias = "MMPUSARB")]
pub type Mmpusarb = crate::Reg<mmpusarb::MmpusarbSpec>;
///Master Memory Protection Unit Security Attribution Register B
pub mod mmpusarb;
/**TZFSAR (rw) register accessor: TrustZone Filter Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`tzfsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzfsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tzfsar`] module*/
#[doc(alias = "TZFSAR")]
pub type Tzfsar = crate::Reg<tzfsar::TzfsarSpec>;
///TrustZone Filter Security Attribution Register
pub mod tzfsar;
/**CPUDSAR (rw) register accessor: CPU Debug Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`cpudsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cpudsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cpudsar`] module*/
#[doc(alias = "CPUDSAR")]
pub type Cpudsar = crate::Reg<cpudsar::CpudsarSpec>;
///CPU Debug Security Attribution Register
pub mod cpudsar;
