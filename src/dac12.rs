#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dadr: [Dadr; 2],
    dacr: Dacr,
    dadpr: Dadpr,
    daadscr: Daadscr,
    _reserved4: [u8; 0x01],
    daampcr: Daampcr,
    _reserved5: [u8; 0x13],
    daaswcr: Daaswcr,
    _reserved6: [u8; 0x10a3],
    daadusr: Daadusr,
}
impl RegisterBlock {
    ///0x00 - D/A Data Register %s
    #[inline(always)]
    pub const fn dadr(&self, n: usize) -> &Dadr {
        &self.dadr[n]
    }
    ///Iterator for array of:
    ///0x00 - D/A Data Register %s
    #[inline(always)]
    pub fn dadr_iter(&self) -> impl Iterator<Item = &Dadr> {
        self.dadr.iter()
    }
    ///0x04 - D/A Control Register
    #[inline(always)]
    pub const fn dacr(&self) -> &Dacr {
        &self.dacr
    }
    ///0x05 - DADRn Format Select Register
    #[inline(always)]
    pub const fn dadpr(&self) -> &Dadpr {
        &self.dadpr
    }
    ///0x06 - D/A A/D Synchronous Start Control Register
    #[inline(always)]
    pub const fn daadscr(&self) -> &Daadscr {
        &self.daadscr
    }
    ///0x08 - D/A Output Amplifier Control Register
    #[inline(always)]
    pub const fn daampcr(&self) -> &Daampcr {
        &self.daampcr
    }
    ///0x1c - D/A Amplifier Stabilization Wait Control Register
    #[inline(always)]
    pub const fn daaswcr(&self) -> &Daaswcr {
        &self.daaswcr
    }
    ///0x10c0 - D/A A/D Synchronous Unit Select Register
    #[inline(always)]
    pub const fn daadusr(&self) -> &Daadusr {
        &self.daadusr
    }
}
/**DADR (rw) register accessor: D/A Data Register %s

You can [`read`](crate::Reg::read) this register and get [`dadr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dadr`] module*/
#[doc(alias = "DADR")]
pub type Dadr = crate::Reg<dadr::DadrSpec>;
///D/A Data Register %s
pub mod dadr;
/**DACR (rw) register accessor: D/A Control Register

You can [`read`](crate::Reg::read) this register and get [`dacr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dacr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dacr`] module*/
#[doc(alias = "DACR")]
pub type Dacr = crate::Reg<dacr::DacrSpec>;
///D/A Control Register
pub mod dacr;
/**DADPR (rw) register accessor: DADRn Format Select Register

You can [`read`](crate::Reg::read) this register and get [`dadpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dadpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dadpr`] module*/
#[doc(alias = "DADPR")]
pub type Dadpr = crate::Reg<dadpr::DadprSpec>;
///DADRn Format Select Register
pub mod dadpr;
/**DAADSCR (rw) register accessor: D/A A/D Synchronous Start Control Register

You can [`read`](crate::Reg::read) this register and get [`daadscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daadscr`] module*/
#[doc(alias = "DAADSCR")]
pub type Daadscr = crate::Reg<daadscr::DaadscrSpec>;
///D/A A/D Synchronous Start Control Register
pub mod daadscr;
/**DAAMPCR (rw) register accessor: D/A Output Amplifier Control Register

You can [`read`](crate::Reg::read) this register and get [`daampcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daampcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daampcr`] module*/
#[doc(alias = "DAAMPCR")]
pub type Daampcr = crate::Reg<daampcr::DaampcrSpec>;
///D/A Output Amplifier Control Register
pub mod daampcr;
/**DAASWCR (rw) register accessor: D/A Amplifier Stabilization Wait Control Register

You can [`read`](crate::Reg::read) this register and get [`daaswcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daaswcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daaswcr`] module*/
#[doc(alias = "DAASWCR")]
pub type Daaswcr = crate::Reg<daaswcr::DaaswcrSpec>;
///D/A Amplifier Stabilization Wait Control Register
pub mod daaswcr;
/**DAADUSR (rw) register accessor: D/A A/D Synchronous Unit Select Register

You can [`read`](crate::Reg::read) this register and get [`daadusr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`daadusr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@daadusr`] module*/
#[doc(alias = "DAADUSR")]
pub type Daadusr = crate::Reg<daadusr::DaadusrSpec>;
///D/A A/D Synchronous Unit Select Register
pub mod daadusr;
