#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    sfmsmd: Sfmsmd,
    sfmssc: Sfmssc,
    sfmskc: Sfmskc,
    sfmsst: Sfmsst,
    sfmcom: Sfmcom,
    sfmcmd: Sfmcmd,
    sfmcst: Sfmcst,
    _reserved7: [u8; 0x04],
    sfmsic: Sfmsic,
    sfmsac: Sfmsac,
    sfmsdc: Sfmsdc,
    _reserved10: [u8; 0x04],
    sfmspc: Sfmspc,
    sfmpmd: Sfmpmd,
    _reserved12: [u8; 0x07cc],
    sfmcnt1: Sfmcnt1,
}
impl RegisterBlock {
    ///0x00 - Transfer Mode Control Register
    #[inline(always)]
    pub const fn sfmsmd(&self) -> &Sfmsmd {
        &self.sfmsmd
    }
    ///0x04 - Chip Selection Control Register
    #[inline(always)]
    pub const fn sfmssc(&self) -> &Sfmssc {
        &self.sfmssc
    }
    ///0x08 - Clock Control Register
    #[inline(always)]
    pub const fn sfmskc(&self) -> &Sfmskc {
        &self.sfmskc
    }
    ///0x0c - Status Register
    #[inline(always)]
    pub const fn sfmsst(&self) -> &Sfmsst {
        &self.sfmsst
    }
    ///0x10 - Communication Port Register
    #[inline(always)]
    pub const fn sfmcom(&self) -> &Sfmcom {
        &self.sfmcom
    }
    ///0x14 - Communication Mode Control Register
    #[inline(always)]
    pub const fn sfmcmd(&self) -> &Sfmcmd {
        &self.sfmcmd
    }
    ///0x18 - Communication Status Register
    #[inline(always)]
    pub const fn sfmcst(&self) -> &Sfmcst {
        &self.sfmcst
    }
    ///0x20 - Instruction Code Register
    #[inline(always)]
    pub const fn sfmsic(&self) -> &Sfmsic {
        &self.sfmsic
    }
    ///0x24 - Address Mode Control Register
    #[inline(always)]
    pub const fn sfmsac(&self) -> &Sfmsac {
        &self.sfmsac
    }
    ///0x28 - Dummy Cycle Control Register
    #[inline(always)]
    pub const fn sfmsdc(&self) -> &Sfmsdc {
        &self.sfmsdc
    }
    ///0x30 - SPI Protocol Control Register
    #[inline(always)]
    pub const fn sfmspc(&self) -> &Sfmspc {
        &self.sfmspc
    }
    ///0x34 - Port Control Register
    #[inline(always)]
    pub const fn sfmpmd(&self) -> &Sfmpmd {
        &self.sfmpmd
    }
    ///0x804 - External QSPI Address Register
    #[inline(always)]
    pub const fn sfmcnt1(&self) -> &Sfmcnt1 {
        &self.sfmcnt1
    }
}
/**SFMSMD (rw) register accessor: Transfer Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsmd`] module*/
#[doc(alias = "SFMSMD")]
pub type Sfmsmd = crate::Reg<sfmsmd::SfmsmdSpec>;
///Transfer Mode Control Register
pub mod sfmsmd;
/**SFMSSC (rw) register accessor: Chip Selection Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmssc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmssc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmssc`] module*/
#[doc(alias = "SFMSSC")]
pub type Sfmssc = crate::Reg<sfmssc::SfmsscSpec>;
///Chip Selection Control Register
pub mod sfmssc;
/**SFMSKC (rw) register accessor: Clock Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmskc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmskc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmskc`] module*/
#[doc(alias = "SFMSKC")]
pub type Sfmskc = crate::Reg<sfmskc::SfmskcSpec>;
///Clock Control Register
pub mod sfmskc;
/**SFMSST (r) register accessor: Status Register

You can [`read`](crate::Reg::read) this register and get [`sfmsst::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsst`] module*/
#[doc(alias = "SFMSST")]
pub type Sfmsst = crate::Reg<sfmsst::SfmsstSpec>;
///Status Register
pub mod sfmsst;
/**SFMCOM (rw) register accessor: Communication Port Register

You can [`read`](crate::Reg::read) this register and get [`sfmcom::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcom::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmcom`] module*/
#[doc(alias = "SFMCOM")]
pub type Sfmcom = crate::Reg<sfmcom::SfmcomSpec>;
///Communication Port Register
pub mod sfmcom;
/**SFMCMD (rw) register accessor: Communication Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmcmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmcmd`] module*/
#[doc(alias = "SFMCMD")]
pub type Sfmcmd = crate::Reg<sfmcmd::SfmcmdSpec>;
///Communication Mode Control Register
pub mod sfmcmd;
/**SFMCST (rw) register accessor: Communication Status Register

You can [`read`](crate::Reg::read) this register and get [`sfmcst::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcst::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmcst`] module*/
#[doc(alias = "SFMCST")]
pub type Sfmcst = crate::Reg<sfmcst::SfmcstSpec>;
///Communication Status Register
pub mod sfmcst;
/**SFMSIC (rw) register accessor: Instruction Code Register

You can [`read`](crate::Reg::read) this register and get [`sfmsic::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsic::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsic`] module*/
#[doc(alias = "SFMSIC")]
pub type Sfmsic = crate::Reg<sfmsic::SfmsicSpec>;
///Instruction Code Register
pub mod sfmsic;
/**SFMSAC (rw) register accessor: Address Mode Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsac::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsac::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsac`] module*/
#[doc(alias = "SFMSAC")]
pub type Sfmsac = crate::Reg<sfmsac::SfmsacSpec>;
///Address Mode Control Register
pub mod sfmsac;
/**SFMSDC (rw) register accessor: Dummy Cycle Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmsdc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmsdc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmsdc`] module*/
#[doc(alias = "SFMSDC")]
pub type Sfmsdc = crate::Reg<sfmsdc::SfmsdcSpec>;
///Dummy Cycle Control Register
pub mod sfmsdc;
/**SFMSPC (rw) register accessor: SPI Protocol Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmspc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmspc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmspc`] module*/
#[doc(alias = "SFMSPC")]
pub type Sfmspc = crate::Reg<sfmspc::SfmspcSpec>;
///SPI Protocol Control Register
pub mod sfmspc;
/**SFMPMD (rw) register accessor: Port Control Register

You can [`read`](crate::Reg::read) this register and get [`sfmpmd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmpmd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmpmd`] module*/
#[doc(alias = "SFMPMD")]
pub type Sfmpmd = crate::Reg<sfmpmd::SfmpmdSpec>;
///Port Control Register
pub mod sfmpmd;
/**SFMCNT1 (rw) register accessor: External QSPI Address Register

You can [`read`](crate::Reg::read) this register and get [`sfmcnt1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sfmcnt1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sfmcnt1`] module*/
#[doc(alias = "SFMCNT1")]
pub type Sfmcnt1 = crate::Reg<sfmcnt1::Sfmcnt1Spec>;
///External QSPI Address Register
pub mod sfmcnt1;
