#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x40],
    fckmhz: Fckmhz,
}
impl RegisterBlock {
    ///0x40 - Data Flash Access Frequency Register
    #[inline(always)]
    pub const fn fckmhz(&self) -> &Fckmhz {
        &self.fckmhz
    }
}
/**FCKMHZ (rw) register accessor: Data Flash Access Frequency Register

You can [`read`](crate::Reg::read) this register and get [`fckmhz::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fckmhz::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fckmhz`] module*/
#[doc(alias = "FCKMHZ")]
pub type Fckmhz = crate::Reg<fckmhz::FckmhzSpec>;
///Data Flash Access Frequency Register
pub mod fckmhz;
