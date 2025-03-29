#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    irqcr: [Irqcr; 16],
    _reserved1: [u8; 0xf0],
    nmicr: Nmicr,
    _reserved2: [u8; 0x1f],
    nmier: Nmier,
    _reserved3: [u8; 0x0e],
    nmiclr: Nmiclr,
    _reserved4: [u8; 0x0e],
    nmisr: Nmisr,
    _reserved5: [u8; 0x5e],
    wupen0: Wupen0,
    wupen1: Wupen1,
    _reserved7: [u8; 0x58],
    selsr0: Selsr0,
    _reserved8: [u8; 0x7e],
    delsr: [Delsr; 8],
    _reserved9: [u8; 0x60],
    ielsr: [Ielsr; 96],
}
impl RegisterBlock {
    ///0x00..0x10 - IRQ Control Register %s
    #[inline(always)]
    pub const fn irqcr(&self, n: usize) -> &Irqcr {
        &self.irqcr[n]
    }
    ///Iterator for array of:
    ///0x00..0x10 - IRQ Control Register %s
    #[inline(always)]
    pub fn irqcr_iter(&self) -> impl Iterator<Item = &Irqcr> {
        self.irqcr.iter()
    }
    ///0x100 - NMI Pin Interrupt Control Register
    #[inline(always)]
    pub const fn nmicr(&self) -> &Nmicr {
        &self.nmicr
    }
    ///0x120 - Non-Maskable Interrupt Enable Register
    #[inline(always)]
    pub const fn nmier(&self) -> &Nmier {
        &self.nmier
    }
    ///0x130 - Non-Maskable Interrupt Status Clear Register
    #[inline(always)]
    pub const fn nmiclr(&self) -> &Nmiclr {
        &self.nmiclr
    }
    ///0x140 - Non-Maskable Interrupt Status Register
    #[inline(always)]
    pub const fn nmisr(&self) -> &Nmisr {
        &self.nmisr
    }
    ///0x1a0 - Wake Up Interrupt Enable Register 0
    #[inline(always)]
    pub const fn wupen0(&self) -> &Wupen0 {
        &self.wupen0
    }
    ///0x1a4 - Wake Up interrupt enable register 1
    #[inline(always)]
    pub const fn wupen1(&self) -> &Wupen1 {
        &self.wupen1
    }
    ///0x200 - SYS Event Link Setting Register
    #[inline(always)]
    pub const fn selsr0(&self) -> &Selsr0 {
        &self.selsr0
    }
    ///0x280..0x2a0 - DMAC Event Link Setting Register %s
    #[inline(always)]
    pub const fn delsr(&self, n: usize) -> &Delsr {
        &self.delsr[n]
    }
    ///Iterator for array of:
    ///0x280..0x2a0 - DMAC Event Link Setting Register %s
    #[inline(always)]
    pub fn delsr_iter(&self) -> impl Iterator<Item = &Delsr> {
        self.delsr.iter()
    }
    ///0x300..0x480 - ICU Event Link Setting Register %s
    #[inline(always)]
    pub const fn ielsr(&self, n: usize) -> &Ielsr {
        &self.ielsr[n]
    }
    ///Iterator for array of:
    ///0x300..0x480 - ICU Event Link Setting Register %s
    #[inline(always)]
    pub fn ielsr_iter(&self) -> impl Iterator<Item = &Ielsr> {
        self.ielsr.iter()
    }
}
/**IRQCR (rw) register accessor: IRQ Control Register %s

You can [`read`](crate::Reg::read) this register and get [`irqcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`irqcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@irqcr`] module*/
#[doc(alias = "IRQCR")]
pub type Irqcr = crate::Reg<irqcr::IrqcrSpec>;
///IRQ Control Register %s
pub mod irqcr;
/**NMICR (rw) register accessor: NMI Pin Interrupt Control Register

You can [`read`](crate::Reg::read) this register and get [`nmicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nmicr`] module*/
#[doc(alias = "NMICR")]
pub type Nmicr = crate::Reg<nmicr::NmicrSpec>;
///NMI Pin Interrupt Control Register
pub mod nmicr;
/**NMIER (rw) register accessor: Non-Maskable Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`nmier::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmier::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nmier`] module*/
#[doc(alias = "NMIER")]
pub type Nmier = crate::Reg<nmier::NmierSpec>;
///Non-Maskable Interrupt Enable Register
pub mod nmier;
/**NMICLR (rw) register accessor: Non-Maskable Interrupt Status Clear Register

You can [`read`](crate::Reg::read) this register and get [`nmiclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmiclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nmiclr`] module*/
#[doc(alias = "NMICLR")]
pub type Nmiclr = crate::Reg<nmiclr::NmiclrSpec>;
///Non-Maskable Interrupt Status Clear Register
pub mod nmiclr;
/**NMISR (r) register accessor: Non-Maskable Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nmisr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nmisr`] module*/
#[doc(alias = "NMISR")]
pub type Nmisr = crate::Reg<nmisr::NmisrSpec>;
///Non-Maskable Interrupt Status Register
pub mod nmisr;
/**WUPEN0 (rw) register accessor: Wake Up Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`wupen0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wupen0`] module*/
#[doc(alias = "WUPEN0")]
pub type Wupen0 = crate::Reg<wupen0::Wupen0Spec>;
///Wake Up Interrupt Enable Register 0
pub mod wupen0;
/**WUPEN1 (rw) register accessor: Wake Up interrupt enable register 1

You can [`read`](crate::Reg::read) this register and get [`wupen1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wupen1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@wupen1`] module*/
#[doc(alias = "WUPEN1")]
pub type Wupen1 = crate::Reg<wupen1::Wupen1Spec>;
///Wake Up interrupt enable register 1
pub mod wupen1;
/**SELSR0 (rw) register accessor: SYS Event Link Setting Register

You can [`read`](crate::Reg::read) this register and get [`selsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`selsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@selsr0`] module*/
#[doc(alias = "SELSR0")]
pub type Selsr0 = crate::Reg<selsr0::Selsr0Spec>;
///SYS Event Link Setting Register
pub mod selsr0;
/**DELSR (rw) register accessor: DMAC Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`delsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`delsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@delsr`] module*/
#[doc(alias = "DELSR")]
pub type Delsr = crate::Reg<delsr::DelsrSpec>;
///DMAC Event Link Setting Register %s
pub mod delsr;
/**IELSR (rw) register accessor: ICU Event Link Setting Register %s

You can [`read`](crate::Reg::read) this register and get [`ielsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ielsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ielsr`] module*/
#[doc(alias = "IELSR")]
pub type Ielsr = crate::Reg<ielsr::IelsrSpec>;
///ICU Event Link Setting Register %s
pub mod ielsr;
