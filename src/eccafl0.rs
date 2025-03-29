#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    ec710ctl: Ec710ctl,
    ec710tmc: Ec710tmc,
    _reserved2: [u8; 0x06],
    ec710ted: Ec710ted,
    ec710ead0: Ec710ead0,
}
impl RegisterBlock {
    ///0x00 - ECC Control Register
    #[inline(always)]
    pub const fn ec710ctl(&self) -> &Ec710ctl {
        &self.ec710ctl
    }
    ///0x04 - ECC Test Mode Control Register
    #[inline(always)]
    pub const fn ec710tmc(&self) -> &Ec710tmc {
        &self.ec710tmc
    }
    ///0x0c - ECC Test Substitute Data Register
    #[inline(always)]
    pub const fn ec710ted(&self) -> &Ec710ted {
        &self.ec710ted
    }
    ///0x10 - ECC Error Address Register
    #[inline(always)]
    pub const fn ec710ead0(&self) -> &Ec710ead0 {
        &self.ec710ead0
    }
}
/**EC710CTL (rw) register accessor: ECC Control Register

You can [`read`](crate::Reg::read) this register and get [`ec710ctl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ec710ctl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ec710ctl`] module*/
#[doc(alias = "EC710CTL")]
pub type Ec710ctl = crate::Reg<ec710ctl::Ec710ctlSpec>;
///ECC Control Register
pub mod ec710ctl;
/**EC710TMC (rw) register accessor: ECC Test Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`ec710tmc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ec710tmc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ec710tmc`] module*/
#[doc(alias = "EC710TMC")]
pub type Ec710tmc = crate::Reg<ec710tmc::Ec710tmcSpec>;
///ECC Test Mode Control Register
pub mod ec710tmc;
/**EC710TED (rw) register accessor: ECC Test Substitute Data Register

You can [`read`](crate::Reg::read) this register and get [`ec710ted::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ec710ted::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ec710ted`] module*/
#[doc(alias = "EC710TED")]
pub type Ec710ted = crate::Reg<ec710ted::Ec710tedSpec>;
///ECC Test Substitute Data Register
pub mod ec710ted;
/**EC710EAD0 (r) register accessor: ECC Error Address Register

You can [`read`](crate::Reg::read) this register and get [`ec710ead0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ec710ead0`] module*/
#[doc(alias = "EC710EAD0")]
pub type Ec710ead0 = crate::Reg<ec710ead0::Ec710ead0Spec>;
///ECC Error Address Register
pub mod ec710ead0;
