#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    poegga: Poegga,
    _reserved1: [u8; 0xfc],
    poeggb: Poeggb,
    _reserved2: [u8; 0xfc],
    poeggc: Poeggc,
    _reserved3: [u8; 0xfc],
    poeggd: Poeggd,
}
impl RegisterBlock {
    ///0x00 - POEG Group A Setting Register
    #[inline(always)]
    pub const fn poegga(&self) -> &Poegga {
        &self.poegga
    }
    ///0x100 - POEG Group B Setting Register
    #[inline(always)]
    pub const fn poeggb(&self) -> &Poeggb {
        &self.poeggb
    }
    ///0x200 - POEG Group C Setting Register
    #[inline(always)]
    pub const fn poeggc(&self) -> &Poeggc {
        &self.poeggc
    }
    ///0x300 - POEG Group D Setting Register
    #[inline(always)]
    pub const fn poeggd(&self) -> &Poeggd {
        &self.poeggd
    }
}
/**POEGGA (rw) register accessor: POEG Group A Setting Register

You can [`read`](crate::Reg::read) this register and get [`poegga::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poegga::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@poegga`] module*/
#[doc(alias = "POEGGA")]
pub type Poegga = crate::Reg<poegga::PoeggaSpec>;
///POEG Group A Setting Register
pub mod poegga;
/**POEGGB (rw) register accessor: POEG Group B Setting Register

You can [`read`](crate::Reg::read) this register and get [`poeggb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poeggb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@poeggb`] module*/
#[doc(alias = "POEGGB")]
pub type Poeggb = crate::Reg<poeggb::PoeggbSpec>;
///POEG Group B Setting Register
pub mod poeggb;
/**POEGGC (rw) register accessor: POEG Group C Setting Register

You can [`read`](crate::Reg::read) this register and get [`poeggc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poeggc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@poeggc`] module*/
#[doc(alias = "POEGGC")]
pub type Poeggc = crate::Reg<poeggc::PoeggcSpec>;
///POEG Group C Setting Register
pub mod poeggc;
/**POEGGD (rw) register accessor: POEG Group D Setting Register

You can [`read`](crate::Reg::read) this register and get [`poeggd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`poeggd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@poeggd`] module*/
#[doc(alias = "POEGGD")]
pub type Poeggd = crate::Reg<poeggd::PoeggdSpec>;
///POEG Group D Setting Register
pub mod poeggd;
