#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    tzfoad: Tzfoad,
    _reserved1: [u8; 0x02],
    tzfpt: Tzfpt,
}
impl RegisterBlock {
    ///0x00 - TrustZone Filter Operation After Detection Register
    #[inline(always)]
    pub const fn tzfoad(&self) -> &Tzfoad {
        &self.tzfoad
    }
    ///0x04 - TrustZone Filter Protect Register
    #[inline(always)]
    pub const fn tzfpt(&self) -> &Tzfpt {
        &self.tzfpt
    }
}
/**TZFOAD (rw) register accessor: TrustZone Filter Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`tzfoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzfoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tzfoad`] module*/
#[doc(alias = "TZFOAD")]
pub type Tzfoad = crate::Reg<tzfoad::TzfoadSpec>;
///TrustZone Filter Operation After Detection Register
pub mod tzfoad;
/**TZFPT (rw) register accessor: TrustZone Filter Protect Register

You can [`read`](crate::Reg::read) this register and get [`tzfpt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tzfpt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tzfpt`] module*/
#[doc(alias = "TZFPT")]
pub type Tzfpt = crate::Reg<tzfpt::TzfptSpec>;
///TrustZone Filter Protect Register
pub mod tzfpt;
