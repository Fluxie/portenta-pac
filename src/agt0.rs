#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    agt: Agt,
    agtcma: Agtcma,
    agtcmb: Agtcmb,
    _reserved3: [u8; 0x02],
    agtcr: Agtcr,
    agtmr1: Agtmr1,
    agtmr2: Agtmr2,
    _reserved6: [u8; 0x01],
    agtioc: Agtioc,
    agtisr: Agtisr,
    agtcmsr: Agtcmsr,
    agtiosel: Agtiosel,
}
impl RegisterBlock {
    ///0x00 - AGT Counter Register
    #[inline(always)]
    pub const fn agt(&self) -> &Agt {
        &self.agt
    }
    ///0x02 - AGT Compare Match A Register
    #[inline(always)]
    pub const fn agtcma(&self) -> &Agtcma {
        &self.agtcma
    }
    ///0x04 - AGT Compare Match B Register
    #[inline(always)]
    pub const fn agtcmb(&self) -> &Agtcmb {
        &self.agtcmb
    }
    ///0x08 - AGT Control Register
    #[inline(always)]
    pub const fn agtcr(&self) -> &Agtcr {
        &self.agtcr
    }
    ///0x09 - AGT Mode Register 1
    #[inline(always)]
    pub const fn agtmr1(&self) -> &Agtmr1 {
        &self.agtmr1
    }
    ///0x0a - AGT Mode Register 2
    #[inline(always)]
    pub const fn agtmr2(&self) -> &Agtmr2 {
        &self.agtmr2
    }
    ///0x0c - AGT I/O Control Register
    #[inline(always)]
    pub const fn agtioc(&self) -> &Agtioc {
        &self.agtioc
    }
    ///0x0d - AGT Event Pin Select Register
    #[inline(always)]
    pub const fn agtisr(&self) -> &Agtisr {
        &self.agtisr
    }
    ///0x0e - AGT Compare Match Function Select Register
    #[inline(always)]
    pub const fn agtcmsr(&self) -> &Agtcmsr {
        &self.agtcmsr
    }
    ///0x0f - AGT Pin Select Register
    #[inline(always)]
    pub const fn agtiosel(&self) -> &Agtiosel {
        &self.agtiosel
    }
}
/**AGT (rw) register accessor: AGT Counter Register

You can [`read`](crate::Reg::read) this register and get [`agt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agt`] module*/
#[doc(alias = "AGT")]
pub type Agt = crate::Reg<agt::AgtSpec>;
///AGT Counter Register
pub mod agt;
/**AGTCMA (rw) register accessor: AGT Compare Match A Register

You can [`read`](crate::Reg::read) this register and get [`agtcma::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcma::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtcma`] module*/
#[doc(alias = "AGTCMA")]
pub type Agtcma = crate::Reg<agtcma::AgtcmaSpec>;
///AGT Compare Match A Register
pub mod agtcma;
/**AGTCMB (rw) register accessor: AGT Compare Match B Register

You can [`read`](crate::Reg::read) this register and get [`agtcmb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtcmb`] module*/
#[doc(alias = "AGTCMB")]
pub type Agtcmb = crate::Reg<agtcmb::AgtcmbSpec>;
///AGT Compare Match B Register
pub mod agtcmb;
/**AGTCR (rw) register accessor: AGT Control Register

You can [`read`](crate::Reg::read) this register and get [`agtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtcr`] module*/
#[doc(alias = "AGTCR")]
pub type Agtcr = crate::Reg<agtcr::AgtcrSpec>;
///AGT Control Register
pub mod agtcr;
/**AGTMR1 (rw) register accessor: AGT Mode Register 1

You can [`read`](crate::Reg::read) this register and get [`agtmr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtmr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtmr1`] module*/
#[doc(alias = "AGTMR1")]
pub type Agtmr1 = crate::Reg<agtmr1::Agtmr1Spec>;
///AGT Mode Register 1
pub mod agtmr1;
/**AGTMR2 (rw) register accessor: AGT Mode Register 2

You can [`read`](crate::Reg::read) this register and get [`agtmr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtmr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtmr2`] module*/
#[doc(alias = "AGTMR2")]
pub type Agtmr2 = crate::Reg<agtmr2::Agtmr2Spec>;
///AGT Mode Register 2
pub mod agtmr2;
/**AGTIOC (rw) register accessor: AGT I/O Control Register

You can [`read`](crate::Reg::read) this register and get [`agtioc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtioc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtioc`] module*/
#[doc(alias = "AGTIOC")]
pub type Agtioc = crate::Reg<agtioc::AgtiocSpec>;
///AGT I/O Control Register
pub mod agtioc;
/**AGTISR (rw) register accessor: AGT Event Pin Select Register

You can [`read`](crate::Reg::read) this register and get [`agtisr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtisr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtisr`] module*/
#[doc(alias = "AGTISR")]
pub type Agtisr = crate::Reg<agtisr::AgtisrSpec>;
///AGT Event Pin Select Register
pub mod agtisr;
/**AGTCMSR (rw) register accessor: AGT Compare Match Function Select Register

You can [`read`](crate::Reg::read) this register and get [`agtcmsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtcmsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtcmsr`] module*/
#[doc(alias = "AGTCMSR")]
pub type Agtcmsr = crate::Reg<agtcmsr::AgtcmsrSpec>;
///AGT Compare Match Function Select Register
pub mod agtcmsr;
/**AGTIOSEL (rw) register accessor: AGT Pin Select Register

You can [`read`](crate::Reg::read) this register and get [`agtiosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`agtiosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@agtiosel`] module*/
#[doc(alias = "AGTIOSEL")]
pub type Agtiosel = crate::Reg<agtiosel::AgtioselSpec>;
///AGT Pin Select Register
pub mod agtiosel;
