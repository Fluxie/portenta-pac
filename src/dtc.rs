#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dtccr: Dtccr,
    _reserved1: [u8; 0x03],
    dtcvbr: Dtcvbr,
    _reserved2: [u8; 0x04],
    dtcst: Dtcst,
    _reserved3: [u8; 0x01],
    dtcsts: Dtcsts,
    dtccr_sec: DtccrSec,
    _reserved5: [u8; 0x03],
    dtcvbr_sec: DtcvbrSec,
    _reserved6: [u8; 0x08],
    dtevr: Dtevr,
}
impl RegisterBlock {
    ///0x00 - DTC Control Register
    #[inline(always)]
    pub const fn dtccr(&self) -> &Dtccr {
        &self.dtccr
    }
    ///0x04 - DTC Vector Base Register
    #[inline(always)]
    pub const fn dtcvbr(&self) -> &Dtcvbr {
        &self.dtcvbr
    }
    ///0x0c - DTC Module Start Register
    #[inline(always)]
    pub const fn dtcst(&self) -> &Dtcst {
        &self.dtcst
    }
    ///0x0e - DTC Status Register
    #[inline(always)]
    pub const fn dtcsts(&self) -> &Dtcsts {
        &self.dtcsts
    }
    ///0x10 - DTC Control Register for secure Region
    #[inline(always)]
    pub const fn dtccr_sec(&self) -> &DtccrSec {
        &self.dtccr_sec
    }
    ///0x14 - DTC Vector Base Register for secure Region
    #[inline(always)]
    pub const fn dtcvbr_sec(&self) -> &DtcvbrSec {
        &self.dtcvbr_sec
    }
    ///0x20 - DTC Error Vector Register
    #[inline(always)]
    pub const fn dtevr(&self) -> &Dtevr {
        &self.dtevr
    }
}
/**DTCCR (rw) register accessor: DTC Control Register

You can [`read`](crate::Reg::read) this register and get [`dtccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtccr`] module*/
#[doc(alias = "DTCCR")]
pub type Dtccr = crate::Reg<dtccr::DtccrSpec>;
///DTC Control Register
pub mod dtccr;
/**DTCVBR (rw) register accessor: DTC Vector Base Register

You can [`read`](crate::Reg::read) this register and get [`dtcvbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcvbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtcvbr`] module*/
#[doc(alias = "DTCVBR")]
pub type Dtcvbr = crate::Reg<dtcvbr::DtcvbrSpec>;
///DTC Vector Base Register
pub mod dtcvbr;
/**DTCST (rw) register accessor: DTC Module Start Register

You can [`read`](crate::Reg::read) this register and get [`dtcst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtcst`] module*/
#[doc(alias = "DTCST")]
pub type Dtcst = crate::Reg<dtcst::DtcstSpec>;
///DTC Module Start Register
pub mod dtcst;
/**DTCSTS (r) register accessor: DTC Status Register

You can [`read`](crate::Reg::read) this register and get [`dtcsts::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtcsts`] module*/
#[doc(alias = "DTCSTS")]
pub type Dtcsts = crate::Reg<dtcsts::DtcstsSpec>;
///DTC Status Register
pub mod dtcsts;
/**DTCCR_SEC (rw) register accessor: DTC Control Register for secure Region

You can [`read`](crate::Reg::read) this register and get [`dtccr_sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtccr_sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtccr_sec`] module*/
#[doc(alias = "DTCCR_SEC")]
pub type DtccrSec = crate::Reg<dtccr_sec::DtccrSecSpec>;
///DTC Control Register for secure Region
pub mod dtccr_sec;
/**DTCVBR_SEC (rw) register accessor: DTC Vector Base Register for secure Region

You can [`read`](crate::Reg::read) this register and get [`dtcvbr_sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtcvbr_sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtcvbr_sec`] module*/
#[doc(alias = "DTCVBR_SEC")]
pub type DtcvbrSec = crate::Reg<dtcvbr_sec::DtcvbrSecSpec>;
///DTC Vector Base Register for secure Region
pub mod dtcvbr_sec;
/**DTEVR (rw) register accessor: DTC Error Vector Register

You can [`read`](crate::Reg::read) this register and get [`dtevr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dtevr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dtevr`] module*/
#[doc(alias = "DTEVR")]
pub type Dtevr = crate::Reg<dtevr::DtevrSpec>;
///DTC Error Vector Register
pub mod dtevr;
