#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    adcsr: Adcsr,
    _reserved1: [u8; 0x02],
    adansa0: Adansa0,
    adansa1: Adansa1,
    adads0: Adads0,
    adads1: Adads1,
    adadc: Adadc,
    _reserved6: [u8; 0x01],
    adcer: Adcer,
    adstrgr: Adstrgr,
    adexicr: Adexicr,
    adansb0: Adansb0,
    adansb1: Adansb1,
    addbldr: Addbldr,
    adtsdr: Adtsdr,
    adocdr: Adocdr,
    adrd: Adrd,
    addr: [Addr; 3],
    _reserved16: [u8; 0x1a],
    addr16: Addr,
    addr17: Addr,
    addr18: Addr,
    addr19: Addr,
    addr20: Addr,
    addr21: Addr,
    addr22: Addr,
    addr23: Addr,
    addr24: Addr,
    addr25: Addr,
    addr26: Addr,
    addr27: Addr,
    addr28: Addr,
    _reserved29: [u8; 0x20],
    addiscr: Addiscr,
    _reserved30: [u8; 0x05],
    adgspcr: Adgspcr,
    _reserved31: [u8; 0x02],
    addbldra: Addbldra,
    addbldrb: Addbldrb,
    _reserved33: [u8; 0x04],
    adwinmon: Adwinmon,
    _reserved34: [u8; 0x03],
    adcmpcr: Adcmpcr,
    adcmpanser: Adcmpanser,
    adcmpler: Adcmpler,
    adcmpansr0: Adcmpansr0,
    adcmpansr1: Adcmpansr1,
    adcmplr0: Adcmplr0,
    adcmplr1: Adcmplr1,
    adcmpdr: [Adcmpdr; 2],
    adcmpsr0: Adcmpsr0,
    adcmpsr1: Adcmpsr1,
    adcmpser: Adcmpser,
    _reserved45: [u8; 0x01],
    adcmpbnsr: Adcmpbnsr,
    _reserved46: [u8; 0x01],
    adwinllb: Adwinllb,
    adwinulb: Adwinulb,
    adcmpbsr: Adcmpbsr,
    _reserved49: [u8; 0x03],
    adbuf: [Adbuf; 16],
    adbufen: Adbufen,
    _reserved51: [u8; 0x01],
    adbufptr: Adbufptr,
    _reserved52: [u8; 0x0a],
    adsstrl: Adsstrl,
    adsstrt: Adsstrt,
    adsstro: Adsstro,
    adsstr: [Adsstr; 3],
}
impl RegisterBlock {
    ///0x00 - A/D Control Register
    #[inline(always)]
    pub const fn adcsr(&self) -> &Adcsr {
        &self.adcsr
    }
    ///0x04 - A/D Channel Select Register A0
    #[inline(always)]
    pub const fn adansa0(&self) -> &Adansa0 {
        &self.adansa0
    }
    ///0x06 - A/D Channel Select Register A1
    #[inline(always)]
    pub const fn adansa1(&self) -> &Adansa1 {
        &self.adansa1
    }
    ///0x08 - A/D-Converted Value Addition/Average Channel Select Register 0
    #[inline(always)]
    pub const fn adads0(&self) -> &Adads0 {
        &self.adads0
    }
    ///0x0a - A/D-Converted Value Addition/Average Channel Select Register 1
    #[inline(always)]
    pub const fn adads1(&self) -> &Adads1 {
        &self.adads1
    }
    ///0x0c - A/D-Converted Value Addition/Average Count Select Register
    #[inline(always)]
    pub const fn adadc(&self) -> &Adadc {
        &self.adadc
    }
    ///0x0e - A/D Control Extended Register
    #[inline(always)]
    pub const fn adcer(&self) -> &Adcer {
        &self.adcer
    }
    ///0x10 - A/D Conversion Start Trigger Select Register
    #[inline(always)]
    pub const fn adstrgr(&self) -> &Adstrgr {
        &self.adstrgr
    }
    ///0x12 - A/D Conversion Extended Input Control Registers
    #[inline(always)]
    pub const fn adexicr(&self) -> &Adexicr {
        &self.adexicr
    }
    ///0x14 - A/D Channel Select Register B0
    #[inline(always)]
    pub const fn adansb0(&self) -> &Adansb0 {
        &self.adansb0
    }
    ///0x16 - A/D Channel Select Register B1
    #[inline(always)]
    pub const fn adansb1(&self) -> &Adansb1 {
        &self.adansb1
    }
    ///0x18 - A/D Data Duplexing Register
    #[inline(always)]
    pub const fn addbldr(&self) -> &Addbldr {
        &self.addbldr
    }
    ///0x1a - A/D Temperature Sensor Data Register
    #[inline(always)]
    pub const fn adtsdr(&self) -> &Adtsdr {
        &self.adtsdr
    }
    ///0x1c - A/D Internal Reference Voltage Data Register
    #[inline(always)]
    pub const fn adocdr(&self) -> &Adocdr {
        &self.adocdr
    }
    ///0x1e - A/D Self-Diagnosis Data Register
    #[inline(always)]
    pub const fn adrd(&self) -> &Adrd {
        &self.adrd
    }
    ///0x20..0x26 - A/D Data Registers %s
    #[inline(always)]
    pub const fn addr(&self, n: usize) -> &Addr {
        &self.addr[n]
    }
    ///Iterator for array of:
    ///0x20..0x26 - A/D Data Registers %s
    #[inline(always)]
    pub fn addr_iter(&self) -> impl Iterator<Item = &Addr> {
        self.addr.iter()
    }
    ///0x40 - A/D Data Registers 16
    #[inline(always)]
    pub const fn addr16(&self) -> &Addr {
        &self.addr16
    }
    ///0x40 - A/D Data Registers 17
    #[inline(always)]
    pub const fn addr17(&self) -> &Addr {
        &self.addr17
    }
    ///0x40 - A/D Data Registers 18
    #[inline(always)]
    pub const fn addr18(&self) -> &Addr {
        &self.addr18
    }
    ///0x40 - A/D Data Registers 19
    #[inline(always)]
    pub const fn addr19(&self) -> &Addr {
        &self.addr19
    }
    ///0x40 - A/D Data Registers 20
    #[inline(always)]
    pub const fn addr20(&self) -> &Addr {
        &self.addr20
    }
    ///0x40 - A/D Data Registers 21
    #[inline(always)]
    pub const fn addr21(&self) -> &Addr {
        &self.addr21
    }
    ///0x40 - A/D Data Registers 22
    #[inline(always)]
    pub const fn addr22(&self) -> &Addr {
        &self.addr22
    }
    ///0x40 - A/D Data Registers 23
    #[inline(always)]
    pub const fn addr23(&self) -> &Addr {
        &self.addr23
    }
    ///0x40 - A/D Data Registers 24
    #[inline(always)]
    pub const fn addr24(&self) -> &Addr {
        &self.addr24
    }
    ///0x40 - A/D Data Registers 25
    #[inline(always)]
    pub const fn addr25(&self) -> &Addr {
        &self.addr25
    }
    ///0x40 - A/D Data Registers 26
    #[inline(always)]
    pub const fn addr26(&self) -> &Addr {
        &self.addr26
    }
    ///0x40 - A/D Data Registers 27
    #[inline(always)]
    pub const fn addr27(&self) -> &Addr {
        &self.addr27
    }
    ///0x40 - A/D Data Registers 28
    #[inline(always)]
    pub const fn addr28(&self) -> &Addr {
        &self.addr28
    }
    ///0x7a - A/D Disconnection Detection Control Register
    #[inline(always)]
    pub const fn addiscr(&self) -> &Addiscr {
        &self.addiscr
    }
    ///0x80 - A/D Group Scan Priority Control Register
    #[inline(always)]
    pub const fn adgspcr(&self) -> &Adgspcr {
        &self.adgspcr
    }
    ///0x84 - A/D Data Duplexing Register A
    #[inline(always)]
    pub const fn addbldra(&self) -> &Addbldra {
        &self.addbldra
    }
    ///0x86 - A/D Data Duplexing Register B
    #[inline(always)]
    pub const fn addbldrb(&self) -> &Addbldrb {
        &self.addbldrb
    }
    ///0x8c - A/D Compare Function Window A/B Status Monitor Register
    #[inline(always)]
    pub const fn adwinmon(&self) -> &Adwinmon {
        &self.adwinmon
    }
    ///0x90 - A/D Compare Function Control Register
    #[inline(always)]
    pub const fn adcmpcr(&self) -> &Adcmpcr {
        &self.adcmpcr
    }
    ///0x92 - A/D Compare Function Window A Extended Input Select Register
    #[inline(always)]
    pub const fn adcmpanser(&self) -> &Adcmpanser {
        &self.adcmpanser
    }
    ///0x93 - A/D Compare Function Window A Extended Input Comparison Condition Setting Register
    #[inline(always)]
    pub const fn adcmpler(&self) -> &Adcmpler {
        &self.adcmpler
    }
    ///0x94 - A/D Compare Function Window A Channel Select Register 0
    #[inline(always)]
    pub const fn adcmpansr0(&self) -> &Adcmpansr0 {
        &self.adcmpansr0
    }
    ///0x96 - A/D Compare Function Window A Channel Select Register 1
    #[inline(always)]
    pub const fn adcmpansr1(&self) -> &Adcmpansr1 {
        &self.adcmpansr1
    }
    ///0x98 - A/D Compare Function Window A Comparison Condition Setting Register 0
    #[inline(always)]
    pub const fn adcmplr0(&self) -> &Adcmplr0 {
        &self.adcmplr0
    }
    ///0x9a - A/D Compare Function Window A Comparison Condition Setting Register 1
    #[inline(always)]
    pub const fn adcmplr1(&self) -> &Adcmplr1 {
        &self.adcmplr1
    }
    ///0x9c - A/D Compare Function Window A Lower-Side/Upper-Side Level Setting Register
    #[inline(always)]
    pub const fn adcmpdr(&self, n: usize) -> &Adcmpdr {
        &self.adcmpdr[n]
    }
    ///Iterator for array of:
    ///0x9c - A/D Compare Function Window A Lower-Side/Upper-Side Level Setting Register
    #[inline(always)]
    pub fn adcmpdr_iter(&self) -> impl Iterator<Item = &Adcmpdr> {
        self.adcmpdr.iter()
    }
    ///0xa0 - A/D Compare Function Window A Channel Status Register 0
    #[inline(always)]
    pub const fn adcmpsr0(&self) -> &Adcmpsr0 {
        &self.adcmpsr0
    }
    ///0xa2 - A/D Compare Function Window A Channel Status Register1
    #[inline(always)]
    pub const fn adcmpsr1(&self) -> &Adcmpsr1 {
        &self.adcmpsr1
    }
    ///0xa4 - A/D Compare Function Window A Extended Input Channel Status Register
    #[inline(always)]
    pub const fn adcmpser(&self) -> &Adcmpser {
        &self.adcmpser
    }
    ///0xa6 - A/D Compare Function Window B Channel Select Register
    #[inline(always)]
    pub const fn adcmpbnsr(&self) -> &Adcmpbnsr {
        &self.adcmpbnsr
    }
    ///0xa8 - A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register
    #[inline(always)]
    pub const fn adwinllb(&self) -> &Adwinllb {
        &self.adwinllb
    }
    ///0xaa - A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register
    #[inline(always)]
    pub const fn adwinulb(&self) -> &Adwinulb {
        &self.adwinulb
    }
    ///0xac - A/D Compare Function Window B Status Register
    #[inline(always)]
    pub const fn adcmpbsr(&self) -> &Adcmpbsr {
        &self.adcmpbsr
    }
    ///0xb0..0xd0 - A/D Data Buffer Registers %s
    #[inline(always)]
    pub const fn adbuf(&self, n: usize) -> &Adbuf {
        &self.adbuf[n]
    }
    ///Iterator for array of:
    ///0xb0..0xd0 - A/D Data Buffer Registers %s
    #[inline(always)]
    pub fn adbuf_iter(&self) -> impl Iterator<Item = &Adbuf> {
        self.adbuf.iter()
    }
    ///0xd0 - A/D Data Buffer Enable Register
    #[inline(always)]
    pub const fn adbufen(&self) -> &Adbufen {
        &self.adbufen
    }
    ///0xd2 - A/D Data Buffer Pointer Register
    #[inline(always)]
    pub const fn adbufptr(&self) -> &Adbufptr {
        &self.adbufptr
    }
    ///0xdd - A/D Sampling State Register
    #[inline(always)]
    pub const fn adsstrl(&self) -> &Adsstrl {
        &self.adsstrl
    }
    ///0xde - A/D Sampling State Register
    #[inline(always)]
    pub const fn adsstrt(&self) -> &Adsstrt {
        &self.adsstrt
    }
    ///0xdf - A/D Sampling State Register
    #[inline(always)]
    pub const fn adsstro(&self) -> &Adsstro {
        &self.adsstro
    }
    ///0xe0 - A/D Sampling State Register
    #[inline(always)]
    pub const fn adsstr(&self, n: usize) -> &Adsstr {
        &self.adsstr[n]
    }
    ///Iterator for array of:
    ///0xe0 - A/D Sampling State Register
    #[inline(always)]
    pub fn adsstr_iter(&self) -> impl Iterator<Item = &Adsstr> {
        self.adsstr.iter()
    }
}
/**ADCSR (rw) register accessor: A/D Control Register

You can [`read`](crate::Reg::read) this register and get [`adcsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcsr`] module*/
#[doc(alias = "ADCSR")]
pub type Adcsr = crate::Reg<adcsr::AdcsrSpec>;
///A/D Control Register
pub mod adcsr;
/**ADANSA0 (rw) register accessor: A/D Channel Select Register A0

You can [`read`](crate::Reg::read) this register and get [`adansa0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adansa0`] module*/
#[doc(alias = "ADANSA0")]
pub type Adansa0 = crate::Reg<adansa0::Adansa0Spec>;
///A/D Channel Select Register A0
pub mod adansa0;
/**ADANSA1 (rw) register accessor: A/D Channel Select Register A1

You can [`read`](crate::Reg::read) this register and get [`adansa1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansa1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adansa1`] module*/
#[doc(alias = "ADANSA1")]
pub type Adansa1 = crate::Reg<adansa1::Adansa1Spec>;
///A/D Channel Select Register A1
pub mod adansa1;
/**ADADS0 (rw) register accessor: A/D-Converted Value Addition/Average Channel Select Register 0

You can [`read`](crate::Reg::read) this register and get [`adads0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adads0`] module*/
#[doc(alias = "ADADS0")]
pub type Adads0 = crate::Reg<adads0::Adads0Spec>;
///A/D-Converted Value Addition/Average Channel Select Register 0
pub mod adads0;
/**ADADS1 (rw) register accessor: A/D-Converted Value Addition/Average Channel Select Register 1

You can [`read`](crate::Reg::read) this register and get [`adads1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adads1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adads1`] module*/
#[doc(alias = "ADADS1")]
pub type Adads1 = crate::Reg<adads1::Adads1Spec>;
///A/D-Converted Value Addition/Average Channel Select Register 1
pub mod adads1;
/**ADADC (rw) register accessor: A/D-Converted Value Addition/Average Count Select Register

You can [`read`](crate::Reg::read) this register and get [`adadc::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adadc::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adadc`] module*/
#[doc(alias = "ADADC")]
pub type Adadc = crate::Reg<adadc::AdadcSpec>;
///A/D-Converted Value Addition/Average Count Select Register
pub mod adadc;
/**ADCER (rw) register accessor: A/D Control Extended Register

You can [`read`](crate::Reg::read) this register and get [`adcer::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcer::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcer`] module*/
#[doc(alias = "ADCER")]
pub type Adcer = crate::Reg<adcer::AdcerSpec>;
///A/D Control Extended Register
pub mod adcer;
/**ADSTRGR (rw) register accessor: A/D Conversion Start Trigger Select Register

You can [`read`](crate::Reg::read) this register and get [`adstrgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adstrgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adstrgr`] module*/
#[doc(alias = "ADSTRGR")]
pub type Adstrgr = crate::Reg<adstrgr::AdstrgrSpec>;
///A/D Conversion Start Trigger Select Register
pub mod adstrgr;
/**ADEXICR (rw) register accessor: A/D Conversion Extended Input Control Registers

You can [`read`](crate::Reg::read) this register and get [`adexicr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adexicr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adexicr`] module*/
#[doc(alias = "ADEXICR")]
pub type Adexicr = crate::Reg<adexicr::AdexicrSpec>;
///A/D Conversion Extended Input Control Registers
pub mod adexicr;
/**ADANSB0 (rw) register accessor: A/D Channel Select Register B0

You can [`read`](crate::Reg::read) this register and get [`adansb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adansb0`] module*/
#[doc(alias = "ADANSB0")]
pub type Adansb0 = crate::Reg<adansb0::Adansb0Spec>;
///A/D Channel Select Register B0
pub mod adansb0;
/**ADANSB1 (rw) register accessor: A/D Channel Select Register B1

You can [`read`](crate::Reg::read) this register and get [`adansb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adansb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adansb1`] module*/
#[doc(alias = "ADANSB1")]
pub type Adansb1 = crate::Reg<adansb1::Adansb1Spec>;
///A/D Channel Select Register B1
pub mod adansb1;
/**ADDBLDR (r) register accessor: A/D Data Duplexing Register

You can [`read`](crate::Reg::read) this register and get [`addbldr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addbldr`] module*/
#[doc(alias = "ADDBLDR")]
pub type Addbldr = crate::Reg<addbldr::AddbldrSpec>;
///A/D Data Duplexing Register
pub mod addbldr;
/**ADTSDR (r) register accessor: A/D Temperature Sensor Data Register

You can [`read`](crate::Reg::read) this register and get [`adtsdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adtsdr`] module*/
#[doc(alias = "ADTSDR")]
pub type Adtsdr = crate::Reg<adtsdr::AdtsdrSpec>;
///A/D Temperature Sensor Data Register
pub mod adtsdr;
/**ADOCDR (r) register accessor: A/D Internal Reference Voltage Data Register

You can [`read`](crate::Reg::read) this register and get [`adocdr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adocdr`] module*/
#[doc(alias = "ADOCDR")]
pub type Adocdr = crate::Reg<adocdr::AdocdrSpec>;
///A/D Internal Reference Voltage Data Register
pub mod adocdr;
/**ADRD (r) register accessor: A/D Self-Diagnosis Data Register

You can [`read`](crate::Reg::read) this register and get [`adrd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adrd`] module*/
#[doc(alias = "ADRD")]
pub type Adrd = crate::Reg<adrd::AdrdSpec>;
///A/D Self-Diagnosis Data Register
pub mod adrd;
/**ADDR (r) register accessor: A/D Data Registers %s

You can [`read`](crate::Reg::read) this register and get [`addr::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addr`] module*/
#[doc(alias = "ADDR")]
pub type Addr = crate::Reg<addr::AddrSpec>;
///A/D Data Registers %s
pub mod addr;
pub use Addr as Addr16;
pub use addr as addr16;
/**ADDISCR (rw) register accessor: A/D Disconnection Detection Control Register

You can [`read`](crate::Reg::read) this register and get [`addiscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`addiscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addiscr`] module*/
#[doc(alias = "ADDISCR")]
pub type Addiscr = crate::Reg<addiscr::AddiscrSpec>;
///A/D Disconnection Detection Control Register
pub mod addiscr;
/**ADGSPCR (rw) register accessor: A/D Group Scan Priority Control Register

You can [`read`](crate::Reg::read) this register and get [`adgspcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adgspcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adgspcr`] module*/
#[doc(alias = "ADGSPCR")]
pub type Adgspcr = crate::Reg<adgspcr::AdgspcrSpec>;
///A/D Group Scan Priority Control Register
pub mod adgspcr;
/**ADDBLDRA (r) register accessor: A/D Data Duplexing Register A

You can [`read`](crate::Reg::read) this register and get [`addbldra::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addbldra`] module*/
#[doc(alias = "ADDBLDRA")]
pub type Addbldra = crate::Reg<addbldra::AddbldraSpec>;
///A/D Data Duplexing Register A
pub mod addbldra;
/**ADDBLDRB (r) register accessor: A/D Data Duplexing Register B

You can [`read`](crate::Reg::read) this register and get [`addbldrb::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@addbldrb`] module*/
#[doc(alias = "ADDBLDRB")]
pub type Addbldrb = crate::Reg<addbldrb::AddbldrbSpec>;
///A/D Data Duplexing Register B
pub mod addbldrb;
/**ADWINMON (r) register accessor: A/D Compare Function Window A/B Status Monitor Register

You can [`read`](crate::Reg::read) this register and get [`adwinmon::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adwinmon`] module*/
#[doc(alias = "ADWINMON")]
pub type Adwinmon = crate::Reg<adwinmon::AdwinmonSpec>;
///A/D Compare Function Window A/B Status Monitor Register
pub mod adwinmon;
/**ADCMPCR (rw) register accessor: A/D Compare Function Control Register

You can [`read`](crate::Reg::read) this register and get [`adcmpcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpcr`] module*/
#[doc(alias = "ADCMPCR")]
pub type Adcmpcr = crate::Reg<adcmpcr::AdcmpcrSpec>;
///A/D Compare Function Control Register
pub mod adcmpcr;
/**ADCMPANSER (rw) register accessor: A/D Compare Function Window A Extended Input Select Register

You can [`read`](crate::Reg::read) this register and get [`adcmpanser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpanser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpanser`] module*/
#[doc(alias = "ADCMPANSER")]
pub type Adcmpanser = crate::Reg<adcmpanser::AdcmpanserSpec>;
///A/D Compare Function Window A Extended Input Select Register
pub mod adcmpanser;
/**ADCMPLER (rw) register accessor: A/D Compare Function Window A Extended Input Comparison Condition Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpler::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpler::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpler`] module*/
#[doc(alias = "ADCMPLER")]
pub type Adcmpler = crate::Reg<adcmpler::AdcmplerSpec>;
///A/D Compare Function Window A Extended Input Comparison Condition Setting Register
pub mod adcmpler;
/**ADCMPANSR0 (rw) register accessor: A/D Compare Function Window A Channel Select Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmpansr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpansr0`] module*/
#[doc(alias = "ADCMPANSR0")]
pub type Adcmpansr0 = crate::Reg<adcmpansr0::Adcmpansr0Spec>;
///A/D Compare Function Window A Channel Select Register 0
pub mod adcmpansr0;
/**ADCMPANSR1 (rw) register accessor: A/D Compare Function Window A Channel Select Register 1

You can [`read`](crate::Reg::read) this register and get [`adcmpansr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpansr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpansr1`] module*/
#[doc(alias = "ADCMPANSR1")]
pub type Adcmpansr1 = crate::Reg<adcmpansr1::Adcmpansr1Spec>;
///A/D Compare Function Window A Channel Select Register 1
pub mod adcmpansr1;
/**ADCMPLR0 (rw) register accessor: A/D Compare Function Window A Comparison Condition Setting Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmplr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmplr0`] module*/
#[doc(alias = "ADCMPLR0")]
pub type Adcmplr0 = crate::Reg<adcmplr0::Adcmplr0Spec>;
///A/D Compare Function Window A Comparison Condition Setting Register 0
pub mod adcmplr0;
/**ADCMPLR1 (rw) register accessor: A/D Compare Function Window A Comparison Condition Setting Register 1

You can [`read`](crate::Reg::read) this register and get [`adcmplr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmplr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmplr1`] module*/
#[doc(alias = "ADCMPLR1")]
pub type Adcmplr1 = crate::Reg<adcmplr1::Adcmplr1Spec>;
///A/D Compare Function Window A Comparison Condition Setting Register 1
pub mod adcmplr1;
/**ADCMPDR (rw) register accessor: A/D Compare Function Window A Lower-Side/Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adcmpdr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpdr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpdr`] module*/
#[doc(alias = "ADCMPDR")]
pub type Adcmpdr = crate::Reg<adcmpdr::AdcmpdrSpec>;
///A/D Compare Function Window A Lower-Side/Upper-Side Level Setting Register
pub mod adcmpdr;
/**ADCMPSR0 (rw) register accessor: A/D Compare Function Window A Channel Status Register 0

You can [`read`](crate::Reg::read) this register and get [`adcmpsr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpsr0`] module*/
#[doc(alias = "ADCMPSR0")]
pub type Adcmpsr0 = crate::Reg<adcmpsr0::Adcmpsr0Spec>;
///A/D Compare Function Window A Channel Status Register 0
pub mod adcmpsr0;
/**ADCMPSR1 (rw) register accessor: A/D Compare Function Window A Channel Status Register1

You can [`read`](crate::Reg::read) this register and get [`adcmpsr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpsr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpsr1`] module*/
#[doc(alias = "ADCMPSR1")]
pub type Adcmpsr1 = crate::Reg<adcmpsr1::Adcmpsr1Spec>;
///A/D Compare Function Window A Channel Status Register1
pub mod adcmpsr1;
/**ADCMPSER (rw) register accessor: A/D Compare Function Window A Extended Input Channel Status Register

You can [`read`](crate::Reg::read) this register and get [`adcmpser::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpser::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpser`] module*/
#[doc(alias = "ADCMPSER")]
pub type Adcmpser = crate::Reg<adcmpser::AdcmpserSpec>;
///A/D Compare Function Window A Extended Input Channel Status Register
pub mod adcmpser;
/**ADCMPBNSR (rw) register accessor: A/D Compare Function Window B Channel Select Register

You can [`read`](crate::Reg::read) this register and get [`adcmpbnsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbnsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpbnsr`] module*/
#[doc(alias = "ADCMPBNSR")]
pub type Adcmpbnsr = crate::Reg<adcmpbnsr::AdcmpbnsrSpec>;
///A/D Compare Function Window B Channel Select Register
pub mod adcmpbnsr;
/**ADWINLLB (rw) register accessor: A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adwinllb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinllb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adwinllb`] module*/
#[doc(alias = "ADWINLLB")]
pub type Adwinllb = crate::Reg<adwinllb::AdwinllbSpec>;
///A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register
pub mod adwinllb;
/**ADWINULB (rw) register accessor: A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register

You can [`read`](crate::Reg::read) this register and get [`adwinulb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adwinulb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adwinulb`] module*/
#[doc(alias = "ADWINULB")]
pub type Adwinulb = crate::Reg<adwinulb::AdwinulbSpec>;
///A/D Compare Function Window B Lower-Side/Upper-Side Level Setting Register
pub mod adwinulb;
/**ADCMPBSR (rw) register accessor: A/D Compare Function Window B Status Register

You can [`read`](crate::Reg::read) this register and get [`adcmpbsr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adcmpbsr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adcmpbsr`] module*/
#[doc(alias = "ADCMPBSR")]
pub type Adcmpbsr = crate::Reg<adcmpbsr::AdcmpbsrSpec>;
///A/D Compare Function Window B Status Register
pub mod adcmpbsr;
/**ADBUF (r) register accessor: A/D Data Buffer Registers %s

You can [`read`](crate::Reg::read) this register and get [`adbuf::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adbuf`] module*/
#[doc(alias = "ADBUF")]
pub type Adbuf = crate::Reg<adbuf::AdbufSpec>;
///A/D Data Buffer Registers %s
pub mod adbuf;
/**ADBUFEN (rw) register accessor: A/D Data Buffer Enable Register

You can [`read`](crate::Reg::read) this register and get [`adbufen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adbufen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adbufen`] module*/
#[doc(alias = "ADBUFEN")]
pub type Adbufen = crate::Reg<adbufen::AdbufenSpec>;
///A/D Data Buffer Enable Register
pub mod adbufen;
/**ADBUFPTR (rw) register accessor: A/D Data Buffer Pointer Register

You can [`read`](crate::Reg::read) this register and get [`adbufptr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adbufptr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adbufptr`] module*/
#[doc(alias = "ADBUFPTR")]
pub type Adbufptr = crate::Reg<adbufptr::AdbufptrSpec>;
///A/D Data Buffer Pointer Register
pub mod adbufptr;
/**ADSSTRL (rw) register accessor: A/D Sampling State Register

You can [`read`](crate::Reg::read) this register and get [`adsstrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adsstrl`] module*/
#[doc(alias = "ADSSTRL")]
pub type Adsstrl = crate::Reg<adsstrl::AdsstrlSpec>;
///A/D Sampling State Register
pub mod adsstrl;
/**ADSSTRT (rw) register accessor: A/D Sampling State Register

You can [`read`](crate::Reg::read) this register and get [`adsstrt::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstrt::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adsstrt`] module*/
#[doc(alias = "ADSSTRT")]
pub type Adsstrt = crate::Reg<adsstrt::AdsstrtSpec>;
///A/D Sampling State Register
pub mod adsstrt;
/**ADSSTRO (rw) register accessor: A/D Sampling State Register

You can [`read`](crate::Reg::read) this register and get [`adsstro::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstro::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adsstro`] module*/
#[doc(alias = "ADSSTRO")]
pub type Adsstro = crate::Reg<adsstro::AdsstroSpec>;
///A/D Sampling State Register
pub mod adsstro;
/**ADSSTR (rw) register accessor: A/D Sampling State Register

You can [`read`](crate::Reg::read) this register and get [`adsstr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`adsstr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@adsstr`] module*/
#[doc(alias = "ADSSTR")]
pub type Adsstr = crate::Reg<adsstr::AdsstrSpec>;
///A/D Sampling State Register
pub mod adsstr;
