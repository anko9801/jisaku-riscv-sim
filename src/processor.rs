use enumn::N;
use strum_macros::EnumString;

pub enum XLEN {
    R32,
    R64,
}

#[derive(Debug, PartialEq, EnumString, N)]
pub enum XprName {
    #[strum(serialize = "zero")]
    zero,
    #[strum(serialize = "ra")]
    ra,
    #[strum(serialize = "sp")]
    sp,
    #[strum(serialize = "gp")]
    gp,
    #[strum(serialize = "tp")]
    tp,
    #[strum(serialize = "t0")]
    t0,
    #[strum(serialize = "t1")]
    t1,
    #[strum(serialize = "t2")]
    t2,
    #[strum(serialize = "s0")]
    s0,
    #[strum(serialize = "s1")]
    s1,
    #[strum(serialize = "a0")]
    a0,
    #[strum(serialize = "a1")]
    a1,
    #[strum(serialize = "a2")]
    a2,
    #[strum(serialize = "a3")]
    a3,
    #[strum(serialize = "a4")]
    a4,
    #[strum(serialize = "a5")]
    a5,
    #[strum(serialize = "a6")]
    a6,
    #[strum(serialize = "a7")]
    a7,
    #[strum(serialize = "s2")]
    s2,
    #[strum(serialize = "s3")]
    s3,
    #[strum(serialize = "s4")]
    s4,
    #[strum(serialize = "s5")]
    s5,
    #[strum(serialize = "s6")]
    s6,
    #[strum(serialize = "s7")]
    s7,
    #[strum(serialize = "s8")]
    s8,
    #[strum(serialize = "s9")]
    s9,
    #[strum(serialize = "s10")]
    s10,
    #[strum(serialize = "s11")]
    s11,
    #[strum(serialize = "t3")]
    t3,
    #[strum(serialize = "t4")]
    t4,
    #[strum(serialize = "t5")]
    t5,
    #[strum(serialize = "t6")]
    t6,
}
impl Into<usize> for XprName {
    fn into(self) -> usize {
        self as usize
    }
}

#[derive(Debug, PartialEq, EnumString, N)]
pub enum FprName {
    #[strum(serialize = "ft0")]
    ft0,
    #[strum(serialize = "ft1")]
    ft1,
    #[strum(serialize = "ft2")]
    ft2,
    #[strum(serialize = "ft3")]
    ft3,
    #[strum(serialize = "ft4")]
    ft4,
    #[strum(serialize = "ft5")]
    ft5,
    #[strum(serialize = "ft6")]
    ft6,
    #[strum(serialize = "ft7")]
    ft7,
    #[strum(serialize = "fs0")]
    fs0,
    #[strum(serialize = "fs1")]
    fs1,
    #[strum(serialize = "fa0")]
    fa0,
    #[strum(serialize = "fa1")]
    fa1,
    #[strum(serialize = "fa2")]
    fa2,
    #[strum(serialize = "fa3")]
    fa3,
    #[strum(serialize = "fa4")]
    fa4,
    #[strum(serialize = "fa5")]
    fa5,
    #[strum(serialize = "fa6")]
    fa6,
    #[strum(serialize = "fa7")]
    fa7,
    #[strum(serialize = "fs2")]
    fs2,
    #[strum(serialize = "fs3")]
    fs3,
    #[strum(serialize = "fs4")]
    fs4,
    #[strum(serialize = "fs5")]
    fs5,
    #[strum(serialize = "fs6")]
    fs6,
    #[strum(serialize = "fs7")]
    fs7,
    #[strum(serialize = "fs8")]
    fs8,
    #[strum(serialize = "fs9")]
    fs9,
    #[strum(serialize = "fs10")]
    fs10,
    #[strum(serialize = "fs11")]
    fs11,
    #[strum(serialize = "ft8")]
    ft8,
    #[strum(serialize = "ft9")]
    ft9,
    #[strum(serialize = "ft10")]
    ft10,
    #[strum(serialize = "ft11")]
    ft11,
}

#[derive(Debug, PartialEq, EnumString)]
pub enum VR {
    #[strum(serialize = "v0")]
    v0,
    #[strum(serialize = "v1")]
    v1,
    #[strum(serialize = "v2")]
    v2,
    #[strum(serialize = "v3")]
    v3,
    #[strum(serialize = "v4")]
    v4,
    #[strum(serialize = "v5")]
    v5,
    #[strum(serialize = "v6")]
    v6,
    #[strum(serialize = "v7")]
    v7,
    #[strum(serialize = "v8")]
    v8,
    #[strum(serialize = "v9")]
    v9,
    #[strum(serialize = "v10")]
    v10,
    #[strum(serialize = "v11")]
    v11,
    #[strum(serialize = "v12")]
    v12,
    #[strum(serialize = "v13")]
    v13,
    #[strum(serialize = "v14")]
    v14,
    #[strum(serialize = "v15")]
    v15,
    #[strum(serialize = "v16")]
    v16,
    #[strum(serialize = "v17")]
    v17,
    #[strum(serialize = "v18")]
    v18,
    #[strum(serialize = "v19")]
    v19,
    #[strum(serialize = "v20")]
    v20,
    #[strum(serialize = "v21")]
    v21,
    #[strum(serialize = "v22")]
    v22,
    #[strum(serialize = "v23")]
    v23,
    #[strum(serialize = "v24")]
    v24,
    #[strum(serialize = "v25")]
    v25,
    #[strum(serialize = "v26")]
    v26,
    #[strum(serialize = "v27")]
    v27,
    #[strum(serialize = "v28")]
    v28,
    #[strum(serialize = "v29")]
    v29,
    #[strum(serialize = "v30")]
    v30,
    #[strum(serialize = "v31")]
    v31,
}

type Reg = i64;
pub struct XPR {
    data: [Reg; 32],
}
impl XPR {
    pub fn new() -> Self {
        XPR { data: [0; 32] }
    }
    pub fn get<N: Into<usize>>(&self, reg: N) -> Reg {
        self.data[reg.into()]
    }
    pub fn set<N: Into<usize>>(&mut self, reg: N, value: Reg) {
        self.data[reg.into()] = value;
    }
}

type FReg = f64;
struct FPR {
    data: [FReg; 32],
}
impl FPR {
    pub fn get(&self, name: FprName) -> FReg {
        self.data[name as usize]
    }
    pub fn set(&mut self, name: FprName, value: FReg) {
        self.data[name as usize] = value;
    }
}

pub struct State {
    pub regs: XPR,
    pub pc: Reg,
    pub xlen: XLEN,
}

impl State {
    pub fn new() -> Self {
        State {
            regs: XPR::new(),
            pc: 0,
            xlen: XLEN::R64,
        }
    }
}
