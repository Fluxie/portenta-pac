#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved_0_pdr: [u8; 0x04],
    _reserved_1_pidr: [u8; 0x04],
    _reserved_2_porr: [u8; 0x04],
}
impl RegisterBlock {
    ///0x00 - Port Control Register 1
    #[inline(always)]
    pub const fn podr(&self) -> &Podr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x00 - Port Control Register 1
    #[inline(always)]
    pub const fn pcntr1(&self) -> &Pcntr1 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().cast() }
    }
    ///0x02 - Port Control Register 1
    #[inline(always)]
    pub const fn pdr(&self) -> &Pdr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).cast() }
    }
    ///0x04 - Port Control Register 2
    #[inline(always)]
    pub const fn pcntr2(&self) -> &Pcntr2 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x06 - Port Control Register 2
    #[inline(always)]
    pub const fn pidr(&self) -> &Pidr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6).cast() }
    }
    ///0x08 - Port Control Register 3
    #[inline(always)]
    pub const fn porr(&self) -> &Porr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - Port Control Register 3
    #[inline(always)]
    pub const fn pcntr3(&self) -> &Pcntr3 {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x0a - Port Control Register 3
    #[inline(always)]
    pub const fn posr(&self) -> &Posr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(10).cast() }
    }
}
/**PCNTR1 (rw) register accessor: Port Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pcntr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcntr1`] module*/
#[doc(alias = "PCNTR1")]
pub type Pcntr1 = crate::Reg<pcntr1::Pcntr1Spec>;
///Port Control Register 1
pub mod pcntr1;
/**PODR (rw) register accessor: Port Control Register 1

You can [`read`](crate::Reg::read) this register and get [`podr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`podr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@podr`] module*/
#[doc(alias = "PODR")]
pub type Podr = crate::Reg<podr::PodrSpec>;
///Port Control Register 1
pub mod podr;
/**PDR (rw) register accessor: Port Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pdr`] module*/
#[doc(alias = "PDR")]
pub type Pdr = crate::Reg<pdr::PdrSpec>;
///Port Control Register 1
pub mod pdr;
/**PCNTR2 (r) register accessor: Port Control Register 2

You can [`read`](crate::Reg::read) this register and get [`pcntr2::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcntr2`] module*/
#[doc(alias = "PCNTR2")]
pub type Pcntr2 = crate::Reg<pcntr2::Pcntr2Spec>;
///Port Control Register 2
pub mod pcntr2;
/**PIDR (r) register accessor: Port Control Register 2

You can [`read`](crate::Reg::read) this register and get [`pidr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pidr`] module*/
#[doc(alias = "PIDR")]
pub type Pidr = crate::Reg<pidr::PidrSpec>;
///Port Control Register 2
pub mod pidr;
/**PCNTR3 (w) register accessor: Port Control Register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pcntr3::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pcntr3`] module*/
#[doc(alias = "PCNTR3")]
pub type Pcntr3 = crate::Reg<pcntr3::Pcntr3Spec>;
///Port Control Register 3
pub mod pcntr3;
/**PORR (w) register accessor: Port Control Register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@porr`] module*/
#[doc(alias = "PORR")]
pub type Porr = crate::Reg<porr::PorrSpec>;
///Port Control Register 3
pub mod porr;
/**POSR (w) register accessor: Port Control Register 3

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`posr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@posr`] module*/
#[doc(alias = "POSR")]
pub type Posr = crate::Reg<posr::PosrSpec>;
///Port Control Register 3
pub mod posr;
