#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    cacr0: Cacr0,
    cacr1: Cacr1,
    cacr2: Cacr2,
    caicr: Caicr,
    castr: Castr,
    _reserved5: [u8; 0x01],
    caulvr: Caulvr,
    callvr: Callvr,
    cacntbr: Cacntbr,
}
impl RegisterBlock {
    ///0x00 - CAC Control Register 0
    #[inline(always)]
    pub const fn cacr0(&self) -> &Cacr0 {
        &self.cacr0
    }
    ///0x01 - CAC Control Register 1
    #[inline(always)]
    pub const fn cacr1(&self) -> &Cacr1 {
        &self.cacr1
    }
    ///0x02 - CAC Control Register 2
    #[inline(always)]
    pub const fn cacr2(&self) -> &Cacr2 {
        &self.cacr2
    }
    ///0x03 - CAC Interrupt Control Register
    #[inline(always)]
    pub const fn caicr(&self) -> &Caicr {
        &self.caicr
    }
    ///0x04 - CAC Status Register
    #[inline(always)]
    pub const fn castr(&self) -> &Castr {
        &self.castr
    }
    ///0x06 - CAC Upper-Limit Value Setting Register
    #[inline(always)]
    pub const fn caulvr(&self) -> &Caulvr {
        &self.caulvr
    }
    ///0x08 - CAC Lower-Limit Value Setting Register
    #[inline(always)]
    pub const fn callvr(&self) -> &Callvr {
        &self.callvr
    }
    ///0x0a - CAC Counter Buffer Register
    #[inline(always)]
    pub const fn cacntbr(&self) -> &Cacntbr {
        &self.cacntbr
    }
}
/**CACR0 (rw) register accessor: CAC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`cacr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacr0`] module*/
#[doc(alias = "CACR0")]
pub type Cacr0 = crate::Reg<cacr0::Cacr0Spec>;
///CAC Control Register 0
pub mod cacr0;
/**CACR1 (rw) register accessor: CAC Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cacr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacr1`] module*/
#[doc(alias = "CACR1")]
pub type Cacr1 = crate::Reg<cacr1::Cacr1Spec>;
///CAC Control Register 1
pub mod cacr1;
/**CACR2 (rw) register accessor: CAC Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cacr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cacr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacr2`] module*/
#[doc(alias = "CACR2")]
pub type Cacr2 = crate::Reg<cacr2::Cacr2Spec>;
///CAC Control Register 2
pub mod cacr2;
/**CAICR (rw) register accessor: CAC Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`caicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@caicr`] module*/
#[doc(alias = "CAICR")]
pub type Caicr = crate::Reg<caicr::CaicrSpec>;
///CAC Interrupt Control Register
pub mod caicr;
/**CASTR (r) register accessor: CAC Status Register

You can [`read`](crate::Reg::read) this register and get [`castr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@castr`] module*/
#[doc(alias = "CASTR")]
pub type Castr = crate::Reg<castr::CastrSpec>;
///CAC Status Register
pub mod castr;
/**CAULVR (rw) register accessor: CAC Upper-Limit Value Setting Register

You can [`read`](crate::Reg::read) this register and get [`caulvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caulvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@caulvr`] module*/
#[doc(alias = "CAULVR")]
pub type Caulvr = crate::Reg<caulvr::CaulvrSpec>;
///CAC Upper-Limit Value Setting Register
pub mod caulvr;
/**CALLVR (rw) register accessor: CAC Lower-Limit Value Setting Register

You can [`read`](crate::Reg::read) this register and get [`callvr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`callvr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@callvr`] module*/
#[doc(alias = "CALLVR")]
pub type Callvr = crate::Reg<callvr::CallvrSpec>;
///CAC Lower-Limit Value Setting Register
pub mod callvr;
/**CACNTBR (r) register accessor: CAC Counter Buffer Register

You can [`read`](crate::Reg::read) this register and get [`cacntbr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cacntbr`] module*/
#[doc(alias = "CACNTBR")]
pub type Cacntbr = crate::Reg<cacntbr::CacntbrSpec>;
///CAC Counter Buffer Register
pub mod cacntbr;
