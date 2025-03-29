#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    syscfg: Syscfg,
    _reserved1: [u8; 0x02],
    syssts0: Syssts0,
    _reserved2: [u8; 0x02],
    dvstctr0: Dvstctr0,
    _reserved3: [u8; 0x0a],
    _reserved_3_cfifo: [u8; 0x02],
    _reserved4: [u8; 0x02],
    dfifol: (),
    dfifo: (),
    _reserved6: [u8; 0x08],
    cfifosel: Cfifosel,
    cfifoctr: Cfifoctr,
    _reserved8: [u8; 0x04],
    dfifosel: (),
    _reserved9: [u8; 0x02],
    dfifoctr: (),
    _reserved10: [u8; 0x06],
    intenb0: Intenb0,
    intenb1: Intenb1,
    _reserved12: [u8; 0x02],
    brdyenb: Brdyenb,
    nrdyenb: Nrdyenb,
    bempenb: Bempenb,
    sofcfg: Sofcfg,
    _reserved16: [u8; 0x02],
    intsts0: Intsts0,
    intsts1: Intsts1,
    _reserved18: [u8; 0x02],
    brdysts: Brdysts,
    nrdysts: Nrdysts,
    bempsts: Bempsts,
    frmnum: Frmnum,
    dvchgr: Dvchgr,
    usbaddr: Usbaddr,
    _reserved24: [u8; 0x02],
    usbreq: Usbreq,
    usbval: Usbval,
    usbindx: Usbindx,
    usbleng: Usbleng,
    dcpcfg: Dcpcfg,
    dcpmaxp: Dcpmaxp,
    dcpctr: Dcpctr,
    _reserved31: [u8; 0x02],
    pipesel: Pipesel,
    _reserved32: [u8; 0x02],
    pipecfg: Pipecfg,
    _reserved33: [u8; 0x02],
    pipemaxp: Pipemaxp,
    pipeperi: Pipeperi,
    pipectr: [Pipectr; 5],
    pipe6ctr: Pipectr,
    pipe7ctr: Pipectr,
    pipe8ctr: Pipectr,
    pipe9ctr: Pipectr,
    _reserved40: [u8; 0x0e],
    pipetre: (),
    _reserved41: [u8; 0x02],
    pipetrn: (),
    _reserved42: [u8; 0x1e],
    bcctrl1: Bcctrl1,
    bcctrl2: Bcctrl2,
    _reserved44: [u8; 0x18],
    devadd: [Devadd; 6],
    _reserved45: [u8; 0x18],
    physectrl: Physectrl,
    _reserved46: [u8; 0x0308],
    dpusr0r: Dpusr0r,
    dpusr1r: Dpusr1r,
}
impl RegisterBlock {
    ///0x00 - System Configuration Control Register
    #[inline(always)]
    pub const fn syscfg(&self) -> &Syscfg {
        &self.syscfg
    }
    ///0x04 - System Configuration Status Register 0
    #[inline(always)]
    pub const fn syssts0(&self) -> &Syssts0 {
        &self.syssts0
    }
    ///0x08 - Device State Control Register 0
    #[inline(always)]
    pub const fn dvstctr0(&self) -> &Dvstctr0 {
        &self.dvstctr0
    }
    ///0x14 - CFIFO Port Register
    #[inline(always)]
    pub const fn cfifol(&self) -> &Cfifol {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x14 - CFIFO Port Register
    #[inline(always)]
    pub const fn cfifo(&self) -> &Cfifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x18 - D%sFIFO Port Register
    #[inline(always)]
    pub const fn dfifol(&self, n: usize) -> &Dfifol {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x18 - D%sFIFO Port Register
    #[inline(always)]
    pub fn dfifol_iter(&self) -> impl Iterator<Item = &Dfifol> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24).add(4 * n).cast()
            })
    }
    ///0x18 - D0FIFO Port Register
    #[inline(always)]
    pub const fn d0fifol(&self) -> &Dfifol {
        self.dfifol(0)
    }
    ///0x1c - D1FIFO Port Register
    #[inline(always)]
    pub const fn d1fifol(&self) -> &Dfifol {
        self.dfifol(1)
    }
    ///0x18 - D%sFIFO Port Register
    #[inline(always)]
    pub const fn dfifo(&self, n: usize) -> &Dfifo {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x18 - D%sFIFO Port Register
    #[inline(always)]
    pub fn dfifo_iter(&self) -> impl Iterator<Item = &Dfifo> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(24).add(4 * n).cast()
            })
    }
    ///0x18 - D0FIFO Port Register
    #[inline(always)]
    pub const fn d0fifo(&self) -> &Dfifo {
        self.dfifo(0)
    }
    ///0x1c - D1FIFO Port Register
    #[inline(always)]
    pub const fn d1fifo(&self) -> &Dfifo {
        self.dfifo(1)
    }
    ///0x20 - CFIFO Port Select Register
    #[inline(always)]
    pub const fn cfifosel(&self) -> &Cfifosel {
        &self.cfifosel
    }
    ///0x22 - CFIFO Port Control Register
    #[inline(always)]
    pub const fn cfifoctr(&self) -> &Cfifoctr {
        &self.cfifoctr
    }
    ///0x28 - D%sFIFO Port Select Register
    #[inline(always)]
    pub const fn dfifosel(&self, n: usize) -> &Dfifosel {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x28 - D%sFIFO Port Select Register
    #[inline(always)]
    pub fn dfifosel_iter(&self) -> impl Iterator<Item = &Dfifosel> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(40).add(4 * n).cast()
            })
    }
    ///0x28 - D0FIFO Port Select Register
    #[inline(always)]
    pub const fn d0fifosel(&self) -> &Dfifosel {
        self.dfifosel(0)
    }
    ///0x2c - D1FIFO Port Select Register
    #[inline(always)]
    pub const fn d1fifosel(&self) -> &Dfifosel {
        self.dfifosel(1)
    }
    ///0x2a - D%sFIFO Port Control Register
    #[inline(always)]
    pub const fn dfifoctr(&self, n: usize) -> &Dfifoctr {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2a - D%sFIFO Port Control Register
    #[inline(always)]
    pub fn dfifoctr_iter(&self) -> impl Iterator<Item = &Dfifoctr> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(42).add(4 * n).cast()
            })
    }
    ///0x2a - D0FIFO Port Control Register
    #[inline(always)]
    pub const fn d0fifoctr(&self) -> &Dfifoctr {
        self.dfifoctr(0)
    }
    ///0x2e - D1FIFO Port Control Register
    #[inline(always)]
    pub const fn d1fifoctr(&self) -> &Dfifoctr {
        self.dfifoctr(1)
    }
    ///0x30 - Interrupt Enable Register 0
    #[inline(always)]
    pub const fn intenb0(&self) -> &Intenb0 {
        &self.intenb0
    }
    ///0x32 - Interrupt Enable Register 1
    #[inline(always)]
    pub const fn intenb1(&self) -> &Intenb1 {
        &self.intenb1
    }
    ///0x36 - BRDY Interrupt Enable Register
    #[inline(always)]
    pub const fn brdyenb(&self) -> &Brdyenb {
        &self.brdyenb
    }
    ///0x38 - NRDY Interrupt Enable Register
    #[inline(always)]
    pub const fn nrdyenb(&self) -> &Nrdyenb {
        &self.nrdyenb
    }
    ///0x3a - BEMP Interrupt Enable Register
    #[inline(always)]
    pub const fn bempenb(&self) -> &Bempenb {
        &self.bempenb
    }
    ///0x3c - SOF Output Configuration Register
    #[inline(always)]
    pub const fn sofcfg(&self) -> &Sofcfg {
        &self.sofcfg
    }
    ///0x40 - Interrupt Status Register 0
    #[inline(always)]
    pub const fn intsts0(&self) -> &Intsts0 {
        &self.intsts0
    }
    ///0x42 - Interrupt Status Register 1
    #[inline(always)]
    pub const fn intsts1(&self) -> &Intsts1 {
        &self.intsts1
    }
    ///0x46 - BRDY Interrupt Status Register
    #[inline(always)]
    pub const fn brdysts(&self) -> &Brdysts {
        &self.brdysts
    }
    ///0x48 - NRDY Interrupt Status Register
    #[inline(always)]
    pub const fn nrdysts(&self) -> &Nrdysts {
        &self.nrdysts
    }
    ///0x4a - BEMP Interrupt Status Register
    #[inline(always)]
    pub const fn bempsts(&self) -> &Bempsts {
        &self.bempsts
    }
    ///0x4c - Frame Number Register
    #[inline(always)]
    pub const fn frmnum(&self) -> &Frmnum {
        &self.frmnum
    }
    ///0x4e - Device State Change Register
    #[inline(always)]
    pub const fn dvchgr(&self) -> &Dvchgr {
        &self.dvchgr
    }
    ///0x50 - USB Address Register
    #[inline(always)]
    pub const fn usbaddr(&self) -> &Usbaddr {
        &self.usbaddr
    }
    ///0x54 - USB Request Type Register
    #[inline(always)]
    pub const fn usbreq(&self) -> &Usbreq {
        &self.usbreq
    }
    ///0x56 - USB Request Value Register
    #[inline(always)]
    pub const fn usbval(&self) -> &Usbval {
        &self.usbval
    }
    ///0x58 - USB Request Index Register
    #[inline(always)]
    pub const fn usbindx(&self) -> &Usbindx {
        &self.usbindx
    }
    ///0x5a - USB Request Length Register
    #[inline(always)]
    pub const fn usbleng(&self) -> &Usbleng {
        &self.usbleng
    }
    ///0x5c - DCP Configuration Register
    #[inline(always)]
    pub const fn dcpcfg(&self) -> &Dcpcfg {
        &self.dcpcfg
    }
    ///0x5e - DCP Maximum Packet Size Register
    #[inline(always)]
    pub const fn dcpmaxp(&self) -> &Dcpmaxp {
        &self.dcpmaxp
    }
    ///0x60 - DCP Control Register
    #[inline(always)]
    pub const fn dcpctr(&self) -> &Dcpctr {
        &self.dcpctr
    }
    ///0x64 - Pipe Window Select Register
    #[inline(always)]
    pub const fn pipesel(&self) -> &Pipesel {
        &self.pipesel
    }
    ///0x68 - Pipe Configuration Register
    #[inline(always)]
    pub const fn pipecfg(&self) -> &Pipecfg {
        &self.pipecfg
    }
    ///0x6c - Pipe Maximum Packet Size Register
    #[inline(always)]
    pub const fn pipemaxp(&self) -> &Pipemaxp {
        &self.pipemaxp
    }
    ///0x6e - Pipe Cycle Control Register
    #[inline(always)]
    pub const fn pipeperi(&self) -> &Pipeperi {
        &self.pipeperi
    }
    ///0x70..0x7a - PIPE%s Control Registers
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1CTR` register.</div>
    #[inline(always)]
    pub const fn pipectr(&self, n: usize) -> &Pipectr {
        &self.pipectr[n]
    }
    ///Iterator for array of:
    ///0x70..0x7a - PIPE%s Control Registers
    #[inline(always)]
    pub fn pipectr_iter(&self) -> impl Iterator<Item = &Pipectr> {
        self.pipectr.iter()
    }
    ///0x70 - PIPE1 Control Registers
    #[inline(always)]
    pub const fn pipe1ctr(&self) -> &Pipectr {
        self.pipectr(0)
    }
    ///0x72 - PIPE2 Control Registers
    #[inline(always)]
    pub const fn pipe2ctr(&self) -> &Pipectr {
        self.pipectr(1)
    }
    ///0x74 - PIPE3 Control Registers
    #[inline(always)]
    pub const fn pipe3ctr(&self) -> &Pipectr {
        self.pipectr(2)
    }
    ///0x76 - PIPE4 Control Registers
    #[inline(always)]
    pub const fn pipe4ctr(&self) -> &Pipectr {
        self.pipectr(3)
    }
    ///0x78 - PIPE5 Control Registers
    #[inline(always)]
    pub const fn pipe5ctr(&self) -> &Pipectr {
        self.pipectr(4)
    }
    ///0x7a - PIPE6 Control Registers
    #[inline(always)]
    pub const fn pipe6ctr(&self) -> &Pipectr {
        &self.pipe6ctr
    }
    ///0x7a - PIPE7 Control Registers
    #[inline(always)]
    pub const fn pipe7ctr(&self) -> &Pipectr {
        &self.pipe7ctr
    }
    ///0x7a - PIPE8 Control Registers
    #[inline(always)]
    pub const fn pipe8ctr(&self) -> &Pipectr {
        &self.pipe8ctr
    }
    ///0x7a - PIPE9 Control Registers
    #[inline(always)]
    pub const fn pipe9ctr(&self) -> &Pipectr {
        &self.pipe9ctr
    }
    ///0x90..0x9a - PIPE%s Transaction Counter Enable Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1TRE` register.</div>
    #[inline(always)]
    pub const fn pipetre(&self, n: usize) -> &Pipetre {
        #[allow(clippy::no_effect)] [(); 5][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x90..0x9a - PIPE%s Transaction Counter Enable Register
    #[inline(always)]
    pub fn pipetre_iter(&self) -> impl Iterator<Item = &Pipetre> {
        (0..5)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(144).add(4 * n).cast()
            })
    }
    ///0x90 - PIPE1 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe1tre(&self) -> &Pipetre {
        self.pipetre(0)
    }
    ///0x94 - PIPE2 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe2tre(&self) -> &Pipetre {
        self.pipetre(1)
    }
    ///0x98 - PIPE3 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe3tre(&self) -> &Pipetre {
        self.pipetre(2)
    }
    ///0x9c - PIPE4 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe4tre(&self) -> &Pipetre {
        self.pipetre(3)
    }
    ///0xa0 - PIPE5 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe5tre(&self) -> &Pipetre {
        self.pipetre(4)
    }
    ///0x92..0x9c - PIPE%s Transaction Counter Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1TRN` register.</div>
    #[inline(always)]
    pub const fn pipetrn(&self, n: usize) -> &Pipetrn {
        #[allow(clippy::no_effect)] [(); 5][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(146).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x92..0x9c - PIPE%s Transaction Counter Register
    #[inline(always)]
    pub fn pipetrn_iter(&self) -> impl Iterator<Item = &Pipetrn> {
        (0..5)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(146).add(4 * n).cast()
            })
    }
    ///0x92 - PIPE1 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe1trn(&self) -> &Pipetrn {
        self.pipetrn(0)
    }
    ///0x96 - PIPE2 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe2trn(&self) -> &Pipetrn {
        self.pipetrn(1)
    }
    ///0x9a - PIPE3 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe3trn(&self) -> &Pipetrn {
        self.pipetrn(2)
    }
    ///0x9e - PIPE4 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe4trn(&self) -> &Pipetrn {
        self.pipetrn(3)
    }
    ///0xa2 - PIPE5 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe5trn(&self) -> &Pipetrn {
        self.pipetrn(4)
    }
    ///0xb0 - Battery Charging Control Register 1
    #[inline(always)]
    pub const fn bcctrl1(&self) -> &Bcctrl1 {
        &self.bcctrl1
    }
    ///0xb4 - Battery Charging Control Register 2
    #[inline(always)]
    pub const fn bcctrl2(&self) -> &Bcctrl2 {
        &self.bcctrl2
    }
    ///0xd0..0xdc - Device Address %s Configuration Register
    #[inline(always)]
    pub const fn devadd(&self, n: usize) -> &Devadd {
        &self.devadd[n]
    }
    ///Iterator for array of:
    ///0xd0..0xdc - Device Address %s Configuration Register
    #[inline(always)]
    pub fn devadd_iter(&self) -> impl Iterator<Item = &Devadd> {
        self.devadd.iter()
    }
    ///0xf4 - PHY Single-ended Receiver Control Register
    #[inline(always)]
    pub const fn physectrl(&self) -> &Physectrl {
        &self.physectrl
    }
    ///0x400 - Deep Software Standby USB Transceiver Control/Pin Monitor Register
    #[inline(always)]
    pub const fn dpusr0r(&self) -> &Dpusr0r {
        &self.dpusr0r
    }
    ///0x404 - Deep Software Standby USB Suspend/Resume Interrupt Register
    #[inline(always)]
    pub const fn dpusr1r(&self) -> &Dpusr1r {
        &self.dpusr1r
    }
}
/**SYSCFG (rw) register accessor: System Configuration Control Register

You can [`read`](crate::Reg::read) this register and get [`syscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscfg`] module*/
#[doc(alias = "SYSCFG")]
pub type Syscfg = crate::Reg<syscfg::SyscfgSpec>;
///System Configuration Control Register
pub mod syscfg;
/**SYSSTS0 (r) register accessor: System Configuration Status Register 0

You can [`read`](crate::Reg::read) this register and get [`syssts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syssts0`] module*/
#[doc(alias = "SYSSTS0")]
pub type Syssts0 = crate::Reg<syssts0::Syssts0Spec>;
///System Configuration Status Register 0
pub mod syssts0;
/**DVSTCTR0 (rw) register accessor: Device State Control Register 0

You can [`read`](crate::Reg::read) this register and get [`dvstctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvstctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dvstctr0`] module*/
#[doc(alias = "DVSTCTR0")]
pub type Dvstctr0 = crate::Reg<dvstctr0::Dvstctr0Spec>;
///Device State Control Register 0
pub mod dvstctr0;
/**CFIFO (rw) register accessor: CFIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifo`] module*/
#[doc(alias = "CFIFO")]
pub type Cfifo = crate::Reg<cfifo::CfifoSpec>;
///CFIFO Port Register
pub mod cfifo;
/**CFIFOL (rw) register accessor: CFIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifol`] module*/
#[doc(alias = "CFIFOL")]
pub type Cfifol = crate::Reg<cfifol::CfifolSpec>;
///CFIFO Port Register
pub mod cfifol;
/**DFIFO (rw) register accessor: D%sFIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`dfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfifo`] module*/
#[doc(alias = "DFIFO")]
pub type Dfifo = crate::Reg<dfifo::DfifoSpec>;
///D%sFIFO Port Register
pub mod dfifo;
/**DFIFOL (rw) register accessor: D%sFIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`dfifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfifol`] module*/
#[doc(alias = "DFIFOL")]
pub type Dfifol = crate::Reg<dfifol::DfifolSpec>;
///D%sFIFO Port Register
pub mod dfifol;
/**CFIFOSEL (rw) register accessor: CFIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`cfifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifosel`] module*/
#[doc(alias = "CFIFOSEL")]
pub type Cfifosel = crate::Reg<cfifosel::CfifoselSpec>;
///CFIFO Port Select Register
pub mod cfifosel;
/**CFIFOCTR (rw) register accessor: CFIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`cfifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifoctr`] module*/
#[doc(alias = "CFIFOCTR")]
pub type Cfifoctr = crate::Reg<cfifoctr::CfifoctrSpec>;
///CFIFO Port Control Register
pub mod cfifoctr;
/**DFIFOSEL (rw) register accessor: D%sFIFO Port Select Register

You can [`read`](crate::Reg::read) this register and get [`dfifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfifosel`] module*/
#[doc(alias = "DFIFOSEL")]
pub type Dfifosel = crate::Reg<dfifosel::DfifoselSpec>;
///D%sFIFO Port Select Register
pub mod dfifosel;
/**DFIFOCTR (rw) register accessor: D%sFIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`dfifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfifoctr`] module*/
#[doc(alias = "DFIFOCTR")]
pub type Dfifoctr = crate::Reg<dfifoctr::DfifoctrSpec>;
///D%sFIFO Port Control Register
pub mod dfifoctr;
/**INTENB0 (rw) register accessor: Interrupt Enable Register 0

You can [`read`](crate::Reg::read) this register and get [`intenb0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intenb0`] module*/
#[doc(alias = "INTENB0")]
pub type Intenb0 = crate::Reg<intenb0::Intenb0Spec>;
///Interrupt Enable Register 0
pub mod intenb0;
/**INTENB1 (rw) register accessor: Interrupt Enable Register 1

You can [`read`](crate::Reg::read) this register and get [`intenb1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenb1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intenb1`] module*/
#[doc(alias = "INTENB1")]
pub type Intenb1 = crate::Reg<intenb1::Intenb1Spec>;
///Interrupt Enable Register 1
pub mod intenb1;
/**BRDYENB (rw) register accessor: BRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`brdyenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdyenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brdyenb`] module*/
#[doc(alias = "BRDYENB")]
pub type Brdyenb = crate::Reg<brdyenb::BrdyenbSpec>;
///BRDY Interrupt Enable Register
pub mod brdyenb;
/**NRDYENB (rw) register accessor: NRDY Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`nrdyenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdyenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nrdyenb`] module*/
#[doc(alias = "NRDYENB")]
pub type Nrdyenb = crate::Reg<nrdyenb::NrdyenbSpec>;
///NRDY Interrupt Enable Register
pub mod nrdyenb;
/**BEMPENB (rw) register accessor: BEMP Interrupt Enable Register

You can [`read`](crate::Reg::read) this register and get [`bempenb::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempenb::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bempenb`] module*/
#[doc(alias = "BEMPENB")]
pub type Bempenb = crate::Reg<bempenb::BempenbSpec>;
///BEMP Interrupt Enable Register
pub mod bempenb;
/**SOFCFG (rw) register accessor: SOF Output Configuration Register

You can [`read`](crate::Reg::read) this register and get [`sofcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sofcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@sofcfg`] module*/
#[doc(alias = "SOFCFG")]
pub type Sofcfg = crate::Reg<sofcfg::SofcfgSpec>;
///SOF Output Configuration Register
pub mod sofcfg;
/**INTSTS0 (rw) register accessor: Interrupt Status Register 0

You can [`read`](crate::Reg::read) this register and get [`intsts0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intsts0`] module*/
#[doc(alias = "INTSTS0")]
pub type Intsts0 = crate::Reg<intsts0::Intsts0Spec>;
///Interrupt Status Register 0
pub mod intsts0;
/**INTSTS1 (rw) register accessor: Interrupt Status Register 1

You can [`read`](crate::Reg::read) this register and get [`intsts1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intsts1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@intsts1`] module*/
#[doc(alias = "INTSTS1")]
pub type Intsts1 = crate::Reg<intsts1::Intsts1Spec>;
///Interrupt Status Register 1
pub mod intsts1;
/**BRDYSTS (rw) register accessor: BRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`brdysts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`brdysts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@brdysts`] module*/
#[doc(alias = "BRDYSTS")]
pub type Brdysts = crate::Reg<brdysts::BrdystsSpec>;
///BRDY Interrupt Status Register
pub mod brdysts;
/**NRDYSTS (rw) register accessor: NRDY Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`nrdysts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nrdysts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@nrdysts`] module*/
#[doc(alias = "NRDYSTS")]
pub type Nrdysts = crate::Reg<nrdysts::NrdystsSpec>;
///NRDY Interrupt Status Register
pub mod nrdysts;
/**BEMPSTS (rw) register accessor: BEMP Interrupt Status Register

You can [`read`](crate::Reg::read) this register and get [`bempsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bempsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bempsts`] module*/
#[doc(alias = "BEMPSTS")]
pub type Bempsts = crate::Reg<bempsts::BempstsSpec>;
///BEMP Interrupt Status Register
pub mod bempsts;
/**FRMNUM (rw) register accessor: Frame Number Register

You can [`read`](crate::Reg::read) this register and get [`frmnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`frmnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@frmnum`] module*/
#[doc(alias = "FRMNUM")]
pub type Frmnum = crate::Reg<frmnum::FrmnumSpec>;
///Frame Number Register
pub mod frmnum;
/**DVCHGR (rw) register accessor: Device State Change Register

You can [`read`](crate::Reg::read) this register and get [`dvchgr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvchgr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dvchgr`] module*/
#[doc(alias = "DVCHGR")]
pub type Dvchgr = crate::Reg<dvchgr::DvchgrSpec>;
///Device State Change Register
pub mod dvchgr;
/**USBADDR (rw) register accessor: USB Address Register

You can [`read`](crate::Reg::read) this register and get [`usbaddr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbaddr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbaddr`] module*/
#[doc(alias = "USBADDR")]
pub type Usbaddr = crate::Reg<usbaddr::UsbaddrSpec>;
///USB Address Register
pub mod usbaddr;
/**USBREQ (rw) register accessor: USB Request Type Register

You can [`read`](crate::Reg::read) this register and get [`usbreq::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbreq::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbreq`] module*/
#[doc(alias = "USBREQ")]
pub type Usbreq = crate::Reg<usbreq::UsbreqSpec>;
///USB Request Type Register
pub mod usbreq;
/**USBVAL (rw) register accessor: USB Request Value Register

You can [`read`](crate::Reg::read) this register and get [`usbval::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbval::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbval`] module*/
#[doc(alias = "USBVAL")]
pub type Usbval = crate::Reg<usbval::UsbvalSpec>;
///USB Request Value Register
pub mod usbval;
/**USBINDX (rw) register accessor: USB Request Index Register

You can [`read`](crate::Reg::read) this register and get [`usbindx::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbindx::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbindx`] module*/
#[doc(alias = "USBINDX")]
pub type Usbindx = crate::Reg<usbindx::UsbindxSpec>;
///USB Request Index Register
pub mod usbindx;
/**USBLENG (rw) register accessor: USB Request Length Register

You can [`read`](crate::Reg::read) this register and get [`usbleng::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usbleng::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@usbleng`] module*/
#[doc(alias = "USBLENG")]
pub type Usbleng = crate::Reg<usbleng::UsblengSpec>;
///USB Request Length Register
pub mod usbleng;
/**DCPCFG (rw) register accessor: DCP Configuration Register

You can [`read`](crate::Reg::read) this register and get [`dcpcfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpcfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcpcfg`] module*/
#[doc(alias = "DCPCFG")]
pub type Dcpcfg = crate::Reg<dcpcfg::DcpcfgSpec>;
///DCP Configuration Register
pub mod dcpcfg;
/**DCPMAXP (rw) register accessor: DCP Maximum Packet Size Register

You can [`read`](crate::Reg::read) this register and get [`dcpmaxp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpmaxp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcpmaxp`] module*/
#[doc(alias = "DCPMAXP")]
pub type Dcpmaxp = crate::Reg<dcpmaxp::DcpmaxpSpec>;
///DCP Maximum Packet Size Register
pub mod dcpmaxp;
/**DCPCTR (rw) register accessor: DCP Control Register

You can [`read`](crate::Reg::read) this register and get [`dcpctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcpctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dcpctr`] module*/
#[doc(alias = "DCPCTR")]
pub type Dcpctr = crate::Reg<dcpctr::DcpctrSpec>;
///DCP Control Register
pub mod dcpctr;
/**PIPESEL (rw) register accessor: Pipe Window Select Register

You can [`read`](crate::Reg::read) this register and get [`pipesel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipesel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipesel`] module*/
#[doc(alias = "PIPESEL")]
pub type Pipesel = crate::Reg<pipesel::PipeselSpec>;
///Pipe Window Select Register
pub mod pipesel;
/**PIPECFG (rw) register accessor: Pipe Configuration Register

You can [`read`](crate::Reg::read) this register and get [`pipecfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipecfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipecfg`] module*/
#[doc(alias = "PIPECFG")]
pub type Pipecfg = crate::Reg<pipecfg::PipecfgSpec>;
///Pipe Configuration Register
pub mod pipecfg;
/**PIPEMAXP (rw) register accessor: Pipe Maximum Packet Size Register

You can [`read`](crate::Reg::read) this register and get [`pipemaxp::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipemaxp::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipemaxp`] module*/
#[doc(alias = "PIPEMAXP")]
pub type Pipemaxp = crate::Reg<pipemaxp::PipemaxpSpec>;
///Pipe Maximum Packet Size Register
pub mod pipemaxp;
/**PIPEPERI (rw) register accessor: Pipe Cycle Control Register

You can [`read`](crate::Reg::read) this register and get [`pipeperi::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipeperi::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipeperi`] module*/
#[doc(alias = "PIPEPERI")]
pub type Pipeperi = crate::Reg<pipeperi::PipeperiSpec>;
///Pipe Cycle Control Register
pub mod pipeperi;
/**PIPECTR (rw) register accessor: PIPE%s Control Registers

You can [`read`](crate::Reg::read) this register and get [`pipectr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipectr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipectr`] module*/
#[doc(alias = "PIPECTR")]
pub type Pipectr = crate::Reg<pipectr::PipectrSpec>;
///PIPE%s Control Registers
pub mod pipectr;
pub use Pipectr as Pipe6ctr;
pub use pipectr as pipe6ctr;
/**PIPETRE (rw) register accessor: PIPE%s Transaction Counter Enable Register

You can [`read`](crate::Reg::read) this register and get [`pipetre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipetre`] module*/
#[doc(alias = "PIPETRE")]
pub type Pipetre = crate::Reg<pipetre::PipetreSpec>;
///PIPE%s Transaction Counter Enable Register
pub mod pipetre;
/**PIPETRN (rw) register accessor: PIPE%s Transaction Counter Register

You can [`read`](crate::Reg::read) this register and get [`pipetrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipetrn`] module*/
#[doc(alias = "PIPETRN")]
pub type Pipetrn = crate::Reg<pipetrn::PipetrnSpec>;
///PIPE%s Transaction Counter Register
pub mod pipetrn;
/**BCCTRL1 (rw) register accessor: Battery Charging Control Register 1

You can [`read`](crate::Reg::read) this register and get [`bcctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcctrl1`] module*/
#[doc(alias = "BCCTRL1")]
pub type Bcctrl1 = crate::Reg<bcctrl1::Bcctrl1Spec>;
///Battery Charging Control Register 1
pub mod bcctrl1;
/**BCCTRL2 (rw) register accessor: Battery Charging Control Register 2

You can [`read`](crate::Reg::read) this register and get [`bcctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcctrl2`] module*/
#[doc(alias = "BCCTRL2")]
pub type Bcctrl2 = crate::Reg<bcctrl2::Bcctrl2Spec>;
///Battery Charging Control Register 2
pub mod bcctrl2;
/**DEVADD (rw) register accessor: Device Address %s Configuration Register

You can [`read`](crate::Reg::read) this register and get [`devadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@devadd`] module*/
#[doc(alias = "DEVADD")]
pub type Devadd = crate::Reg<devadd::DevaddSpec>;
///Device Address %s Configuration Register
pub mod devadd;
/**PHYSECTRL (rw) register accessor: PHY Single-ended Receiver Control Register

You can [`read`](crate::Reg::read) this register and get [`physectrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physectrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@physectrl`] module*/
#[doc(alias = "PHYSECTRL")]
pub type Physectrl = crate::Reg<physectrl::PhysectrlSpec>;
///PHY Single-ended Receiver Control Register
pub mod physectrl;
/**DPUSR0R (rw) register accessor: Deep Software Standby USB Transceiver Control/Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dpusr0r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr0r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusr0r`] module*/
#[doc(alias = "DPUSR0R")]
pub type Dpusr0r = crate::Reg<dpusr0r::Dpusr0rSpec>;
///Deep Software Standby USB Transceiver Control/Pin Monitor Register
pub mod dpusr0r;
/**DPUSR1R (rw) register accessor: Deep Software Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr1r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr1r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusr1r`] module*/
#[doc(alias = "DPUSR1R")]
pub type Dpusr1r = crate::Reg<dpusr1r::Dpusr1rSpec>;
///Deep Software Standby USB Suspend/Resume Interrupt Register
pub mod dpusr1r;
