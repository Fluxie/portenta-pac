#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    spcr: Spcr,
    sslp: Sslp,
    sppcr: Sppcr,
    spsr: Spsr,
    _reserved_4_spdr: [u8; 0x04],
    spscr: Spscr,
    spssr: Spssr,
    spbr: Spbr,
    spdcr: Spdcr,
    spckd: Spckd,
    sslnd: Sslnd,
    spnd: Spnd,
    spcr2: Spcr2,
    spcmd: [Spcmd; 8],
    spdcr2: Spdcr2,
    spcr3: Spcr3,
}
impl RegisterBlock {
    ///0x00 - SPI Control Register
    #[inline(always)]
    pub const fn spcr(&self) -> &Spcr {
        &self.spcr
    }
    ///0x01 - SPI Slave Select Polarity Register
    #[inline(always)]
    pub const fn sslp(&self) -> &Sslp {
        &self.sslp
    }
    ///0x02 - SPI Pin Control Register
    #[inline(always)]
    pub const fn sppcr(&self) -> &Sppcr {
        &self.sppcr
    }
    ///0x03 - SPI Status Register
    #[inline(always)]
    pub const fn spsr(&self) -> &Spsr {
        &self.spsr
    }
    ///0x04 - SPI Data Register
    #[inline(always)]
    pub const fn spdr_by(&self) -> &SpdrBy {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - SPI Data Register
    #[inline(always)]
    pub const fn spdr_ha(&self) -> &SpdrHa {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x04 - SPI Data Register
    #[inline(always)]
    pub const fn spdr(&self) -> &Spdr {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).cast() }
    }
    ///0x08 - SPI Sequence Control Register
    #[inline(always)]
    pub const fn spscr(&self) -> &Spscr {
        &self.spscr
    }
    ///0x09 - SPI Sequence Status Register
    #[inline(always)]
    pub const fn spssr(&self) -> &Spssr {
        &self.spssr
    }
    ///0x0a - SPI Bit Rate Register
    #[inline(always)]
    pub const fn spbr(&self) -> &Spbr {
        &self.spbr
    }
    ///0x0b - SPI Data Control Register
    #[inline(always)]
    pub const fn spdcr(&self) -> &Spdcr {
        &self.spdcr
    }
    ///0x0c - SPI Clock Delay Register
    #[inline(always)]
    pub const fn spckd(&self) -> &Spckd {
        &self.spckd
    }
    ///0x0d - SPI Slave Select Negation Delay Register
    #[inline(always)]
    pub const fn sslnd(&self) -> &Sslnd {
        &self.sslnd
    }
    ///0x0e - SPI Next-Access Delay Register
    #[inline(always)]
    pub const fn spnd(&self) -> &Spnd {
        &self.spnd
    }
    ///0x0f - SPI Control Register 2
    #[inline(always)]
    pub const fn spcr2(&self) -> &Spcr2 {
        &self.spcr2
    }
    ///0x10..0x20 - SPI Command Register %s
    #[inline(always)]
    pub const fn spcmd(&self, n: usize) -> &Spcmd {
        &self.spcmd[n]
    }
    ///Iterator for array of:
    ///0x10..0x20 - SPI Command Register %s
    #[inline(always)]
    pub fn spcmd_iter(&self) -> impl Iterator<Item = &Spcmd> {
        self.spcmd.iter()
    }
    ///0x20 - SPI Data Control Register 2
    #[inline(always)]
    pub const fn spdcr2(&self) -> &Spdcr2 {
        &self.spdcr2
    }
    ///0x21 - SPI Control Register 3
    #[inline(always)]
    pub const fn spcr3(&self) -> &Spcr3 {
        &self.spcr3
    }
}
/**SPCR (rw) register accessor: SPI Control Register

You can [`read`](crate::Reg::read) this register and get [`spcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spcr`] module*/
#[doc(alias = "SPCR")]
pub type Spcr = crate::Reg<spcr::SpcrSpec>;
///SPI Control Register
pub mod spcr;
/**SSLP (rw) register accessor: SPI Slave Select Polarity Register

You can [`read`](crate::Reg::read) this register and get [`sslp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sslp`] module*/
#[doc(alias = "SSLP")]
pub type Sslp = crate::Reg<sslp::SslpSpec>;
///SPI Slave Select Polarity Register
pub mod sslp;
/**SPPCR (rw) register accessor: SPI Pin Control Register

You can [`read`](crate::Reg::read) this register and get [`sppcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sppcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sppcr`] module*/
#[doc(alias = "SPPCR")]
pub type Sppcr = crate::Reg<sppcr::SppcrSpec>;
///SPI Pin Control Register
pub mod sppcr;
/**SPSR (rw) register accessor: SPI Status Register

You can [`read`](crate::Reg::read) this register and get [`spsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spsr`] module*/
#[doc(alias = "SPSR")]
pub type Spsr = crate::Reg<spsr::SpsrSpec>;
///SPI Status Register
pub mod spsr;
/**SPDR (rw) register accessor: SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spdr`] module*/
#[doc(alias = "SPDR")]
pub type Spdr = crate::Reg<spdr::SpdrSpec>;
///SPI Data Register
pub mod spdr;
/**SPDR_HA (rw) register accessor: SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr_ha::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr_ha::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spdr_ha`] module*/
#[doc(alias = "SPDR_HA")]
pub type SpdrHa = crate::Reg<spdr_ha::SpdrHaSpec>;
///SPI Data Register
pub mod spdr_ha;
/**SPDR_BY (rw) register accessor: SPI Data Register

You can [`read`](crate::Reg::read) this register and get [`spdr_by::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdr_by::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spdr_by`] module*/
#[doc(alias = "SPDR_BY")]
pub type SpdrBy = crate::Reg<spdr_by::SpdrBySpec>;
///SPI Data Register
pub mod spdr_by;
/**SPSCR (rw) register accessor: SPI Sequence Control Register

You can [`read`](crate::Reg::read) this register and get [`spscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spscr`] module*/
#[doc(alias = "SPSCR")]
pub type Spscr = crate::Reg<spscr::SpscrSpec>;
///SPI Sequence Control Register
pub mod spscr;
/**SPSSR (r) register accessor: SPI Sequence Status Register

You can [`read`](crate::Reg::read) this register and get [`spssr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spssr`] module*/
#[doc(alias = "SPSSR")]
pub type Spssr = crate::Reg<spssr::SpssrSpec>;
///SPI Sequence Status Register
pub mod spssr;
/**SPBR (rw) register accessor: SPI Bit Rate Register

You can [`read`](crate::Reg::read) this register and get [`spbr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spbr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spbr`] module*/
#[doc(alias = "SPBR")]
pub type Spbr = crate::Reg<spbr::SpbrSpec>;
///SPI Bit Rate Register
pub mod spbr;
/**SPDCR (rw) register accessor: SPI Data Control Register

You can [`read`](crate::Reg::read) this register and get [`spdcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spdcr`] module*/
#[doc(alias = "SPDCR")]
pub type Spdcr = crate::Reg<spdcr::SpdcrSpec>;
///SPI Data Control Register
pub mod spdcr;
/**SPCKD (rw) register accessor: SPI Clock Delay Register

You can [`read`](crate::Reg::read) this register and get [`spckd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spckd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spckd`] module*/
#[doc(alias = "SPCKD")]
pub type Spckd = crate::Reg<spckd::SpckdSpec>;
///SPI Clock Delay Register
pub mod spckd;
/**SSLND (rw) register accessor: SPI Slave Select Negation Delay Register

You can [`read`](crate::Reg::read) this register and get [`sslnd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sslnd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sslnd`] module*/
#[doc(alias = "SSLND")]
pub type Sslnd = crate::Reg<sslnd::SslndSpec>;
///SPI Slave Select Negation Delay Register
pub mod sslnd;
/**SPND (rw) register accessor: SPI Next-Access Delay Register

You can [`read`](crate::Reg::read) this register and get [`spnd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spnd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spnd`] module*/
#[doc(alias = "SPND")]
pub type Spnd = crate::Reg<spnd::SpndSpec>;
///SPI Next-Access Delay Register
pub mod spnd;
/**SPCR2 (rw) register accessor: SPI Control Register 2

You can [`read`](crate::Reg::read) this register and get [`spcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spcr2`] module*/
#[doc(alias = "SPCR2")]
pub type Spcr2 = crate::Reg<spcr2::Spcr2Spec>;
///SPI Control Register 2
pub mod spcr2;
/**SPCMD (rw) register accessor: SPI Command Register %s

You can [`read`](crate::Reg::read) this register and get [`spcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spcmd`] module*/
#[doc(alias = "SPCMD")]
pub type Spcmd = crate::Reg<spcmd::SpcmdSpec>;
///SPI Command Register %s
pub mod spcmd;
/**SPDCR2 (rw) register accessor: SPI Data Control Register 2

You can [`read`](crate::Reg::read) this register and get [`spdcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spdcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spdcr2`] module*/
#[doc(alias = "SPDCR2")]
pub type Spdcr2 = crate::Reg<spdcr2::Spdcr2Spec>;
///SPI Data Control Register 2
pub mod spdcr2;
/**SPCR3 (rw) register accessor: SPI Control Register 3

You can [`read`](crate::Reg::read) this register and get [`spcr3::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`spcr3::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@spcr3`] module*/
#[doc(alias = "SPCR3")]
pub type Spcr3 = crate::Reg<spcr3::Spcr3Spec>;
///SPI Control Register 3
pub mod spcr3;
