#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cadr: Cadr,
    cecctl1: Cecctl1,
    _reserved2: [u8; 0x01],
    statb: Statb,
    statl: Statl,
    lgc0l: Lgc0l,
    lgc1l: Lgc1l,
    datb: Datb,
    nomt: Nomt,
    statll: Statll,
    statlh: Statlh,
    statbl: Statbl,
    statbh: Statbh,
    lgc0ll: Lgc0ll,
    lgc0lh: Lgc0lh,
    lgc1ll: Lgc1ll,
    lgc1lh: Lgc1lh,
    datbl: Datbl,
    datbh: Datbh,
    nomp: Nomp,
    _reserved19: [u8; 0x02],
    cecexmd: Cecexmd,
    _reserved20: [u8; 0x01],
    cecexmon: Cecexmon,
    _reserved21: [u8; 0x15],
    ctxd: Ctxd,
    crxd: Crxd,
    ceces: Ceces,
    cecs: Cecs,
    cecfc: Cecfc,
    cecctl0: Cecctl0,
}
impl RegisterBlock {
    ///0x00 - CEC Local Address Setting Register
    #[inline(always)]
    pub const fn cadr(&self) -> &Cadr {
        &self.cadr
    }
    ///0x02 - CEC Control Register 1
    #[inline(always)]
    pub const fn cecctl1(&self) -> &Cecctl1 {
        &self.cecctl1
    }
    ///0x04 - CEC Transmission Start Bit Width Setting Register
    #[inline(always)]
    pub const fn statb(&self) -> &Statb {
        &self.statb
    }
    ///0x06 - CEC Transmission Start Bit Low Width Setting Register
    #[inline(always)]
    pub const fn statl(&self) -> &Statl {
        &self.statl
    }
    ///0x08 - CEC Transmission Logical 0 Low Width Setting Register
    #[inline(always)]
    pub const fn lgc0l(&self) -> &Lgc0l {
        &self.lgc0l
    }
    ///0x0a - CEC Transmission Logical 1 Low Width Setting Register
    #[inline(always)]
    pub const fn lgc1l(&self) -> &Lgc1l {
        &self.lgc1l
    }
    ///0x0c - CEC Transmission Data Bit Width Setting Register
    #[inline(always)]
    pub const fn datb(&self) -> &Datb {
        &self.datb
    }
    ///0x0e - CEC Reception Data Sampling Time Setting Register
    #[inline(always)]
    pub const fn nomt(&self) -> &Nomt {
        &self.nomt
    }
    ///0x10 - CEC Reception Start Bit Minimum Low Width Setting Register
    #[inline(always)]
    pub const fn statll(&self) -> &Statll {
        &self.statll
    }
    ///0x12 - CEC Reception Start Bit Maximum Low Width Setting Register
    #[inline(always)]
    pub const fn statlh(&self) -> &Statlh {
        &self.statlh
    }
    ///0x14 - CEC Reception Start Bit Minimum Bit Width Setting Register
    #[inline(always)]
    pub const fn statbl(&self) -> &Statbl {
        &self.statbl
    }
    ///0x16 - CEC Reception Start Bit Maximum Bit Width Setting Register
    #[inline(always)]
    pub const fn statbh(&self) -> &Statbh {
        &self.statbh
    }
    ///0x18 - CEC Reception Logical 0 Minimum Low Width Setting Register
    #[inline(always)]
    pub const fn lgc0ll(&self) -> &Lgc0ll {
        &self.lgc0ll
    }
    ///0x1a - CEC Reception Logical 0 Maximum Low Width Setting Register
    #[inline(always)]
    pub const fn lgc0lh(&self) -> &Lgc0lh {
        &self.lgc0lh
    }
    ///0x1c - CEC Reception Logical 1 Minimum Low Width Setting Register
    #[inline(always)]
    pub const fn lgc1ll(&self) -> &Lgc1ll {
        &self.lgc1ll
    }
    ///0x1e - CEC Reception Logical 1 Maximum Low Width Setting Register
    #[inline(always)]
    pub const fn lgc1lh(&self) -> &Lgc1lh {
        &self.lgc1lh
    }
    ///0x20 - CEC Reception Data Bit Minimum Bit Width Setting Register
    #[inline(always)]
    pub const fn datbl(&self) -> &Datbl {
        &self.datbl
    }
    ///0x22 - CEC Reception Data Bit Maximum Bit Width Setting Register
    #[inline(always)]
    pub const fn datbh(&self) -> &Datbh {
        &self.datbh
    }
    ///0x24 - CEC Data Bit Reference Width Setting Register
    #[inline(always)]
    pub const fn nomp(&self) -> &Nomp {
        &self.nomp
    }
    ///0x28 - CEC Extension Mode Register
    #[inline(always)]
    pub const fn cecexmd(&self) -> &Cecexmd {
        &self.cecexmd
    }
    ///0x2a - CEC Extension Monitor Register
    #[inline(always)]
    pub const fn cecexmon(&self) -> &Cecexmon {
        &self.cecexmon
    }
    ///0x40 - CEC Transmission Buffer Register
    #[inline(always)]
    pub const fn ctxd(&self) -> &Ctxd {
        &self.ctxd
    }
    ///0x41 - CEC Reception Buffer Register
    #[inline(always)]
    pub const fn crxd(&self) -> &Crxd {
        &self.crxd
    }
    ///0x42 - CEC Communication Error Status Register
    #[inline(always)]
    pub const fn ceces(&self) -> &Ceces {
        &self.ceces
    }
    ///0x43 - CEC Communication Status Register
    #[inline(always)]
    pub const fn cecs(&self) -> &Cecs {
        &self.cecs
    }
    ///0x44 - CEC Communication Error Flag Clear Trigger Register
    #[inline(always)]
    pub const fn cecfc(&self) -> &Cecfc {
        &self.cecfc
    }
    ///0x45 - CEC Control Register 0
    #[inline(always)]
    pub const fn cecctl0(&self) -> &Cecctl0 {
        &self.cecctl0
    }
}
/**CADR (rw) register accessor: CEC Local Address Setting Register

You can [`read`](crate::Reg::read) this register and get [`cadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cadr`] module*/
#[doc(alias = "CADR")]
pub type Cadr = crate::Reg<cadr::CadrSpec>;
///CEC Local Address Setting Register
pub mod cadr;
/**CECCTL1 (rw) register accessor: CEC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cecctl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecctl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cecctl1`] module*/
#[doc(alias = "CECCTL1")]
pub type Cecctl1 = crate::Reg<cecctl1::Cecctl1Spec>;
///CEC Control Register 1
pub mod cecctl1;
/**STATB (rw) register accessor: CEC Transmission Start Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@statb`] module*/
#[doc(alias = "STATB")]
pub type Statb = crate::Reg<statb::StatbSpec>;
///CEC Transmission Start Bit Width Setting Register
pub mod statb;
/**STATL (rw) register accessor: CEC Transmission Start Bit Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@statl`] module*/
#[doc(alias = "STATL")]
pub type Statl = crate::Reg<statl::StatlSpec>;
///CEC Transmission Start Bit Low Width Setting Register
pub mod statl;
/**LGC0L (rw) register accessor: CEC Transmission Logical 0 Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc0l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc0l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lgc0l`] module*/
#[doc(alias = "LGC0L")]
pub type Lgc0l = crate::Reg<lgc0l::Lgc0lSpec>;
///CEC Transmission Logical 0 Low Width Setting Register
pub mod lgc0l;
/**LGC1L (rw) register accessor: CEC Transmission Logical 1 Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc1l::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc1l::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lgc1l`] module*/
#[doc(alias = "LGC1L")]
pub type Lgc1l = crate::Reg<lgc1l::Lgc1lSpec>;
///CEC Transmission Logical 1 Low Width Setting Register
pub mod lgc1l;
/**DATB (rw) register accessor: CEC Transmission Data Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`datb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@datb`] module*/
#[doc(alias = "DATB")]
pub type Datb = crate::Reg<datb::DatbSpec>;
///CEC Transmission Data Bit Width Setting Register
pub mod datb;
/**NOMT (rw) register accessor: CEC Reception Data Sampling Time Setting Register

You can [`read`](crate::Reg::read) this register and get [`nomt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nomt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nomt`] module*/
#[doc(alias = "NOMT")]
pub type Nomt = crate::Reg<nomt::NomtSpec>;
///CEC Reception Data Sampling Time Setting Register
pub mod nomt;
/**STATLL (rw) register accessor: CEC Reception Start Bit Minimum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@statll`] module*/
#[doc(alias = "STATLL")]
pub type Statll = crate::Reg<statll::StatllSpec>;
///CEC Reception Start Bit Minimum Low Width Setting Register
pub mod statll;
/**STATLH (rw) register accessor: CEC Reception Start Bit Maximum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statlh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statlh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@statlh`] module*/
#[doc(alias = "STATLH")]
pub type Statlh = crate::Reg<statlh::StatlhSpec>;
///CEC Reception Start Bit Maximum Low Width Setting Register
pub mod statlh;
/**STATBL (rw) register accessor: CEC Reception Start Bit Minimum Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statbl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statbl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@statbl`] module*/
#[doc(alias = "STATBL")]
pub type Statbl = crate::Reg<statbl::StatblSpec>;
///CEC Reception Start Bit Minimum Bit Width Setting Register
pub mod statbl;
/**STATBH (rw) register accessor: CEC Reception Start Bit Maximum Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`statbh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`statbh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@statbh`] module*/
#[doc(alias = "STATBH")]
pub type Statbh = crate::Reg<statbh::StatbhSpec>;
///CEC Reception Start Bit Maximum Bit Width Setting Register
pub mod statbh;
/**LGC0LL (rw) register accessor: CEC Reception Logical 0 Minimum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc0ll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc0ll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lgc0ll`] module*/
#[doc(alias = "LGC0LL")]
pub type Lgc0ll = crate::Reg<lgc0ll::Lgc0llSpec>;
///CEC Reception Logical 0 Minimum Low Width Setting Register
pub mod lgc0ll;
/**LGC0LH (rw) register accessor: CEC Reception Logical 0 Maximum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc0lh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc0lh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lgc0lh`] module*/
#[doc(alias = "LGC0LH")]
pub type Lgc0lh = crate::Reg<lgc0lh::Lgc0lhSpec>;
///CEC Reception Logical 0 Maximum Low Width Setting Register
pub mod lgc0lh;
/**LGC1LL (rw) register accessor: CEC Reception Logical 1 Minimum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc1ll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc1ll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lgc1ll`] module*/
#[doc(alias = "LGC1LL")]
pub type Lgc1ll = crate::Reg<lgc1ll::Lgc1llSpec>;
///CEC Reception Logical 1 Minimum Low Width Setting Register
pub mod lgc1ll;
/**LGC1LH (rw) register accessor: CEC Reception Logical 1 Maximum Low Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`lgc1lh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lgc1lh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lgc1lh`] module*/
#[doc(alias = "LGC1LH")]
pub type Lgc1lh = crate::Reg<lgc1lh::Lgc1lhSpec>;
///CEC Reception Logical 1 Maximum Low Width Setting Register
pub mod lgc1lh;
/**DATBL (rw) register accessor: CEC Reception Data Bit Minimum Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`datbl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datbl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@datbl`] module*/
#[doc(alias = "DATBL")]
pub type Datbl = crate::Reg<datbl::DatblSpec>;
///CEC Reception Data Bit Minimum Bit Width Setting Register
pub mod datbl;
/**DATBH (rw) register accessor: CEC Reception Data Bit Maximum Bit Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`datbh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`datbh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@datbh`] module*/
#[doc(alias = "DATBH")]
pub type Datbh = crate::Reg<datbh::DatbhSpec>;
///CEC Reception Data Bit Maximum Bit Width Setting Register
pub mod datbh;
/**NOMP (rw) register accessor: CEC Data Bit Reference Width Setting Register

You can [`read`](crate::Reg::read) this register and get [`nomp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nomp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nomp`] module*/
#[doc(alias = "NOMP")]
pub type Nomp = crate::Reg<nomp::NompSpec>;
///CEC Data Bit Reference Width Setting Register
pub mod nomp;
/**CECEXMD (rw) register accessor: CEC Extension Mode Register

You can [`read`](crate::Reg::read) this register and get [`cecexmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecexmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cecexmd`] module*/
#[doc(alias = "CECEXMD")]
pub type Cecexmd = crate::Reg<cecexmd::CecexmdSpec>;
///CEC Extension Mode Register
pub mod cecexmd;
/**CECEXMON (rw) register accessor: CEC Extension Monitor Register

You can [`read`](crate::Reg::read) this register and get [`cecexmon::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecexmon::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cecexmon`] module*/
#[doc(alias = "CECEXMON")]
pub type Cecexmon = crate::Reg<cecexmon::CecexmonSpec>;
///CEC Extension Monitor Register
pub mod cecexmon;
/**CTXD (rw) register accessor: CEC Transmission Buffer Register

You can [`read`](crate::Reg::read) this register and get [`ctxd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctxd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctxd`] module*/
#[doc(alias = "CTXD")]
pub type Ctxd = crate::Reg<ctxd::CtxdSpec>;
///CEC Transmission Buffer Register
pub mod ctxd;
/**CRXD (rw) register accessor: CEC Reception Buffer Register

You can [`read`](crate::Reg::read) this register and get [`crxd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crxd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crxd`] module*/
#[doc(alias = "CRXD")]
pub type Crxd = crate::Reg<crxd::CrxdSpec>;
///CEC Reception Buffer Register
pub mod crxd;
/**CECES (rw) register accessor: CEC Communication Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ceces::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ceces::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ceces`] module*/
#[doc(alias = "CECES")]
pub type Ceces = crate::Reg<ceces::CecesSpec>;
///CEC Communication Error Status Register
pub mod ceces;
/**CECS (rw) register accessor: CEC Communication Status Register

You can [`read`](crate::Reg::read) this register and get [`cecs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cecs`] module*/
#[doc(alias = "CECS")]
pub type Cecs = crate::Reg<cecs::CecsSpec>;
///CEC Communication Status Register
pub mod cecs;
/**CECFC (rw) register accessor: CEC Communication Error Flag Clear Trigger Register

You can [`read`](crate::Reg::read) this register and get [`cecfc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecfc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cecfc`] module*/
#[doc(alias = "CECFC")]
pub type Cecfc = crate::Reg<cecfc::CecfcSpec>;
///CEC Communication Error Flag Clear Trigger Register
pub mod cecfc;
/**CECCTL0 (rw) register accessor: CEC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`cecctl0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecctl0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cecctl0`] module*/
#[doc(alias = "CECCTL0")]
pub type Cecctl0 = crate::Reg<cecctl0::Cecctl0Spec>;
///CEC Control Register 0
pub mod cecctl0;
