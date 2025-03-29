#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    _reserved0: [u8; 0x02],
    csmod: (),
    _reserved1: [u8; 0x02],
    cswcr1: (),
    _reserved2: [u8; 0x04],
    cswcr2: (),
    _reserved3: [u8; 0x07fa],
    cscr: (),
    _reserved4: [u8; 0x08],
    csrec: (),
    _reserved5: [u8; 0x76],
    csrecen: Csrecen,
    _reserved6: [u8; 0x087e],
    busscntfhbiu: Busscntfhbiu,
    _reserved7: [u8; 0x02],
    busscntflbiu: Busscntflbiu,
    _reserved8: [u8; 0x0a],
    busscnts0biu: Busscnts0biu,
    _reserved9: [u8; 0x0e],
    busscntpsbiu: Busscntpsbiu,
    _reserved10: [u8; 0x0e],
    busscntplbiu: Busscntplbiu,
    _reserved11: [u8; 0x02],
    busscntphbiu: Busscntphbiu,
    _reserved12: [u8; 0x0a],
    busscnteqbiu: Busscnteqbiu,
    _reserved13: [u8; 0x02],
    busscnteobiu: Busscnteobiu,
    _reserved14: [u8; 0x02],
    busscntecbiu: Busscntecbiu,
    _reserved15: [u8; 0x06b6],
    buserradd: (),
    _reserved16: [u8; 0x04],
    buserrrw: (),
    _reserved17: [u8; 0xfc],
    btzferradd: (),
    _reserved18: [u8; 0x04],
    btzferrrw: (),
    _reserved19: [u8; 0xfc],
    buserrstat: (),
    _reserved20: [u8; 0x08],
    buserrclr: (),
    _reserved21: [u8; 0x1c],
    dmacdtcerrstat: Dmacdtcerrstat,
    _reserved22: [u8; 0x07],
    dmacdtcerrclr: Dmacdtcerrclr,
}
impl RegisterBlock {
    ///0x02..0x12 - CS%s Mode Register
    #[inline(always)]
    pub const fn csmod(&self, n: usize) -> &Csmod {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x02..0x12 - CS%s Mode Register
    #[inline(always)]
    pub fn csmod_iter(&self) -> impl Iterator<Item = &Csmod> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(2).add(16 * n).cast()
            })
    }
    ///0x02 - CS0 Mode Register
    #[inline(always)]
    pub const fn cs0mod(&self) -> &Csmod {
        self.csmod(0)
    }
    ///0x12 - CS1 Mode Register
    #[inline(always)]
    pub const fn cs1mod(&self) -> &Csmod {
        self.csmod(1)
    }
    ///0x22 - CS2 Mode Register
    #[inline(always)]
    pub const fn cs2mod(&self) -> &Csmod {
        self.csmod(2)
    }
    ///0x32 - CS3 Mode Register
    #[inline(always)]
    pub const fn cs3mod(&self) -> &Csmod {
        self.csmod(3)
    }
    ///0x42 - CS4 Mode Register
    #[inline(always)]
    pub const fn cs4mod(&self) -> &Csmod {
        self.csmod(4)
    }
    ///0x52 - CS5 Mode Register
    #[inline(always)]
    pub const fn cs5mod(&self) -> &Csmod {
        self.csmod(5)
    }
    ///0x62 - CS6 Mode Register
    #[inline(always)]
    pub const fn cs6mod(&self) -> &Csmod {
        self.csmod(6)
    }
    ///0x72 - CS7 Mode Register
    #[inline(always)]
    pub const fn cs7mod(&self) -> &Csmod {
        self.csmod(7)
    }
    ///0x04..0x24 - CS%s Wait Control Register 1
    #[inline(always)]
    pub const fn cswcr1(&self, n: usize) -> &Cswcr1 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(4).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x04..0x24 - CS%s Wait Control Register 1
    #[inline(always)]
    pub fn cswcr1_iter(&self) -> impl Iterator<Item = &Cswcr1> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(4).add(16 * n).cast()
            })
    }
    ///0x04 - CS0 Wait Control Register 1
    #[inline(always)]
    pub const fn cs0wcr1(&self) -> &Cswcr1 {
        self.cswcr1(0)
    }
    ///0x14 - CS1 Wait Control Register 1
    #[inline(always)]
    pub const fn cs1wcr1(&self) -> &Cswcr1 {
        self.cswcr1(1)
    }
    ///0x24 - CS2 Wait Control Register 1
    #[inline(always)]
    pub const fn cs2wcr1(&self) -> &Cswcr1 {
        self.cswcr1(2)
    }
    ///0x34 - CS3 Wait Control Register 1
    #[inline(always)]
    pub const fn cs3wcr1(&self) -> &Cswcr1 {
        self.cswcr1(3)
    }
    ///0x44 - CS4 Wait Control Register 1
    #[inline(always)]
    pub const fn cs4wcr1(&self) -> &Cswcr1 {
        self.cswcr1(4)
    }
    ///0x54 - CS5 Wait Control Register 1
    #[inline(always)]
    pub const fn cs5wcr1(&self) -> &Cswcr1 {
        self.cswcr1(5)
    }
    ///0x64 - CS6 Wait Control Register 1
    #[inline(always)]
    pub const fn cs6wcr1(&self) -> &Cswcr1 {
        self.cswcr1(6)
    }
    ///0x74 - CS7 Wait Control Register 1
    #[inline(always)]
    pub const fn cs7wcr1(&self) -> &Cswcr1 {
        self.cswcr1(7)
    }
    ///0x08..0x28 - CS%s Wait Control Register 2
    #[inline(always)]
    pub const fn cswcr2(&self, n: usize) -> &Cswcr2 {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(8).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x08..0x28 - CS%s Wait Control Register 2
    #[inline(always)]
    pub fn cswcr2_iter(&self) -> impl Iterator<Item = &Cswcr2> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(8).add(16 * n).cast()
            })
    }
    ///0x08 - CS0 Wait Control Register 2
    #[inline(always)]
    pub const fn cs0wcr2(&self) -> &Cswcr2 {
        self.cswcr2(0)
    }
    ///0x18 - CS1 Wait Control Register 2
    #[inline(always)]
    pub const fn cs1wcr2(&self) -> &Cswcr2 {
        self.cswcr2(1)
    }
    ///0x28 - CS2 Wait Control Register 2
    #[inline(always)]
    pub const fn cs2wcr2(&self) -> &Cswcr2 {
        self.cswcr2(2)
    }
    ///0x38 - CS3 Wait Control Register 2
    #[inline(always)]
    pub const fn cs3wcr2(&self) -> &Cswcr2 {
        self.cswcr2(3)
    }
    ///0x48 - CS4 Wait Control Register 2
    #[inline(always)]
    pub const fn cs4wcr2(&self) -> &Cswcr2 {
        self.cswcr2(4)
    }
    ///0x58 - CS5 Wait Control Register 2
    #[inline(always)]
    pub const fn cs5wcr2(&self) -> &Cswcr2 {
        self.cswcr2(5)
    }
    ///0x68 - CS6 Wait Control Register 2
    #[inline(always)]
    pub const fn cs6wcr2(&self) -> &Cswcr2 {
        self.cswcr2(6)
    }
    ///0x78 - CS7 Wait Control Register 2
    #[inline(always)]
    pub const fn cs7wcr2(&self) -> &Cswcr2 {
        self.cswcr2(7)
    }
    ///0x802..0x812 - CS%s Control Register
    #[inline(always)]
    pub const fn cscr(&self, n: usize) -> &Cscr {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2050).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x802..0x812 - CS%s Control Register
    #[inline(always)]
    pub fn cscr_iter(&self) -> impl Iterator<Item = &Cscr> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(2050).add(16 * n).cast()
            })
    }
    ///0x802 - CS0 Control Register
    #[inline(always)]
    pub const fn cs0cr(&self) -> &Cscr {
        self.cscr(0)
    }
    ///0x812 - CS1 Control Register
    #[inline(always)]
    pub const fn cs1cr(&self) -> &Cscr {
        self.cscr(1)
    }
    ///0x822 - CS2 Control Register
    #[inline(always)]
    pub const fn cs2cr(&self) -> &Cscr {
        self.cscr(2)
    }
    ///0x832 - CS3 Control Register
    #[inline(always)]
    pub const fn cs3cr(&self) -> &Cscr {
        self.cscr(3)
    }
    ///0x842 - CS4 Control Register
    #[inline(always)]
    pub const fn cs4cr(&self) -> &Cscr {
        self.cscr(4)
    }
    ///0x852 - CS5 Control Register
    #[inline(always)]
    pub const fn cs5cr(&self) -> &Cscr {
        self.cscr(5)
    }
    ///0x862 - CS6 Control Register
    #[inline(always)]
    pub const fn cs6cr(&self) -> &Cscr {
        self.cscr(6)
    }
    ///0x872 - CS7 Control Register
    #[inline(always)]
    pub const fn cs7cr(&self) -> &Cscr {
        self.cscr(7)
    }
    ///0x80a..0x81a - CS%s Recovery Cycle Register
    #[inline(always)]
    pub const fn csrec(&self, n: usize) -> &Csrec {
        #[allow(clippy::no_effect)] [(); 8][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(2058).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x80a..0x81a - CS%s Recovery Cycle Register
    #[inline(always)]
    pub fn csrec_iter(&self) -> impl Iterator<Item = &Csrec> {
        (0..8)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(2058).add(16 * n).cast()
            })
    }
    ///0x80a - CS0 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs0rec(&self) -> &Csrec {
        self.csrec(0)
    }
    ///0x81a - CS1 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs1rec(&self) -> &Csrec {
        self.csrec(1)
    }
    ///0x82a - CS2 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs2rec(&self) -> &Csrec {
        self.csrec(2)
    }
    ///0x83a - CS3 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs3rec(&self) -> &Csrec {
        self.csrec(3)
    }
    ///0x84a - CS4 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs4rec(&self) -> &Csrec {
        self.csrec(4)
    }
    ///0x85a - CS5 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs5rec(&self) -> &Csrec {
        self.csrec(5)
    }
    ///0x86a - CS6 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs6rec(&self) -> &Csrec {
        self.csrec(6)
    }
    ///0x87a - CS7 Recovery Cycle Register
    #[inline(always)]
    pub const fn cs7rec(&self) -> &Csrec {
        self.csrec(7)
    }
    ///0x880 - CS Recovery Cycle Insertion Enable Register
    #[inline(always)]
    pub const fn csrecen(&self) -> &Csrecen {
        &self.csrecen
    }
    ///0x1100 - Slave Bus Control Register
    #[inline(always)]
    pub const fn busscntfhbiu(&self) -> &Busscntfhbiu {
        &self.busscntfhbiu
    }
    ///0x1104 - Slave Bus Control Register
    #[inline(always)]
    pub const fn busscntflbiu(&self) -> &Busscntflbiu {
        &self.busscntflbiu
    }
    ///0x1110 - Slave Bus Control Register
    #[inline(always)]
    pub const fn busscnts0biu(&self) -> &Busscnts0biu {
        &self.busscnts0biu
    }
    ///0x1120 - Slave Bus Control Register
    #[inline(always)]
    pub const fn busscntpsbiu(&self) -> &Busscntpsbiu {
        &self.busscntpsbiu
    }
    ///0x1130 - Slave Bus Control Register
    #[inline(always)]
    pub const fn busscntplbiu(&self) -> &Busscntplbiu {
        &self.busscntplbiu
    }
    ///0x1134 - Slave Bus Control Register
    #[inline(always)]
    pub const fn busscntphbiu(&self) -> &Busscntphbiu {
        &self.busscntphbiu
    }
    ///0x1140 - Slave Bus Control Register
    #[inline(always)]
    pub const fn busscnteqbiu(&self) -> &Busscnteqbiu {
        &self.busscnteqbiu
    }
    ///0x1144 - Slave Bus Control Register
    #[inline(always)]
    pub const fn busscnteobiu(&self) -> &Busscnteobiu {
        &self.busscnteobiu
    }
    ///0x1148 - Slave Bus Control Register
    #[inline(always)]
    pub const fn busscntecbiu(&self) -> &Busscntecbiu {
        &self.busscntecbiu
    }
    ///0x1800..0x1810 - BUS Error Address Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRADD` register.</div>
    #[inline(always)]
    pub const fn buserradd(&self, n: usize) -> &Buserradd {
        #[allow(clippy::no_effect)] [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6144).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1800..0x1810 - BUS Error Address Register
    #[inline(always)]
    pub fn buserradd_iter(&self) -> impl Iterator<Item = &Buserradd> {
        (0..4)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6144).add(16 * n).cast()
            })
    }
    ///0x1800 - BUS Error Address Register
    #[inline(always)]
    pub const fn bus1erradd(&self) -> &Buserradd {
        self.buserradd(0)
    }
    ///0x1810 - BUS Error Address Register
    #[inline(always)]
    pub const fn bus2erradd(&self) -> &Buserradd {
        self.buserradd(1)
    }
    ///0x1820 - BUS Error Address Register
    #[inline(always)]
    pub const fn bus3erradd(&self) -> &Buserradd {
        self.buserradd(2)
    }
    ///0x1830 - BUS Error Address Register
    #[inline(always)]
    pub const fn bus4erradd(&self) -> &Buserradd {
        self.buserradd(3)
    }
    ///0x1804 - BUS Error Read Write Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRRW` register.</div>
    #[inline(always)]
    pub const fn buserrrw(&self, n: usize) -> &Buserrrw {
        #[allow(clippy::no_effect)] [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6148).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1804 - BUS Error Read Write Register
    #[inline(always)]
    pub fn buserrrw_iter(&self) -> impl Iterator<Item = &Buserrrw> {
        (0..4)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6148).add(16 * n).cast()
            })
    }
    ///0x1804 - BUS Error Read Write Register
    #[inline(always)]
    pub const fn bus1errrw(&self) -> &Buserrrw {
        self.buserrrw(0)
    }
    ///0x1814 - BUS Error Read Write Register
    #[inline(always)]
    pub const fn bus2errrw(&self) -> &Buserrrw {
        self.buserrrw(1)
    }
    ///0x1824 - BUS Error Read Write Register
    #[inline(always)]
    pub const fn bus3errrw(&self) -> &Buserrrw {
        self.buserrrw(2)
    }
    ///0x1834 - BUS Error Read Write Register
    #[inline(always)]
    pub const fn bus4errrw(&self) -> &Buserrrw {
        self.buserrrw(3)
    }
    ///0x1900..0x1910 - BUS TZF Error Address Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BTZF1ERRADD` register.</div>
    #[inline(always)]
    pub const fn btzferradd(&self, n: usize) -> &Btzferradd {
        #[allow(clippy::no_effect)] [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6400).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1900..0x1910 - BUS TZF Error Address Register
    #[inline(always)]
    pub fn btzferradd_iter(&self) -> impl Iterator<Item = &Btzferradd> {
        (0..4)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6400).add(16 * n).cast()
            })
    }
    ///0x1900 - BUS TZF Error Address Register
    #[inline(always)]
    pub const fn btzf1erradd(&self) -> &Btzferradd {
        self.btzferradd(0)
    }
    ///0x1910 - BUS TZF Error Address Register
    #[inline(always)]
    pub const fn btzf2erradd(&self) -> &Btzferradd {
        self.btzferradd(1)
    }
    ///0x1920 - BUS TZF Error Address Register
    #[inline(always)]
    pub const fn btzf3erradd(&self) -> &Btzferradd {
        self.btzferradd(2)
    }
    ///0x1930 - BUS TZF Error Address Register
    #[inline(always)]
    pub const fn btzf4erradd(&self) -> &Btzferradd {
        self.btzferradd(3)
    }
    ///0x1904 - BUS TZF Error Read Write Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BTZF1ERRRW` register.</div>
    #[inline(always)]
    pub const fn btzferrrw(&self, n: usize) -> &Btzferrrw {
        #[allow(clippy::no_effect)] [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6404).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1904 - BUS TZF Error Read Write Register
    #[inline(always)]
    pub fn btzferrrw_iter(&self) -> impl Iterator<Item = &Btzferrrw> {
        (0..4)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6404).add(16 * n).cast()
            })
    }
    ///0x1904 - BUS TZF Error Read Write Register
    #[inline(always)]
    pub const fn btzf1errrw(&self) -> &Btzferrrw {
        self.btzferrrw(0)
    }
    ///0x1914 - BUS TZF Error Read Write Register
    #[inline(always)]
    pub const fn btzf2errrw(&self) -> &Btzferrrw {
        self.btzferrrw(1)
    }
    ///0x1924 - BUS TZF Error Read Write Register
    #[inline(always)]
    pub const fn btzf3errrw(&self) -> &Btzferrrw {
        self.btzferrrw(2)
    }
    ///0x1934 - BUS TZF Error Read Write Register
    #[inline(always)]
    pub const fn btzf4errrw(&self) -> &Btzferrrw {
        self.btzferrrw(3)
    }
    ///0x1a00 - BUS Error Status Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRSTAT` register.</div>
    #[inline(always)]
    pub const fn buserrstat(&self, n: usize) -> &Buserrstat {
        #[allow(clippy::no_effect)] [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6656).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1a00 - BUS Error Status Register %s
    #[inline(always)]
    pub fn buserrstat_iter(&self) -> impl Iterator<Item = &Buserrstat> {
        (0..4)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6656).add(16 * n).cast()
            })
    }
    ///0x1a00 - BUS Error Status Register 1
    #[inline(always)]
    pub const fn bus1errstat(&self) -> &Buserrstat {
        self.buserrstat(0)
    }
    ///0x1a10 - BUS Error Status Register 2
    #[inline(always)]
    pub const fn bus2errstat(&self) -> &Buserrstat {
        self.buserrstat(1)
    }
    ///0x1a20 - BUS Error Status Register 3
    #[inline(always)]
    pub const fn bus3errstat(&self) -> &Buserrstat {
        self.buserrstat(2)
    }
    ///0x1a30 - BUS Error Status Register 4
    #[inline(always)]
    pub const fn bus4errstat(&self) -> &Buserrstat {
        self.buserrstat(3)
    }
    ///0x1a08 - BUS Error Clear Register %s
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `BUS1ERRCLR` register.</div>
    #[inline(always)]
    pub const fn buserrclr(&self, n: usize) -> &Buserrclr {
        #[allow(clippy::no_effect)] [(); 4][n];
        unsafe { &*core::ptr::from_ref(self).cast::<u8>().add(6664).add(16 * n).cast() }
    }
    ///Iterator for array of:
    ///0x1a08 - BUS Error Clear Register %s
    #[inline(always)]
    pub fn buserrclr_iter(&self) -> impl Iterator<Item = &Buserrclr> {
        (0..4)
            .map(move |n| unsafe {
                &*core::ptr::from_ref(self).cast::<u8>().add(6664).add(16 * n).cast()
            })
    }
    ///0x1a08 - BUS Error Clear Register 1
    #[inline(always)]
    pub const fn bus1errclr(&self) -> &Buserrclr {
        self.buserrclr(0)
    }
    ///0x1a18 - BUS Error Clear Register 2
    #[inline(always)]
    pub const fn bus2errclr(&self) -> &Buserrclr {
        self.buserrclr(1)
    }
    ///0x1a28 - BUS Error Clear Register 3
    #[inline(always)]
    pub const fn bus3errclr(&self) -> &Buserrclr {
        self.buserrclr(2)
    }
    ///0x1a38 - BUS Error Clear Register 4
    #[inline(always)]
    pub const fn bus4errclr(&self) -> &Buserrclr {
        self.buserrclr(3)
    }
    ///0x1a24 - DMAC/DTC Error Status Register
    #[inline(always)]
    pub const fn dmacdtcerrstat(&self) -> &Dmacdtcerrstat {
        &self.dmacdtcerrstat
    }
    ///0x1a2c - DMAC/DTC Error Clear Register
    #[inline(always)]
    pub const fn dmacdtcerrclr(&self) -> &Dmacdtcerrclr {
        &self.dmacdtcerrclr
    }
}
/**CSMOD (rw) register accessor: CS%s Mode Register

You can [`read`](crate::Reg::read) this register and get [`csmod::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csmod::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@csmod`] module*/
#[doc(alias = "CSMOD")]
pub type Csmod = crate::Reg<csmod::CsmodSpec>;
///CS%s Mode Register
pub mod csmod;
/**CSWCR1 (rw) register accessor: CS%s Wait Control Register 1

You can [`read`](crate::Reg::read) this register and get [`cswcr1::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cswcr1::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cswcr1`] module*/
#[doc(alias = "CSWCR1")]
pub type Cswcr1 = crate::Reg<cswcr1::Cswcr1Spec>;
///CS%s Wait Control Register 1
pub mod cswcr1;
/**CSWCR2 (rw) register accessor: CS%s Wait Control Register 2

You can [`read`](crate::Reg::read) this register and get [`cswcr2::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cswcr2::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cswcr2`] module*/
#[doc(alias = "CSWCR2")]
pub type Cswcr2 = crate::Reg<cswcr2::Cswcr2Spec>;
///CS%s Wait Control Register 2
pub mod cswcr2;
/**CSCR (rw) register accessor: CS%s Control Register

You can [`read`](crate::Reg::read) this register and get [`cscr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cscr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@cscr`] module*/
#[doc(alias = "CSCR")]
pub type Cscr = crate::Reg<cscr::CscrSpec>;
///CS%s Control Register
pub mod cscr;
/**CSREC (rw) register accessor: CS%s Recovery Cycle Register

You can [`read`](crate::Reg::read) this register and get [`csrec::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrec::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@csrec`] module*/
#[doc(alias = "CSREC")]
pub type Csrec = crate::Reg<csrec::CsrecSpec>;
///CS%s Recovery Cycle Register
pub mod csrec;
/**CSRECEN (rw) register accessor: CS Recovery Cycle Insertion Enable Register

You can [`read`](crate::Reg::read) this register and get [`csrecen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`csrecen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@csrecen`] module*/
#[doc(alias = "CSRECEN")]
pub type Csrecen = crate::Reg<csrecen::CsrecenSpec>;
///CS Recovery Cycle Insertion Enable Register
pub mod csrecen;
/**BUSSCNTFHBIU (rw) register accessor: Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscntfhbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntfhbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntfhbiu`] module*/
#[doc(alias = "BUSSCNTFHBIU")]
pub type Busscntfhbiu = crate::Reg<busscntfhbiu::BusscntfhbiuSpec>;
///Slave Bus Control Register
pub mod busscntfhbiu;
/**BUSSCNTFLBIU (rw) register accessor: Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscntflbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntflbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntflbiu`] module*/
#[doc(alias = "BUSSCNTFLBIU")]
pub type Busscntflbiu = crate::Reg<busscntflbiu::BusscntflbiuSpec>;
///Slave Bus Control Register
pub mod busscntflbiu;
/**BUSSCNTS0BIU (rw) register accessor: Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscnts0biu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscnts0biu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscnts0biu`] module*/
#[doc(alias = "BUSSCNTS0BIU")]
pub type Busscnts0biu = crate::Reg<busscnts0biu::Busscnts0biuSpec>;
///Slave Bus Control Register
pub mod busscnts0biu;
/**BUSSCNTPSBIU (rw) register accessor: Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscntpsbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntpsbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntpsbiu`] module*/
#[doc(alias = "BUSSCNTPSBIU")]
pub type Busscntpsbiu = crate::Reg<busscntpsbiu::BusscntpsbiuSpec>;
///Slave Bus Control Register
pub mod busscntpsbiu;
/**BUSSCNTPLBIU (rw) register accessor: Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscntplbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntplbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntplbiu`] module*/
#[doc(alias = "BUSSCNTPLBIU")]
pub type Busscntplbiu = crate::Reg<busscntplbiu::BusscntplbiuSpec>;
///Slave Bus Control Register
pub mod busscntplbiu;
/**BUSSCNTPHBIU (rw) register accessor: Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscntphbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntphbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntphbiu`] module*/
#[doc(alias = "BUSSCNTPHBIU")]
pub type Busscntphbiu = crate::Reg<busscntphbiu::BusscntphbiuSpec>;
///Slave Bus Control Register
pub mod busscntphbiu;
/**BUSSCNTEQBIU (rw) register accessor: Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscnteqbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscnteqbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscnteqbiu`] module*/
#[doc(alias = "BUSSCNTEQBIU")]
pub type Busscnteqbiu = crate::Reg<busscnteqbiu::BusscnteqbiuSpec>;
///Slave Bus Control Register
pub mod busscnteqbiu;
/**BUSSCNTEOBIU (rw) register accessor: Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscnteobiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscnteobiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscnteobiu`] module*/
#[doc(alias = "BUSSCNTEOBIU")]
pub type Busscnteobiu = crate::Reg<busscnteobiu::BusscnteobiuSpec>;
///Slave Bus Control Register
pub mod busscnteobiu;
/**BUSSCNTECBIU (rw) register accessor: Slave Bus Control Register

You can [`read`](crate::Reg::read) this register and get [`busscntecbiu::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`busscntecbiu::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@busscntecbiu`] module*/
#[doc(alias = "BUSSCNTECBIU")]
pub type Busscntecbiu = crate::Reg<busscntecbiu::BusscntecbiuSpec>;
///Slave Bus Control Register
pub mod busscntecbiu;
/**BUSERRADD (r) register accessor: BUS Error Address Register

You can [`read`](crate::Reg::read) this register and get [`buserradd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buserradd`] module*/
#[doc(alias = "BUSERRADD")]
pub type Buserradd = crate::Reg<buserradd::BuserraddSpec>;
///BUS Error Address Register
pub mod buserradd;
/**BUSERRRW (rw) register accessor: BUS Error Read Write Register

You can [`read`](crate::Reg::read) this register and get [`buserrrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserrrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buserrrw`] module*/
#[doc(alias = "BUSERRRW")]
pub type Buserrrw = crate::Reg<buserrrw::BuserrrwSpec>;
///BUS Error Read Write Register
pub mod buserrrw;
/**BTZFERRADD (r) register accessor: BUS TZF Error Address Register

You can [`read`](crate::Reg::read) this register and get [`btzferradd::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@btzferradd`] module*/
#[doc(alias = "BTZFERRADD")]
pub type Btzferradd = crate::Reg<btzferradd::BtzferraddSpec>;
///BUS TZF Error Address Register
pub mod btzferradd;
/**BTZFERRRW (rw) register accessor: BUS TZF Error Read Write Register

You can [`read`](crate::Reg::read) this register and get [`btzferrrw::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`btzferrrw::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@btzferrrw`] module*/
#[doc(alias = "BTZFERRRW")]
pub type Btzferrrw = crate::Reg<btzferrrw::BtzferrrwSpec>;
///BUS TZF Error Read Write Register
pub mod btzferrrw;
/**BUSERRSTAT (r) register accessor: BUS Error Status Register %s

You can [`read`](crate::Reg::read) this register and get [`buserrstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buserrstat`] module*/
#[doc(alias = "BUSERRSTAT")]
pub type Buserrstat = crate::Reg<buserrstat::BuserrstatSpec>;
///BUS Error Status Register %s
pub mod buserrstat;
/**BUSERRCLR (rw) register accessor: BUS Error Clear Register %s

You can [`read`](crate::Reg::read) this register and get [`buserrclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`buserrclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@buserrclr`] module*/
#[doc(alias = "BUSERRCLR")]
pub type Buserrclr = crate::Reg<buserrclr::BuserrclrSpec>;
///BUS Error Clear Register %s
pub mod buserrclr;
/**DMACDTCERRSTAT (r) register accessor: DMAC/DTC Error Status Register

You can [`read`](crate::Reg::read) this register and get [`dmacdtcerrstat::R`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacdtcerrstat`] module*/
#[doc(alias = "DMACDTCERRSTAT")]
pub type Dmacdtcerrstat = crate::Reg<dmacdtcerrstat::DmacdtcerrstatSpec>;
///DMAC/DTC Error Status Register
pub mod dmacdtcerrstat;
/**DMACDTCERRCLR (rw) register accessor: DMAC/DTC Error Clear Register

You can [`read`](crate::Reg::read) this register and get [`dmacdtcerrclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dmacdtcerrclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@dmacdtcerrclr`] module*/
#[doc(alias = "DMACDTCERRCLR")]
pub type Dmacdtcerrclr = crate::Reg<dmacdtcerrclr::DmacdtcerrclrSpec>;
///DMAC/DTC Error Clear Register
pub mod dmacdtcerrclr;
