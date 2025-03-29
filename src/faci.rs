#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x10],
    fastat: Fastat,
    _reserved1: [u8; 0x03],
    faeint: Faeint,
    _reserved2: [u8; 0x03],
    frdyie: Frdyie,
    _reserved3: [u8; 0x17],
    fsaddr: Fsaddr,
    feaddr: Feaddr,
    _reserved5: [u8; 0x0c],
    fmeprot: Fmeprot,
    _reserved6: [u8; 0x32],
    fbprot0: Fbprot0,
    _reserved7: [u8; 0x02],
    fbprot1: Fbprot1,
    _reserved8: [u8; 0x02],
    fstatr: Fstatr,
    fentryr: Fentryr,
    _reserved10: [u8; 0x06],
    fsuinitr: Fsuinitr,
    _reserved11: [u8; 0x12],
    fcmdr: Fcmdr,
    _reserved12: [u8; 0x2e],
    fbccnt: Fbccnt,
    _reserved13: [u8; 0x03],
    fbcstat: Fbcstat,
    _reserved14: [u8; 0x03],
    fpsaddr: Fpsaddr,
    fsuasmon: Fsuasmon,
    fcpsr: Fcpsr,
    _reserved17: [u8; 0x02],
    fpckar: Fpckar,
    _reserved18: [u8; 0x02],
    fsuacr: Fsuacr,
}
impl RegisterBlock {
    ///0x10 - Flash Access Status Register
    #[inline(always)]
    pub const fn fastat(&self) -> &Fastat {
        &self.fastat
    }
    ///0x14 - Flash Access Error Interrupt Enable Register
    #[inline(always)]
    pub const fn faeint(&self) -> &Faeint {
        &self.faeint
    }
    ///0x18 - Flash Ready Interrupt Enable Register
    #[inline(always)]
    pub const fn frdyie(&self) -> &Frdyie {
        &self.frdyie
    }
    ///0x30 - FACI Command Start Address Register
    #[inline(always)]
    pub const fn fsaddr(&self) -> &Fsaddr {
        &self.fsaddr
    }
    ///0x34 - FACI Command End Address Register
    #[inline(always)]
    pub const fn feaddr(&self) -> &Feaddr {
        &self.feaddr
    }
    ///0x44 - Flash P/E Mode Entry Protection Register
    #[inline(always)]
    pub const fn fmeprot(&self) -> &Fmeprot {
        &self.fmeprot
    }
    ///0x78 - Flash Block Protection Register
    #[inline(always)]
    pub const fn fbprot0(&self) -> &Fbprot0 {
        &self.fbprot0
    }
    ///0x7c - Flash Block Protection for Secure Register
    #[inline(always)]
    pub const fn fbprot1(&self) -> &Fbprot1 {
        &self.fbprot1
    }
    ///0x80 - Flash Status Register
    #[inline(always)]
    pub const fn fstatr(&self) -> &Fstatr {
        &self.fstatr
    }
    ///0x84 - Flash P/E Mode Entry Register
    #[inline(always)]
    pub const fn fentryr(&self) -> &Fentryr {
        &self.fentryr
    }
    ///0x8c - Flash Sequencer Setup Initialization Register
    #[inline(always)]
    pub const fn fsuinitr(&self) -> &Fsuinitr {
        &self.fsuinitr
    }
    ///0xa0 - FACI Command Register
    #[inline(always)]
    pub const fn fcmdr(&self) -> &Fcmdr {
        &self.fcmdr
    }
    ///0xd0 - Blank Check Control Register
    #[inline(always)]
    pub const fn fbccnt(&self) -> &Fbccnt {
        &self.fbccnt
    }
    ///0xd4 - Blank Check Status Register
    #[inline(always)]
    pub const fn fbcstat(&self) -> &Fbcstat {
        &self.fbcstat
    }
    ///0xd8 - Data Flash Programming Start Address Register
    #[inline(always)]
    pub const fn fpsaddr(&self) -> &Fpsaddr {
        &self.fpsaddr
    }
    ///0xdc - Flash Startup Area Select Monitor Register
    #[inline(always)]
    pub const fn fsuasmon(&self) -> &Fsuasmon {
        &self.fsuasmon
    }
    ///0xe0 - Flash Sequencer Processing Switching Register
    #[inline(always)]
    pub const fn fcpsr(&self) -> &Fcpsr {
        &self.fcpsr
    }
    ///0xe4 - Flash Sequencer Processing Clock Notification Register
    #[inline(always)]
    pub const fn fpckar(&self) -> &Fpckar {
        &self.fpckar
    }
    ///0xe8 - Flash Startup Area Control Register
    #[inline(always)]
    pub const fn fsuacr(&self) -> &Fsuacr {
        &self.fsuacr
    }
}
/**FASTAT (rw) register accessor: Flash Access Status Register

You can [`read`](crate::Reg::read) this register and get [`fastat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fastat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fastat`] module*/
#[doc(alias = "FASTAT")]
pub type Fastat = crate::Reg<fastat::FastatSpec>;
///Flash Access Status Register
pub mod fastat;
/**FAEINT (rw) register accessor: Flash Access Error Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`faeint::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`faeint::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@faeint`] module*/
#[doc(alias = "FAEINT")]
pub type Faeint = crate::Reg<faeint::FaeintSpec>;
///Flash Access Error Interrupt Enable Register
pub mod faeint;
/**FRDYIE (rw) register accessor: Flash Ready Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`frdyie::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frdyie::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frdyie`] module*/
#[doc(alias = "FRDYIE")]
pub type Frdyie = crate::Reg<frdyie::FrdyieSpec>;
///Flash Ready Interrupt Enable Register
pub mod frdyie;
/**FSADDR (rw) register accessor: FACI Command Start Address Register

You can [`read`](crate::Reg::read) this register and get [`fsaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsaddr`] module*/
#[doc(alias = "FSADDR")]
pub type Fsaddr = crate::Reg<fsaddr::FsaddrSpec>;
///FACI Command Start Address Register
pub mod fsaddr;
/**FEADDR (rw) register accessor: FACI Command End Address Register

You can [`read`](crate::Reg::read) this register and get [`feaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`feaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@feaddr`] module*/
#[doc(alias = "FEADDR")]
pub type Feaddr = crate::Reg<feaddr::FeaddrSpec>;
///FACI Command End Address Register
pub mod feaddr;
/**FMEPROT (rw) register accessor: Flash P/E Mode Entry Protection Register

You can [`read`](crate::Reg::read) this register and get [`fmeprot::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fmeprot::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fmeprot`] module*/
#[doc(alias = "FMEPROT")]
pub type Fmeprot = crate::Reg<fmeprot::FmeprotSpec>;
///Flash P/E Mode Entry Protection Register
pub mod fmeprot;
/**FBPROT0 (rw) register accessor: Flash Block Protection Register

You can [`read`](crate::Reg::read) this register and get [`fbprot0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbprot0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fbprot0`] module*/
#[doc(alias = "FBPROT0")]
pub type Fbprot0 = crate::Reg<fbprot0::Fbprot0Spec>;
///Flash Block Protection Register
pub mod fbprot0;
/**FBPROT1 (rw) register accessor: Flash Block Protection for Secure Register

You can [`read`](crate::Reg::read) this register and get [`fbprot1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbprot1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fbprot1`] module*/
#[doc(alias = "FBPROT1")]
pub type Fbprot1 = crate::Reg<fbprot1::Fbprot1Spec>;
///Flash Block Protection for Secure Register
pub mod fbprot1;
/**FSTATR (rw) register accessor: Flash Status Register

You can [`read`](crate::Reg::read) this register and get [`fstatr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fstatr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fstatr`] module*/
#[doc(alias = "FSTATR")]
pub type Fstatr = crate::Reg<fstatr::FstatrSpec>;
///Flash Status Register
pub mod fstatr;
/**FENTRYR (rw) register accessor: Flash P/E Mode Entry Register

You can [`read`](crate::Reg::read) this register and get [`fentryr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fentryr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fentryr`] module*/
#[doc(alias = "FENTRYR")]
pub type Fentryr = crate::Reg<fentryr::FentryrSpec>;
///Flash P/E Mode Entry Register
pub mod fentryr;
/**FSUINITR (rw) register accessor: Flash Sequencer Setup Initialization Register

You can [`read`](crate::Reg::read) this register and get [`fsuinitr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsuinitr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsuinitr`] module*/
#[doc(alias = "FSUINITR")]
pub type Fsuinitr = crate::Reg<fsuinitr::FsuinitrSpec>;
///Flash Sequencer Setup Initialization Register
pub mod fsuinitr;
/**FCMDR (r) register accessor: FACI Command Register

You can [`read`](crate::Reg::read) this register and get [`fcmdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcmdr`] module*/
#[doc(alias = "FCMDR")]
pub type Fcmdr = crate::Reg<fcmdr::FcmdrSpec>;
///FACI Command Register
pub mod fcmdr;
/**FBCCNT (rw) register accessor: Blank Check Control Register

You can [`read`](crate::Reg::read) this register and get [`fbccnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbccnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fbccnt`] module*/
#[doc(alias = "FBCCNT")]
pub type Fbccnt = crate::Reg<fbccnt::FbccntSpec>;
///Blank Check Control Register
pub mod fbccnt;
/**FBCSTAT (rw) register accessor: Blank Check Status Register

You can [`read`](crate::Reg::read) this register and get [`fbcstat::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fbcstat::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fbcstat`] module*/
#[doc(alias = "FBCSTAT")]
pub type Fbcstat = crate::Reg<fbcstat::FbcstatSpec>;
///Blank Check Status Register
pub mod fbcstat;
/**FPSADDR (rw) register accessor: Data Flash Programming Start Address Register

You can [`read`](crate::Reg::read) this register and get [`fpsaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpsaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fpsaddr`] module*/
#[doc(alias = "FPSADDR")]
pub type Fpsaddr = crate::Reg<fpsaddr::FpsaddrSpec>;
///Data Flash Programming Start Address Register
pub mod fpsaddr;
/**FSUASMON (r) register accessor: Flash Startup Area Select Monitor Register

You can [`read`](crate::Reg::read) this register and get [`fsuasmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsuasmon`] module*/
#[doc(alias = "FSUASMON")]
pub type Fsuasmon = crate::Reg<fsuasmon::FsuasmonSpec>;
///Flash Startup Area Select Monitor Register
pub mod fsuasmon;
/**FCPSR (rw) register accessor: Flash Sequencer Processing Switching Register

You can [`read`](crate::Reg::read) this register and get [`fcpsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcpsr`] module*/
#[doc(alias = "FCPSR")]
pub type Fcpsr = crate::Reg<fcpsr::FcpsrSpec>;
///Flash Sequencer Processing Switching Register
pub mod fcpsr;
/**FPCKAR (rw) register accessor: Flash Sequencer Processing Clock Notification Register

You can [`read`](crate::Reg::read) this register and get [`fpckar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fpckar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fpckar`] module*/
#[doc(alias = "FPCKAR")]
pub type Fpckar = crate::Reg<fpckar::FpckarSpec>;
///Flash Sequencer Processing Clock Notification Register
pub mod fpckar;
/**FSUACR (rw) register accessor: Flash Startup Area Control Register

You can [`read`](crate::Reg::read) this register and get [`fsuacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsuacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsuacr`] module*/
#[doc(alias = "FSUACR")]
pub type Fsuacr = crate::Reg<fsuacr::FsuacrSpec>;
///Flash Startup Area Control Register
pub mod fsuacr;
