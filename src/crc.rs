#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    crccr0: Crccr0,
    _reserved1: [u8; 0x03],
    _reserved_1_crcdir: [u8; 0x04],
    _reserved_2_crcdor: [u8; 0x04],
}
impl RegisterBlock {
    ///0x00 - CRC Control Register 0
    #[inline(always)]
    pub const fn crccr0(&self) -> &Crccr0 {
        &self.crccr0
    }
    ///0x04 - CRC Data Input Register
    #[inline(always)]
    pub const fn crcdir_by(&self) -> &CrcdirBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - CRC Data Input Register
    #[inline(always)]
    pub const fn crcdir(&self) -> &Crcdir {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - CRC Data Output Register
    #[inline(always)]
    pub const fn crcdor_by(&self) -> &CrcdorBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - CRC Data Output Register
    #[inline(always)]
    pub const fn crcdor_ha(&self) -> &CrcdorHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
    ///0x08 - CRC Data Output Register
    #[inline(always)]
    pub const fn crcdor(&self) -> &Crcdor {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).cast() }
    }
}
/**CRCCR0 (rw) register accessor: CRC Control Register 0

You can [`read`](crate::Reg::read) this register and get [`crccr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crccr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crccr0`] module*/
#[doc(alias = "CRCCR0")]
pub type Crccr0 = crate::Reg<crccr0::Crccr0Spec>;
///CRC Control Register 0
pub mod crccr0;
/**CRCDIR (rw) register accessor: CRC Data Input Register

You can [`read`](crate::Reg::read) this register and get [`crcdir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdir`] module*/
#[doc(alias = "CRCDIR")]
pub type Crcdir = crate::Reg<crcdir::CrcdirSpec>;
///CRC Data Input Register
pub mod crcdir;
/**CRCDIR_BY (rw) register accessor: CRC Data Input Register

You can [`read`](crate::Reg::read) this register and get [`crcdir_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdir_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdir_by`] module*/
#[doc(alias = "CRCDIR_BY")]
pub type CrcdirBy = crate::Reg<crcdir_by::CrcdirBySpec>;
///CRC Data Input Register
pub mod crcdir_by;
/**CRCDOR (rw) register accessor: CRC Data Output Register

You can [`read`](crate::Reg::read) this register and get [`crcdor::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdor`] module*/
#[doc(alias = "CRCDOR")]
pub type Crcdor = crate::Reg<crcdor::CrcdorSpec>;
///CRC Data Output Register
pub mod crcdor;
/**CRCDOR_HA (rw) register accessor: CRC Data Output Register

You can [`read`](crate::Reg::read) this register and get [`crcdor_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdor_ha`] module*/
#[doc(alias = "CRCDOR_HA")]
pub type CrcdorHa = crate::Reg<crcdor_ha::CrcdorHaSpec>;
///CRC Data Output Register
pub mod crcdor_ha;
/**CRCDOR_BY (rw) register accessor: CRC Data Output Register

You can [`read`](crate::Reg::read) this register and get [`crcdor_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crcdor_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crcdor_by`] module*/
#[doc(alias = "CRCDOR_BY")]
pub type CrcdorBy = crate::Reg<crcdor_by::CrcdorBySpec>;
///CRC Data Output Register
pub mod crcdor_by;
