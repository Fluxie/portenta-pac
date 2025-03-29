#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    fcachee: Fcachee,
    _reserved1: [u8; 0x02],
    fcacheiv: Fcacheiv,
    _reserved2: [u8; 0x16],
    flwt: Flwt,
    _reserved3: [u8; 0x23],
    fsar: Fsar,
}
impl RegisterBlock {
    ///0x00 - Flash Cache Enable Register
    #[inline(always)]
    pub const fn fcachee(&self) -> &Fcachee {
        &self.fcachee
    }
    ///0x04 - Flash Cache Invalidate Register
    #[inline(always)]
    pub const fn fcacheiv(&self) -> &Fcacheiv {
        &self.fcacheiv
    }
    ///0x1c - Flash Wait Cycle Register
    #[inline(always)]
    pub const fn flwt(&self) -> &Flwt {
        &self.flwt
    }
    ///0x40 - Flash Security Attribution Register
    #[inline(always)]
    pub const fn fsar(&self) -> &Fsar {
        &self.fsar
    }
}
/**FCACHEE (rw) register accessor: Flash Cache Enable Register

You can [`read`](crate::Reg::read) this register and get [`fcachee::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcachee::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcachee`] module*/
#[doc(alias = "FCACHEE")]
pub type Fcachee = crate::Reg<fcachee::FcacheeSpec>;
///Flash Cache Enable Register
pub mod fcachee;
/**FCACHEIV (rw) register accessor: Flash Cache Invalidate Register

You can [`read`](crate::Reg::read) this register and get [`fcacheiv::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcacheiv::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcacheiv`] module*/
#[doc(alias = "FCACHEIV")]
pub type Fcacheiv = crate::Reg<fcacheiv::FcacheivSpec>;
///Flash Cache Invalidate Register
pub mod fcacheiv;
/**FLWT (rw) register accessor: Flash Wait Cycle Register

You can [`read`](crate::Reg::read) this register and get [`flwt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`flwt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@flwt`] module*/
#[doc(alias = "FLWT")]
pub type Flwt = crate::Reg<flwt::FlwtSpec>;
///Flash Wait Cycle Register
pub mod flwt;
/**FSAR (rw) register accessor: Flash Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`fsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fsar`] module*/
#[doc(alias = "FSAR")]
pub type Fsar = crate::Reg<fsar::FsarSpec>;
///Flash Security Attribution Register
pub mod fsar;
