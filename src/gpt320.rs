#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    gtwp: Gtwp,
    gtstr: Gtstr,
    gtstp: Gtstp,
    gtclr: Gtclr,
    gtssr: Gtssr,
    gtpsr: Gtpsr,
    gtcsr: Gtcsr,
    gtupsr: Gtupsr,
    gtdnsr: Gtdnsr,
    gticasr: Gticasr,
    gticbsr: Gticbsr,
    gtcr: Gtcr,
    gtuddtyc: Gtuddtyc,
    gtior: Gtior,
    gtintad: Gtintad,
    gtst: Gtst,
    gtber: Gtber,
    _reserved17: [u8; 0x04],
    gtcnt: Gtcnt,
    gtccra: Gtccra,
    gtccrb: Gtccrb,
    gtccrc: Gtccrc,
    gtccre: Gtccre,
    gtccrd: Gtccrd,
    gtccrf: Gtccrf,
    gtpr: Gtpr,
    gtpbr: Gtpbr,
    _reserved26: [u8; 0x1c],
    gtdtcr: Gtdtcr,
    gtdvu: Gtdvu,
    _reserved28: [u8; 0x28],
    gticlf: Gticlf,
    gtpc: Gtpc,
    _reserved30: [u8; 0x10],
    gtsecsr: Gtsecsr,
    gtsecr: Gtsecr,
}
impl RegisterBlock {
    ///0x00 - General PWM Timer Write-Protection Register
    #[inline(always)]
    pub const fn gtwp(&self) -> &Gtwp {
        &self.gtwp
    }
    ///0x04 - General PWM Timer Software Start Register
    #[inline(always)]
    pub const fn gtstr(&self) -> &Gtstr {
        &self.gtstr
    }
    ///0x08 - General PWM Timer Software Stop Register
    #[inline(always)]
    pub const fn gtstp(&self) -> &Gtstp {
        &self.gtstp
    }
    ///0x0c - General PWM Timer Software Clear Register
    #[inline(always)]
    pub const fn gtclr(&self) -> &Gtclr {
        &self.gtclr
    }
    ///0x10 - General PWM Timer Start Source Select Register
    #[inline(always)]
    pub const fn gtssr(&self) -> &Gtssr {
        &self.gtssr
    }
    ///0x14 - General PWM Timer Stop Source Select Register
    #[inline(always)]
    pub const fn gtpsr(&self) -> &Gtpsr {
        &self.gtpsr
    }
    ///0x18 - General PWM Timer Clear Source Select Register
    #[inline(always)]
    pub const fn gtcsr(&self) -> &Gtcsr {
        &self.gtcsr
    }
    ///0x1c - General PWM Timer Up Count Source Select Register
    #[inline(always)]
    pub const fn gtupsr(&self) -> &Gtupsr {
        &self.gtupsr
    }
    ///0x20 - General PWM Timer Down Count Source Select Register
    #[inline(always)]
    pub const fn gtdnsr(&self) -> &Gtdnsr {
        &self.gtdnsr
    }
    ///0x24 - General PWM Timer Input Capture Source Select Register A
    #[inline(always)]
    pub const fn gticasr(&self) -> &Gticasr {
        &self.gticasr
    }
    ///0x28 - General PWM Timer Input Capture Source Select Register B
    #[inline(always)]
    pub const fn gticbsr(&self) -> &Gticbsr {
        &self.gticbsr
    }
    ///0x2c - General PWM Timer Control Register
    #[inline(always)]
    pub const fn gtcr(&self) -> &Gtcr {
        &self.gtcr
    }
    ///0x30 - General PWM Timer Count Direction and Duty Setting Register
    #[inline(always)]
    pub const fn gtuddtyc(&self) -> &Gtuddtyc {
        &self.gtuddtyc
    }
    ///0x34 - General PWM Timer I/O Control Register
    #[inline(always)]
    pub const fn gtior(&self) -> &Gtior {
        &self.gtior
    }
    ///0x38 - General PWM Timer Interrupt Output Setting Register
    #[inline(always)]
    pub const fn gtintad(&self) -> &Gtintad {
        &self.gtintad
    }
    ///0x3c - General PWM Timer Status Register
    #[inline(always)]
    pub const fn gtst(&self) -> &Gtst {
        &self.gtst
    }
    ///0x40 - General PWM Timer Buffer Enable Register
    #[inline(always)]
    pub const fn gtber(&self) -> &Gtber {
        &self.gtber
    }
    ///0x48 - General PWM Timer Counter
    #[inline(always)]
    pub const fn gtcnt(&self) -> &Gtcnt {
        &self.gtcnt
    }
    ///0x4c - General PWM Timer Compare Capture Register A
    #[inline(always)]
    pub const fn gtccra(&self) -> &Gtccra {
        &self.gtccra
    }
    ///0x50 - General PWM Timer Compare Capture Register B
    #[inline(always)]
    pub const fn gtccrb(&self) -> &Gtccrb {
        &self.gtccrb
    }
    ///0x54 - General PWM Timer Compare Capture Register C
    #[inline(always)]
    pub const fn gtccrc(&self) -> &Gtccrc {
        &self.gtccrc
    }
    ///0x58 - General PWM Timer Compare Capture Register E
    #[inline(always)]
    pub const fn gtccre(&self) -> &Gtccre {
        &self.gtccre
    }
    ///0x5c - General PWM Timer Compare Capture Register D
    #[inline(always)]
    pub const fn gtccrd(&self) -> &Gtccrd {
        &self.gtccrd
    }
    ///0x60 - General PWM Timer Compare Capture Register F
    #[inline(always)]
    pub const fn gtccrf(&self) -> &Gtccrf {
        &self.gtccrf
    }
    ///0x64 - General PWM Timer Cycle Setting Register
    #[inline(always)]
    pub const fn gtpr(&self) -> &Gtpr {
        &self.gtpr
    }
    ///0x68 - General PWM Timer Cycle Setting Buffer Register
    #[inline(always)]
    pub const fn gtpbr(&self) -> &Gtpbr {
        &self.gtpbr
    }
    ///0x88 - General PWM Timer Dead Time Control Register
    #[inline(always)]
    pub const fn gtdtcr(&self) -> &Gtdtcr {
        &self.gtdtcr
    }
    ///0x8c - General PWM Timer Dead Time Value Register U
    #[inline(always)]
    pub const fn gtdvu(&self) -> &Gtdvu {
        &self.gtdvu
    }
    ///0xb8 - General PWM Timer Inter Channel Logical Operation Function Setting Register
    #[inline(always)]
    pub const fn gticlf(&self) -> &Gticlf {
        &self.gticlf
    }
    ///0xbc - General PWM Timer Period Count Register
    #[inline(always)]
    pub const fn gtpc(&self) -> &Gtpc {
        &self.gtpc
    }
    ///0xd0 - General PWM Timer Operation Enable Bit Simultaneous Control Channel Select Register
    #[inline(always)]
    pub const fn gtsecsr(&self) -> &Gtsecsr {
        &self.gtsecsr
    }
    ///0xd4 - General PWM Timer Operation Enable Bit Simultaneous Control Register
    #[inline(always)]
    pub const fn gtsecr(&self) -> &Gtsecr {
        &self.gtsecr
    }
}
/**GTWP (rw) register accessor: General PWM Timer Write-Protection Register

You can [`read`](crate::Reg::read) this register and get [`gtwp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtwp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtwp`] module*/
#[doc(alias = "GTWP")]
pub type Gtwp = crate::Reg<gtwp::GtwpSpec>;
///General PWM Timer Write-Protection Register
pub mod gtwp;
/**GTSTR (rw) register accessor: General PWM Timer Software Start Register

You can [`read`](crate::Reg::read) this register and get [`gtstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtstr`] module*/
#[doc(alias = "GTSTR")]
pub type Gtstr = crate::Reg<gtstr::GtstrSpec>;
///General PWM Timer Software Start Register
pub mod gtstr;
/**GTSTP (rw) register accessor: General PWM Timer Software Stop Register

You can [`read`](crate::Reg::read) this register and get [`gtstp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtstp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtstp`] module*/
#[doc(alias = "GTSTP")]
pub type Gtstp = crate::Reg<gtstp::GtstpSpec>;
///General PWM Timer Software Stop Register
pub mod gtstp;
/**GTCLR (w) register accessor: General PWM Timer Software Clear Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtclr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtclr`] module*/
#[doc(alias = "GTCLR")]
pub type Gtclr = crate::Reg<gtclr::GtclrSpec>;
///General PWM Timer Software Clear Register
pub mod gtclr;
/**GTSSR (rw) register accessor: General PWM Timer Start Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtssr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtssr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtssr`] module*/
#[doc(alias = "GTSSR")]
pub type Gtssr = crate::Reg<gtssr::GtssrSpec>;
///General PWM Timer Start Source Select Register
pub mod gtssr;
/**GTPSR (rw) register accessor: General PWM Timer Stop Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtpsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtpsr`] module*/
#[doc(alias = "GTPSR")]
pub type Gtpsr = crate::Reg<gtpsr::GtpsrSpec>;
///General PWM Timer Stop Source Select Register
pub mod gtpsr;
/**GTCSR (rw) register accessor: General PWM Timer Clear Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtcsr`] module*/
#[doc(alias = "GTCSR")]
pub type Gtcsr = crate::Reg<gtcsr::GtcsrSpec>;
///General PWM Timer Clear Source Select Register
pub mod gtcsr;
/**GTUPSR (rw) register accessor: General PWM Timer Up Count Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtupsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtupsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtupsr`] module*/
#[doc(alias = "GTUPSR")]
pub type Gtupsr = crate::Reg<gtupsr::GtupsrSpec>;
///General PWM Timer Up Count Source Select Register
pub mod gtupsr;
/**GTDNSR (rw) register accessor: General PWM Timer Down Count Source Select Register

You can [`read`](crate::Reg::read) this register and get [`gtdnsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdnsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdnsr`] module*/
#[doc(alias = "GTDNSR")]
pub type Gtdnsr = crate::Reg<gtdnsr::GtdnsrSpec>;
///General PWM Timer Down Count Source Select Register
pub mod gtdnsr;
/**GTICASR (rw) register accessor: General PWM Timer Input Capture Source Select Register A

You can [`read`](crate::Reg::read) this register and get [`gticasr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticasr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gticasr`] module*/
#[doc(alias = "GTICASR")]
pub type Gticasr = crate::Reg<gticasr::GticasrSpec>;
///General PWM Timer Input Capture Source Select Register A
pub mod gticasr;
/**GTICBSR (rw) register accessor: General PWM Timer Input Capture Source Select Register B

You can [`read`](crate::Reg::read) this register and get [`gticbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gticbsr`] module*/
#[doc(alias = "GTICBSR")]
pub type Gticbsr = crate::Reg<gticbsr::GticbsrSpec>;
///General PWM Timer Input Capture Source Select Register B
pub mod gticbsr;
/**GTCR (rw) register accessor: General PWM Timer Control Register

You can [`read`](crate::Reg::read) this register and get [`gtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtcr`] module*/
#[doc(alias = "GTCR")]
pub type Gtcr = crate::Reg<gtcr::GtcrSpec>;
///General PWM Timer Control Register
pub mod gtcr;
/**GTUDDTYC (rw) register accessor: General PWM Timer Count Direction and Duty Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtuddtyc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtuddtyc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtuddtyc`] module*/
#[doc(alias = "GTUDDTYC")]
pub type Gtuddtyc = crate::Reg<gtuddtyc::GtuddtycSpec>;
///General PWM Timer Count Direction and Duty Setting Register
pub mod gtuddtyc;
/**GTIOR (rw) register accessor: General PWM Timer I/O Control Register

You can [`read`](crate::Reg::read) this register and get [`gtior::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtior::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtior`] module*/
#[doc(alias = "GTIOR")]
pub type Gtior = crate::Reg<gtior::GtiorSpec>;
///General PWM Timer I/O Control Register
pub mod gtior;
/**GTINTAD (rw) register accessor: General PWM Timer Interrupt Output Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtintad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtintad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtintad`] module*/
#[doc(alias = "GTINTAD")]
pub type Gtintad = crate::Reg<gtintad::GtintadSpec>;
///General PWM Timer Interrupt Output Setting Register
pub mod gtintad;
/**GTST (rw) register accessor: General PWM Timer Status Register

You can [`read`](crate::Reg::read) this register and get [`gtst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtst`] module*/
#[doc(alias = "GTST")]
pub type Gtst = crate::Reg<gtst::GtstSpec>;
///General PWM Timer Status Register
pub mod gtst;
/**GTBER (rw) register accessor: General PWM Timer Buffer Enable Register

You can [`read`](crate::Reg::read) this register and get [`gtber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtber`] module*/
#[doc(alias = "GTBER")]
pub type Gtber = crate::Reg<gtber::GtberSpec>;
///General PWM Timer Buffer Enable Register
pub mod gtber;
/**GTCNT (rw) register accessor: General PWM Timer Counter

You can [`read`](crate::Reg::read) this register and get [`gtcnt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtcnt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtcnt`] module*/
#[doc(alias = "GTCNT")]
pub type Gtcnt = crate::Reg<gtcnt::GtcntSpec>;
///General PWM Timer Counter
pub mod gtcnt;
/**GTCCRA (rw) register accessor: General PWM Timer Compare Capture Register A

You can [`read`](crate::Reg::read) this register and get [`gtccra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccra`] module*/
#[doc(alias = "GTCCRA")]
pub type Gtccra = crate::Reg<gtccra::GtccraSpec>;
///General PWM Timer Compare Capture Register A
pub mod gtccra;
/**GTCCRB (rw) register accessor: General PWM Timer Compare Capture Register B

You can [`read`](crate::Reg::read) this register and get [`gtccrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccrb`] module*/
#[doc(alias = "GTCCRB")]
pub type Gtccrb = crate::Reg<gtccrb::GtccrbSpec>;
///General PWM Timer Compare Capture Register B
pub mod gtccrb;
/**GTCCRC (rw) register accessor: General PWM Timer Compare Capture Register C

You can [`read`](crate::Reg::read) this register and get [`gtccrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccrc`] module*/
#[doc(alias = "GTCCRC")]
pub type Gtccrc = crate::Reg<gtccrc::GtccrcSpec>;
///General PWM Timer Compare Capture Register C
pub mod gtccrc;
/**GTCCRE (rw) register accessor: General PWM Timer Compare Capture Register E

You can [`read`](crate::Reg::read) this register and get [`gtccre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccre`] module*/
#[doc(alias = "GTCCRE")]
pub type Gtccre = crate::Reg<gtccre::GtccreSpec>;
///General PWM Timer Compare Capture Register E
pub mod gtccre;
/**GTCCRD (rw) register accessor: General PWM Timer Compare Capture Register D

You can [`read`](crate::Reg::read) this register and get [`gtccrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccrd`] module*/
#[doc(alias = "GTCCRD")]
pub type Gtccrd = crate::Reg<gtccrd::GtccrdSpec>;
///General PWM Timer Compare Capture Register D
pub mod gtccrd;
/**GTCCRF (rw) register accessor: General PWM Timer Compare Capture Register F

You can [`read`](crate::Reg::read) this register and get [`gtccrf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtccrf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtccrf`] module*/
#[doc(alias = "GTCCRF")]
pub type Gtccrf = crate::Reg<gtccrf::GtccrfSpec>;
///General PWM Timer Compare Capture Register F
pub mod gtccrf;
/**GTPR (rw) register accessor: General PWM Timer Cycle Setting Register

You can [`read`](crate::Reg::read) this register and get [`gtpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtpr`] module*/
#[doc(alias = "GTPR")]
pub type Gtpr = crate::Reg<gtpr::GtprSpec>;
///General PWM Timer Cycle Setting Register
pub mod gtpr;
/**GTPBR (rw) register accessor: General PWM Timer Cycle Setting Buffer Register

You can [`read`](crate::Reg::read) this register and get [`gtpbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtpbr`] module*/
#[doc(alias = "GTPBR")]
pub type Gtpbr = crate::Reg<gtpbr::GtpbrSpec>;
///General PWM Timer Cycle Setting Buffer Register
pub mod gtpbr;
/**GTDTCR (rw) register accessor: General PWM Timer Dead Time Control Register

You can [`read`](crate::Reg::read) this register and get [`gtdtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdtcr`] module*/
#[doc(alias = "GTDTCR")]
pub type Gtdtcr = crate::Reg<gtdtcr::GtdtcrSpec>;
///General PWM Timer Dead Time Control Register
pub mod gtdtcr;
/**GTDVU (rw) register accessor: General PWM Timer Dead Time Value Register U

You can [`read`](crate::Reg::read) this register and get [`gtdvu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtdvu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtdvu`] module*/
#[doc(alias = "GTDVU")]
pub type Gtdvu = crate::Reg<gtdvu::GtdvuSpec>;
///General PWM Timer Dead Time Value Register U
pub mod gtdvu;
/**GTICLF (rw) register accessor: General PWM Timer Inter Channel Logical Operation Function Setting Register

You can [`read`](crate::Reg::read) this register and get [`gticlf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gticlf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gticlf`] module*/
#[doc(alias = "GTICLF")]
pub type Gticlf = crate::Reg<gticlf::GticlfSpec>;
///General PWM Timer Inter Channel Logical Operation Function Setting Register
pub mod gticlf;
/**GTPC (rw) register accessor: General PWM Timer Period Count Register

You can [`read`](crate::Reg::read) this register and get [`gtpc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtpc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtpc`] module*/
#[doc(alias = "GTPC")]
pub type Gtpc = crate::Reg<gtpc::GtpcSpec>;
///General PWM Timer Period Count Register
pub mod gtpc;
/**GTSECSR (rw) register accessor: General PWM Timer Operation Enable Bit Simultaneous Control Channel Select Register

You can [`read`](crate::Reg::read) this register and get [`gtsecsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtsecsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtsecsr`] module*/
#[doc(alias = "GTSECSR")]
pub type Gtsecsr = crate::Reg<gtsecsr::GtsecsrSpec>;
///General PWM Timer Operation Enable Bit Simultaneous Control Channel Select Register
pub mod gtsecsr;
/**GTSECR (rw) register accessor: General PWM Timer Operation Enable Bit Simultaneous Control Register

You can [`read`](crate::Reg::read) this register and get [`gtsecr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gtsecr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@gtsecr`] module*/
#[doc(alias = "GTSECR")]
pub type Gtsecr = crate::Reg<gtsecr::GtsecrSpec>;
///General PWM Timer Operation Enable Bit Simultaneous Control Register
pub mod gtsecr;
