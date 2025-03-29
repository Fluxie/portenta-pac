#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ccactl: Ccactl,
    ccafct: Ccafct,
    ccalcf: Ccalcf,
    _reserved3: [u8; 0x34],
    scactl: Scactl,
    scafct: Scafct,
    scalcf: Scalcf,
    _reserved6: [u8; 0x01b4],
    capoad: Capoad,
    caprcr: Caprcr,
}
impl RegisterBlock {
    ///0x00 - C-Cache Control Register
    #[inline(always)]
    pub const fn ccactl(&self) -> &Ccactl {
        &self.ccactl
    }
    ///0x04 - C-Cache Flush Control Register
    #[inline(always)]
    pub const fn ccafct(&self) -> &Ccafct {
        &self.ccafct
    }
    ///0x08 - C-Cache Line Configuration Register
    #[inline(always)]
    pub const fn ccalcf(&self) -> &Ccalcf {
        &self.ccalcf
    }
    ///0x40 - S-Cache Control Register
    #[inline(always)]
    pub const fn scactl(&self) -> &Scactl {
        &self.scactl
    }
    ///0x44 - S-Cache Flush Control Register
    #[inline(always)]
    pub const fn scafct(&self) -> &Scafct {
        &self.scafct
    }
    ///0x48 - S-Cache Line Configuration Register
    #[inline(always)]
    pub const fn scalcf(&self) -> &Scalcf {
        &self.scalcf
    }
    ///0x200 - Cache Parity Error Operation After Detection Register
    #[inline(always)]
    pub const fn capoad(&self) -> &Capoad {
        &self.capoad
    }
    ///0x204 - Cache Protection Register
    #[inline(always)]
    pub const fn caprcr(&self) -> &Caprcr {
        &self.caprcr
    }
}
/**CCACTL (rw) register accessor: C-Cache Control Register

You can [`read`](crate::Reg::read) this register and get [`ccactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccactl`] module*/
#[doc(alias = "CCACTL")]
pub type Ccactl = crate::Reg<ccactl::CcactlSpec>;
///C-Cache Control Register
pub mod ccactl;
/**CCAFCT (rw) register accessor: C-Cache Flush Control Register

You can [`read`](crate::Reg::read) this register and get [`ccafct::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccafct::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccafct`] module*/
#[doc(alias = "CCAFCT")]
pub type Ccafct = crate::Reg<ccafct::CcafctSpec>;
///C-Cache Flush Control Register
pub mod ccafct;
/**CCALCF (rw) register accessor: C-Cache Line Configuration Register

You can [`read`](crate::Reg::read) this register and get [`ccalcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ccalcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ccalcf`] module*/
#[doc(alias = "CCALCF")]
pub type Ccalcf = crate::Reg<ccalcf::CcalcfSpec>;
///C-Cache Line Configuration Register
pub mod ccalcf;
/**SCACTL (rw) register accessor: S-Cache Control Register

You can [`read`](crate::Reg::read) this register and get [`scactl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scactl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scactl`] module*/
#[doc(alias = "SCACTL")]
pub type Scactl = crate::Reg<scactl::ScactlSpec>;
///S-Cache Control Register
pub mod scactl;
/**SCAFCT (rw) register accessor: S-Cache Flush Control Register

You can [`read`](crate::Reg::read) this register and get [`scafct::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scafct::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scafct`] module*/
#[doc(alias = "SCAFCT")]
pub type Scafct = crate::Reg<scafct::ScafctSpec>;
///S-Cache Flush Control Register
pub mod scafct;
/**SCALCF (rw) register accessor: S-Cache Line Configuration Register

You can [`read`](crate::Reg::read) this register and get [`scalcf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`scalcf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@scalcf`] module*/
#[doc(alias = "SCALCF")]
pub type Scalcf = crate::Reg<scalcf::ScalcfSpec>;
///S-Cache Line Configuration Register
pub mod scalcf;
/**CAPOAD (rw) register accessor: Cache Parity Error Operation After Detection Register

You can [`read`](crate::Reg::read) this register and get [`capoad::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`capoad::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@capoad`] module*/
#[doc(alias = "CAPOAD")]
pub type Capoad = crate::Reg<capoad::CapoadSpec>;
///Cache Parity Error Operation After Detection Register
pub mod capoad;
/**CAPRCR (rw) register accessor: Cache Protection Register

You can [`read`](crate::Reg::read) this register and get [`caprcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`caprcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@caprcr`] module*/
#[doc(alias = "CAPRCR")]
pub type Caprcr = crate::Reg<caprcr::CaprcrSpec>;
///Cache Protection Register
pub mod caprcr;
