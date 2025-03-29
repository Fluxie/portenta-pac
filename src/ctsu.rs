#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ctsucr0: Ctsucr0,
    ctsucr1: Ctsucr1,
    ctsusdprs: Ctsusdprs,
    ctsusst: Ctsusst,
    ctsumch0: Ctsumch0,
    ctsumch1: Ctsumch1,
    ctsuchac0: Ctsuchac0,
    ctsuchac1: Ctsuchac1,
    ctsuchac2: Ctsuchac2,
    _reserved9: [u8; 0x02],
    ctsuchtrc0: Ctsuchtrc0,
    ctsuchtrc1: Ctsuchtrc1,
    ctsuchtrc2: Ctsuchtrc2,
    _reserved12: [u8; 0x02],
    ctsudclkc: Ctsudclkc,
    ctsust: Ctsust,
    ctsussc: Ctsussc,
    ctsuso0: Ctsuso0,
    ctsuso1: Ctsuso1,
    ctsusc: Ctsusc,
    ctsurc: Ctsurc,
    ctsuerrs: Ctsuerrs,
    _reserved20: [u8; 0x02],
    ctsutrmr: Ctsutrmr,
}
impl RegisterBlock {
    ///0x00 - CTSU Control Register 0
    #[inline(always)]
    pub const fn ctsucr0(&self) -> &Ctsucr0 {
        &self.ctsucr0
    }
    ///0x01 - CTSU Control Register 1
    #[inline(always)]
    pub const fn ctsucr1(&self) -> &Ctsucr1 {
        &self.ctsucr1
    }
    ///0x02 - CTSU Synchronous Noise Reduction Setting Register
    #[inline(always)]
    pub const fn ctsusdprs(&self) -> &Ctsusdprs {
        &self.ctsusdprs
    }
    ///0x03 - CTSU Sensor Stabilization Wait Control Register
    #[inline(always)]
    pub const fn ctsusst(&self) -> &Ctsusst {
        &self.ctsusst
    }
    ///0x04 - CTSU Measurement Channel Register 0
    #[inline(always)]
    pub const fn ctsumch0(&self) -> &Ctsumch0 {
        &self.ctsumch0
    }
    ///0x05 - CTSU Measurement Channel Register 1
    #[inline(always)]
    pub const fn ctsumch1(&self) -> &Ctsumch1 {
        &self.ctsumch1
    }
    ///0x06 - CTSU Channel Enable Control Register 0
    #[inline(always)]
    pub const fn ctsuchac0(&self) -> &Ctsuchac0 {
        &self.ctsuchac0
    }
    ///0x07 - CTSU Channel Enable Control Register 1
    #[inline(always)]
    pub const fn ctsuchac1(&self) -> &Ctsuchac1 {
        &self.ctsuchac1
    }
    ///0x08 - CTSU Channel Enable Control Register 2
    #[inline(always)]
    pub const fn ctsuchac2(&self) -> &Ctsuchac2 {
        &self.ctsuchac2
    }
    ///0x0b - CTSU Channel Transmit/Receive Control Register 0
    #[inline(always)]
    pub const fn ctsuchtrc0(&self) -> &Ctsuchtrc0 {
        &self.ctsuchtrc0
    }
    ///0x0c - CTSU Channel Transmit/Receive Control Register 1
    #[inline(always)]
    pub const fn ctsuchtrc1(&self) -> &Ctsuchtrc1 {
        &self.ctsuchtrc1
    }
    ///0x0d - CTSU Channel Transmit/Receive Control Register 2
    #[inline(always)]
    pub const fn ctsuchtrc2(&self) -> &Ctsuchtrc2 {
        &self.ctsuchtrc2
    }
    ///0x10 - CTSU High-Pass Noise Reduction Control Register
    #[inline(always)]
    pub const fn ctsudclkc(&self) -> &Ctsudclkc {
        &self.ctsudclkc
    }
    ///0x11 - CTSU Status Register
    #[inline(always)]
    pub const fn ctsust(&self) -> &Ctsust {
        &self.ctsust
    }
    ///0x12 - CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register
    #[inline(always)]
    pub const fn ctsussc(&self) -> &Ctsussc {
        &self.ctsussc
    }
    ///0x14 - CTSU Sensor Offset Register 0
    #[inline(always)]
    pub const fn ctsuso0(&self) -> &Ctsuso0 {
        &self.ctsuso0
    }
    ///0x16 - CTSU Sensor Offset Register 1
    #[inline(always)]
    pub const fn ctsuso1(&self) -> &Ctsuso1 {
        &self.ctsuso1
    }
    ///0x18 - CTSU Sensor Counter
    #[inline(always)]
    pub const fn ctsusc(&self) -> &Ctsusc {
        &self.ctsusc
    }
    ///0x1a - CTSU Reference Counter
    #[inline(always)]
    pub const fn ctsurc(&self) -> &Ctsurc {
        &self.ctsurc
    }
    ///0x1c - CTSU Error Status Register
    #[inline(always)]
    pub const fn ctsuerrs(&self) -> &Ctsuerrs {
        &self.ctsuerrs
    }
    ///0x20 - CTSU Reference Current Calibration Register
    #[inline(always)]
    pub const fn ctsutrmr(&self) -> &Ctsutrmr {
        &self.ctsutrmr
    }
}
/**CTSUCR0 (rw) register accessor: CTSU Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsucr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsucr0`] module*/
#[doc(alias = "CTSUCR0")]
pub type Ctsucr0 = crate::Reg<ctsucr0::Ctsucr0Spec>;
///CTSU Control Register 0
pub mod ctsucr0;
/**CTSUCR1 (rw) register accessor: CTSU Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsucr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsucr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsucr1`] module*/
#[doc(alias = "CTSUCR1")]
pub type Ctsucr1 = crate::Reg<ctsucr1::Ctsucr1Spec>;
///CTSU Control Register 1
pub mod ctsucr1;
/**CTSUSDPRS (rw) register accessor: CTSU Synchronous Noise Reduction Setting Register

You can [`read`](crate::Reg::read) this register and get [`ctsusdprs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusdprs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsusdprs`] module*/
#[doc(alias = "CTSUSDPRS")]
pub type Ctsusdprs = crate::Reg<ctsusdprs::CtsusdprsSpec>;
///CTSU Synchronous Noise Reduction Setting Register
pub mod ctsusdprs;
/**CTSUSST (rw) register accessor: CTSU Sensor Stabilization Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsusst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsusst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsusst`] module*/
#[doc(alias = "CTSUSST")]
pub type Ctsusst = crate::Reg<ctsusst::CtsusstSpec>;
///CTSU Sensor Stabilization Wait Control Register
pub mod ctsusst;
/**CTSUMCH0 (rw) register accessor: CTSU Measurement Channel Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsumch0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsumch0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsumch0`] module*/
#[doc(alias = "CTSUMCH0")]
pub type Ctsumch0 = crate::Reg<ctsumch0::Ctsumch0Spec>;
///CTSU Measurement Channel Register 0
pub mod ctsumch0;
/**CTSUMCH1 (r) register accessor: CTSU Measurement Channel Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsumch1::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsumch1`] module*/
#[doc(alias = "CTSUMCH1")]
pub type Ctsumch1 = crate::Reg<ctsumch1::Ctsumch1Spec>;
///CTSU Measurement Channel Register 1
pub mod ctsumch1;
/**CTSUCHAC0 (rw) register accessor: CTSU Channel Enable Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuchac0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchac0`] module*/
#[doc(alias = "CTSUCHAC0")]
pub type Ctsuchac0 = crate::Reg<ctsuchac0::Ctsuchac0Spec>;
///CTSU Channel Enable Control Register 0
pub mod ctsuchac0;
/**CTSUCHAC1 (rw) register accessor: CTSU Channel Enable Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuchac1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchac1`] module*/
#[doc(alias = "CTSUCHAC1")]
pub type Ctsuchac1 = crate::Reg<ctsuchac1::Ctsuchac1Spec>;
///CTSU Channel Enable Control Register 1
pub mod ctsuchac1;
/**CTSUCHAC2 (rw) register accessor: CTSU Channel Enable Control Register 2

You can [`read`](crate::Reg::read) this register and get [`ctsuchac2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchac2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchac2`] module*/
#[doc(alias = "CTSUCHAC2")]
pub type Ctsuchac2 = crate::Reg<ctsuchac2::Ctsuchac2Spec>;
///CTSU Channel Enable Control Register 2
pub mod ctsuchac2;
/**CTSUCHTRC0 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchtrc0`] module*/
#[doc(alias = "CTSUCHTRC0")]
pub type Ctsuchtrc0 = crate::Reg<ctsuchtrc0::Ctsuchtrc0Spec>;
///CTSU Channel Transmit/Receive Control Register 0
pub mod ctsuchtrc0;
/**CTSUCHTRC1 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchtrc1`] module*/
#[doc(alias = "CTSUCHTRC1")]
pub type Ctsuchtrc1 = crate::Reg<ctsuchtrc1::Ctsuchtrc1Spec>;
///CTSU Channel Transmit/Receive Control Register 1
pub mod ctsuchtrc1;
/**CTSUCHTRC2 (rw) register accessor: CTSU Channel Transmit/Receive Control Register 2

You can [`read`](crate::Reg::read) this register and get [`ctsuchtrc2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuchtrc2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuchtrc2`] module*/
#[doc(alias = "CTSUCHTRC2")]
pub type Ctsuchtrc2 = crate::Reg<ctsuchtrc2::Ctsuchtrc2Spec>;
///CTSU Channel Transmit/Receive Control Register 2
pub mod ctsuchtrc2;
/**CTSUDCLKC (rw) register accessor: CTSU High-Pass Noise Reduction Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsudclkc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsudclkc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsudclkc`] module*/
#[doc(alias = "CTSUDCLKC")]
pub type Ctsudclkc = crate::Reg<ctsudclkc::CtsudclkcSpec>;
///CTSU High-Pass Noise Reduction Control Register
pub mod ctsudclkc;
/**CTSUST (rw) register accessor: CTSU Status Register

You can [`read`](crate::Reg::read) this register and get [`ctsust::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsust::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsust`] module*/
#[doc(alias = "CTSUST")]
pub type Ctsust = crate::Reg<ctsust::CtsustSpec>;
///CTSU Status Register
pub mod ctsust;
/**CTSUSSC (rw) register accessor: CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register

You can [`read`](crate::Reg::read) this register and get [`ctsussc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsussc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsussc`] module*/
#[doc(alias = "CTSUSSC")]
pub type Ctsussc = crate::Reg<ctsussc::CtsusscSpec>;
///CTSU High-Pass Noise Reduction Spectrum Diffusion Control Register
pub mod ctsussc;
/**CTSUSO0 (rw) register accessor: CTSU Sensor Offset Register 0

You can [`read`](crate::Reg::read) this register and get [`ctsuso0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuso0`] module*/
#[doc(alias = "CTSUSO0")]
pub type Ctsuso0 = crate::Reg<ctsuso0::Ctsuso0Spec>;
///CTSU Sensor Offset Register 0
pub mod ctsuso0;
/**CTSUSO1 (rw) register accessor: CTSU Sensor Offset Register 1

You can [`read`](crate::Reg::read) this register and get [`ctsuso1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuso1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuso1`] module*/
#[doc(alias = "CTSUSO1")]
pub type Ctsuso1 = crate::Reg<ctsuso1::Ctsuso1Spec>;
///CTSU Sensor Offset Register 1
pub mod ctsuso1;
/**CTSUSC (r) register accessor: CTSU Sensor Counter

You can [`read`](crate::Reg::read) this register and get [`ctsusc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsusc`] module*/
#[doc(alias = "CTSUSC")]
pub type Ctsusc = crate::Reg<ctsusc::CtsuscSpec>;
///CTSU Sensor Counter
pub mod ctsusc;
/**CTSURC (r) register accessor: CTSU Reference Counter

You can [`read`](crate::Reg::read) this register and get [`ctsurc::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsurc`] module*/
#[doc(alias = "CTSURC")]
pub type Ctsurc = crate::Reg<ctsurc::CtsurcSpec>;
///CTSU Reference Counter
pub mod ctsurc;
/**CTSUERRS (rw) register accessor: CTSU Error Status Register

You can [`read`](crate::Reg::read) this register and get [`ctsuerrs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsuerrs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsuerrs`] module*/
#[doc(alias = "CTSUERRS")]
pub type Ctsuerrs = crate::Reg<ctsuerrs::CtsuerrsSpec>;
///CTSU Error Status Register
pub mod ctsuerrs;
/**CTSUTRMR (rw) register accessor: CTSU Reference Current Calibration Register

You can [`read`](crate::Reg::read) this register and get [`ctsutrmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctsutrmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ctsutrmr`] module*/
#[doc(alias = "CTSUTRMR")]
pub type Ctsutrmr = crate::Reg<ctsutrmr::CtsutrmrSpec>;
///CTSU Reference Current Calibration Register
pub mod ctsutrmr;
