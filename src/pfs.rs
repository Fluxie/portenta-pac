#[repr(C)]
#[derive(Debug)]
///Register block
pub struct RegisterBlock {
    p00pfs: [P00pfs; 8],
    p008pfs: P008pfs,
    p009pfs: P009pfs,
    p010pfs: P010pfs,
    _reserved4: [u8; 0x0c],
    p0pfs: [P0pfs; 2],
    p10pfs: [P10pfs; 10],
    p1pfs: [P1pfs; 6],
    p200pfs: P200pfs,
    p201pfs: P201pfs,
    p20pfs: [P20pfs; 8],
    p210pfs: P2pfs,
    p211pfs: P2pfs,
    p212pfs: P2pfs,
    p213pfs: P2pfs,
    p214pfs: P2pfs,
    _reserved15: [u8; 0x04],
    p300pfs: P300pfs,
    p30pfs: [P30pfs; 9],
    p3pfs: [P3pfs; 6],
    p40pfs: [P40pfs; 10],
    p4pfs: [P4pfs; 6],
    p50pfs: [P50pfs; 9],
    _reserved21: [u8; 0x08],
    p5pfs: [P5pfs; 3],
    _reserved22: [u8; 0x08],
    p60pfs: [P60pfs; 10],
    p6pfs: [P6pfs; 6],
    p70pfs: [P70pfs; 8],
    p708pfs: P70pfs,
    p709pfs: P70pfs,
    p7pfs: [P7pfs; 4],
    _reserved28: [u8; 0x08],
    p80pfs: [P80pfs; 7],
    _reserved29: [u8; 0x24],
    p90pfs: [P90pfs; 2],
    _reserved30: [u8; 0x0c],
    p905pfs: P90pfs,
    p906pfs: P90pfs,
    p907pfs: P90pfs,
    p908pfs: P90pfs,
    _reserved34: [u8; 0x1c],
    pa0pfs: [Pa0pfs; 2],
    _reserved35: [u8; 0x18],
    pa08pfs: Pa0pfs,
    pa09pfs: Pa0pfs,
    pa10pfs: Pa10pfs,
    _reserved38: [u8; 0x14],
    pb0pfs: [Pb0pfs; 2],
    _reserved39: [u8; 0x0238],
    pfenet: Pfenet,
    _reserved40: [u8; 0x02],
    pwpr: Pwpr,
    _reserved41: [u8; 0x01],
    pwprs: Pwprs,
    _reserved42: [u8; 0x0a],
    psar: [Psar; 10],
    pasar: Psar,
    pbsar: Psar,
}
impl RegisterBlock {
    ///0x00..0x20 - Port 00%s Pin Function Select Register
    #[inline(always)]
    pub const fn p00pfs(&self, n: usize) -> &P00pfs {
        &self.p00pfs[n]
    }
    ///Iterator for array of:
    ///0x00..0x20 - Port 00%s Pin Function Select Register
    #[inline(always)]
    pub fn p00pfs_iter(&self) -> impl Iterator<Item = &P00pfs> {
        self.p00pfs.iter()
    }
    ///0x00 - Port 000 Pin Function Select Register
    #[inline(always)]
    pub const fn p000pfs(&self) -> &P00pfs {
        self.p00pfs(0)
    }
    ///0x04 - Port 001 Pin Function Select Register
    #[inline(always)]
    pub const fn p001pfs(&self) -> &P00pfs {
        self.p00pfs(1)
    }
    ///0x08 - Port 002 Pin Function Select Register
    #[inline(always)]
    pub const fn p002pfs(&self) -> &P00pfs {
        self.p00pfs(2)
    }
    ///0x0c - Port 003 Pin Function Select Register
    #[inline(always)]
    pub const fn p003pfs(&self) -> &P00pfs {
        self.p00pfs(3)
    }
    ///0x10 - Port 004 Pin Function Select Register
    #[inline(always)]
    pub const fn p004pfs(&self) -> &P00pfs {
        self.p00pfs(4)
    }
    ///0x14 - Port 005 Pin Function Select Register
    #[inline(always)]
    pub const fn p005pfs(&self) -> &P00pfs {
        self.p00pfs(5)
    }
    ///0x18 - Port 006 Pin Function Select Register
    #[inline(always)]
    pub const fn p006pfs(&self) -> &P00pfs {
        self.p00pfs(6)
    }
    ///0x1c - Port 007 Pin Function Select Register
    #[inline(always)]
    pub const fn p007pfs(&self) -> &P00pfs {
        self.p00pfs(7)
    }
    ///0x20 - Port 008 Pin Function Select Register
    #[inline(always)]
    pub const fn p008pfs(&self) -> &P008pfs {
        &self.p008pfs
    }
    ///0x24 - Port 009 Pin Function Select Register
    #[inline(always)]
    pub const fn p009pfs(&self) -> &P009pfs {
        &self.p009pfs
    }
    ///0x28 - Port 010 Pin Function Select Register
    #[inline(always)]
    pub const fn p010pfs(&self) -> &P010pfs {
        &self.p010pfs
    }
    ///0x38..0x40 - Port 0%s Pin Function Select Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `P014PFS` register.</div>
    #[inline(always)]
    pub const fn p0pfs(&self, n: usize) -> &P0pfs {
        &self.p0pfs[n]
    }
    ///Iterator for array of:
    ///0x38..0x40 - Port 0%s Pin Function Select Register
    #[inline(always)]
    pub fn p0pfs_iter(&self) -> impl Iterator<Item = &P0pfs> {
        self.p0pfs.iter()
    }
    ///0x38 - Port 014 Pin Function Select Register
    #[inline(always)]
    pub const fn p014pfs(&self) -> &P0pfs {
        self.p0pfs(0)
    }
    ///0x3c - Port 015 Pin Function Select Register
    #[inline(always)]
    pub const fn p015pfs(&self) -> &P0pfs {
        self.p0pfs(1)
    }
    ///0x40..0x68 - Port 10%s Pin Function Select Register
    #[inline(always)]
    pub const fn p10pfs(&self, n: usize) -> &P10pfs {
        &self.p10pfs[n]
    }
    ///Iterator for array of:
    ///0x40..0x68 - Port 10%s Pin Function Select Register
    #[inline(always)]
    pub fn p10pfs_iter(&self) -> impl Iterator<Item = &P10pfs> {
        self.p10pfs.iter()
    }
    ///0x40 - Port 100 Pin Function Select Register
    #[inline(always)]
    pub const fn p100pfs(&self) -> &P10pfs {
        self.p10pfs(0)
    }
    ///0x44 - Port 101 Pin Function Select Register
    #[inline(always)]
    pub const fn p101pfs(&self) -> &P10pfs {
        self.p10pfs(1)
    }
    ///0x48 - Port 102 Pin Function Select Register
    #[inline(always)]
    pub const fn p102pfs(&self) -> &P10pfs {
        self.p10pfs(2)
    }
    ///0x4c - Port 103 Pin Function Select Register
    #[inline(always)]
    pub const fn p103pfs(&self) -> &P10pfs {
        self.p10pfs(3)
    }
    ///0x50 - Port 104 Pin Function Select Register
    #[inline(always)]
    pub const fn p104pfs(&self) -> &P10pfs {
        self.p10pfs(4)
    }
    ///0x54 - Port 105 Pin Function Select Register
    #[inline(always)]
    pub const fn p105pfs(&self) -> &P10pfs {
        self.p10pfs(5)
    }
    ///0x58 - Port 106 Pin Function Select Register
    #[inline(always)]
    pub const fn p106pfs(&self) -> &P10pfs {
        self.p10pfs(6)
    }
    ///0x5c - Port 107 Pin Function Select Register
    #[inline(always)]
    pub const fn p107pfs(&self) -> &P10pfs {
        self.p10pfs(7)
    }
    ///0x60 - Port 108 Pin Function Select Register
    #[inline(always)]
    pub const fn p108pfs(&self) -> &P10pfs {
        self.p10pfs(8)
    }
    ///0x64 - Port 109 Pin Function Select Register
    #[inline(always)]
    pub const fn p109pfs(&self) -> &P10pfs {
        self.p10pfs(9)
    }
    ///0x68..0x80 - Port 1%s Pin Function Select Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `P110PFS` register.</div>
    #[inline(always)]
    pub const fn p1pfs(&self, n: usize) -> &P1pfs {
        &self.p1pfs[n]
    }
    ///Iterator for array of:
    ///0x68..0x80 - Port 1%s Pin Function Select Register
    #[inline(always)]
    pub fn p1pfs_iter(&self) -> impl Iterator<Item = &P1pfs> {
        self.p1pfs.iter()
    }
    ///0x68 - Port 110 Pin Function Select Register
    #[inline(always)]
    pub const fn p110pfs(&self) -> &P1pfs {
        self.p1pfs(0)
    }
    ///0x6c - Port 111 Pin Function Select Register
    #[inline(always)]
    pub const fn p111pfs(&self) -> &P1pfs {
        self.p1pfs(1)
    }
    ///0x70 - Port 112 Pin Function Select Register
    #[inline(always)]
    pub const fn p112pfs(&self) -> &P1pfs {
        self.p1pfs(2)
    }
    ///0x74 - Port 113 Pin Function Select Register
    #[inline(always)]
    pub const fn p113pfs(&self) -> &P1pfs {
        self.p1pfs(3)
    }
    ///0x78 - Port 114 Pin Function Select Register
    #[inline(always)]
    pub const fn p114pfs(&self) -> &P1pfs {
        self.p1pfs(4)
    }
    ///0x7c - Port 115 Pin Function Select Register
    #[inline(always)]
    pub const fn p115pfs(&self) -> &P1pfs {
        self.p1pfs(5)
    }
    ///0x80 - Port 200 Pin Function Select Register
    #[inline(always)]
    pub const fn p200pfs(&self) -> &P200pfs {
        &self.p200pfs
    }
    ///0x84 - Port 201 Pin Function Select Register
    #[inline(always)]
    pub const fn p201pfs(&self) -> &P201pfs {
        &self.p201pfs
    }
    ///0x88..0xa8 - Port 20%s Pin Function Select Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `P202PFS` register.</div>
    #[inline(always)]
    pub const fn p20pfs(&self, n: usize) -> &P20pfs {
        &self.p20pfs[n]
    }
    ///Iterator for array of:
    ///0x88..0xa8 - Port 20%s Pin Function Select Register
    #[inline(always)]
    pub fn p20pfs_iter(&self) -> impl Iterator<Item = &P20pfs> {
        self.p20pfs.iter()
    }
    ///0x88 - Port 202 Pin Function Select Register
    #[inline(always)]
    pub const fn p202pfs(&self) -> &P20pfs {
        self.p20pfs(0)
    }
    ///0x8c - Port 203 Pin Function Select Register
    #[inline(always)]
    pub const fn p203pfs(&self) -> &P20pfs {
        self.p20pfs(1)
    }
    ///0x90 - Port 204 Pin Function Select Register
    #[inline(always)]
    pub const fn p204pfs(&self) -> &P20pfs {
        self.p20pfs(2)
    }
    ///0x94 - Port 205 Pin Function Select Register
    #[inline(always)]
    pub const fn p205pfs(&self) -> &P20pfs {
        self.p20pfs(3)
    }
    ///0x98 - Port 206 Pin Function Select Register
    #[inline(always)]
    pub const fn p206pfs(&self) -> &P20pfs {
        self.p20pfs(4)
    }
    ///0x9c - Port 207 Pin Function Select Register
    #[inline(always)]
    pub const fn p207pfs(&self) -> &P20pfs {
        self.p20pfs(5)
    }
    ///0xa0 - Port 208 Pin Function Select Register
    #[inline(always)]
    pub const fn p208pfs(&self) -> &P20pfs {
        self.p20pfs(6)
    }
    ///0xa4 - Port 209 Pin Function Select Register
    #[inline(always)]
    pub const fn p209pfs(&self) -> &P20pfs {
        self.p20pfs(7)
    }
    ///0xa8 - Port 210 Pin Function Select Register
    #[inline(always)]
    pub const fn p210pfs(&self) -> &P2pfs {
        &self.p210pfs
    }
    ///0xa8 - Port 211 Pin Function Select Register
    #[inline(always)]
    pub const fn p211pfs(&self) -> &P2pfs {
        &self.p211pfs
    }
    ///0xa8 - Port 212 Pin Function Select Register
    #[inline(always)]
    pub const fn p212pfs(&self) -> &P2pfs {
        &self.p212pfs
    }
    ///0xa8 - Port 213 Pin Function Select Register
    #[inline(always)]
    pub const fn p213pfs(&self) -> &P2pfs {
        &self.p213pfs
    }
    ///0xa8 - Port 214 Pin Function Select Register
    #[inline(always)]
    pub const fn p214pfs(&self) -> &P2pfs {
        &self.p214pfs
    }
    ///0xc0 - Port 300 Pin Function Select Register
    #[inline(always)]
    pub const fn p300pfs(&self) -> &P300pfs {
        &self.p300pfs
    }
    ///0xc4..0xe8 - Port 30%s Pin Function Select Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `P301PFS` register.</div>
    #[inline(always)]
    pub const fn p30pfs(&self, n: usize) -> &P30pfs {
        &self.p30pfs[n]
    }
    ///Iterator for array of:
    ///0xc4..0xe8 - Port 30%s Pin Function Select Register
    #[inline(always)]
    pub fn p30pfs_iter(&self) -> impl Iterator<Item = &P30pfs> {
        self.p30pfs.iter()
    }
    ///0xc4 - Port 301 Pin Function Select Register
    #[inline(always)]
    pub const fn p301pfs(&self) -> &P30pfs {
        self.p30pfs(0)
    }
    ///0xc8 - Port 302 Pin Function Select Register
    #[inline(always)]
    pub const fn p302pfs(&self) -> &P30pfs {
        self.p30pfs(1)
    }
    ///0xcc - Port 303 Pin Function Select Register
    #[inline(always)]
    pub const fn p303pfs(&self) -> &P30pfs {
        self.p30pfs(2)
    }
    ///0xd0 - Port 304 Pin Function Select Register
    #[inline(always)]
    pub const fn p304pfs(&self) -> &P30pfs {
        self.p30pfs(3)
    }
    ///0xd4 - Port 305 Pin Function Select Register
    #[inline(always)]
    pub const fn p305pfs(&self) -> &P30pfs {
        self.p30pfs(4)
    }
    ///0xd8 - Port 306 Pin Function Select Register
    #[inline(always)]
    pub const fn p306pfs(&self) -> &P30pfs {
        self.p30pfs(5)
    }
    ///0xdc - Port 307 Pin Function Select Register
    #[inline(always)]
    pub const fn p307pfs(&self) -> &P30pfs {
        self.p30pfs(6)
    }
    ///0xe0 - Port 308 Pin Function Select Register
    #[inline(always)]
    pub const fn p308pfs(&self) -> &P30pfs {
        self.p30pfs(7)
    }
    ///0xe4 - Port 309 Pin Function Select Register
    #[inline(always)]
    pub const fn p309pfs(&self) -> &P30pfs {
        self.p30pfs(8)
    }
    ///0xe8..0x100 - Port 3%s Pin Function Select Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `P310PFS` register.</div>
    #[inline(always)]
    pub const fn p3pfs(&self, n: usize) -> &P3pfs {
        &self.p3pfs[n]
    }
    ///Iterator for array of:
    ///0xe8..0x100 - Port 3%s Pin Function Select Register
    #[inline(always)]
    pub fn p3pfs_iter(&self) -> impl Iterator<Item = &P3pfs> {
        self.p3pfs.iter()
    }
    ///0xe8 - Port 310 Pin Function Select Register
    #[inline(always)]
    pub const fn p310pfs(&self) -> &P3pfs {
        self.p3pfs(0)
    }
    ///0xec - Port 311 Pin Function Select Register
    #[inline(always)]
    pub const fn p311pfs(&self) -> &P3pfs {
        self.p3pfs(1)
    }
    ///0xf0 - Port 312 Pin Function Select Register
    #[inline(always)]
    pub const fn p312pfs(&self) -> &P3pfs {
        self.p3pfs(2)
    }
    ///0xf4 - Port 313 Pin Function Select Register
    #[inline(always)]
    pub const fn p313pfs(&self) -> &P3pfs {
        self.p3pfs(3)
    }
    ///0xf8 - Port 314 Pin Function Select Register
    #[inline(always)]
    pub const fn p314pfs(&self) -> &P3pfs {
        self.p3pfs(4)
    }
    ///0xfc - Port 315 Pin Function Select Register
    #[inline(always)]
    pub const fn p315pfs(&self) -> &P3pfs {
        self.p3pfs(5)
    }
    ///0x100..0x128 - Port 40%s Pin Function Select Register
    #[inline(always)]
    pub const fn p40pfs(&self, n: usize) -> &P40pfs {
        &self.p40pfs[n]
    }
    ///Iterator for array of:
    ///0x100..0x128 - Port 40%s Pin Function Select Register
    #[inline(always)]
    pub fn p40pfs_iter(&self) -> impl Iterator<Item = &P40pfs> {
        self.p40pfs.iter()
    }
    ///0x100 - Port 400 Pin Function Select Register
    #[inline(always)]
    pub const fn p400pfs(&self) -> &P40pfs {
        self.p40pfs(0)
    }
    ///0x104 - Port 401 Pin Function Select Register
    #[inline(always)]
    pub const fn p401pfs(&self) -> &P40pfs {
        self.p40pfs(1)
    }
    ///0x108 - Port 402 Pin Function Select Register
    #[inline(always)]
    pub const fn p402pfs(&self) -> &P40pfs {
        self.p40pfs(2)
    }
    ///0x10c - Port 403 Pin Function Select Register
    #[inline(always)]
    pub const fn p403pfs(&self) -> &P40pfs {
        self.p40pfs(3)
    }
    ///0x110 - Port 404 Pin Function Select Register
    #[inline(always)]
    pub const fn p404pfs(&self) -> &P40pfs {
        self.p40pfs(4)
    }
    ///0x114 - Port 405 Pin Function Select Register
    #[inline(always)]
    pub const fn p405pfs(&self) -> &P40pfs {
        self.p40pfs(5)
    }
    ///0x118 - Port 406 Pin Function Select Register
    #[inline(always)]
    pub const fn p406pfs(&self) -> &P40pfs {
        self.p40pfs(6)
    }
    ///0x11c - Port 407 Pin Function Select Register
    #[inline(always)]
    pub const fn p407pfs(&self) -> &P40pfs {
        self.p40pfs(7)
    }
    ///0x120 - Port 408 Pin Function Select Register
    #[inline(always)]
    pub const fn p408pfs(&self) -> &P40pfs {
        self.p40pfs(8)
    }
    ///0x124 - Port 409 Pin Function Select Register
    #[inline(always)]
    pub const fn p409pfs(&self) -> &P40pfs {
        self.p40pfs(9)
    }
    ///0x128..0x140 - Port 4%s Pin Function Select Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `P410PFS` register.</div>
    #[inline(always)]
    pub const fn p4pfs(&self, n: usize) -> &P4pfs {
        &self.p4pfs[n]
    }
    ///Iterator for array of:
    ///0x128..0x140 - Port 4%s Pin Function Select Register
    #[inline(always)]
    pub fn p4pfs_iter(&self) -> impl Iterator<Item = &P4pfs> {
        self.p4pfs.iter()
    }
    ///0x128 - Port 410 Pin Function Select Register
    #[inline(always)]
    pub const fn p410pfs(&self) -> &P4pfs {
        self.p4pfs(0)
    }
    ///0x12c - Port 411 Pin Function Select Register
    #[inline(always)]
    pub const fn p411pfs(&self) -> &P4pfs {
        self.p4pfs(1)
    }
    ///0x130 - Port 412 Pin Function Select Register
    #[inline(always)]
    pub const fn p412pfs(&self) -> &P4pfs {
        self.p4pfs(2)
    }
    ///0x134 - Port 413 Pin Function Select Register
    #[inline(always)]
    pub const fn p413pfs(&self) -> &P4pfs {
        self.p4pfs(3)
    }
    ///0x138 - Port 414 Pin Function Select Register
    #[inline(always)]
    pub const fn p414pfs(&self) -> &P4pfs {
        self.p4pfs(4)
    }
    ///0x13c - Port 415 Pin Function Select Register
    #[inline(always)]
    pub const fn p415pfs(&self) -> &P4pfs {
        self.p4pfs(5)
    }
    ///0x140..0x164 - Port 50%s Pin Function Select Register
    #[inline(always)]
    pub const fn p50pfs(&self, n: usize) -> &P50pfs {
        &self.p50pfs[n]
    }
    ///Iterator for array of:
    ///0x140..0x164 - Port 50%s Pin Function Select Register
    #[inline(always)]
    pub fn p50pfs_iter(&self) -> impl Iterator<Item = &P50pfs> {
        self.p50pfs.iter()
    }
    ///0x140 - Port 500 Pin Function Select Register
    #[inline(always)]
    pub const fn p500pfs(&self) -> &P50pfs {
        self.p50pfs(0)
    }
    ///0x144 - Port 501 Pin Function Select Register
    #[inline(always)]
    pub const fn p501pfs(&self) -> &P50pfs {
        self.p50pfs(1)
    }
    ///0x148 - Port 502 Pin Function Select Register
    #[inline(always)]
    pub const fn p502pfs(&self) -> &P50pfs {
        self.p50pfs(2)
    }
    ///0x14c - Port 503 Pin Function Select Register
    #[inline(always)]
    pub const fn p503pfs(&self) -> &P50pfs {
        self.p50pfs(3)
    }
    ///0x150 - Port 504 Pin Function Select Register
    #[inline(always)]
    pub const fn p504pfs(&self) -> &P50pfs {
        self.p50pfs(4)
    }
    ///0x154 - Port 505 Pin Function Select Register
    #[inline(always)]
    pub const fn p505pfs(&self) -> &P50pfs {
        self.p50pfs(5)
    }
    ///0x158 - Port 506 Pin Function Select Register
    #[inline(always)]
    pub const fn p506pfs(&self) -> &P50pfs {
        self.p50pfs(6)
    }
    ///0x15c - Port 507 Pin Function Select Register
    #[inline(always)]
    pub const fn p507pfs(&self) -> &P50pfs {
        self.p50pfs(7)
    }
    ///0x160 - Port 508 Pin Function Select Register
    #[inline(always)]
    pub const fn p508pfs(&self) -> &P50pfs {
        self.p50pfs(8)
    }
    ///0x16c..0x178 - Port 5%s Pin Function Select Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `P511PFS` register.</div>
    #[inline(always)]
    pub const fn p5pfs(&self, n: usize) -> &P5pfs {
        &self.p5pfs[n]
    }
    ///Iterator for array of:
    ///0x16c..0x178 - Port 5%s Pin Function Select Register
    #[inline(always)]
    pub fn p5pfs_iter(&self) -> impl Iterator<Item = &P5pfs> {
        self.p5pfs.iter()
    }
    ///0x16c - Port 511 Pin Function Select Register
    #[inline(always)]
    pub const fn p511pfs(&self) -> &P5pfs {
        self.p5pfs(0)
    }
    ///0x170 - Port 512 Pin Function Select Register
    #[inline(always)]
    pub const fn p512pfs(&self) -> &P5pfs {
        self.p5pfs(1)
    }
    ///0x174 - Port 513 Pin Function Select Register
    #[inline(always)]
    pub const fn p513pfs(&self) -> &P5pfs {
        self.p5pfs(2)
    }
    ///0x180..0x1a8 - Port 60%s Pin Function Select Register
    #[inline(always)]
    pub const fn p60pfs(&self, n: usize) -> &P60pfs {
        &self.p60pfs[n]
    }
    ///Iterator for array of:
    ///0x180..0x1a8 - Port 60%s Pin Function Select Register
    #[inline(always)]
    pub fn p60pfs_iter(&self) -> impl Iterator<Item = &P60pfs> {
        self.p60pfs.iter()
    }
    ///0x180 - Port 600 Pin Function Select Register
    #[inline(always)]
    pub const fn p600pfs(&self) -> &P60pfs {
        self.p60pfs(0)
    }
    ///0x184 - Port 601 Pin Function Select Register
    #[inline(always)]
    pub const fn p601pfs(&self) -> &P60pfs {
        self.p60pfs(1)
    }
    ///0x188 - Port 602 Pin Function Select Register
    #[inline(always)]
    pub const fn p602pfs(&self) -> &P60pfs {
        self.p60pfs(2)
    }
    ///0x18c - Port 603 Pin Function Select Register
    #[inline(always)]
    pub const fn p603pfs(&self) -> &P60pfs {
        self.p60pfs(3)
    }
    ///0x190 - Port 604 Pin Function Select Register
    #[inline(always)]
    pub const fn p604pfs(&self) -> &P60pfs {
        self.p60pfs(4)
    }
    ///0x194 - Port 605 Pin Function Select Register
    #[inline(always)]
    pub const fn p605pfs(&self) -> &P60pfs {
        self.p60pfs(5)
    }
    ///0x198 - Port 606 Pin Function Select Register
    #[inline(always)]
    pub const fn p606pfs(&self) -> &P60pfs {
        self.p60pfs(6)
    }
    ///0x19c - Port 607 Pin Function Select Register
    #[inline(always)]
    pub const fn p607pfs(&self) -> &P60pfs {
        self.p60pfs(7)
    }
    ///0x1a0 - Port 608 Pin Function Select Register
    #[inline(always)]
    pub const fn p608pfs(&self) -> &P60pfs {
        self.p60pfs(8)
    }
    ///0x1a4 - Port 609 Pin Function Select Register
    #[inline(always)]
    pub const fn p609pfs(&self) -> &P60pfs {
        self.p60pfs(9)
    }
    ///0x1a8..0x1c0 - Port 6%s Pin Function Select Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `P610PFS` register.</div>
    #[inline(always)]
    pub const fn p6pfs(&self, n: usize) -> &P6pfs {
        &self.p6pfs[n]
    }
    ///Iterator for array of:
    ///0x1a8..0x1c0 - Port 6%s Pin Function Select Register
    #[inline(always)]
    pub fn p6pfs_iter(&self) -> impl Iterator<Item = &P6pfs> {
        self.p6pfs.iter()
    }
    ///0x1a8 - Port 610 Pin Function Select Register
    #[inline(always)]
    pub const fn p610pfs(&self) -> &P6pfs {
        self.p6pfs(0)
    }
    ///0x1ac - Port 611 Pin Function Select Register
    #[inline(always)]
    pub const fn p611pfs(&self) -> &P6pfs {
        self.p6pfs(1)
    }
    ///0x1b0 - Port 612 Pin Function Select Register
    #[inline(always)]
    pub const fn p612pfs(&self) -> &P6pfs {
        self.p6pfs(2)
    }
    ///0x1b4 - Port 613 Pin Function Select Register
    #[inline(always)]
    pub const fn p613pfs(&self) -> &P6pfs {
        self.p6pfs(3)
    }
    ///0x1b8 - Port 614 Pin Function Select Register
    #[inline(always)]
    pub const fn p614pfs(&self) -> &P6pfs {
        self.p6pfs(4)
    }
    ///0x1bc - Port 615 Pin Function Select Register
    #[inline(always)]
    pub const fn p615pfs(&self) -> &P6pfs {
        self.p6pfs(5)
    }
    ///0x1c0..0x1e0 - Port 70%s Pin Function Select Register
    #[inline(always)]
    pub const fn p70pfs(&self, n: usize) -> &P70pfs {
        &self.p70pfs[n]
    }
    ///Iterator for array of:
    ///0x1c0..0x1e0 - Port 70%s Pin Function Select Register
    #[inline(always)]
    pub fn p70pfs_iter(&self) -> impl Iterator<Item = &P70pfs> {
        self.p70pfs.iter()
    }
    ///0x1c0 - Port 700 Pin Function Select Register
    #[inline(always)]
    pub const fn p700pfs(&self) -> &P70pfs {
        self.p70pfs(0)
    }
    ///0x1c4 - Port 701 Pin Function Select Register
    #[inline(always)]
    pub const fn p701pfs(&self) -> &P70pfs {
        self.p70pfs(1)
    }
    ///0x1c8 - Port 702 Pin Function Select Register
    #[inline(always)]
    pub const fn p702pfs(&self) -> &P70pfs {
        self.p70pfs(2)
    }
    ///0x1cc - Port 703 Pin Function Select Register
    #[inline(always)]
    pub const fn p703pfs(&self) -> &P70pfs {
        self.p70pfs(3)
    }
    ///0x1d0 - Port 704 Pin Function Select Register
    #[inline(always)]
    pub const fn p704pfs(&self) -> &P70pfs {
        self.p70pfs(4)
    }
    ///0x1d4 - Port 705 Pin Function Select Register
    #[inline(always)]
    pub const fn p705pfs(&self) -> &P70pfs {
        self.p70pfs(5)
    }
    ///0x1d8 - Port 706 Pin Function Select Register
    #[inline(always)]
    pub const fn p706pfs(&self) -> &P70pfs {
        self.p70pfs(6)
    }
    ///0x1dc - Port 707 Pin Function Select Register
    #[inline(always)]
    pub const fn p707pfs(&self) -> &P70pfs {
        self.p70pfs(7)
    }
    ///0x1e0 - Port 708 Pin Function Select Register
    #[inline(always)]
    pub const fn p708pfs(&self) -> &P70pfs {
        &self.p708pfs
    }
    ///0x1e0 - Port 709 Pin Function Select Register
    #[inline(always)]
    pub const fn p709pfs(&self) -> &P70pfs {
        &self.p709pfs
    }
    ///0x1e8..0x1f8 - Port 7%s Pin Function Select Register
    ///
    ///<div class="warning">`n` is the index of register in the array. `n == 0` corresponds to `P710PFS` register.</div>
    #[inline(always)]
    pub const fn p7pfs(&self, n: usize) -> &P7pfs {
        &self.p7pfs[n]
    }
    ///Iterator for array of:
    ///0x1e8..0x1f8 - Port 7%s Pin Function Select Register
    #[inline(always)]
    pub fn p7pfs_iter(&self) -> impl Iterator<Item = &P7pfs> {
        self.p7pfs.iter()
    }
    ///0x1e8 - Port 710 Pin Function Select Register
    #[inline(always)]
    pub const fn p710pfs(&self) -> &P7pfs {
        self.p7pfs(0)
    }
    ///0x1ec - Port 711 Pin Function Select Register
    #[inline(always)]
    pub const fn p711pfs(&self) -> &P7pfs {
        self.p7pfs(1)
    }
    ///0x1f0 - Port 712 Pin Function Select Register
    #[inline(always)]
    pub const fn p712pfs(&self) -> &P7pfs {
        self.p7pfs(2)
    }
    ///0x1f4 - Port 713 Pin Function Select Register
    #[inline(always)]
    pub const fn p713pfs(&self) -> &P7pfs {
        self.p7pfs(3)
    }
    ///0x200..0x21c - Port 80%s Pin Function Select Register
    #[inline(always)]
    pub const fn p80pfs(&self, n: usize) -> &P80pfs {
        &self.p80pfs[n]
    }
    ///Iterator for array of:
    ///0x200..0x21c - Port 80%s Pin Function Select Register
    #[inline(always)]
    pub fn p80pfs_iter(&self) -> impl Iterator<Item = &P80pfs> {
        self.p80pfs.iter()
    }
    ///0x200 - Port 800 Pin Function Select Register
    #[inline(always)]
    pub const fn p800pfs(&self) -> &P80pfs {
        self.p80pfs(0)
    }
    ///0x204 - Port 801 Pin Function Select Register
    #[inline(always)]
    pub const fn p801pfs(&self) -> &P80pfs {
        self.p80pfs(1)
    }
    ///0x208 - Port 802 Pin Function Select Register
    #[inline(always)]
    pub const fn p802pfs(&self) -> &P80pfs {
        self.p80pfs(2)
    }
    ///0x20c - Port 803 Pin Function Select Register
    #[inline(always)]
    pub const fn p803pfs(&self) -> &P80pfs {
        self.p80pfs(3)
    }
    ///0x210 - Port 804 Pin Function Select Register
    #[inline(always)]
    pub const fn p804pfs(&self) -> &P80pfs {
        self.p80pfs(4)
    }
    ///0x214 - Port 805 Pin Function Select Register
    #[inline(always)]
    pub const fn p805pfs(&self) -> &P80pfs {
        self.p80pfs(5)
    }
    ///0x218 - Port 806 Pin Function Select Register
    #[inline(always)]
    pub const fn p806pfs(&self) -> &P80pfs {
        self.p80pfs(6)
    }
    ///0x240..0x248 - Port 90%s Pin Function Select Register
    #[inline(always)]
    pub const fn p90pfs(&self, n: usize) -> &P90pfs {
        &self.p90pfs[n]
    }
    ///Iterator for array of:
    ///0x240..0x248 - Port 90%s Pin Function Select Register
    #[inline(always)]
    pub fn p90pfs_iter(&self) -> impl Iterator<Item = &P90pfs> {
        self.p90pfs.iter()
    }
    ///0x240 - Port 900 Pin Function Select Register
    #[inline(always)]
    pub const fn p900pfs(&self) -> &P90pfs {
        self.p90pfs(0)
    }
    ///0x244 - Port 901 Pin Function Select Register
    #[inline(always)]
    pub const fn p901pfs(&self) -> &P90pfs {
        self.p90pfs(1)
    }
    ///0x254 - Port 905 Pin Function Select Register
    #[inline(always)]
    pub const fn p905pfs(&self) -> &P90pfs {
        &self.p905pfs
    }
    ///0x254 - Port 906 Pin Function Select Register
    #[inline(always)]
    pub const fn p906pfs(&self) -> &P90pfs {
        &self.p906pfs
    }
    ///0x254 - Port 907 Pin Function Select Register
    #[inline(always)]
    pub const fn p907pfs(&self) -> &P90pfs {
        &self.p907pfs
    }
    ///0x254 - Port 908 Pin Function Select Register
    #[inline(always)]
    pub const fn p908pfs(&self) -> &P90pfs {
        &self.p908pfs
    }
    ///0x280..0x288 - Port A0%s Pin Function Select Register
    #[inline(always)]
    pub const fn pa0pfs(&self, n: usize) -> &Pa0pfs {
        &self.pa0pfs[n]
    }
    ///Iterator for array of:
    ///0x280..0x288 - Port A0%s Pin Function Select Register
    #[inline(always)]
    pub fn pa0pfs_iter(&self) -> impl Iterator<Item = &Pa0pfs> {
        self.pa0pfs.iter()
    }
    ///0x280 - Port A00 Pin Function Select Register
    #[inline(always)]
    pub const fn pa00pfs(&self) -> &Pa0pfs {
        self.pa0pfs(0)
    }
    ///0x284 - Port A01 Pin Function Select Register
    #[inline(always)]
    pub const fn pa01pfs(&self) -> &Pa0pfs {
        self.pa0pfs(1)
    }
    ///0x2a0 - Port A08 Pin Function Select Register
    #[inline(always)]
    pub const fn pa08pfs(&self) -> &Pa0pfs {
        &self.pa08pfs
    }
    ///0x2a0 - Port A09 Pin Function Select Register
    #[inline(always)]
    pub const fn pa09pfs(&self) -> &Pa0pfs {
        &self.pa09pfs
    }
    ///0x2a8 - Port A10 Pin Function Select Register
    #[inline(always)]
    pub const fn pa10pfs(&self) -> &Pa10pfs {
        &self.pa10pfs
    }
    ///0x2c0..0x2c8 - Port B0%s Pin Function Select Register
    #[inline(always)]
    pub const fn pb0pfs(&self, n: usize) -> &Pb0pfs {
        &self.pb0pfs[n]
    }
    ///Iterator for array of:
    ///0x2c0..0x2c8 - Port B0%s Pin Function Select Register
    #[inline(always)]
    pub fn pb0pfs_iter(&self) -> impl Iterator<Item = &Pb0pfs> {
        self.pb0pfs.iter()
    }
    ///0x2c0 - Port B00 Pin Function Select Register
    #[inline(always)]
    pub const fn pb00pfs(&self) -> &Pb0pfs {
        self.pb0pfs(0)
    }
    ///0x2c4 - Port B01 Pin Function Select Register
    #[inline(always)]
    pub const fn pb01pfs(&self) -> &Pb0pfs {
        self.pb0pfs(1)
    }
    ///0x500 - Ethernet Control Register
    #[inline(always)]
    pub const fn pfenet(&self) -> &Pfenet {
        &self.pfenet
    }
    ///0x503 - Write-Protect Register
    #[inline(always)]
    pub const fn pwpr(&self) -> &Pwpr {
        &self.pwpr
    }
    ///0x505 - Write-Protect Register for Secure
    #[inline(always)]
    pub const fn pwprs(&self) -> &Pwprs {
        &self.pwprs
    }
    ///0x510..0x524 - Port Security Attribution register
    #[inline(always)]
    pub const fn psar(&self, n: usize) -> &Psar {
        &self.psar[n]
    }
    ///Iterator for array of:
    ///0x510..0x524 - Port Security Attribution register
    #[inline(always)]
    pub fn psar_iter(&self) -> impl Iterator<Item = &Psar> {
        self.psar.iter()
    }
    ///0x510 - Port Security Attribution register
    #[inline(always)]
    pub const fn p0sar(&self) -> &Psar {
        self.psar(0)
    }
    ///0x512 - Port Security Attribution register
    #[inline(always)]
    pub const fn p1sar(&self) -> &Psar {
        self.psar(1)
    }
    ///0x514 - Port Security Attribution register
    #[inline(always)]
    pub const fn p2sar(&self) -> &Psar {
        self.psar(2)
    }
    ///0x516 - Port Security Attribution register
    #[inline(always)]
    pub const fn p3sar(&self) -> &Psar {
        self.psar(3)
    }
    ///0x518 - Port Security Attribution register
    #[inline(always)]
    pub const fn p4sar(&self) -> &Psar {
        self.psar(4)
    }
    ///0x51a - Port Security Attribution register
    #[inline(always)]
    pub const fn p5sar(&self) -> &Psar {
        self.psar(5)
    }
    ///0x51c - Port Security Attribution register
    #[inline(always)]
    pub const fn p6sar(&self) -> &Psar {
        self.psar(6)
    }
    ///0x51e - Port Security Attribution register
    #[inline(always)]
    pub const fn p7sar(&self) -> &Psar {
        self.psar(7)
    }
    ///0x520 - Port Security Attribution register
    #[inline(always)]
    pub const fn p8sar(&self) -> &Psar {
        self.psar(8)
    }
    ///0x522 - Port Security Attribution register
    #[inline(always)]
    pub const fn p9sar(&self) -> &Psar {
        self.psar(9)
    }
    ///0x524 - Port Security Attribution register
    #[inline(always)]
    pub const fn pasar(&self) -> &Psar {
        &self.pasar
    }
    ///0x524 - Port Security Attribution register
    #[inline(always)]
    pub const fn pbsar(&self) -> &Psar {
        &self.pbsar
    }
}
/**P00PFS (rw) register accessor: Port 00%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p00pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p00pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p00pfs`] module*/
#[doc(alias = "P00PFS")]
pub type P00pfs = crate::Reg<p00pfs::P00pfsSpec>;
///Port 00%s Pin Function Select Register
pub mod p00pfs;
/**P008PFS (rw) register accessor: Port 008 Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p008pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p008pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p008pfs`] module*/
#[doc(alias = "P008PFS")]
pub type P008pfs = crate::Reg<p008pfs::P008pfsSpec>;
///Port 008 Pin Function Select Register
pub mod p008pfs;
/**P009PFS (rw) register accessor: Port 009 Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p009pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p009pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p009pfs`] module*/
#[doc(alias = "P009PFS")]
pub type P009pfs = crate::Reg<p009pfs::P009pfsSpec>;
///Port 009 Pin Function Select Register
pub mod p009pfs;
pub use P0pfs as P010pfs;
pub use p0pfs as p010pfs;
/**P0PFS (rw) register accessor: Port 0%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p0pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p0pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p0pfs`] module*/
#[doc(alias = "P0PFS")]
pub type P0pfs = crate::Reg<p0pfs::P0pfsSpec>;
///Port 0%s Pin Function Select Register
pub mod p0pfs;
/**P10PFS (rw) register accessor: Port 10%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p10pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p10pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p10pfs`] module*/
#[doc(alias = "P10PFS")]
pub type P10pfs = crate::Reg<p10pfs::P10pfsSpec>;
///Port 10%s Pin Function Select Register
pub mod p10pfs;
/**P1PFS (rw) register accessor: Port 1%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p1pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p1pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p1pfs`] module*/
#[doc(alias = "P1PFS")]
pub type P1pfs = crate::Reg<p1pfs::P1pfsSpec>;
///Port 1%s Pin Function Select Register
pub mod p1pfs;
pub use P20pfs as P200pfs;
pub use p20pfs as p200pfs;
/**P201PFS (rw) register accessor: Port 201 Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p201pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p201pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p201pfs`] module*/
#[doc(alias = "P201PFS")]
pub type P201pfs = crate::Reg<p201pfs::P201pfsSpec>;
///Port 201 Pin Function Select Register
pub mod p201pfs;
/**P20PFS (rw) register accessor: Port 20%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p20pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p20pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p20pfs`] module*/
#[doc(alias = "P20PFS")]
pub type P20pfs = crate::Reg<p20pfs::P20pfsSpec>;
///Port 20%s Pin Function Select Register
pub mod p20pfs;
pub use P20pfs as P210pfs;
pub use p20pfs as p210pfs;
/**P300PFS (rw) register accessor: Port 300 Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p300pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p300pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p300pfs`] module*/
#[doc(alias = "P300PFS")]
pub type P300pfs = crate::Reg<p300pfs::P300pfsSpec>;
///Port 300 Pin Function Select Register
pub mod p300pfs;
/**P30PFS (rw) register accessor: Port 30%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p30pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p30pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p30pfs`] module*/
#[doc(alias = "P30PFS")]
pub type P30pfs = crate::Reg<p30pfs::P30pfsSpec>;
///Port 30%s Pin Function Select Register
pub mod p30pfs;
/**P3PFS (rw) register accessor: Port 3%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p3pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p3pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p3pfs`] module*/
#[doc(alias = "P3PFS")]
pub type P3pfs = crate::Reg<p3pfs::P3pfsSpec>;
///Port 3%s Pin Function Select Register
pub mod p3pfs;
/**P40PFS (rw) register accessor: Port 40%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p40pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p40pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p40pfs`] module*/
#[doc(alias = "P40PFS")]
pub type P40pfs = crate::Reg<p40pfs::P40pfsSpec>;
///Port 40%s Pin Function Select Register
pub mod p40pfs;
/**P4PFS (rw) register accessor: Port 4%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p4pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p4pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p4pfs`] module*/
#[doc(alias = "P4PFS")]
pub type P4pfs = crate::Reg<p4pfs::P4pfsSpec>;
///Port 4%s Pin Function Select Register
pub mod p4pfs;
/**P50PFS (rw) register accessor: Port 50%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p50pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p50pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p50pfs`] module*/
#[doc(alias = "P50PFS")]
pub type P50pfs = crate::Reg<p50pfs::P50pfsSpec>;
///Port 50%s Pin Function Select Register
pub mod p50pfs;
/**P5PFS (rw) register accessor: Port 5%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p5pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p5pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p5pfs`] module*/
#[doc(alias = "P5PFS")]
pub type P5pfs = crate::Reg<p5pfs::P5pfsSpec>;
///Port 5%s Pin Function Select Register
pub mod p5pfs;
/**P60PFS (rw) register accessor: Port 60%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p60pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p60pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p60pfs`] module*/
#[doc(alias = "P60PFS")]
pub type P60pfs = crate::Reg<p60pfs::P60pfsSpec>;
///Port 60%s Pin Function Select Register
pub mod p60pfs;
/**P6PFS (rw) register accessor: Port 6%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p6pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p6pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p6pfs`] module*/
#[doc(alias = "P6PFS")]
pub type P6pfs = crate::Reg<p6pfs::P6pfsSpec>;
///Port 6%s Pin Function Select Register
pub mod p6pfs;
/**P70PFS (rw) register accessor: Port 70%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p70pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p70pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p70pfs`] module*/
#[doc(alias = "P70PFS")]
pub type P70pfs = crate::Reg<p70pfs::P70pfsSpec>;
///Port 70%s Pin Function Select Register
pub mod p70pfs;
pub use P70pfs as P708pfs;
pub use p70pfs as p708pfs;
/**P7PFS (rw) register accessor: Port 7%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p7pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p7pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p7pfs`] module*/
#[doc(alias = "P7PFS")]
pub type P7pfs = crate::Reg<p7pfs::P7pfsSpec>;
///Port 7%s Pin Function Select Register
pub mod p7pfs;
/**P80PFS (rw) register accessor: Port 80%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p80pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p80pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p80pfs`] module*/
#[doc(alias = "P80PFS")]
pub type P80pfs = crate::Reg<p80pfs::P80pfsSpec>;
///Port 80%s Pin Function Select Register
pub mod p80pfs;
/**P90PFS (rw) register accessor: Port 90%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`p90pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`p90pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@p90pfs`] module*/
#[doc(alias = "P90PFS")]
pub type P90pfs = crate::Reg<p90pfs::P90pfsSpec>;
///Port 90%s Pin Function Select Register
pub mod p90pfs;
pub use P90pfs as P905pfs;
pub use p90pfs as p905pfs;
/**PA0PFS (rw) register accessor: Port A0%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`pa0pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa0pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pa0pfs`] module*/
#[doc(alias = "PA0PFS")]
pub type Pa0pfs = crate::Reg<pa0pfs::Pa0pfsSpec>;
///Port A0%s Pin Function Select Register
pub mod pa0pfs;
pub use Pa0pfs as Pa08pfs;
pub use pa0pfs as pa08pfs;
/**PA10PFS (rw) register accessor: Port A10 Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`pa10pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pa10pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pa10pfs`] module*/
#[doc(alias = "PA10PFS")]
pub type Pa10pfs = crate::Reg<pa10pfs::Pa10pfsSpec>;
///Port A10 Pin Function Select Register
pub mod pa10pfs;
/**PB0PFS (rw) register accessor: Port B0%s Pin Function Select Register

You can [`read`](crate::Reg::read) this register and get [`pb0pfs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pb0pfs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pb0pfs`] module*/
#[doc(alias = "PB0PFS")]
pub type Pb0pfs = crate::Reg<pb0pfs::Pb0pfsSpec>;
///Port B0%s Pin Function Select Register
pub mod pb0pfs;
/**PFENET (rw) register accessor: Ethernet Control Register

You can [`read`](crate::Reg::read) this register and get [`pfenet::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pfenet::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pfenet`] module*/
#[doc(alias = "PFENET")]
pub type Pfenet = crate::Reg<pfenet::PfenetSpec>;
///Ethernet Control Register
pub mod pfenet;
/**PWPR (rw) register accessor: Write-Protect Register

You can [`read`](crate::Reg::read) this register and get [`pwpr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwpr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwpr`] module*/
#[doc(alias = "PWPR")]
pub type Pwpr = crate::Reg<pwpr::PwprSpec>;
///Write-Protect Register
pub mod pwpr;
/**PWPRS (rw) register accessor: Write-Protect Register for Secure

You can [`read`](crate::Reg::read) this register and get [`pwprs::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pwprs::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@pwprs`] module*/
#[doc(alias = "PWPRS")]
pub type Pwprs = crate::Reg<pwprs::PwprsSpec>;
///Write-Protect Register for Secure
pub mod pwprs;
/**PSAR (rw) register accessor: Port Security Attribution register

You can [`read`](crate::Reg::read) this register and get [`psar::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psar::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).

For information about available fields see [`mod@psar`] module*/
#[doc(alias = "PSAR")]
pub type Psar = crate::Reg<psar::PsarSpec>;
///Port Security Attribution register
pub mod psar;
pub use Psar as Pasar;
pub use psar as pasar;
