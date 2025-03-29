#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    dcr: Dcr,
    dar: Dar,
    dcsr: Dcsr,
    dsr0: Dsr0,
    dsr1: Dsr1,
    mdtr: Mdtr,
    actr: Actr,
    acar0: Acar0,
    acar1: Acar1,
    _reserved9: [u8; 0x10],
    drcstr: Drcstr,
    dwcstr: Dwcstr,
    dcstr: Dcstr,
    cdsr: Cdsr,
    mdlr: Mdlr,
    mrwcr0: Mrwcr0,
    mrwcr1: Mrwcr1,
    mrwcsr: Mrwcsr,
    esr: Esr,
    cwndr: Cwndr,
    cwdr: Cwdr,
    crr: Crr,
    acsr: Acsr,
    _reserved22: [u8; 0x14],
    dcsmxr: Dcsmxr,
    dwsctsr: Dwsctsr,
}
impl RegisterBlock {
    ///0x00 - Device Command Register
    #[inline(always)]
    pub const fn dcr(&self) -> &Dcr {
        &self.dcr
    }
    ///0x04 - Device Address Register
    #[inline(always)]
    pub const fn dar(&self) -> &Dar {
        &self.dar
    }
    ///0x08 - Device Command Setting Register
    #[inline(always)]
    pub const fn dcsr(&self) -> &Dcsr {
        &self.dcsr
    }
    ///0x0c - Device Size Register 0
    #[inline(always)]
    pub const fn dsr0(&self) -> &Dsr0 {
        &self.dsr0
    }
    ///0x10 - Device Size Register 1
    #[inline(always)]
    pub const fn dsr1(&self) -> &Dsr1 {
        &self.dsr1
    }
    ///0x14 - Memory Delay Trim Register
    #[inline(always)]
    pub const fn mdtr(&self) -> &Mdtr {
        &self.mdtr
    }
    ///0x18 - Auto-Calibration Timer Register
    #[inline(always)]
    pub const fn actr(&self) -> &Actr {
        &self.actr
    }
    ///0x1c - Auto-Calibration Address Register 0
    #[inline(always)]
    pub const fn acar0(&self) -> &Acar0 {
        &self.acar0
    }
    ///0x20 - Auto-Calibration Address Register 1
    #[inline(always)]
    pub const fn acar1(&self) -> &Acar1 {
        &self.acar1
    }
    ///0x34 - Device Memory Map Read Chip Select Timing Setting Register
    #[inline(always)]
    pub const fn drcstr(&self) -> &Drcstr {
        &self.drcstr
    }
    ///0x38 - Device Memory Map Write Chip Select Timing Setting Register
    #[inline(always)]
    pub const fn dwcstr(&self) -> &Dwcstr {
        &self.dwcstr
    }
    ///0x3c - Device Chip Select Timing Setting Register
    #[inline(always)]
    pub const fn dcstr(&self) -> &Dcstr {
        &self.dcstr
    }
    ///0x40 - Controller and Device Setting Register
    #[inline(always)]
    pub const fn cdsr(&self) -> &Cdsr {
        &self.cdsr
    }
    ///0x44 - Memory Map Dummy Length Register
    #[inline(always)]
    pub const fn mdlr(&self) -> &Mdlr {
        &self.mdlr
    }
    ///0x48 - Memory Map Read/Write Command Register 0
    #[inline(always)]
    pub const fn mrwcr0(&self) -> &Mrwcr0 {
        &self.mrwcr0
    }
    ///0x4c - Memory Map Read/Write Command Register 1
    #[inline(always)]
    pub const fn mrwcr1(&self) -> &Mrwcr1 {
        &self.mrwcr1
    }
    ///0x50 - Memory Map Read/Write Setting Register
    #[inline(always)]
    pub const fn mrwcsr(&self) -> &Mrwcsr {
        &self.mrwcsr
    }
    ///0x54 - Error Status Register
    #[inline(always)]
    pub const fn esr(&self) -> &Esr {
        &self.esr
    }
    ///0x58 - Configure Write without Data Register
    #[inline(always)]
    pub const fn cwndr(&self) -> &Cwndr {
        &self.cwndr
    }
    ///0x5c - Configure Write Data Register
    #[inline(always)]
    pub const fn cwdr(&self) -> &Cwdr {
        &self.cwdr
    }
    ///0x60 - Configure Read Register
    #[inline(always)]
    pub const fn crr(&self) -> &Crr {
        &self.crr
    }
    ///0x64 - Auto-Calibration Status Register
    #[inline(always)]
    pub const fn acsr(&self) -> &Acsr {
        &self.acsr
    }
    ///0x7c - Device Chip Select Maximum Period Register
    #[inline(always)]
    pub const fn dcsmxr(&self) -> &Dcsmxr {
        &self.dcsmxr
    }
    ///0x80 - Device Memory Map Write single continuous translating size Register
    #[inline(always)]
    pub const fn dwsctsr(&self) -> &Dwsctsr {
        &self.dwsctsr
    }
}
/**DCR (rw) register accessor: Device Command Register

You can [`read`](crate::Reg::read) this register and get [`dcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcr`] module*/
#[doc(alias = "DCR")]
pub type Dcr = crate::Reg<dcr::DcrSpec>;
///Device Command Register
pub mod dcr;
/**DAR (rw) register accessor: Device Address Register

You can [`read`](crate::Reg::read) this register and get [`dar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dar`] module*/
#[doc(alias = "DAR")]
pub type Dar = crate::Reg<dar::DarSpec>;
///Device Address Register
pub mod dar;
/**DCSR (rw) register accessor: Device Command Setting Register

You can [`read`](crate::Reg::read) this register and get [`dcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcsr`] module*/
#[doc(alias = "DCSR")]
pub type Dcsr = crate::Reg<dcsr::DcsrSpec>;
///Device Command Setting Register
pub mod dcsr;
/**DSR0 (rw) register accessor: Device Size Register 0

You can [`read`](crate::Reg::read) this register and get [`dsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dsr0`] module*/
#[doc(alias = "DSR0")]
pub type Dsr0 = crate::Reg<dsr0::Dsr0Spec>;
///Device Size Register 0
pub mod dsr0;
/**DSR1 (rw) register accessor: Device Size Register 1

You can [`read`](crate::Reg::read) this register and get [`dsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dsr1`] module*/
#[doc(alias = "DSR1")]
pub type Dsr1 = crate::Reg<dsr1::Dsr1Spec>;
///Device Size Register 1
pub mod dsr1;
/**MDTR (rw) register accessor: Memory Delay Trim Register

You can [`read`](crate::Reg::read) this register and get [`mdtr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdtr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mdtr`] module*/
#[doc(alias = "MDTR")]
pub type Mdtr = crate::Reg<mdtr::MdtrSpec>;
///Memory Delay Trim Register
pub mod mdtr;
/**ACTR (rw) register accessor: Auto-Calibration Timer Register

You can [`read`](crate::Reg::read) this register and get [`actr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`actr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@actr`] module*/
#[doc(alias = "ACTR")]
pub type Actr = crate::Reg<actr::ActrSpec>;
///Auto-Calibration Timer Register
pub mod actr;
/**ACAR0 (rw) register accessor: Auto-Calibration Address Register 0

You can [`read`](crate::Reg::read) this register and get [`acar0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acar0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acar0`] module*/
#[doc(alias = "ACAR0")]
pub type Acar0 = crate::Reg<acar0::Acar0Spec>;
///Auto-Calibration Address Register 0
pub mod acar0;
/**ACAR1 (rw) register accessor: Auto-Calibration Address Register 1

You can [`read`](crate::Reg::read) this register and get [`acar1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acar1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acar1`] module*/
#[doc(alias = "ACAR1")]
pub type Acar1 = crate::Reg<acar1::Acar1Spec>;
///Auto-Calibration Address Register 1
pub mod acar1;
/**DRCSTR (rw) register accessor: Device Memory Map Read Chip Select Timing Setting Register

You can [`read`](crate::Reg::read) this register and get [`drcstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`drcstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@drcstr`] module*/
#[doc(alias = "DRCSTR")]
pub type Drcstr = crate::Reg<drcstr::DrcstrSpec>;
///Device Memory Map Read Chip Select Timing Setting Register
pub mod drcstr;
/**DWCSTR (rw) register accessor: Device Memory Map Write Chip Select Timing Setting Register

You can [`read`](crate::Reg::read) this register and get [`dwcstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwcstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dwcstr`] module*/
#[doc(alias = "DWCSTR")]
pub type Dwcstr = crate::Reg<dwcstr::DwcstrSpec>;
///Device Memory Map Write Chip Select Timing Setting Register
pub mod dwcstr;
/**DCSTR (rw) register accessor: Device Chip Select Timing Setting Register

You can [`read`](crate::Reg::read) this register and get [`dcstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcstr`] module*/
#[doc(alias = "DCSTR")]
pub type Dcstr = crate::Reg<dcstr::DcstrSpec>;
///Device Chip Select Timing Setting Register
pub mod dcstr;
/**CDSR (rw) register accessor: Controller and Device Setting Register

You can [`read`](crate::Reg::read) this register and get [`cdsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cdsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cdsr`] module*/
#[doc(alias = "CDSR")]
pub type Cdsr = crate::Reg<cdsr::CdsrSpec>;
///Controller and Device Setting Register
pub mod cdsr;
/**MDLR (rw) register accessor: Memory Map Dummy Length Register

You can [`read`](crate::Reg::read) this register and get [`mdlr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mdlr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mdlr`] module*/
#[doc(alias = "MDLR")]
pub type Mdlr = crate::Reg<mdlr::MdlrSpec>;
///Memory Map Dummy Length Register
pub mod mdlr;
/**MRWCR0 (rw) register accessor: Memory Map Read/Write Command Register 0

You can [`read`](crate::Reg::read) this register and get [`mrwcr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrwcr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mrwcr0`] module*/
#[doc(alias = "MRWCR0")]
pub type Mrwcr0 = crate::Reg<mrwcr0::Mrwcr0Spec>;
///Memory Map Read/Write Command Register 0
pub mod mrwcr0;
/**MRWCR1 (rw) register accessor: Memory Map Read/Write Command Register 1

You can [`read`](crate::Reg::read) this register and get [`mrwcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrwcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mrwcr1`] module*/
#[doc(alias = "MRWCR1")]
pub type Mrwcr1 = crate::Reg<mrwcr1::Mrwcr1Spec>;
///Memory Map Read/Write Command Register 1
pub mod mrwcr1;
/**MRWCSR (rw) register accessor: Memory Map Read/Write Setting Register

You can [`read`](crate::Reg::read) this register and get [`mrwcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mrwcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@mrwcsr`] module*/
#[doc(alias = "MRWCSR")]
pub type Mrwcsr = crate::Reg<mrwcsr::MrwcsrSpec>;
///Memory Map Read/Write Setting Register
pub mod mrwcsr;
/**ESR (rw) register accessor: Error Status Register

You can [`read`](crate::Reg::read) this register and get [`esr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`esr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@esr`] module*/
#[doc(alias = "ESR")]
pub type Esr = crate::Reg<esr::EsrSpec>;
///Error Status Register
pub mod esr;
/**CWNDR (w) register accessor: Configure Write without Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwndr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cwndr`] module*/
#[doc(alias = "CWNDR")]
pub type Cwndr = crate::Reg<cwndr::CwndrSpec>;
///Configure Write without Data Register
pub mod cwndr;
/**CWDR (w) register accessor: Configure Write Data Register

You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cwdr::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cwdr`] module*/
#[doc(alias = "CWDR")]
pub type Cwdr = crate::Reg<cwdr::CwdrSpec>;
///Configure Write Data Register
pub mod cwdr;
/**CRR (r) register accessor: Configure Read Register

You can [`read`](crate::Reg::read) this register and get [`crr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@crr`] module*/
#[doc(alias = "CRR")]
pub type Crr = crate::Reg<crr::CrrSpec>;
///Configure Read Register
pub mod crr;
/**ACSR (rw) register accessor: Auto-Calibration Status Register

You can [`read`](crate::Reg::read) this register and get [`acsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`acsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@acsr`] module*/
#[doc(alias = "ACSR")]
pub type Acsr = crate::Reg<acsr::AcsrSpec>;
///Auto-Calibration Status Register
pub mod acsr;
/**DCSMXR (rw) register accessor: Device Chip Select Maximum Period Register

You can [`read`](crate::Reg::read) this register and get [`dcsmxr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcsmxr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcsmxr`] module*/
#[doc(alias = "DCSMXR")]
pub type Dcsmxr = crate::Reg<dcsmxr::DcsmxrSpec>;
///Device Chip Select Maximum Period Register
pub mod dcsmxr;
/**DWSCTSR (rw) register accessor: Device Memory Map Write single continuous translating size Register

You can [`read`](crate::Reg::read) this register and get [`dwsctsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dwsctsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dwsctsr`] module*/
#[doc(alias = "DWSCTSR")]
pub type Dwsctsr = crate::Reg<dwsctsr::DwsctsrSpec>;
///Device Memory Map Write single continuous translating size Register
pub mod dwsctsr;
