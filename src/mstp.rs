#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    mstpcra: Mstpcra,
    mstpcrb: Mstpcrb,
    mstpcrc: Mstpcrc,
    mstpcrd: Mstpcrd,
    mstpcre: Mstpcre,
}
impl RegisterBlock {
    ///0x00 - Module Stop Control Register A
    #[inline(always)]
    pub const fn mstpcra(&self) -> &Mstpcra {
        &self.mstpcra
    }
    ///0x04 - Module Stop Control Register B
    #[inline(always)]
    pub const fn mstpcrb(&self) -> &Mstpcrb {
        &self.mstpcrb
    }
    ///0x08 - Module Stop Control Register C
    #[inline(always)]
    pub const fn mstpcrc(&self) -> &Mstpcrc {
        &self.mstpcrc
    }
    ///0x0c - Module Stop Control Register D
    #[inline(always)]
    pub const fn mstpcrd(&self) -> &Mstpcrd {
        &self.mstpcrd
    }
    ///0x10 - Module Stop Control Register E
    #[inline(always)]
    pub const fn mstpcre(&self) -> &Mstpcre {
        &self.mstpcre
    }
}
/**MSTPCRA (rw) register accessor: Module Stop Control Register A

You can [`read`](crate::Reg::read) this register and get [`mstpcra::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcra::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mstpcra`] module*/
#[doc(alias = "MSTPCRA")]
pub type Mstpcra = crate::Reg<mstpcra::MstpcraSpec>;
///Module Stop Control Register A
pub mod mstpcra;
/**MSTPCRB (rw) register accessor: Module Stop Control Register B

You can [`read`](crate::Reg::read) this register and get [`mstpcrb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mstpcrb`] module*/
#[doc(alias = "MSTPCRB")]
pub type Mstpcrb = crate::Reg<mstpcrb::MstpcrbSpec>;
///Module Stop Control Register B
pub mod mstpcrb;
/**MSTPCRC (rw) register accessor: Module Stop Control Register C

You can [`read`](crate::Reg::read) this register and get [`mstpcrc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mstpcrc`] module*/
#[doc(alias = "MSTPCRC")]
pub type Mstpcrc = crate::Reg<mstpcrc::MstpcrcSpec>;
///Module Stop Control Register C
pub mod mstpcrc;
/**MSTPCRD (rw) register accessor: Module Stop Control Register D

You can [`read`](crate::Reg::read) this register and get [`mstpcrd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcrd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mstpcrd`] module*/
#[doc(alias = "MSTPCRD")]
pub type Mstpcrd = crate::Reg<mstpcrd::MstpcrdSpec>;
///Module Stop Control Register D
pub mod mstpcrd;
/**MSTPCRE (rw) register accessor: Module Stop Control Register E

You can [`read`](crate::Reg::read) this register and get [`mstpcre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mstpcre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mstpcre`] module*/
#[doc(alias = "MSTPCRE")]
pub type Mstpcre = crate::Reg<mstpcre::MstpcreSpec>;
///Module Stop Control Register E
pub mod mstpcre;
