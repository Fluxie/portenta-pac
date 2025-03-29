#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mmpuoad: Mmpuoad,
    _reserved1: [u8; 0x02],
    mmpuoadpt: Mmpuoadpt,
    _reserved2: [u8; 0xfa],
    mmpuendmac: Mmpuendmac,
    _reserved3: [u8; 0x02],
    mmpuenptdmac: Mmpuenptdmac,
    _reserved4: [u8; 0x02],
    mmpurptdmac: Mmpurptdmac,
    _reserved5: [u8; 0x02],
    mmpurptdmac_sec: MmpurptdmacSec,
    _reserved6: [u8; 0xf2],
    mmpuacdmac: (),
    _reserved7: [u8; 0x04],
    mmpusdmac: (),
    _reserved8: [u8; 0x04],
    mmpuedmac: (),
    _reserved9: [u8; 0x02f8],
    mmpuenedmac: Mmpuenedmac,
    _reserved10: [u8; 0x02],
    mmpuenptedmac: Mmpuenptedmac,
    _reserved11: [u8; 0x02],
    mmpurptedmac: Mmpurptedmac,
    _reserved12: [u8; 0xf6],
    mmpuacedmac: (),
    _reserved13: [u8; 0x04],
    mmpusedmac: (),
    _reserved14: [u8; 0x04],
    mmpueedmac: (),
}
impl RegisterBlock {
    ///0x00 - MMPU Operation After Detection Register
    #[inline(always)]
    pub const fn mmpuoad(&self) -> &Mmpuoad {
        &self.mmpuoad
    }
    ///0x04 - MMPU Operation After Detection Protect Register
    #[inline(always)]
    pub const fn mmpuoadpt(&self) -> &Mmpuoadpt {
        &self.mmpuoadpt
    }
    ///0x100 - MMPU Enable Register for DMAC
    #[inline(always)]
    pub const fn mmpuendmac(&self) -> &Mmpuendmac {
        &self.mmpuendmac
    }
    ///0x104 - MMPU Enable Protect Register for DMAC
    #[inline(always)]
    pub const fn mmpuenptdmac(&self) -> &Mmpuenptdmac {
        &self.mmpuenptdmac
    }
    ///0x108 - MMPU Regions Protect Register for DMAC
    #[inline(always)]
    pub const fn mmpurptdmac(&self) -> &Mmpurptdmac {
        &self.mmpurptdmac
    }
    ///0x10c - MMPU Regions Protect register for DMAC Secure
    #[inline(always)]
    pub const fn mmpurptdmac_sec(&self) -> &MmpurptdmacSec {
        &self.mmpurptdmac_sec
    }
    ///0x200..0x210 - MMPU Access Control Register for DMAC
    #[inline(always)]
    pub const fn mmpuacdmac(&self, n: usize) -> &Mmpuacdmac {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(512).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x200..0x210 - MMPU Access Control Register for DMAC
    #[inline(always)]
    pub fn mmpuacdmac_iter(&self) -> impl Iterator<Item = &Mmpuacdmac> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(512).add(16 * n).cast()
            })
    }
    ///0x204..0x224 - MMPU Start Address Register for DMAC
    #[inline(always)]
    pub const fn mmpusdmac(&self, n: usize) -> &Mmpusdmac {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(516).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x204..0x224 - MMPU Start Address Register for DMAC
    #[inline(always)]
    pub fn mmpusdmac_iter(&self) -> impl Iterator<Item = &Mmpusdmac> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(516).add(16 * n).cast()
            })
    }
    ///0x208..0x228 - MMPU End Address Register for DMAC
    #[inline(always)]
    pub const fn mmpuedmac(&self, n: usize) -> &Mmpuedmac {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(520).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x208..0x228 - MMPU End Address Register for DMAC
    #[inline(always)]
    pub fn mmpuedmac_iter(&self) -> impl Iterator<Item = &Mmpuedmac> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(520).add(16 * n).cast()
            })
    }
    ///0x500 - MMPU Enable Register for EDMAC
    #[inline(always)]
    pub const fn mmpuenedmac(&self) -> &Mmpuenedmac {
        &self.mmpuenedmac
    }
    ///0x504 - MMPU Enable Protect Register for EDMAC
    #[inline(always)]
    pub const fn mmpuenptedmac(&self) -> &Mmpuenptedmac {
        &self.mmpuenptedmac
    }
    ///0x508 - MMPU Regions Protect Register for EDMAC
    #[inline(always)]
    pub const fn mmpurptedmac(&self) -> &Mmpurptedmac {
        &self.mmpurptedmac
    }
    ///0x600..0x608 - MMPU Access Control Register for EDMAC
    #[inline(always)]
    pub const fn mmpuacedmac(&self, n: usize) -> &Mmpuacedmac {
        #[allow(clippy::no_effect)] [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1536).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x600..0x608 - MMPU Access Control Register for EDMAC
    #[inline(always)]
    pub fn mmpuacedmac_iter(&self) -> impl Iterator<Item = &Mmpuacedmac> {
        (0..4)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(1536).add(16 * n).cast()
            })
    }
    ///0x604..0x614 - MMPU Start Address Register for EDMAC
    #[inline(always)]
    pub const fn mmpusedmac(&self, n: usize) -> &Mmpusedmac {
        #[allow(clippy::no_effect)] [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1540).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x604..0x614 - MMPU Start Address Register for EDMAC
    #[inline(always)]
    pub fn mmpusedmac_iter(&self) -> impl Iterator<Item = &Mmpusedmac> {
        (0..4)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(1540).add(16 * n).cast()
            })
    }
    ///0x608..0x618 - MMPU End Address Register for EDMAC
    #[inline(always)]
    pub const fn mmpueedmac(&self, n: usize) -> &Mmpueedmac {
        #[allow(clippy::no_effect)] [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(1544).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x608..0x618 - MMPU End Address Register for EDMAC
    #[inline(always)]
    pub fn mmpueedmac_iter(&self) -> impl Iterator<Item = &Mmpueedmac> {
        (0..4)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(1544).add(16 * n).cast()
            })
    }
}
/**MMPUOAD (rw) register accessor: MMPU Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`mmpuoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuoad`] module*/
#[doc(alias = "MMPUOAD")]
pub type Mmpuoad = crate::Reg<mmpuoad::MmpuoadSpec>;
///MMPU Operation After Detection Register
pub mod mmpuoad;
/**MMPUOADPT (rw) register accessor: MMPU Operation After Detection Protect Register

You can [`read`](crate::Reg::read) this register and get [`mmpuoadpt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuoadpt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuoadpt`] module*/
#[doc(alias = "MMPUOADPT")]
pub type Mmpuoadpt = crate::Reg<mmpuoadpt::MmpuoadptSpec>;
///MMPU Operation After Detection Protect Register
pub mod mmpuoadpt;
/**MMPUENDMAC (rw) register accessor: MMPU Enable Register for DMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuendmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuendmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuendmac`] module*/
#[doc(alias = "MMPUENDMAC")]
pub type Mmpuendmac = crate::Reg<mmpuendmac::MmpuendmacSpec>;
///MMPU Enable Register for DMAC
pub mod mmpuendmac;
/**MMPUENPTDMAC (rw) register accessor: MMPU Enable Protect Register for DMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuenptdmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuenptdmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuenptdmac`] module*/
#[doc(alias = "MMPUENPTDMAC")]
pub type Mmpuenptdmac = crate::Reg<mmpuenptdmac::MmpuenptdmacSpec>;
///MMPU Enable Protect Register for DMAC
pub mod mmpuenptdmac;
/**MMPURPTDMAC (rw) register accessor: MMPU Regions Protect Register for DMAC

You can [`read`](crate::Reg::read) this register and get [`mmpurptdmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpurptdmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpurptdmac`] module*/
#[doc(alias = "MMPURPTDMAC")]
pub type Mmpurptdmac = crate::Reg<mmpurptdmac::MmpurptdmacSpec>;
///MMPU Regions Protect Register for DMAC
pub mod mmpurptdmac;
/**MMPURPTDMAC_SEC (rw) register accessor: MMPU Regions Protect register for DMAC Secure

You can [`read`](crate::Reg::read) this register and get [`mmpurptdmac_sec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpurptdmac_sec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpurptdmac_sec`] module*/
#[doc(alias = "MMPURPTDMAC_SEC")]
pub type MmpurptdmacSec = crate::Reg<mmpurptdmac_sec::MmpurptdmacSecSpec>;
///MMPU Regions Protect register for DMAC Secure
pub mod mmpurptdmac_sec;
/**MMPUACDMAC (rw) register accessor: MMPU Access Control Register for DMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuacdmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuacdmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuacdmac`] module*/
#[doc(alias = "MMPUACDMAC")]
pub type Mmpuacdmac = crate::Reg<mmpuacdmac::MmpuacdmacSpec>;
///MMPU Access Control Register for DMAC
pub mod mmpuacdmac;
/**MMPUSDMAC (rw) register accessor: MMPU Start Address Register for DMAC

You can [`read`](crate::Reg::read) this register and get [`mmpusdmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusdmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpusdmac`] module*/
#[doc(alias = "MMPUSDMAC")]
pub type Mmpusdmac = crate::Reg<mmpusdmac::MmpusdmacSpec>;
///MMPU Start Address Register for DMAC
pub mod mmpusdmac;
/**MMPUEDMAC (rw) register accessor: MMPU End Address Register for DMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuedmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuedmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuedmac`] module*/
#[doc(alias = "MMPUEDMAC")]
pub type Mmpuedmac = crate::Reg<mmpuedmac::MmpuedmacSpec>;
///MMPU End Address Register for DMAC
pub mod mmpuedmac;
/**MMPUENEDMAC (rw) register accessor: MMPU Enable Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuenedmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuenedmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuenedmac`] module*/
#[doc(alias = "MMPUENEDMAC")]
pub type Mmpuenedmac = crate::Reg<mmpuenedmac::MmpuenedmacSpec>;
///MMPU Enable Register for EDMAC
pub mod mmpuenedmac;
/**MMPUENPTEDMAC (rw) register accessor: MMPU Enable Protect Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuenptedmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuenptedmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuenptedmac`] module*/
#[doc(alias = "MMPUENPTEDMAC")]
pub type Mmpuenptedmac = crate::Reg<mmpuenptedmac::MmpuenptedmacSpec>;
///MMPU Enable Protect Register for EDMAC
pub mod mmpuenptedmac;
/**MMPURPTEDMAC (rw) register accessor: MMPU Regions Protect Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpurptedmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpurptedmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpurptedmac`] module*/
#[doc(alias = "MMPURPTEDMAC")]
pub type Mmpurptedmac = crate::Reg<mmpurptedmac::MmpurptedmacSpec>;
///MMPU Regions Protect Register for EDMAC
pub mod mmpurptedmac;
/**MMPUACEDMAC (rw) register accessor: MMPU Access Control Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpuacedmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpuacedmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpuacedmac`] module*/
#[doc(alias = "MMPUACEDMAC")]
pub type Mmpuacedmac = crate::Reg<mmpuacedmac::MmpuacedmacSpec>;
///MMPU Access Control Register for EDMAC
pub mod mmpuacedmac;
/**MMPUSEDMAC (rw) register accessor: MMPU Start Address Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpusedmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpusedmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpusedmac`] module*/
#[doc(alias = "MMPUSEDMAC")]
pub type Mmpusedmac = crate::Reg<mmpusedmac::MmpusedmacSpec>;
///MMPU Start Address Register for EDMAC
pub mod mmpusedmac;
/**MMPUEEDMAC (rw) register accessor: MMPU End Address Register for EDMAC

You can [`read`](crate::Reg::read) this register and get [`mmpueedmac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mmpueedmac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mmpueedmac`] module*/
#[doc(alias = "MMPUEEDMAC")]
pub type Mmpueedmac = crate::Reg<mmpueedmac::MmpueedmacSpec>;
///MMPU End Address Register for EDMAC
pub mod mmpueedmac;
