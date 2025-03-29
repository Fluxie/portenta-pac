#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tscr: Tscr,
}
impl RegisterBlock {
    ///0x00 - Temperature Sensor Control Register
    #[inline(always)]
    pub const fn tscr(&self) -> &Tscr {
        &self.tscr
    }
}
/**TSCR (rw) register accessor: Temperature Sensor Control Register

You can [`read`](crate::Reg::read) this register and get [`tscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tscr`] module*/
#[doc(alias = "TSCR")]
pub type Tscr = crate::Reg<tscr::TscrSpec>;
///Temperature Sensor Control Register
pub mod tscr;
