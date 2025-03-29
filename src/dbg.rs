#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dbgstr: Dbgstr,
    _reserved1: [u8; 0x0c],
    dbgstopcr: Dbgstopcr,
}
impl RegisterBlock {
    ///0x00 - Debug Status Register
    #[inline(always)]
    pub const fn dbgstr(&self) -> &Dbgstr {
        &self.dbgstr
    }
    ///0x10 - Debug Stop Control Register
    #[inline(always)]
    pub const fn dbgstopcr(&self) -> &Dbgstopcr {
        &self.dbgstopcr
    }
}
/**DBGSTR (r) register accessor: Debug Status Register

You can [`read`](crate::Reg::read) this register and get [`dbgstr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgstr`] module*/
#[doc(alias = "DBGSTR")]
pub type Dbgstr = crate::Reg<dbgstr::DbgstrSpec>;
///Debug Status Register
pub mod dbgstr;
/**DBGSTOPCR (rw) register accessor: Debug Stop Control Register

You can [`read`](crate::Reg::read) this register and get [`dbgstopcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dbgstopcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dbgstopcr`] module*/
#[doc(alias = "DBGSTOPCR")]
pub type Dbgstopcr = crate::Reg<dbgstopcr::DbgstopcrSpec>;
///Debug Stop Control Register
pub mod dbgstopcr;
