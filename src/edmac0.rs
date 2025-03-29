#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    edmr: Edmr,
    _reserved1: [u8; 0x04],
    edtrr: Edtrr,
    _reserved2: [u8; 0x04],
    edrrr: Edrrr,
    _reserved3: [u8; 0x04],
    tdlar: Tdlar,
    _reserved4: [u8; 0x04],
    rdlar: Rdlar,
    _reserved5: [u8; 0x04],
    eesr: Eesr,
    _reserved6: [u8; 0x04],
    eesipr: Eesipr,
    _reserved7: [u8; 0x04],
    trscer: Trscer,
    _reserved8: [u8; 0x04],
    rmfcr: Rmfcr,
    _reserved9: [u8; 0x04],
    tftr: Tftr,
    _reserved10: [u8; 0x04],
    fdr: Fdr,
    _reserved11: [u8; 0x04],
    rmcr: Rmcr,
    _reserved12: [u8; 0x08],
    tfucr: Tfucr,
    rfocr: Rfocr,
    iosr: Iosr,
    fcftr: Fcftr,
    _reserved16: [u8; 0x04],
    rpadir: Rpadir,
    trimd: Trimd,
    _reserved18: [u8; 0x48],
    rbwar: Rbwar,
    rdfar: Rdfar,
    _reserved20: [u8; 0x04],
    tbrar: Tbrar,
    tdfar: Tdfar,
}
impl RegisterBlock {
    ///0x00 - EDMAC Mode Register
    #[inline(always)]
    pub const fn edmr(&self) -> &Edmr {
        &self.edmr
    }
    ///0x08 - EDMAC Transmit Request Register
    #[inline(always)]
    pub const fn edtrr(&self) -> &Edtrr {
        &self.edtrr
    }
    ///0x10 - EDMAC Receive Request Register
    #[inline(always)]
    pub const fn edrrr(&self) -> &Edrrr {
        &self.edrrr
    }
    ///0x18 - Transmit Descriptor List Start Address Register
    #[inline(always)]
    pub const fn tdlar(&self) -> &Tdlar {
        &self.tdlar
    }
    ///0x20 - Receive Descriptor List Start Address Register
    #[inline(always)]
    pub const fn rdlar(&self) -> &Rdlar {
        &self.rdlar
    }
    ///0x28 - ETHERC/EDMAC Status Register
    #[inline(always)]
    pub const fn eesr(&self) -> &Eesr {
        &self.eesr
    }
    ///0x30 - ETHERC/EDMAC Status Interrupt Enable Register
    #[inline(always)]
    pub const fn eesipr(&self) -> &Eesipr {
        &self.eesipr
    }
    ///0x38 - ETHERC/EDMAC Transmit/Receive Status Copy Enable Register
    #[inline(always)]
    pub const fn trscer(&self) -> &Trscer {
        &self.trscer
    }
    ///0x40 - Missed-Frame Counter Register
    #[inline(always)]
    pub const fn rmfcr(&self) -> &Rmfcr {
        &self.rmfcr
    }
    ///0x48 - Transmit FIFO Threshold Register
    #[inline(always)]
    pub const fn tftr(&self) -> &Tftr {
        &self.tftr
    }
    ///0x50 - FIFO Depth Register
    #[inline(always)]
    pub const fn fdr(&self) -> &Fdr {
        &self.fdr
    }
    ///0x58 - Receive Method Control Register
    #[inline(always)]
    pub const fn rmcr(&self) -> &Rmcr {
        &self.rmcr
    }
    ///0x64 - Transmit FIFO Underflow Counter
    #[inline(always)]
    pub const fn tfucr(&self) -> &Tfucr {
        &self.tfucr
    }
    ///0x68 - Receive FIFO Overflow Counter
    #[inline(always)]
    pub const fn rfocr(&self) -> &Rfocr {
        &self.rfocr
    }
    ///0x6c - Independent Output Signal Setting Register
    #[inline(always)]
    pub const fn iosr(&self) -> &Iosr {
        &self.iosr
    }
    ///0x70 - Flow Control Start FIFO Threshold Setting Register
    #[inline(always)]
    pub const fn fcftr(&self) -> &Fcftr {
        &self.fcftr
    }
    ///0x78 - Receive Data Padding Insert Register
    #[inline(always)]
    pub const fn rpadir(&self) -> &Rpadir {
        &self.rpadir
    }
    ///0x7c - Transmit Interrupt Setting Register
    #[inline(always)]
    pub const fn trimd(&self) -> &Trimd {
        &self.trimd
    }
    ///0xc8 - Receive Buffer Write Address Register
    #[inline(always)]
    pub const fn rbwar(&self) -> &Rbwar {
        &self.rbwar
    }
    ///0xcc - Receive Descriptor Fetch Address Register
    #[inline(always)]
    pub const fn rdfar(&self) -> &Rdfar {
        &self.rdfar
    }
    ///0xd4 - Transmit Buffer Read Address Register
    #[inline(always)]
    pub const fn tbrar(&self) -> &Tbrar {
        &self.tbrar
    }
    ///0xd8 - Transmit Descriptor Fetch Address Register
    #[inline(always)]
    pub const fn tdfar(&self) -> &Tdfar {
        &self.tdfar
    }
}
/**EDMR (rw) register accessor: EDMAC Mode Register

You can [`read`](crate::Reg::read) this register and get [`edmr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edmr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@edmr`] module*/
#[doc(alias = "EDMR")]
pub type Edmr = crate::Reg<edmr::EdmrSpec>;
///EDMAC Mode Register
pub mod edmr;
/**EDTRR (rw) register accessor: EDMAC Transmit Request Register

You can [`read`](crate::Reg::read) this register and get [`edtrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edtrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@edtrr`] module*/
#[doc(alias = "EDTRR")]
pub type Edtrr = crate::Reg<edtrr::EdtrrSpec>;
///EDMAC Transmit Request Register
pub mod edtrr;
/**EDRRR (rw) register accessor: EDMAC Receive Request Register

You can [`read`](crate::Reg::read) this register and get [`edrrr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`edrrr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@edrrr`] module*/
#[doc(alias = "EDRRR")]
pub type Edrrr = crate::Reg<edrrr::EdrrrSpec>;
///EDMAC Receive Request Register
pub mod edrrr;
/**TDLAR (rw) register accessor: Transmit Descriptor List Start Address Register

You can [`read`](crate::Reg::read) this register and get [`tdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdlar`] module*/
#[doc(alias = "TDLAR")]
pub type Tdlar = crate::Reg<tdlar::TdlarSpec>;
///Transmit Descriptor List Start Address Register
pub mod tdlar;
/**RDLAR (rw) register accessor: Receive Descriptor List Start Address Register

You can [`read`](crate::Reg::read) this register and get [`rdlar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rdlar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdlar`] module*/
#[doc(alias = "RDLAR")]
pub type Rdlar = crate::Reg<rdlar::RdlarSpec>;
///Receive Descriptor List Start Address Register
pub mod rdlar;
/**EESR (rw) register accessor: ETHERC/EDMAC Status Register

You can [`read`](crate::Reg::read) this register and get [`eesr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eesr`] module*/
#[doc(alias = "EESR")]
pub type Eesr = crate::Reg<eesr::EesrSpec>;
///ETHERC/EDMAC Status Register
pub mod eesr;
/**EESIPR (rw) register accessor: ETHERC/EDMAC Status Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`eesipr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eesipr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@eesipr`] module*/
#[doc(alias = "EESIPR")]
pub type Eesipr = crate::Reg<eesipr::EesiprSpec>;
///ETHERC/EDMAC Status Interrupt Enable Register
pub mod eesipr;
/**TRSCER (rw) register accessor: ETHERC/EDMAC Transmit/Receive Status Copy Enable Register

You can [`read`](crate::Reg::read) this register and get [`trscer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trscer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trscer`] module*/
#[doc(alias = "TRSCER")]
pub type Trscer = crate::Reg<trscer::TrscerSpec>;
///ETHERC/EDMAC Transmit/Receive Status Copy Enable Register
pub mod trscer;
/**RMFCR (rw) register accessor: Missed-Frame Counter Register

You can [`read`](crate::Reg::read) this register and get [`rmfcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmfcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmfcr`] module*/
#[doc(alias = "RMFCR")]
pub type Rmfcr = crate::Reg<rmfcr::RmfcrSpec>;
///Missed-Frame Counter Register
pub mod rmfcr;
/**TFTR (rw) register accessor: Transmit FIFO Threshold Register

You can [`read`](crate::Reg::read) this register and get [`tftr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tftr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tftr`] module*/
#[doc(alias = "TFTR")]
pub type Tftr = crate::Reg<tftr::TftrSpec>;
///Transmit FIFO Threshold Register
pub mod tftr;
/**FDR (rw) register accessor: FIFO Depth Register

You can [`read`](crate::Reg::read) this register and get [`fdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fdr`] module*/
#[doc(alias = "FDR")]
pub type Fdr = crate::Reg<fdr::FdrSpec>;
///FIFO Depth Register
pub mod fdr;
/**RMCR (rw) register accessor: Receive Method Control Register

You can [`read`](crate::Reg::read) this register and get [`rmcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rmcr`] module*/
#[doc(alias = "RMCR")]
pub type Rmcr = crate::Reg<rmcr::RmcrSpec>;
///Receive Method Control Register
pub mod rmcr;
/**TFUCR (rw) register accessor: Transmit FIFO Underflow Counter

You can [`read`](crate::Reg::read) this register and get [`tfucr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tfucr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tfucr`] module*/
#[doc(alias = "TFUCR")]
pub type Tfucr = crate::Reg<tfucr::TfucrSpec>;
///Transmit FIFO Underflow Counter
pub mod tfucr;
/**RFOCR (rw) register accessor: Receive FIFO Overflow Counter

You can [`read`](crate::Reg::read) this register and get [`rfocr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rfocr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rfocr`] module*/
#[doc(alias = "RFOCR")]
pub type Rfocr = crate::Reg<rfocr::RfocrSpec>;
///Receive FIFO Overflow Counter
pub mod rfocr;
/**IOSR (rw) register accessor: Independent Output Signal Setting Register

You can [`read`](crate::Reg::read) this register and get [`iosr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`iosr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@iosr`] module*/
#[doc(alias = "IOSR")]
pub type Iosr = crate::Reg<iosr::IosrSpec>;
///Independent Output Signal Setting Register
pub mod iosr;
/**FCFTR (rw) register accessor: Flow Control Start FIFO Threshold Setting Register

You can [`read`](crate::Reg::read) this register and get [`fcftr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fcftr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@fcftr`] module*/
#[doc(alias = "FCFTR")]
pub type Fcftr = crate::Reg<fcftr::FcftrSpec>;
///Flow Control Start FIFO Threshold Setting Register
pub mod fcftr;
/**RPADIR (rw) register accessor: Receive Data Padding Insert Register

You can [`read`](crate::Reg::read) this register and get [`rpadir::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rpadir::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rpadir`] module*/
#[doc(alias = "RPADIR")]
pub type Rpadir = crate::Reg<rpadir::RpadirSpec>;
///Receive Data Padding Insert Register
pub mod rpadir;
/**TRIMD (rw) register accessor: Transmit Interrupt Setting Register

You can [`read`](crate::Reg::read) this register and get [`trimd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`trimd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@trimd`] module*/
#[doc(alias = "TRIMD")]
pub type Trimd = crate::Reg<trimd::TrimdSpec>;
///Transmit Interrupt Setting Register
pub mod trimd;
/**RBWAR (r) register accessor: Receive Buffer Write Address Register

You can [`read`](crate::Reg::read) this register and get [`rbwar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rbwar`] module*/
#[doc(alias = "RBWAR")]
pub type Rbwar = crate::Reg<rbwar::RbwarSpec>;
///Receive Buffer Write Address Register
pub mod rbwar;
/**RDFAR (r) register accessor: Receive Descriptor Fetch Address Register

You can [`read`](crate::Reg::read) this register and get [`rdfar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@rdfar`] module*/
#[doc(alias = "RDFAR")]
pub type Rdfar = crate::Reg<rdfar::RdfarSpec>;
///Receive Descriptor Fetch Address Register
pub mod rdfar;
/**TBRAR (r) register accessor: Transmit Buffer Read Address Register

You can [`read`](crate::Reg::read) this register and get [`tbrar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tbrar`] module*/
#[doc(alias = "TBRAR")]
pub type Tbrar = crate::Reg<tbrar::TbrarSpec>;
///Transmit Buffer Read Address Register
pub mod tbrar;
/**TDFAR (r) register accessor: Transmit Descriptor Fetch Address Register

You can [`read`](crate::Reg::read) this register and get [`tdfar::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@tdfar`] module*/
#[doc(alias = "TDFAR")]
pub type Tdfar = crate::Reg<tdfar::TdfarSpec>;
///Transmit Descriptor Fetch Address Register
pub mod tdfar;
