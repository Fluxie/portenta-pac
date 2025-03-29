#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x04],
    psarb: Psarb,
    psarc: Psarc,
    psard: Psard,
    psare: Psare,
    mssar: Mssar,
    cfsamona: Cfsamona,
    cfsamonb: Cfsamonb,
    dfsamon: Dfsamon,
    ssamona: Ssamona,
    ssamonb: Ssamonb,
    dlmmon: Dlmmon,
}
impl RegisterBlock {
    ///0x04 - Peripheral Security Attribution Register B
    #[inline(always)]
    pub const fn psarb(&self) -> &Psarb {
        &self.psarb
    }
    ///0x08 - Peripheral Security Attribution Register C
    #[inline(always)]
    pub const fn psarc(&self) -> &Psarc {
        &self.psarc
    }
    ///0x0c - Peripheral Security Attribution Register D
    #[inline(always)]
    pub const fn psard(&self) -> &Psard {
        &self.psard
    }
    ///0x10 - Peripheral Security Attribution Register E
    #[inline(always)]
    pub const fn psare(&self) -> &Psare {
        &self.psare
    }
    ///0x14 - Module Stop Security Attribution Register
    #[inline(always)]
    pub const fn mssar(&self) -> &Mssar {
        &self.mssar
    }
    ///0x18 - Code Flash Security Attribution Monitor Register A
    #[inline(always)]
    pub const fn cfsamona(&self) -> &Cfsamona {
        &self.cfsamona
    }
    ///0x1c - Code Flash Security Attribution Monitor Register B
    #[inline(always)]
    pub const fn cfsamonb(&self) -> &Cfsamonb {
        &self.cfsamonb
    }
    ///0x20 - Data Flash Security Attribution Monitor Register
    #[inline(always)]
    pub const fn dfsamon(&self) -> &Dfsamon {
        &self.dfsamon
    }
    ///0x24 - SRAM Security Attribution Monitor Register A
    #[inline(always)]
    pub const fn ssamona(&self) -> &Ssamona {
        &self.ssamona
    }
    ///0x28 - SRAM Security Attribution Monitor Register B
    #[inline(always)]
    pub const fn ssamonb(&self) -> &Ssamonb {
        &self.ssamonb
    }
    ///0x2c - Device Lifecycle Management State Monitor Register
    #[inline(always)]
    pub const fn dlmmon(&self) -> &Dlmmon {
        &self.dlmmon
    }
}
/**PSARB (rw) register accessor: Peripheral Security Attribution Register B

You can [`read`](crate::Reg::read) this register and get [`psarb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psarb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@psarb`] module*/
#[doc(alias = "PSARB")]
pub type Psarb = crate::Reg<psarb::PsarbSpec>;
///Peripheral Security Attribution Register B
pub mod psarb;
/**PSARC (rw) register accessor: Peripheral Security Attribution Register C

You can [`read`](crate::Reg::read) this register and get [`psarc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psarc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@psarc`] module*/
#[doc(alias = "PSARC")]
pub type Psarc = crate::Reg<psarc::PsarcSpec>;
///Peripheral Security Attribution Register C
pub mod psarc;
/**PSARD (rw) register accessor: Peripheral Security Attribution Register D

You can [`read`](crate::Reg::read) this register and get [`psard::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psard::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@psard`] module*/
#[doc(alias = "PSARD")]
pub type Psard = crate::Reg<psard::PsardSpec>;
///Peripheral Security Attribution Register D
pub mod psard;
/**PSARE (rw) register accessor: Peripheral Security Attribution Register E

You can [`read`](crate::Reg::read) this register and get [`psare::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psare::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@psare`] module*/
#[doc(alias = "PSARE")]
pub type Psare = crate::Reg<psare::PsareSpec>;
///Peripheral Security Attribution Register E
pub mod psare;
/**MSSAR (rw) register accessor: Module Stop Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`mssar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mssar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mssar`] module*/
#[doc(alias = "MSSAR")]
pub type Mssar = crate::Reg<mssar::MssarSpec>;
///Module Stop Security Attribution Register
pub mod mssar;
/**CFSAMONA (r) register accessor: Code Flash Security Attribution Monitor Register A

You can [`read`](crate::Reg::read) this register and get [`cfsamona::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfsamona`] module*/
#[doc(alias = "CFSAMONA")]
pub type Cfsamona = crate::Reg<cfsamona::CfsamonaSpec>;
///Code Flash Security Attribution Monitor Register A
pub mod cfsamona;
/**CFSAMONB (r) register accessor: Code Flash Security Attribution Monitor Register B

You can [`read`](crate::Reg::read) this register and get [`cfsamonb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfsamonb`] module*/
#[doc(alias = "CFSAMONB")]
pub type Cfsamonb = crate::Reg<cfsamonb::CfsamonbSpec>;
///Code Flash Security Attribution Monitor Register B
pub mod cfsamonb;
/**DFSAMON (r) register accessor: Data Flash Security Attribution Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dfsamon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfsamon`] module*/
#[doc(alias = "DFSAMON")]
pub type Dfsamon = crate::Reg<dfsamon::DfsamonSpec>;
///Data Flash Security Attribution Monitor Register
pub mod dfsamon;
/**SSAMONA (r) register accessor: SRAM Security Attribution Monitor Register A

You can [`read`](crate::Reg::read) this register and get [`ssamona::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssamona`] module*/
#[doc(alias = "SSAMONA")]
pub type Ssamona = crate::Reg<ssamona::SsamonaSpec>;
///SRAM Security Attribution Monitor Register A
pub mod ssamona;
/**SSAMONB (r) register accessor: SRAM Security Attribution Monitor Register B

You can [`read`](crate::Reg::read) this register and get [`ssamonb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ssamonb`] module*/
#[doc(alias = "SSAMONB")]
pub type Ssamonb = crate::Reg<ssamonb::SsamonbSpec>;
///SRAM Security Attribution Monitor Register B
pub mod ssamonb;
/**DLMMON (r) register accessor: Device Lifecycle Management State Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dlmmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dlmmon`] module*/
#[doc(alias = "DLMMON")]
pub type Dlmmon = crate::Reg<dlmmon::DlmmonSpec>;
///Device Lifecycle Management State Monitor Register
pub mod dlmmon;
