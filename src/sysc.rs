#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x0c],
    sbycr: Sbycr,
    _reserved1: [u8; 0x12],
    sckdivcr: Sckdivcr,
    _reserved2: [u8; 0x02],
    sckscr: Sckscr,
    _reserved3: [u8; 0x01],
    pllccr: Pllccr,
    pllcr: Pllcr,
    _reserved5: [u8; 0x05],
    bckcr: Bckcr,
    _reserved6: [u8; 0x01],
    mosccr: Mosccr,
    _reserved7: [u8; 0x03],
    hococr: Hococr,
    _reserved8: [u8; 0x01],
    mococr: Mococr,
    fllcr1: Fllcr1,
    fllcr2: Fllcr2,
    oscsf: Oscsf,
    _reserved12: [u8; 0x01],
    ckocr: Ckocr,
    trckcr: Trckcr,
    ostdcr: Ostdcr,
    ostdsr: Ostdsr,
    _reserved16: [u8; 0x06],
    pll2ccr: Pll2ccr,
    pll2cr: Pll2cr,
    _reserved18: [u8; 0x07],
    ebckocr: Ebckocr,
    _reserved19: [u8; 0x0e],
    mocoutcr: Mocoutcr,
    hocoutcr: Hocoutcr,
    _reserved21: [u8; 0x09],
    usbckdivcr: Usbckdivcr,
    octackdivcr: Octackdivcr,
    canfdckdivcr: Canfdckdivcr,
    usb60ckdivcr: Usb60ckdivcr,
    cecckdivcr: Cecckdivcr,
    _reserved26: [u8; 0x03],
    usbckcr: Usbckcr,
    octackcr: Octackcr,
    canfdckcr: Canfdckcr,
    usb60ckcr: Usb60ckcr,
    cecckcr: Cecckcr,
    _reserved31: [u8; 0x0f],
    snzreqcr1: Snzreqcr1,
    _reserved32: [u8; 0x06],
    snzcr: Snzcr,
    _reserved33: [u8; 0x01],
    snzedcr0: Snzedcr0,
    snzedcr1: Snzedcr1,
    _reserved35: [u8; 0x02],
    snzreqcr0: Snzreqcr0,
    _reserved36: [u8; 0x04],
    opccr: Opccr,
    _reserved37: [u8; 0x01],
    moscwtcr: Moscwtcr,
    _reserved38: [u8; 0x07],
    sopccr: Sopccr,
    _reserved39: [u8; 0x15],
    rstsr1: Rstsr1,
    _reserved40: [u8; 0x1e],
    lvd1cr1: Lvd1cr1,
    lvd1sr: Lvd1sr,
    lvd2cr1: Lvd2cr1,
    lvd2sr: Lvd2sr,
    _reserved44: [u8; 0x02dc],
    cgfsar: Cgfsar,
    rstsar: Rstsar,
    lpmsar: Lpmsar,
    lvdsar: Lvdsar,
    bbfsar: Bbfsar,
    _reserved49: [u8; 0x0c],
    dpfsar: Dpfsar,
    _reserved50: [u8; 0x1a],
    prcr: Prcr,
    dpsbycr: Dpsbycr,
    dpswcr: Dpswcr,
    dpsier0: Dpsier0,
    dpsier1: Dpsier1,
    dpsier2: Dpsier2,
    dpsier3: Dpsier3,
    dpsifr0: Dpsifr0,
    dpsifr1: Dpsifr1,
    dpsifr2: Dpsifr2,
    dpsifr3: Dpsifr3,
    dpsiegr0: Dpsiegr0,
    dpsiegr1: Dpsiegr1,
    dpsiegr2: Dpsiegr2,
    _reserved64: [u8; 0x01],
    syocdcr: Syocdcr,
    _reserved65: [u8; 0x01],
    rstsr0: Rstsr0,
    rstsr2: Rstsr2,
    _reserved67: [u8; 0x01],
    momcr: Momcr,
    _reserved68: [u8; 0x02],
    fwepror: Fwepror,
    lvd1cmpcr: Lvd1cmpcr,
    lvd2cmpcr: Lvd2cmpcr,
    _reserved71: [u8; 0x01],
    lvd1cr0: Lvd1cr0,
    lvd2cr0: Lvd2cr0,
    _reserved73: [u8; 0x01],
    vbattmnselr: Vbattmnselr,
    vbattmonr: Vbattmonr,
    _reserved75: [u8; 0x61],
    sosccr: Sosccr,
    somcr: Somcr,
    _reserved77: [u8; 0x0e],
    lococr: Lococr,
    _reserved78: [u8; 0x01],
    locoutcr: Locoutcr,
    _reserved79: [u8; 0x28],
    vbtictlr: Vbtictlr,
    _reserved80: [u8; 0x04],
    vbtber: Vbtber,
    _reserved81: [u8; 0x3f],
    vbtbkr: [Vbtbkr; 128],
}
impl RegisterBlock {
    ///0x0c - Standby Control Register
    #[inline(always)]
    pub const fn sbycr(&self) -> &Sbycr {
        &self.sbycr
    }
    ///0x20 - System Clock Division Control Register
    #[inline(always)]
    pub const fn sckdivcr(&self) -> &Sckdivcr {
        &self.sckdivcr
    }
    ///0x26 - System Clock Source Control Register
    #[inline(always)]
    pub const fn sckscr(&self) -> &Sckscr {
        &self.sckscr
    }
    ///0x28 - PLL Clock Control Register
    #[inline(always)]
    pub const fn pllccr(&self) -> &Pllccr {
        &self.pllccr
    }
    ///0x2a - PLL Control Register
    #[inline(always)]
    pub const fn pllcr(&self) -> &Pllcr {
        &self.pllcr
    }
    ///0x30 - External Bus Clock Control Register
    #[inline(always)]
    pub const fn bckcr(&self) -> &Bckcr {
        &self.bckcr
    }
    ///0x32 - Main Clock Oscillator Control Register
    #[inline(always)]
    pub const fn mosccr(&self) -> &Mosccr {
        &self.mosccr
    }
    ///0x36 - High-Speed On-Chip Oscillator Control Register
    #[inline(always)]
    pub const fn hococr(&self) -> &Hococr {
        &self.hococr
    }
    ///0x38 - Middle-Speed On-Chip Oscillator Control Register
    #[inline(always)]
    pub const fn mococr(&self) -> &Mococr {
        &self.mococr
    }
    ///0x39 - FLL Control Register1
    #[inline(always)]
    pub const fn fllcr1(&self) -> &Fllcr1 {
        &self.fllcr1
    }
    ///0x3a - FLL Control Register2
    #[inline(always)]
    pub const fn fllcr2(&self) -> &Fllcr2 {
        &self.fllcr2
    }
    ///0x3c - Oscillation Stabilization Flag Register
    #[inline(always)]
    pub const fn oscsf(&self) -> &Oscsf {
        &self.oscsf
    }
    ///0x3e - Clock Out Control Register
    #[inline(always)]
    pub const fn ckocr(&self) -> &Ckocr {
        &self.ckocr
    }
    ///0x3f - Trace Clock Control Register
    #[inline(always)]
    pub const fn trckcr(&self) -> &Trckcr {
        &self.trckcr
    }
    ///0x40 - Oscillation Stop Detection Control Register
    #[inline(always)]
    pub const fn ostdcr(&self) -> &Ostdcr {
        &self.ostdcr
    }
    ///0x41 - Oscillation Stop Detection Status Register
    #[inline(always)]
    pub const fn ostdsr(&self) -> &Ostdsr {
        &self.ostdsr
    }
    ///0x48 - PLL2 Clock Control Register
    #[inline(always)]
    pub const fn pll2ccr(&self) -> &Pll2ccr {
        &self.pll2ccr
    }
    ///0x4a - PLL2 Control Register
    #[inline(always)]
    pub const fn pll2cr(&self) -> &Pll2cr {
        &self.pll2cr
    }
    ///0x52 - External Bus Clock Output Control Register
    #[inline(always)]
    pub const fn ebckocr(&self) -> &Ebckocr {
        &self.ebckocr
    }
    ///0x61 - MOCO User Trimming Control Register
    #[inline(always)]
    pub const fn mocoutcr(&self) -> &Mocoutcr {
        &self.mocoutcr
    }
    ///0x62 - HOCO User Trimming Control Register
    #[inline(always)]
    pub const fn hocoutcr(&self) -> &Hocoutcr {
        &self.hocoutcr
    }
    ///0x6c - USB Clock Division Control Register
    #[inline(always)]
    pub const fn usbckdivcr(&self) -> &Usbckdivcr {
        &self.usbckdivcr
    }
    ///0x6d - Octal-SPI Clock Division Control Register
    #[inline(always)]
    pub const fn octackdivcr(&self) -> &Octackdivcr {
        &self.octackdivcr
    }
    ///0x6e - CANFD Clock Division Control Register
    #[inline(always)]
    pub const fn canfdckdivcr(&self) -> &Canfdckdivcr {
        &self.canfdckdivcr
    }
    ///0x6f - USB60 Clock Division Control Register
    #[inline(always)]
    pub const fn usb60ckdivcr(&self) -> &Usb60ckdivcr {
        &self.usb60ckdivcr
    }
    ///0x70 - CEC Clock Division Control Register
    #[inline(always)]
    pub const fn cecckdivcr(&self) -> &Cecckdivcr {
        &self.cecckdivcr
    }
    ///0x74 - USB Clock Control Register
    #[inline(always)]
    pub const fn usbckcr(&self) -> &Usbckcr {
        &self.usbckcr
    }
    ///0x75 - Octal-SPI Clock Control Register
    #[inline(always)]
    pub const fn octackcr(&self) -> &Octackcr {
        &self.octackcr
    }
    ///0x76 - CANFD Clock Control Register
    #[inline(always)]
    pub const fn canfdckcr(&self) -> &Canfdckcr {
        &self.canfdckcr
    }
    ///0x77 - USB60 Clock Control Register
    #[inline(always)]
    pub const fn usb60ckcr(&self) -> &Usb60ckcr {
        &self.usb60ckcr
    }
    ///0x78 - CEC Clock Control Register
    #[inline(always)]
    pub const fn cecckcr(&self) -> &Cecckcr {
        &self.cecckcr
    }
    ///0x88 - Snooze Request Control Register 1
    #[inline(always)]
    pub const fn snzreqcr1(&self) -> &Snzreqcr1 {
        &self.snzreqcr1
    }
    ///0x92 - Snooze Control Register
    #[inline(always)]
    pub const fn snzcr(&self) -> &Snzcr {
        &self.snzcr
    }
    ///0x94 - Snooze End Control Register 0
    #[inline(always)]
    pub const fn snzedcr0(&self) -> &Snzedcr0 {
        &self.snzedcr0
    }
    ///0x95 - Snooze End Control Register 1
    #[inline(always)]
    pub const fn snzedcr1(&self) -> &Snzedcr1 {
        &self.snzedcr1
    }
    ///0x98 - Snooze Request Control Register 0
    #[inline(always)]
    pub const fn snzreqcr0(&self) -> &Snzreqcr0 {
        &self.snzreqcr0
    }
    ///0xa0 - Operating Power Control Register
    #[inline(always)]
    pub const fn opccr(&self) -> &Opccr {
        &self.opccr
    }
    ///0xa2 - Main Clock Oscillator Wait Control Register
    #[inline(always)]
    pub const fn moscwtcr(&self) -> &Moscwtcr {
        &self.moscwtcr
    }
    ///0xaa - Sub Operating Power Control Register
    #[inline(always)]
    pub const fn sopccr(&self) -> &Sopccr {
        &self.sopccr
    }
    ///0xc0 - Reset Status Register 1
    #[inline(always)]
    pub const fn rstsr1(&self) -> &Rstsr1 {
        &self.rstsr1
    }
    ///0xe0 - Voltage Monitor 1 Circuit Control Register
    #[inline(always)]
    pub const fn lvd1cr1(&self) -> &Lvd1cr1 {
        &self.lvd1cr1
    }
    ///0xe1 - Voltage Monitor 1 Circuit Status Register
    #[inline(always)]
    pub const fn lvd1sr(&self) -> &Lvd1sr {
        &self.lvd1sr
    }
    ///0xe2 - Voltage Monitor 2 Circuit Control Register 1
    #[inline(always)]
    pub const fn lvd2cr1(&self) -> &Lvd2cr1 {
        &self.lvd2cr1
    }
    ///0xe3 - Voltage Monitor 2 Circuit Status Register
    #[inline(always)]
    pub const fn lvd2sr(&self) -> &Lvd2sr {
        &self.lvd2sr
    }
    ///0x3c0 - Clock Generation Function Security Attribute Register
    #[inline(always)]
    pub const fn cgfsar(&self) -> &Cgfsar {
        &self.cgfsar
    }
    ///0x3c4 - Reset Security Attribution Register
    #[inline(always)]
    pub const fn rstsar(&self) -> &Rstsar {
        &self.rstsar
    }
    ///0x3c8 - Low Power Mode Security Attribution Register
    #[inline(always)]
    pub const fn lpmsar(&self) -> &Lpmsar {
        &self.lpmsar
    }
    ///0x3cc - Low Voltage Detection Security Attribution Register
    #[inline(always)]
    pub const fn lvdsar(&self) -> &Lvdsar {
        &self.lvdsar
    }
    ///0x3d0 - Battery Backup Function Security Attribute Register
    #[inline(always)]
    pub const fn bbfsar(&self) -> &Bbfsar {
        &self.bbfsar
    }
    ///0x3e0 - Deep Software Standby Interrupt Factor Security Attribution Register
    #[inline(always)]
    pub const fn dpfsar(&self) -> &Dpfsar {
        &self.dpfsar
    }
    ///0x3fe - Protect Register
    #[inline(always)]
    pub const fn prcr(&self) -> &Prcr {
        &self.prcr
    }
    ///0x400 - Deep Software Standby Control Register
    #[inline(always)]
    pub const fn dpsbycr(&self) -> &Dpsbycr {
        &self.dpsbycr
    }
    ///0x401 - Deep Software Standby Wait Control Register
    #[inline(always)]
    pub const fn dpswcr(&self) -> &Dpswcr {
        &self.dpswcr
    }
    ///0x402 - Deep Software Standby Interrupt Enable Register 0
    #[inline(always)]
    pub const fn dpsier0(&self) -> &Dpsier0 {
        &self.dpsier0
    }
    ///0x403 - Deep Software Standby Interrupt Enable Register 1
    #[inline(always)]
    pub const fn dpsier1(&self) -> &Dpsier1 {
        &self.dpsier1
    }
    ///0x404 - Deep Software Standby Interrupt Enable Register 2
    #[inline(always)]
    pub const fn dpsier2(&self) -> &Dpsier2 {
        &self.dpsier2
    }
    ///0x405 - Deep Software Standby Interrupt Enable Register 3
    #[inline(always)]
    pub const fn dpsier3(&self) -> &Dpsier3 {
        &self.dpsier3
    }
    ///0x406 - Deep Software Standby Interrupt Flag Register 0
    #[inline(always)]
    pub const fn dpsifr0(&self) -> &Dpsifr0 {
        &self.dpsifr0
    }
    ///0x407 - Deep Software Standby Interrupt Flag Register 1
    #[inline(always)]
    pub const fn dpsifr1(&self) -> &Dpsifr1 {
        &self.dpsifr1
    }
    ///0x408 - Deep Software Standby Interrupt Flag Register 2
    #[inline(always)]
    pub const fn dpsifr2(&self) -> &Dpsifr2 {
        &self.dpsifr2
    }
    ///0x409 - Deep Software Standby Interrupt Flag Register 3
    #[inline(always)]
    pub const fn dpsifr3(&self) -> &Dpsifr3 {
        &self.dpsifr3
    }
    ///0x40a - Deep Software Standby Interrupt Edge Register 0
    #[inline(always)]
    pub const fn dpsiegr0(&self) -> &Dpsiegr0 {
        &self.dpsiegr0
    }
    ///0x40b - Deep Software Standby Interrupt Edge Register 1
    #[inline(always)]
    pub const fn dpsiegr1(&self) -> &Dpsiegr1 {
        &self.dpsiegr1
    }
    ///0x40c - Deep Software Standby Interrupt Edge Register 2
    #[inline(always)]
    pub const fn dpsiegr2(&self) -> &Dpsiegr2 {
        &self.dpsiegr2
    }
    ///0x40e - System Control OCD Control Register
    #[inline(always)]
    pub const fn syocdcr(&self) -> &Syocdcr {
        &self.syocdcr
    }
    ///0x410 - Reset Status Register 0
    #[inline(always)]
    pub const fn rstsr0(&self) -> &Rstsr0 {
        &self.rstsr0
    }
    ///0x411 - Reset Status Register 2
    #[inline(always)]
    pub const fn rstsr2(&self) -> &Rstsr2 {
        &self.rstsr2
    }
    ///0x413 - Main Clock Oscillator Mode Oscillation Control Register
    #[inline(always)]
    pub const fn momcr(&self) -> &Momcr {
        &self.momcr
    }
    ///0x416 - Flash P/E Protect Register
    #[inline(always)]
    pub const fn fwepror(&self) -> &Fwepror {
        &self.fwepror
    }
    ///0x417 - Voltage Monitoring 1 Comparator Control Register
    #[inline(always)]
    pub const fn lvd1cmpcr(&self) -> &Lvd1cmpcr {
        &self.lvd1cmpcr
    }
    ///0x418 - Voltage Monitoring 2 Comparator Control Register
    #[inline(always)]
    pub const fn lvd2cmpcr(&self) -> &Lvd2cmpcr {
        &self.lvd2cmpcr
    }
    ///0x41a - Voltage Monitor 1 Circuit Control Register 0
    #[inline(always)]
    pub const fn lvd1cr0(&self) -> &Lvd1cr0 {
        &self.lvd1cr0
    }
    ///0x41b - Voltage Monitor 2 Circuit Control Register 0
    #[inline(always)]
    pub const fn lvd2cr0(&self) -> &Lvd2cr0 {
        &self.lvd2cr0
    }
    ///0x41d - Battery Backup Voltage Monitor Function Select Register
    #[inline(always)]
    pub const fn vbattmnselr(&self) -> &Vbattmnselr {
        &self.vbattmnselr
    }
    ///0x41e - Battery Backup Voltage Monitor Register
    #[inline(always)]
    pub const fn vbattmonr(&self) -> &Vbattmonr {
        &self.vbattmonr
    }
    ///0x480 - Sub-Clock Oscillator Control Register
    #[inline(always)]
    pub const fn sosccr(&self) -> &Sosccr {
        &self.sosccr
    }
    ///0x481 - Sub-Clock Oscillator Mode Control Register
    #[inline(always)]
    pub const fn somcr(&self) -> &Somcr {
        &self.somcr
    }
    ///0x490 - Low-Speed On-Chip Oscillator Control Register
    #[inline(always)]
    pub const fn lococr(&self) -> &Lococr {
        &self.lococr
    }
    ///0x492 - LOCO User Trimming Control Register
    #[inline(always)]
    pub const fn locoutcr(&self) -> &Locoutcr {
        &self.locoutcr
    }
    ///0x4bb - VBATT Input Control Register
    #[inline(always)]
    pub const fn vbtictlr(&self) -> &Vbtictlr {
        &self.vbtictlr
    }
    ///0x4c0 - VBATT Backup Enable Register
    #[inline(always)]
    pub const fn vbtber(&self) -> &Vbtber {
        &self.vbtber
    }
    ///0x500..0x580 - VBATT Backup Register
    #[inline(always)]
    pub const fn vbtbkr(&self, n: usize) -> &Vbtbkr {
        &self.vbtbkr[n]
    }
    ///Iterator for array of:
    ///0x500..0x580 - VBATT Backup Register
    #[inline(always)]
    pub fn vbtbkr_iter(&self) -> impl Iterator<Item = &Vbtbkr> {
        self.vbtbkr.iter()
    }
}
/**SBYCR (rw) register accessor: Standby Control Register

You can [`read`](crate::Reg::read) this register and get [`sbycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sbycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sbycr`] module*/
#[doc(alias = "SBYCR")]
pub type Sbycr = crate::Reg<sbycr::SbycrSpec>;
///Standby Control Register
pub mod sbycr;
/**SCKDIVCR (rw) register accessor: System Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`sckdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sckdivcr`] module*/
#[doc(alias = "SCKDIVCR")]
pub type Sckdivcr = crate::Reg<sckdivcr::SckdivcrSpec>;
///System Clock Division Control Register
pub mod sckdivcr;
/**SCKSCR (rw) register accessor: System Clock Source Control Register

You can [`read`](crate::Reg::read) this register and get [`sckscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sckscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sckscr`] module*/
#[doc(alias = "SCKSCR")]
pub type Sckscr = crate::Reg<sckscr::SckscrSpec>;
///System Clock Source Control Register
pub mod sckscr;
/**PLLCCR (rw) register accessor: PLL Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`pllccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pllccr`] module*/
#[doc(alias = "PLLCCR")]
pub type Pllccr = crate::Reg<pllccr::PllccrSpec>;
///PLL Clock Control Register
pub mod pllccr;
/**PLLCR (rw) register accessor: PLL Control Register

You can [`read`](crate::Reg::read) this register and get [`pllcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pllcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pllcr`] module*/
#[doc(alias = "PLLCR")]
pub type Pllcr = crate::Reg<pllcr::PllcrSpec>;
///PLL Control Register
pub mod pllcr;
/**BCKCR (rw) register accessor: External Bus Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`bckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bckcr`] module*/
#[doc(alias = "BCKCR")]
pub type Bckcr = crate::Reg<bckcr::BckcrSpec>;
///External Bus Clock Control Register
pub mod bckcr;
/**MOSCCR (rw) register accessor: Main Clock Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`mosccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mosccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mosccr`] module*/
#[doc(alias = "MOSCCR")]
pub type Mosccr = crate::Reg<mosccr::MosccrSpec>;
///Main Clock Oscillator Control Register
pub mod mosccr;
/**HOCOCR (rw) register accessor: High-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`hococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hococr`] module*/
#[doc(alias = "HOCOCR")]
pub type Hococr = crate::Reg<hococr::HococrSpec>;
///High-Speed On-Chip Oscillator Control Register
pub mod hococr;
/**MOCOCR (rw) register accessor: Middle-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`mococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mococr`] module*/
#[doc(alias = "MOCOCR")]
pub type Mococr = crate::Reg<mococr::MococrSpec>;
///Middle-Speed On-Chip Oscillator Control Register
pub mod mococr;
/**FLLCR1 (rw) register accessor: FLL Control Register1

You can [`read`](crate::Reg::read) this register and get [`fllcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fllcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fllcr1`] module*/
#[doc(alias = "FLLCR1")]
pub type Fllcr1 = crate::Reg<fllcr1::Fllcr1Spec>;
///FLL Control Register1
pub mod fllcr1;
/**FLLCR2 (rw) register accessor: FLL Control Register2

You can [`read`](crate::Reg::read) this register and get [`fllcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fllcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fllcr2`] module*/
#[doc(alias = "FLLCR2")]
pub type Fllcr2 = crate::Reg<fllcr2::Fllcr2Spec>;
///FLL Control Register2
pub mod fllcr2;
/**OSCSF (r) register accessor: Oscillation Stabilization Flag Register

You can [`read`](crate::Reg::read) this register and get [`oscsf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@oscsf`] module*/
#[doc(alias = "OSCSF")]
pub type Oscsf = crate::Reg<oscsf::OscsfSpec>;
///Oscillation Stabilization Flag Register
pub mod oscsf;
/**CKOCR (rw) register accessor: Clock Out Control Register

You can [`read`](crate::Reg::read) this register and get [`ckocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ckocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ckocr`] module*/
#[doc(alias = "CKOCR")]
pub type Ckocr = crate::Reg<ckocr::CkocrSpec>;
///Clock Out Control Register
pub mod ckocr;
/**TRCKCR (rw) register accessor: Trace Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`trckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trckcr`] module*/
#[doc(alias = "TRCKCR")]
pub type Trckcr = crate::Reg<trckcr::TrckcrSpec>;
///Trace Clock Control Register
pub mod trckcr;
/**OSTDCR (rw) register accessor: Oscillation Stop Detection Control Register

You can [`read`](crate::Reg::read) this register and get [`ostdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ostdcr`] module*/
#[doc(alias = "OSTDCR")]
pub type Ostdcr = crate::Reg<ostdcr::OstdcrSpec>;
///Oscillation Stop Detection Control Register
pub mod ostdcr;
/**OSTDSR (rw) register accessor: Oscillation Stop Detection Status Register

You can [`read`](crate::Reg::read) this register and get [`ostdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ostdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ostdsr`] module*/
#[doc(alias = "OSTDSR")]
pub type Ostdsr = crate::Reg<ostdsr::OstdsrSpec>;
///Oscillation Stop Detection Status Register
pub mod ostdsr;
/**PLL2CCR (rw) register accessor: PLL2 Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`pll2ccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2ccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pll2ccr`] module*/
#[doc(alias = "PLL2CCR")]
pub type Pll2ccr = crate::Reg<pll2ccr::Pll2ccrSpec>;
///PLL2 Clock Control Register
pub mod pll2ccr;
/**PLL2CR (rw) register accessor: PLL2 Control Register

You can [`read`](crate::Reg::read) this register and get [`pll2cr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pll2cr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pll2cr`] module*/
#[doc(alias = "PLL2CR")]
pub type Pll2cr = crate::Reg<pll2cr::Pll2crSpec>;
///PLL2 Control Register
pub mod pll2cr;
/**EBCKOCR (rw) register accessor: External Bus Clock Output Control Register

You can [`read`](crate::Reg::read) this register and get [`ebckocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ebckocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ebckocr`] module*/
#[doc(alias = "EBCKOCR")]
pub type Ebckocr = crate::Reg<ebckocr::EbckocrSpec>;
///External Bus Clock Output Control Register
pub mod ebckocr;
/**MOCOUTCR (rw) register accessor: MOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`mocoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mocoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mocoutcr`] module*/
#[doc(alias = "MOCOUTCR")]
pub type Mocoutcr = crate::Reg<mocoutcr::MocoutcrSpec>;
///MOCO User Trimming Control Register
pub mod mocoutcr;
/**HOCOUTCR (rw) register accessor: HOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`hocoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hocoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hocoutcr`] module*/
#[doc(alias = "HOCOUTCR")]
pub type Hocoutcr = crate::Reg<hocoutcr::HocoutcrSpec>;
///HOCO User Trimming Control Register
pub mod hocoutcr;
/**USBCKDIVCR (rw) register accessor: USB Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`usbckdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbckdivcr`] module*/
#[doc(alias = "USBCKDIVCR")]
pub type Usbckdivcr = crate::Reg<usbckdivcr::UsbckdivcrSpec>;
///USB Clock Division Control Register
pub mod usbckdivcr;
/**OCTACKDIVCR (rw) register accessor: Octal-SPI Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`octackdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octackdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@octackdivcr`] module*/
#[doc(alias = "OCTACKDIVCR")]
pub type Octackdivcr = crate::Reg<octackdivcr::OctackdivcrSpec>;
///Octal-SPI Clock Division Control Register
pub mod octackdivcr;
/**CANFDCKDIVCR (rw) register accessor: CANFD Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`canfdckdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canfdckdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@canfdckdivcr`] module*/
#[doc(alias = "CANFDCKDIVCR")]
pub type Canfdckdivcr = crate::Reg<canfdckdivcr::CanfdckdivcrSpec>;
///CANFD Clock Division Control Register
pub mod canfdckdivcr;
/**USB60CKDIVCR (rw) register accessor: USB60 Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`usb60ckdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb60ckdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usb60ckdivcr`] module*/
#[doc(alias = "USB60CKDIVCR")]
pub type Usb60ckdivcr = crate::Reg<usb60ckdivcr::Usb60ckdivcrSpec>;
///USB60 Clock Division Control Register
pub mod usb60ckdivcr;
/**CECCKDIVCR (rw) register accessor: CEC Clock Division Control Register

You can [`read`](crate::Reg::read) this register and get [`cecckdivcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecckdivcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cecckdivcr`] module*/
#[doc(alias = "CECCKDIVCR")]
pub type Cecckdivcr = crate::Reg<cecckdivcr::CecckdivcrSpec>;
///CEC Clock Division Control Register
pub mod cecckdivcr;
/**USBCKCR (rw) register accessor: USB Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`usbckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbckcr`] module*/
#[doc(alias = "USBCKCR")]
pub type Usbckcr = crate::Reg<usbckcr::UsbckcrSpec>;
///USB Clock Control Register
pub mod usbckcr;
/**OCTACKCR (rw) register accessor: Octal-SPI Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`octackcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`octackcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@octackcr`] module*/
#[doc(alias = "OCTACKCR")]
pub type Octackcr = crate::Reg<octackcr::OctackcrSpec>;
///Octal-SPI Clock Control Register
pub mod octackcr;
/**CANFDCKCR (rw) register accessor: CANFD Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`canfdckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`canfdckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@canfdckcr`] module*/
#[doc(alias = "CANFDCKCR")]
pub type Canfdckcr = crate::Reg<canfdckcr::CanfdckcrSpec>;
///CANFD Clock Control Register
pub mod canfdckcr;
/**USB60CKCR (rw) register accessor: USB60 Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`usb60ckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb60ckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usb60ckcr`] module*/
#[doc(alias = "USB60CKCR")]
pub type Usb60ckcr = crate::Reg<usb60ckcr::Usb60ckcrSpec>;
///USB60 Clock Control Register
pub mod usb60ckcr;
/**CECCKCR (rw) register accessor: CEC Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`cecckcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cecckcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cecckcr`] module*/
#[doc(alias = "CECCKCR")]
pub type Cecckcr = crate::Reg<cecckcr::CecckcrSpec>;
///CEC Clock Control Register
pub mod cecckcr;
/**SNZREQCR1 (rw) register accessor: Snooze Request Control Register 1

You can [`read`](crate::Reg::read) this register and get [`snzreqcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snzreqcr1`] module*/
#[doc(alias = "SNZREQCR1")]
pub type Snzreqcr1 = crate::Reg<snzreqcr1::Snzreqcr1Spec>;
///Snooze Request Control Register 1
pub mod snzreqcr1;
/**SNZCR (rw) register accessor: Snooze Control Register

You can [`read`](crate::Reg::read) this register and get [`snzcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snzcr`] module*/
#[doc(alias = "SNZCR")]
pub type Snzcr = crate::Reg<snzcr::SnzcrSpec>;
///Snooze Control Register
pub mod snzcr;
/**SNZEDCR0 (rw) register accessor: Snooze End Control Register 0

You can [`read`](crate::Reg::read) this register and get [`snzedcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snzedcr0`] module*/
#[doc(alias = "SNZEDCR0")]
pub type Snzedcr0 = crate::Reg<snzedcr0::Snzedcr0Spec>;
///Snooze End Control Register 0
pub mod snzedcr0;
/**SNZEDCR1 (rw) register accessor: Snooze End Control Register 1

You can [`read`](crate::Reg::read) this register and get [`snzedcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzedcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snzedcr1`] module*/
#[doc(alias = "SNZEDCR1")]
pub type Snzedcr1 = crate::Reg<snzedcr1::Snzedcr1Spec>;
///Snooze End Control Register 1
pub mod snzedcr1;
/**SNZREQCR0 (rw) register accessor: Snooze Request Control Register 0

You can [`read`](crate::Reg::read) this register and get [`snzreqcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`snzreqcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@snzreqcr0`] module*/
#[doc(alias = "SNZREQCR0")]
pub type Snzreqcr0 = crate::Reg<snzreqcr0::Snzreqcr0Spec>;
///Snooze Request Control Register 0
pub mod snzreqcr0;
/**OPCCR (rw) register accessor: Operating Power Control Register

You can [`read`](crate::Reg::read) this register and get [`opccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`opccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@opccr`] module*/
#[doc(alias = "OPCCR")]
pub type Opccr = crate::Reg<opccr::OpccrSpec>;
///Operating Power Control Register
pub mod opccr;
/**MOSCWTCR (rw) register accessor: Main Clock Oscillator Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`moscwtcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`moscwtcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@moscwtcr`] module*/
#[doc(alias = "MOSCWTCR")]
pub type Moscwtcr = crate::Reg<moscwtcr::MoscwtcrSpec>;
///Main Clock Oscillator Wait Control Register
pub mod moscwtcr;
/**SOPCCR (rw) register accessor: Sub Operating Power Control Register

You can [`read`](crate::Reg::read) this register and get [`sopccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sopccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sopccr`] module*/
#[doc(alias = "SOPCCR")]
pub type Sopccr = crate::Reg<sopccr::SopccrSpec>;
///Sub Operating Power Control Register
pub mod sopccr;
/**RSTSR1 (rw) register accessor: Reset Status Register 1

You can [`read`](crate::Reg::read) this register and get [`rstsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rstsr1`] module*/
#[doc(alias = "RSTSR1")]
pub type Rstsr1 = crate::Reg<rstsr1::Rstsr1Spec>;
///Reset Status Register 1
pub mod rstsr1;
/**LVD1CR1 (rw) register accessor: Voltage Monitor 1 Circuit Control Register

You can [`read`](crate::Reg::read) this register and get [`lvd1cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvd1cr1`] module*/
#[doc(alias = "LVD1CR1")]
pub type Lvd1cr1 = crate::Reg<lvd1cr1::Lvd1cr1Spec>;
///Voltage Monitor 1 Circuit Control Register
pub mod lvd1cr1;
/**LVD1SR (rw) register accessor: Voltage Monitor 1 Circuit Status Register

You can [`read`](crate::Reg::read) this register and get [`lvd1sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvd1sr`] module*/
#[doc(alias = "LVD1SR")]
pub type Lvd1sr = crate::Reg<lvd1sr::Lvd1srSpec>;
///Voltage Monitor 1 Circuit Status Register
pub mod lvd1sr;
/**LVD2CR1 (rw) register accessor: Voltage Monitor 2 Circuit Control Register 1

You can [`read`](crate::Reg::read) this register and get [`lvd2cr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd2cr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvd2cr1`] module*/
#[doc(alias = "LVD2CR1")]
pub type Lvd2cr1 = crate::Reg<lvd2cr1::Lvd2cr1Spec>;
///Voltage Monitor 2 Circuit Control Register 1
pub mod lvd2cr1;
/**LVD2SR (rw) register accessor: Voltage Monitor 2 Circuit Status Register

You can [`read`](crate::Reg::read) this register and get [`lvd2sr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd2sr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvd2sr`] module*/
#[doc(alias = "LVD2SR")]
pub type Lvd2sr = crate::Reg<lvd2sr::Lvd2srSpec>;
///Voltage Monitor 2 Circuit Status Register
pub mod lvd2sr;
/**CGFSAR (rw) register accessor: Clock Generation Function Security Attribute Register

You can [`read`](crate::Reg::read) this register and get [`cgfsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cgfsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cgfsar`] module*/
#[doc(alias = "CGFSAR")]
pub type Cgfsar = crate::Reg<cgfsar::CgfsarSpec>;
///Clock Generation Function Security Attribute Register
pub mod cgfsar;
/**RSTSAR (rw) register accessor: Reset Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`rstsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rstsar`] module*/
#[doc(alias = "RSTSAR")]
pub type Rstsar = crate::Reg<rstsar::RstsarSpec>;
///Reset Security Attribution Register
pub mod rstsar;
/**LPMSAR (rw) register accessor: Low Power Mode Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`lpmsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpmsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpmsar`] module*/
#[doc(alias = "LPMSAR")]
pub type Lpmsar = crate::Reg<lpmsar::LpmsarSpec>;
///Low Power Mode Security Attribution Register
pub mod lpmsar;
/**LVDSAR (rw) register accessor: Low Voltage Detection Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`lvdsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvdsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvdsar`] module*/
#[doc(alias = "LVDSAR")]
pub type Lvdsar = crate::Reg<lvdsar::LvdsarSpec>;
///Low Voltage Detection Security Attribution Register
pub mod lvdsar;
/**BBFSAR (rw) register accessor: Battery Backup Function Security Attribute Register

You can [`read`](crate::Reg::read) this register and get [`bbfsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bbfsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bbfsar`] module*/
#[doc(alias = "BBFSAR")]
pub type Bbfsar = crate::Reg<bbfsar::BbfsarSpec>;
///Battery Backup Function Security Attribute Register
pub mod bbfsar;
/**DPFSAR (rw) register accessor: Deep Software Standby Interrupt Factor Security Attribution Register

You can [`read`](crate::Reg::read) this register and get [`dpfsar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpfsar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpfsar`] module*/
#[doc(alias = "DPFSAR")]
pub type Dpfsar = crate::Reg<dpfsar::DpfsarSpec>;
///Deep Software Standby Interrupt Factor Security Attribution Register
pub mod dpfsar;
/**PRCR (rw) register accessor: Protect Register

You can [`read`](crate::Reg::read) this register and get [`prcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@prcr`] module*/
#[doc(alias = "PRCR")]
pub type Prcr = crate::Reg<prcr::PrcrSpec>;
///Protect Register
pub mod prcr;
/**DPSBYCR (rw) register accessor: Deep Software Standby Control Register

You can [`read`](crate::Reg::read) this register and get [`dpsbycr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsbycr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsbycr`] module*/
#[doc(alias = "DPSBYCR")]
pub type Dpsbycr = crate::Reg<dpsbycr::DpsbycrSpec>;
///Deep Software Standby Control Register
pub mod dpsbycr;
/**DPSWCR (rw) register accessor: Deep Software Standby Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`dpswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpswcr`] module*/
#[doc(alias = "DPSWCR")]
pub type Dpswcr = crate::Reg<dpswcr::DpswcrSpec>;
///Deep Software Standby Wait Control Register
pub mod dpswcr;
/**DPSIER0 (rw) register accessor: Deep Software Standby Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsier0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsier0`] module*/
#[doc(alias = "DPSIER0")]
pub type Dpsier0 = crate::Reg<dpsier0::Dpsier0Spec>;
///Deep Software Standby Interrupt Enable Register 0
pub mod dpsier0;
/**DPSIER1 (rw) register accessor: Deep Software Standby Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsier1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsier1`] module*/
#[doc(alias = "DPSIER1")]
pub type Dpsier1 = crate::Reg<dpsier1::Dpsier1Spec>;
///Deep Software Standby Interrupt Enable Register 1
pub mod dpsier1;
/**DPSIER2 (rw) register accessor: Deep Software Standby Interrupt Enable Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsier2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsier2`] module*/
#[doc(alias = "DPSIER2")]
pub type Dpsier2 = crate::Reg<dpsier2::Dpsier2Spec>;
///Deep Software Standby Interrupt Enable Register 2
pub mod dpsier2;
/**DPSIER3 (rw) register accessor: Deep Software Standby Interrupt Enable Register 3

You can [`read`](crate::Reg::read) this register and get [`dpsier3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsier3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsier3`] module*/
#[doc(alias = "DPSIER3")]
pub type Dpsier3 = crate::Reg<dpsier3::Dpsier3Spec>;
///Deep Software Standby Interrupt Enable Register 3
pub mod dpsier3;
/**DPSIFR0 (rw) register accessor: Deep Software Standby Interrupt Flag Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsifr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsifr0`] module*/
#[doc(alias = "DPSIFR0")]
pub type Dpsifr0 = crate::Reg<dpsifr0::Dpsifr0Spec>;
///Deep Software Standby Interrupt Flag Register 0
pub mod dpsifr0;
/**DPSIFR1 (rw) register accessor: Deep Software Standby Interrupt Flag Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsifr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsifr1`] module*/
#[doc(alias = "DPSIFR1")]
pub type Dpsifr1 = crate::Reg<dpsifr1::Dpsifr1Spec>;
///Deep Software Standby Interrupt Flag Register 1
pub mod dpsifr1;
/**DPSIFR2 (rw) register accessor: Deep Software Standby Interrupt Flag Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsifr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsifr2`] module*/
#[doc(alias = "DPSIFR2")]
pub type Dpsifr2 = crate::Reg<dpsifr2::Dpsifr2Spec>;
///Deep Software Standby Interrupt Flag Register 2
pub mod dpsifr2;
/**DPSIFR3 (rw) register accessor: Deep Software Standby Interrupt Flag Register 3

You can [`read`](crate::Reg::read) this register and get [`dpsifr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsifr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsifr3`] module*/
#[doc(alias = "DPSIFR3")]
pub type Dpsifr3 = crate::Reg<dpsifr3::Dpsifr3Spec>;
///Deep Software Standby Interrupt Flag Register 3
pub mod dpsifr3;
/**DPSIEGR0 (rw) register accessor: Deep Software Standby Interrupt Edge Register 0

You can [`read`](crate::Reg::read) this register and get [`dpsiegr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsiegr0`] module*/
#[doc(alias = "DPSIEGR0")]
pub type Dpsiegr0 = crate::Reg<dpsiegr0::Dpsiegr0Spec>;
///Deep Software Standby Interrupt Edge Register 0
pub mod dpsiegr0;
/**DPSIEGR1 (rw) register accessor: Deep Software Standby Interrupt Edge Register 1

You can [`read`](crate::Reg::read) this register and get [`dpsiegr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsiegr1`] module*/
#[doc(alias = "DPSIEGR1")]
pub type Dpsiegr1 = crate::Reg<dpsiegr1::Dpsiegr1Spec>;
///Deep Software Standby Interrupt Edge Register 1
pub mod dpsiegr1;
/**DPSIEGR2 (rw) register accessor: Deep Software Standby Interrupt Edge Register 2

You can [`read`](crate::Reg::read) this register and get [`dpsiegr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpsiegr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpsiegr2`] module*/
#[doc(alias = "DPSIEGR2")]
pub type Dpsiegr2 = crate::Reg<dpsiegr2::Dpsiegr2Spec>;
///Deep Software Standby Interrupt Edge Register 2
pub mod dpsiegr2;
/**SYOCDCR (rw) register accessor: System Control OCD Control Register

You can [`read`](crate::Reg::read) this register and get [`syocdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syocdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syocdcr`] module*/
#[doc(alias = "SYOCDCR")]
pub type Syocdcr = crate::Reg<syocdcr::SyocdcrSpec>;
///System Control OCD Control Register
pub mod syocdcr;
/**RSTSR0 (rw) register accessor: Reset Status Register 0

You can [`read`](crate::Reg::read) this register and get [`rstsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rstsr0`] module*/
#[doc(alias = "RSTSR0")]
pub type Rstsr0 = crate::Reg<rstsr0::Rstsr0Spec>;
///Reset Status Register 0
pub mod rstsr0;
/**RSTSR2 (rw) register accessor: Reset Status Register 2

You can [`read`](crate::Reg::read) this register and get [`rstsr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rstsr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rstsr2`] module*/
#[doc(alias = "RSTSR2")]
pub type Rstsr2 = crate::Reg<rstsr2::Rstsr2Spec>;
///Reset Status Register 2
pub mod rstsr2;
/**MOMCR (rw) register accessor: Main Clock Oscillator Mode Oscillation Control Register

You can [`read`](crate::Reg::read) this register and get [`momcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`momcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@momcr`] module*/
#[doc(alias = "MOMCR")]
pub type Momcr = crate::Reg<momcr::MomcrSpec>;
///Main Clock Oscillator Mode Oscillation Control Register
pub mod momcr;
/**FWEPROR (rw) register accessor: Flash P/E Protect Register

You can [`read`](crate::Reg::read) this register and get [`fwepror::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fwepror::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fwepror`] module*/
#[doc(alias = "FWEPROR")]
pub type Fwepror = crate::Reg<fwepror::FweprorSpec>;
///Flash P/E Protect Register
pub mod fwepror;
/**LVD1CMPCR (rw) register accessor: Voltage Monitoring 1 Comparator Control Register

You can [`read`](crate::Reg::read) this register and get [`lvd1cmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1cmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvd1cmpcr`] module*/
#[doc(alias = "LVD1CMPCR")]
pub type Lvd1cmpcr = crate::Reg<lvd1cmpcr::Lvd1cmpcrSpec>;
///Voltage Monitoring 1 Comparator Control Register
pub mod lvd1cmpcr;
/**LVD2CMPCR (rw) register accessor: Voltage Monitoring 2 Comparator Control Register

You can [`read`](crate::Reg::read) this register and get [`lvd2cmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd2cmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvd2cmpcr`] module*/
#[doc(alias = "LVD2CMPCR")]
pub type Lvd2cmpcr = crate::Reg<lvd2cmpcr::Lvd2cmpcrSpec>;
///Voltage Monitoring 2 Comparator Control Register
pub mod lvd2cmpcr;
/**LVD1CR0 (rw) register accessor: Voltage Monitor 1 Circuit Control Register 0

You can [`read`](crate::Reg::read) this register and get [`lvd1cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd1cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvd1cr0`] module*/
#[doc(alias = "LVD1CR0")]
pub type Lvd1cr0 = crate::Reg<lvd1cr0::Lvd1cr0Spec>;
///Voltage Monitor 1 Circuit Control Register 0
pub mod lvd1cr0;
/**LVD2CR0 (rw) register accessor: Voltage Monitor 2 Circuit Control Register 0

You can [`read`](crate::Reg::read) this register and get [`lvd2cr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lvd2cr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lvd2cr0`] module*/
#[doc(alias = "LVD2CR0")]
pub type Lvd2cr0 = crate::Reg<lvd2cr0::Lvd2cr0Spec>;
///Voltage Monitor 2 Circuit Control Register 0
pub mod lvd2cr0;
/**VBATTMNSELR (rw) register accessor: Battery Backup Voltage Monitor Function Select Register

You can [`read`](crate::Reg::read) this register and get [`vbattmnselr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbattmnselr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbattmnselr`] module*/
#[doc(alias = "VBATTMNSELR")]
pub type Vbattmnselr = crate::Reg<vbattmnselr::VbattmnselrSpec>;
///Battery Backup Voltage Monitor Function Select Register
pub mod vbattmnselr;
/**VBATTMONR (r) register accessor: Battery Backup Voltage Monitor Register

You can [`read`](crate::Reg::read) this register and get [`vbattmonr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbattmonr`] module*/
#[doc(alias = "VBATTMONR")]
pub type Vbattmonr = crate::Reg<vbattmonr::VbattmonrSpec>;
///Battery Backup Voltage Monitor Register
pub mod vbattmonr;
/**SOSCCR (rw) register accessor: Sub-Clock Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`sosccr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosccr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sosccr`] module*/
#[doc(alias = "SOSCCR")]
pub type Sosccr = crate::Reg<sosccr::SosccrSpec>;
///Sub-Clock Oscillator Control Register
pub mod sosccr;
/**SOMCR (rw) register accessor: Sub-Clock Oscillator Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`somcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`somcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@somcr`] module*/
#[doc(alias = "SOMCR")]
pub type Somcr = crate::Reg<somcr::SomcrSpec>;
///Sub-Clock Oscillator Mode Control Register
pub mod somcr;
/**LOCOCR (rw) register accessor: Low-Speed On-Chip Oscillator Control Register

You can [`read`](crate::Reg::read) this register and get [`lococr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lococr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lococr`] module*/
#[doc(alias = "LOCOCR")]
pub type Lococr = crate::Reg<lococr::LococrSpec>;
///Low-Speed On-Chip Oscillator Control Register
pub mod lococr;
/**LOCOUTCR (rw) register accessor: LOCO User Trimming Control Register

You can [`read`](crate::Reg::read) this register and get [`locoutcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`locoutcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@locoutcr`] module*/
#[doc(alias = "LOCOUTCR")]
pub type Locoutcr = crate::Reg<locoutcr::LocoutcrSpec>;
///LOCO User Trimming Control Register
pub mod locoutcr;
/**VBTICTLR (rw) register accessor: VBATT Input Control Register

You can [`read`](crate::Reg::read) this register and get [`vbtictlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtictlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtictlr`] module*/
#[doc(alias = "VBTICTLR")]
pub type Vbtictlr = crate::Reg<vbtictlr::VbtictlrSpec>;
///VBATT Input Control Register
pub mod vbtictlr;
/**VBTBER (rw) register accessor: VBATT Backup Enable Register

You can [`read`](crate::Reg::read) this register and get [`vbtber::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtber::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtber`] module*/
#[doc(alias = "VBTBER")]
pub type Vbtber = crate::Reg<vbtber::VbtberSpec>;
///VBATT Backup Enable Register
pub mod vbtber;
/**VBTBKR (rw) register accessor: VBATT Backup Register

You can [`read`](crate::Reg::read) this register and get [`vbtbkr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`vbtbkr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@vbtbkr`] module*/
#[doc(alias = "VBTBKR")]
pub type Vbtbkr = crate::Reg<vbtbkr::VbtbkrSpec>;
///VBATT Backup Register
pub mod vbtbkr;
