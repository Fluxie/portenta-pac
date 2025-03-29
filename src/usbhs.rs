#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    syscfg: Syscfg,
    buswait: Buswait,
    syssts0: Syssts0,
    pllsta: Pllsta,
    dvstctr0: Dvstctr0,
    _reserved5: [u8; 0x02],
    testmode: Testmode,
    _reserved6: [u8; 0x06],
    _reserved_6_cfifo: [u8; 0x04],
    _reserved_7_d0fifo: [u8; 0x04],
    _reserved8: [u8; 0x04],
    cfifosel: Cfifosel,
    cfifoctr: Cfifoctr,
    _reserved10: [u8; 0x04],
    dfifosel: (),
    _reserved11: [u8; 0x02],
    dfifoctr: (),
    _reserved12: [u8; 0x06],
    intenb0: Intenb0,
    intenb1: Intenb1,
    _reserved14: [u8; 0x02],
    brdyenb: Brdyenb,
    nrdyenb: Nrdyenb,
    bempenb: Bempenb,
    sofcfg: Sofcfg,
    physet: Physet,
    intsts0: Intsts0,
    intsts1: Intsts1,
    _reserved21: [u8; 0x02],
    brdysts: Brdysts,
    nrdysts: Nrdysts,
    bempsts: Bempsts,
    frmnum: Frmnum,
    ufrmnum: Ufrmnum,
    usbaddr: Usbaddr,
    _reserved27: [u8; 0x02],
    usbreq: Usbreq,
    usbval: Usbval,
    usbindx: Usbindx,
    usbleng: Usbleng,
    dcpcfg: Dcpcfg,
    dcpmaxp: Dcpmaxp,
    dcpctr: Dcpctr,
    _reserved34: [u8; 0x02],
    pipesel: Pipesel,
    _reserved35: [u8; 0x02],
    pipecfg: Pipecfg,
    pipebuf: Pipebuf,
    pipemaxp: Pipemaxp,
    pipeperi: Pipeperi,
    pipectr: (),
    _reserved40: [u8; 0x20],
    pipetre: (),
    _reserved41: [u8; 0x02],
    pipetrn: (),
    _reserved42: [u8; 0x3e],
    devadd: [Devadd; 10],
    devadda: Devadda,
    _reserved44: [u8; 0x1a],
    lpctrl: Lpctrl,
    lpsts: Lpsts,
    _reserved46: [u8; 0x3c],
    bcctrl: Bcctrl,
    _reserved47: [u8; 0x02],
    pl1ctrl1: Pl1ctrl1,
    pl1ctrl2: Pl1ctrl2,
    hl1ctrl1: Hl1ctrl1,
    hl1ctrl2: Hl1ctrl2,
    _reserved51: [u8; 0x14],
    dpusr0r: Dpusr0r,
    dpusr1r: Dpusr1r,
    dpusr2r: Dpusr2r,
    dpusrcr: Dpusrcr,
}
impl RegisterBlock {
    ///0x00 - System Configuration Control Register
    #[inline(always)]
    pub const fn syscfg(&self) -> &Syscfg {
        &self.syscfg
    }
    ///0x02 - CPU Bus Wait Register
    #[inline(always)]
    pub const fn buswait(&self) -> &Buswait {
        &self.buswait
    }
    ///0x04 - System Configuration Status Register
    #[inline(always)]
    pub const fn syssts0(&self) -> &Syssts0 {
        &self.syssts0
    }
    ///0x06 - PLL Status Register
    #[inline(always)]
    pub const fn pllsta(&self) -> &Pllsta {
        &self.pllsta
    }
    ///0x08 - Device State Control Register 0
    #[inline(always)]
    pub const fn dvstctr0(&self) -> &Dvstctr0 {
        &self.dvstctr0
    }
    ///0x0c - USB Test Mode Register
    #[inline(always)]
    pub const fn testmode(&self) -> &Testmode {
        &self.testmode
    }
    ///0x14 - FIFO Port Register
    #[inline(always)]
    pub const fn cfifoll(&self) -> &Cfifoll {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x14 - FIFO Port Register
    #[inline(always)]
    pub const fn cfifol(&self) -> &Cfifol {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x14 - FIFO Port Register
    #[inline(always)]
    pub const fn cfifo(&self) -> &Cfifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(20).cast() }
    }
    ///0x16 - FIFO Port Register
    #[inline(always)]
    pub const fn cfifoh(&self) -> &Cfifoh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(22).cast() }
    }
    ///0x17 - FIFO Port Register
    #[inline(always)]
    pub const fn cfifohh(&self) -> &Cfifohh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(23).cast() }
    }
    ///0x18 - FIFO Port Register
    #[inline(always)]
    pub const fn d1fifoll(&self) -> &D1fifoll {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - FIFO Port Register
    #[inline(always)]
    pub const fn d0fifoll(&self) -> &D0fifoll {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - FIFO Port Register
    #[inline(always)]
    pub const fn d1fifol(&self) -> &D1fifol {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - FIFO Port Register
    #[inline(always)]
    pub const fn d0fifol(&self) -> &D0fifol {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - FIFO Port Register
    #[inline(always)]
    pub const fn d1fifo(&self) -> &D1fifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x18 - FIFO Port Register
    #[inline(always)]
    pub const fn d0fifo(&self) -> &D0fifo {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(24).cast() }
    }
    ///0x1a - FIFO Port Register
    #[inline(always)]
    pub const fn d1fifoh(&self) -> &D1fifoh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    ///0x1a - FIFO Port Register
    #[inline(always)]
    pub const fn d0fifoh(&self) -> &D0fifoh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(26).cast() }
    }
    ///0x1b - FIFO Port Register
    #[inline(always)]
    pub const fn d1fifohh(&self) -> &D1fifohh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(27).cast() }
    }
    ///0x1b - FIFO Port Register
    #[inline(always)]
    pub const fn d0fifohh(&self) -> &D0fifohh {
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(27).cast() }
    }
    ///0x20 - CFIFO Port Selection Register
    #[inline(always)]
    pub const fn cfifosel(&self) -> &Cfifosel {
        &self.cfifosel
    }
    ///0x22 - FIFO Port Control Register
    #[inline(always)]
    pub const fn cfifoctr(&self) -> &Cfifoctr {
        &self.cfifoctr
    }
    ///0x28 - D%sFIFO Port Selection Register
    #[inline(always)]
    pub const fn dfifosel(&self, n: usize) -> &Dfifosel {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(40).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x28 - D%sFIFO Port Selection Register
    #[inline(always)]
    pub fn dfifosel_iter(&self) -> impl Iterator<Item = &Dfifosel> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(40).add(4 * n).cast()
            })
    }
    ///0x28 - D0FIFO Port Selection Register
    #[inline(always)]
    pub const fn d0fifosel(&self) -> &Dfifosel {
        self.dfifosel(0)
    }
    ///0x2c - D1FIFO Port Selection Register
    #[inline(always)]
    pub const fn d1fifosel(&self) -> &Dfifosel {
        self.dfifosel(1)
    }
    ///0x2a - FIFO Port Control Register
    #[inline(always)]
    pub const fn dfifoctr(&self, n: usize) -> &Dfifoctr {
        #[allow(clippy::no_effect)] [(); 2][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(42).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x2a - FIFO Port Control Register
    #[inline(always)]
    pub fn dfifoctr_iter(&self) -> impl Iterator<Item = &Dfifoctr> {
        (0..2)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(42).add(4 * n).cast()
            })
    }
    ///0x2a - FIFO Port Control Register
    #[inline(always)]
    pub const fn d0fifoctr(&self) -> &Dfifoctr {
        self.dfifoctr(0)
    }
    ///0x2e - FIFO Port Control Register
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
    ///0x3e - PHY Setting Register
    #[inline(always)]
    pub const fn physet(&self) -> &Physet {
        &self.physet
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
    ///0x4e - µFrame Number Register
    #[inline(always)]
    pub const fn ufrmnum(&self) -> &Ufrmnum {
        &self.ufrmnum
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
    ///0x6a - Pipe Buffer Register
    #[inline(always)]
    pub const fn pipebuf(&self) -> &Pipebuf {
        &self.pipebuf
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
    ///0x70..0x82 - Pipe %s Control Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1CTR` register.</div>
    #[inline(always)]
    pub const fn pipectr(&self, n: usize) -> &Pipectr {
        #[allow(clippy::no_effect)] [(); 9][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(112).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x70..0x82 - Pipe %s Control Register
    #[inline(always)]
    pub fn pipectr_iter(&self) -> impl Iterator<Item = &Pipectr> {
        (0..9)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(112).add(4 * n).cast()
            })
    }
    ///0x70 - Pipe 1 Control Register
    #[inline(always)]
    pub const fn pipe1ctr(&self) -> &Pipectr {
        self.pipectr(0)
    }
    ///0x74 - Pipe 2 Control Register
    #[inline(always)]
    pub const fn pipe2ctr(&self) -> &Pipectr {
        self.pipectr(1)
    }
    ///0x78 - Pipe 3 Control Register
    #[inline(always)]
    pub const fn pipe3ctr(&self) -> &Pipectr {
        self.pipectr(2)
    }
    ///0x7c - Pipe 4 Control Register
    #[inline(always)]
    pub const fn pipe4ctr(&self) -> &Pipectr {
        self.pipectr(3)
    }
    ///0x80 - Pipe 5 Control Register
    #[inline(always)]
    pub const fn pipe5ctr(&self) -> &Pipectr {
        self.pipectr(4)
    }
    ///0x84 - Pipe 6 Control Register
    #[inline(always)]
    pub const fn pipe6ctr(&self) -> &Pipectr {
        self.pipectr(5)
    }
    ///0x88 - Pipe 7 Control Register
    #[inline(always)]
    pub const fn pipe7ctr(&self) -> &Pipectr {
        self.pipectr(6)
    }
    ///0x8c - Pipe 8 Control Register
    #[inline(always)]
    pub const fn pipe8ctr(&self) -> &Pipectr {
        self.pipectr(7)
    }
    ///0x90 - Pipe 9 Control Register
    #[inline(always)]
    pub const fn pipe9ctr(&self) -> &Pipectr {
        self.pipectr(8)
    }
    ///0x90..0x9a - Pipe %s Transaction Counter Enable Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1TRE` register.</div>
    #[inline(always)]
    pub const fn pipetre(&self, n: usize) -> &Pipetre {
        #[allow(clippy::no_effect)] [(); 5][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(144).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x90..0x9a - Pipe %s Transaction Counter Enable Register
    #[inline(always)]
    pub fn pipetre_iter(&self) -> impl Iterator<Item = &Pipetre> {
        (0..5)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(144).add(4 * n).cast()
            })
    }
    ///0x90 - Pipe 1 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe1tre(&self) -> &Pipetre {
        self.pipetre(0)
    }
    ///0x94 - Pipe 2 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe2tre(&self) -> &Pipetre {
        self.pipetre(1)
    }
    ///0x98 - Pipe 3 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe3tre(&self) -> &Pipetre {
        self.pipetre(2)
    }
    ///0x9c - Pipe 4 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe4tre(&self) -> &Pipetre {
        self.pipetre(3)
    }
    ///0xa0 - Pipe 5 Transaction Counter Enable Register
    #[inline(always)]
    pub const fn pipe5tre(&self) -> &Pipetre {
        self.pipetre(4)
    }
    ///0x92..0x9c - Pipe %s Transaction Counter Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `PIPE1TRN` register.</div>
    #[inline(always)]
    pub const fn pipetrn(&self, n: usize) -> &Pipetrn {
        #[allow(clippy::no_effect)] [(); 5][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(146).add(4 * n).cast() }
    }
    ///Iterator for array of:
    ///0x92..0x9c - Pipe %s Transaction Counter Register
    #[inline(always)]
    pub fn pipetrn_iter(&self) -> impl Iterator<Item = &Pipetrn> {
        (0..5)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(146).add(4 * n).cast()
            })
    }
    ///0x92 - Pipe 1 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe1trn(&self) -> &Pipetrn {
        self.pipetrn(0)
    }
    ///0x96 - Pipe 2 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe2trn(&self) -> &Pipetrn {
        self.pipetrn(1)
    }
    ///0x9a - Pipe 3 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe3trn(&self) -> &Pipetrn {
        self.pipetrn(2)
    }
    ///0x9e - Pipe 4 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe4trn(&self) -> &Pipetrn {
        self.pipetrn(3)
    }
    ///0xa2 - Pipe 5 Transaction Counter Register
    #[inline(always)]
    pub const fn pipe5trn(&self) -> &Pipetrn {
        self.pipetrn(4)
    }
    ///0xd0..0xe4 - Device Address %s Configuration Register
    #[inline(always)]
    pub const fn devadd(&self, n: usize) -> &Devadd {
        &self.devadd[n]
    }
    ///Iterator for array of:
    ///0xd0..0xe4 - Device Address %s Configuration Register
    #[inline(always)]
    pub fn devadd_iter(&self) -> impl Iterator<Item = &Devadd> {
        self.devadd.iter()
    }
    ///0xe4 - Device Address A Configuration Register
    #[inline(always)]
    pub const fn devadda(&self) -> &Devadda {
        &self.devadda
    }
    ///0x100 - Low Power Control Register
    #[inline(always)]
    pub const fn lpctrl(&self) -> &Lpctrl {
        &self.lpctrl
    }
    ///0x102 - Low Power Status Register
    #[inline(always)]
    pub const fn lpsts(&self) -> &Lpsts {
        &self.lpsts
    }
    ///0x140 - Battery Charging Control Register
    #[inline(always)]
    pub const fn bcctrl(&self) -> &Bcctrl {
        &self.bcctrl
    }
    ///0x144 - Function L1 Control Register 1
    #[inline(always)]
    pub const fn pl1ctrl1(&self) -> &Pl1ctrl1 {
        &self.pl1ctrl1
    }
    ///0x146 - Function L1 Control Register 2
    #[inline(always)]
    pub const fn pl1ctrl2(&self) -> &Pl1ctrl2 {
        &self.pl1ctrl2
    }
    ///0x148 - Host L1 Control Register 1
    #[inline(always)]
    pub const fn hl1ctrl1(&self) -> &Hl1ctrl1 {
        &self.hl1ctrl1
    }
    ///0x14a - Host L1 Control Register 2
    #[inline(always)]
    pub const fn hl1ctrl2(&self) -> &Hl1ctrl2 {
        &self.hl1ctrl2
    }
    ///0x160 - Deep Software Standby USB Transceiver Control/Pin Monitor Register
    #[inline(always)]
    pub const fn dpusr0r(&self) -> &Dpusr0r {
        &self.dpusr0r
    }
    ///0x164 - Deep Software Standby USB Suspend/Resume Interrupt Register
    #[inline(always)]
    pub const fn dpusr1r(&self) -> &Dpusr1r {
        &self.dpusr1r
    }
    ///0x168 - Deep Software Standby USB Suspend/Resume Interrupt Register
    #[inline(always)]
    pub const fn dpusr2r(&self) -> &Dpusr2r {
        &self.dpusr2r
    }
    ///0x16a - Deep Software Standby USB Suspend/Resume Command Register
    #[inline(always)]
    pub const fn dpusrcr(&self) -> &Dpusrcr {
        &self.dpusrcr
    }
}
/**SYSCFG (rw) register accessor: System Configuration Control Register

You can [`read`](crate::Reg::read) this register and get [`syscfg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`syscfg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syscfg`] module*/
#[doc(alias = "SYSCFG")]
pub type Syscfg = crate::Reg<syscfg::SyscfgSpec>;
///System Configuration Control Register
pub mod syscfg;
/**BUSWAIT (rw) register accessor: CPU Bus Wait Register

You can [`read`](crate::Reg::read) this register and get [`buswait::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buswait::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buswait`] module*/
#[doc(alias = "BUSWAIT")]
pub type Buswait = crate::Reg<buswait::BuswaitSpec>;
///CPU Bus Wait Register
pub mod buswait;
/**SYSSTS0 (r) register accessor: System Configuration Status Register

You can [`read`](crate::Reg::read) this register and get [`syssts0::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@syssts0`] module*/
#[doc(alias = "SYSSTS0")]
pub type Syssts0 = crate::Reg<syssts0::Syssts0Spec>;
///System Configuration Status Register
pub mod syssts0;
/**PLLSTA (r) register accessor: PLL Status Register

You can [`read`](crate::Reg::read) this register and get [`pllsta::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pllsta`] module*/
#[doc(alias = "PLLSTA")]
pub type Pllsta = crate::Reg<pllsta::PllstaSpec>;
///PLL Status Register
pub mod pllsta;
/**DVSTCTR0 (rw) register accessor: Device State Control Register 0

You can [`read`](crate::Reg::read) this register and get [`dvstctr0::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dvstctr0::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dvstctr0`] module*/
#[doc(alias = "DVSTCTR0")]
pub type Dvstctr0 = crate::Reg<dvstctr0::Dvstctr0Spec>;
///Device State Control Register 0
pub mod dvstctr0;
/**TESTMODE (rw) register accessor: USB Test Mode Register

You can [`read`](crate::Reg::read) this register and get [`testmode::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`testmode::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@testmode`] module*/
#[doc(alias = "TESTMODE")]
pub type Testmode = crate::Reg<testmode::TestmodeSpec>;
///USB Test Mode Register
pub mod testmode;
/**CFIFO (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifo`] module*/
#[doc(alias = "CFIFO")]
pub type Cfifo = crate::Reg<cfifo::CfifoSpec>;
///FIFO Port Register
pub mod cfifo;
/**CFIFOL (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifol`] module*/
#[doc(alias = "CFIFOL")]
pub type Cfifol = crate::Reg<cfifol::CfifolSpec>;
///FIFO Port Register
pub mod cfifol;
/**CFIFOLL (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifoll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifoll`] module*/
#[doc(alias = "CFIFOLL")]
pub type Cfifoll = crate::Reg<cfifoll::CfifollSpec>;
///FIFO Port Register
pub mod cfifoll;
/**CFIFOH (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifoh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifoh`] module*/
#[doc(alias = "CFIFOH")]
pub type Cfifoh = crate::Reg<cfifoh::CfifohSpec>;
///FIFO Port Register
pub mod cfifoh;
/**CFIFOHH (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`cfifohh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifohh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifohh`] module*/
#[doc(alias = "CFIFOHH")]
pub type Cfifohh = crate::Reg<cfifohh::CfifohhSpec>;
///FIFO Port Register
pub mod cfifohh;
/**D0FIFO (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d0fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifo`] module*/
#[doc(alias = "D0FIFO")]
pub type D0fifo = crate::Reg<d0fifo::D0fifoSpec>;
///FIFO Port Register
pub mod d0fifo;
/**D1FIFO (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifo::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifo::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifo`] module*/
#[doc(alias = "D1FIFO")]
pub type D1fifo = crate::Reg<d1fifo::D1fifoSpec>;
///FIFO Port Register
pub mod d1fifo;
/**D0FIFOL (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d0fifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifol`] module*/
#[doc(alias = "D0FIFOL")]
pub type D0fifol = crate::Reg<d0fifol::D0fifolSpec>;
///FIFO Port Register
pub mod d0fifol;
/**D1FIFOL (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifol::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifol::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifol`] module*/
#[doc(alias = "D1FIFOL")]
pub type D1fifol = crate::Reg<d1fifol::D1fifolSpec>;
///FIFO Port Register
pub mod d1fifol;
/**D0FIFOLL (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d0fifoll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifoll`] module*/
#[doc(alias = "D0FIFOLL")]
pub type D0fifoll = crate::Reg<d0fifoll::D0fifollSpec>;
///FIFO Port Register
pub mod d0fifoll;
/**D1FIFOLL (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifoll::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoll::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifoll`] module*/
#[doc(alias = "D1FIFOLL")]
pub type D1fifoll = crate::Reg<d1fifoll::D1fifollSpec>;
///FIFO Port Register
pub mod d1fifoll;
/**D0FIFOH (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d0fifoh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifoh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifoh`] module*/
#[doc(alias = "D0FIFOH")]
pub type D0fifoh = crate::Reg<d0fifoh::D0fifohSpec>;
///FIFO Port Register
pub mod d0fifoh;
/**D1FIFOH (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifoh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifoh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifoh`] module*/
#[doc(alias = "D1FIFOH")]
pub type D1fifoh = crate::Reg<d1fifoh::D1fifohSpec>;
///FIFO Port Register
pub mod d1fifoh;
/**D0FIFOHH (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d0fifohh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d0fifohh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d0fifohh`] module*/
#[doc(alias = "D0FIFOHH")]
pub type D0fifohh = crate::Reg<d0fifohh::D0fifohhSpec>;
///FIFO Port Register
pub mod d0fifohh;
/**D1FIFOHH (rw) register accessor: FIFO Port Register

You can [`read`](crate::Reg::read) this register and get [`d1fifohh::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`d1fifohh::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@d1fifohh`] module*/
#[doc(alias = "D1FIFOHH")]
pub type D1fifohh = crate::Reg<d1fifohh::D1fifohhSpec>;
///FIFO Port Register
pub mod d1fifohh;
/**CFIFOSEL (rw) register accessor: CFIFO Port Selection Register

You can [`read`](crate::Reg::read) this register and get [`cfifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifosel`] module*/
#[doc(alias = "CFIFOSEL")]
pub type Cfifosel = crate::Reg<cfifosel::CfifoselSpec>;
///CFIFO Port Selection Register
pub mod cfifosel;
/**CFIFOCTR (rw) register accessor: FIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`cfifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cfifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cfifoctr`] module*/
#[doc(alias = "CFIFOCTR")]
pub type Cfifoctr = crate::Reg<cfifoctr::CfifoctrSpec>;
///FIFO Port Control Register
pub mod cfifoctr;
/**DFIFOSEL (rw) register accessor: D%sFIFO Port Selection Register

You can [`read`](crate::Reg::read) this register and get [`dfifosel::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfifosel::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfifosel`] module*/
#[doc(alias = "DFIFOSEL")]
pub type Dfifosel = crate::Reg<dfifosel::DfifoselSpec>;
///D%sFIFO Port Selection Register
pub mod dfifosel;
/**DFIFOCTR (rw) register accessor: FIFO Port Control Register

You can [`read`](crate::Reg::read) this register and get [`dfifoctr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dfifoctr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dfifoctr`] module*/
#[doc(alias = "DFIFOCTR")]
pub type Dfifoctr = crate::Reg<dfifoctr::DfifoctrSpec>;
///FIFO Port Control Register
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
/**PHYSET (rw) register accessor: PHY Setting Register

You can [`read`](crate::Reg::read) this register and get [`physet::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`physet::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@physet`] module*/
#[doc(alias = "PHYSET")]
pub type Physet = crate::Reg<physet::PhysetSpec>;
///PHY Setting Register
pub mod physet;
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
/**UFRMNUM (rw) register accessor: µFrame Number Register

You can [`read`](crate::Reg::read) this register and get [`ufrmnum::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ufrmnum::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@ufrmnum`] module*/
#[doc(alias = "UFRMNUM")]
pub type Ufrmnum = crate::Reg<ufrmnum::UfrmnumSpec>;
///µFrame Number Register
pub mod ufrmnum;
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
/**PIPEBUF (rw) register accessor: Pipe Buffer Register

You can [`read`](crate::Reg::read) this register and get [`pipebuf::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipebuf::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipebuf`] module*/
#[doc(alias = "PIPEBUF")]
pub type Pipebuf = crate::Reg<pipebuf::PipebufSpec>;
///Pipe Buffer Register
pub mod pipebuf;
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
/**PIPECTR (rw) register accessor: Pipe %s Control Register

You can [`read`](crate::Reg::read) this register and get [`pipectr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipectr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipectr`] module*/
#[doc(alias = "PIPECTR")]
pub type Pipectr = crate::Reg<pipectr::PipectrSpec>;
///Pipe %s Control Register
pub mod pipectr;
/**PIPETRE (rw) register accessor: Pipe %s Transaction Counter Enable Register

You can [`read`](crate::Reg::read) this register and get [`pipetre::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetre::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipetre`] module*/
#[doc(alias = "PIPETRE")]
pub type Pipetre = crate::Reg<pipetre::PipetreSpec>;
///Pipe %s Transaction Counter Enable Register
pub mod pipetre;
/**PIPETRN (rw) register accessor: Pipe %s Transaction Counter Register

You can [`read`](crate::Reg::read) this register and get [`pipetrn::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pipetrn::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pipetrn`] module*/
#[doc(alias = "PIPETRN")]
pub type Pipetrn = crate::Reg<pipetrn::PipetrnSpec>;
///Pipe %s Transaction Counter Register
pub mod pipetrn;
/**DEVADD (rw) register accessor: Device Address %s Configuration Register

You can [`read`](crate::Reg::read) this register and get [`devadd::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadd::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@devadd`] module*/
#[doc(alias = "DEVADD")]
pub type Devadd = crate::Reg<devadd::DevaddSpec>;
///Device Address %s Configuration Register
pub mod devadd;
/**DEVADDA (rw) register accessor: Device Address A Configuration Register

You can [`read`](crate::Reg::read) this register and get [`devadda::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`devadda::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@devadda`] module*/
#[doc(alias = "DEVADDA")]
pub type Devadda = crate::Reg<devadda::DevaddaSpec>;
///Device Address A Configuration Register
pub mod devadda;
/**LPCTRL (rw) register accessor: Low Power Control Register

You can [`read`](crate::Reg::read) this register and get [`lpctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpctrl`] module*/
#[doc(alias = "LPCTRL")]
pub type Lpctrl = crate::Reg<lpctrl::LpctrlSpec>;
///Low Power Control Register
pub mod lpctrl;
/**LPSTS (rw) register accessor: Low Power Status Register

You can [`read`](crate::Reg::read) this register and get [`lpsts::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpsts::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@lpsts`] module*/
#[doc(alias = "LPSTS")]
pub type Lpsts = crate::Reg<lpsts::LpstsSpec>;
///Low Power Status Register
pub mod lpsts;
/**BCCTRL (rw) register accessor: Battery Charging Control Register

You can [`read`](crate::Reg::read) this register and get [`bcctrl::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`bcctrl::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@bcctrl`] module*/
#[doc(alias = "BCCTRL")]
pub type Bcctrl = crate::Reg<bcctrl::BcctrlSpec>;
///Battery Charging Control Register
pub mod bcctrl;
/**PL1CTRL1 (rw) register accessor: Function L1 Control Register 1

You can [`read`](crate::Reg::read) this register and get [`pl1ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pl1ctrl1`] module*/
#[doc(alias = "PL1CTRL1")]
pub type Pl1ctrl1 = crate::Reg<pl1ctrl1::Pl1ctrl1Spec>;
///Function L1 Control Register 1
pub mod pl1ctrl1;
/**PL1CTRL2 (rw) register accessor: Function L1 Control Register 2

You can [`read`](crate::Reg::read) this register and get [`pl1ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pl1ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pl1ctrl2`] module*/
#[doc(alias = "PL1CTRL2")]
pub type Pl1ctrl2 = crate::Reg<pl1ctrl2::Pl1ctrl2Spec>;
///Function L1 Control Register 2
pub mod pl1ctrl2;
/**HL1CTRL1 (rw) register accessor: Host L1 Control Register 1

You can [`read`](crate::Reg::read) this register and get [`hl1ctrl1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl1ctrl1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hl1ctrl1`] module*/
#[doc(alias = "HL1CTRL1")]
pub type Hl1ctrl1 = crate::Reg<hl1ctrl1::Hl1ctrl1Spec>;
///Host L1 Control Register 1
pub mod hl1ctrl1;
/**HL1CTRL2 (rw) register accessor: Host L1 Control Register 2

You can [`read`](crate::Reg::read) this register and get [`hl1ctrl2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hl1ctrl2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@hl1ctrl2`] module*/
#[doc(alias = "HL1CTRL2")]
pub type Hl1ctrl2 = crate::Reg<hl1ctrl2::Hl1ctrl2Spec>;
///Host L1 Control Register 2
pub mod hl1ctrl2;
/**DPUSR0R (r) register accessor: Deep Software Standby USB Transceiver Control/Pin Monitor Register

You can [`read`](crate::Reg::read) this register and get [`dpusr0r::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

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
/**DPUSR2R (rw) register accessor: Deep Software Standby USB Suspend/Resume Interrupt Register

You can [`read`](crate::Reg::read) this register and get [`dpusr2r::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusr2r::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusr2r`] module*/
#[doc(alias = "DPUSR2R")]
pub type Dpusr2r = crate::Reg<dpusr2r::Dpusr2rSpec>;
///Deep Software Standby USB Suspend/Resume Interrupt Register
pub mod dpusr2r;
/**DPUSRCR (rw) register accessor: Deep Software Standby USB Suspend/Resume Command Register

You can [`read`](crate::Reg::read) this register and get [`dpusrcr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dpusrcr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dpusrcr`] module*/
#[doc(alias = "DPUSRCR")]
pub type Dpusrcr = crate::Reg<dpusrcr::DpusrcrSpec>;
///Deep Software Standby USB Suspend/Resume Command Register
pub mod dpusrcr;
