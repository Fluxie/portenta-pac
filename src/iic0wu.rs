#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x02],
    icwur: Icwur,
    icwur2: Icwur2,
}
impl RegisterBlock {
    ///0x02 - I2C Bus Wakeup Unit Register
    #[inline(always)]
    pub const fn icwur(&self) -> &Icwur {
        &self.icwur
    }
    ///0x03 - I2C Bus Wakeup Unit Register 2
    #[inline(always)]
    pub const fn icwur2(&self) -> &Icwur2 {
        &self.icwur2
    }
}
/**ICWUR (rw) register accessor: I2C Bus Wakeup Unit Register

You can [`read`](crate::Reg::read) this register and get [`icwur::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icwur::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icwur`] module*/
#[doc(alias = "ICWUR")]
pub type Icwur = crate::Reg<icwur::IcwurSpec>;
///I2C Bus Wakeup Unit Register
pub mod icwur;
/**ICWUR2 (rw) register accessor: I2C Bus Wakeup Unit Register 2

You can [`read`](crate::Reg::read) this register and get [`icwur2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icwur2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@icwur2`] module*/
#[doc(alias = "ICWUR2")]
pub type Icwur2 = crate::Reg<icwur2::Icwur2Spec>;
///I2C Bus Wakeup Unit Register 2
pub mod icwur2;
