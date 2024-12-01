#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_chg: [TasksChg; 6],
    _reserved1: [u8; 0x04d0],
    chen: Chen,
    chenset: Chenset,
    chenclr: Chenclr,
    _reserved4: [u8; 0x04],
    ch: [Ch; 20],
    _reserved5: [u8; 0x0250],
    chg: [Chg; 6],
    _reserved6: [u8; 0xf8],
    fork: [Fork; 32],
}
impl RegisterBlock {
    #[doc = "0x00..0x30 - Channel group tasks"]
    #[inline(always)]
    pub const fn tasks_chg(&self, n: usize) -> &TasksChg {
        &self.tasks_chg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x30 - Channel group tasks"]
    #[inline(always)]
    pub fn tasks_chg_iter(&self) -> impl Iterator<Item = &TasksChg> {
        self.tasks_chg.iter()
    }
    #[doc = "0x500 - Channel enable register"]
    #[inline(always)]
    pub const fn chen(&self) -> &Chen {
        &self.chen
    }
    #[doc = "0x504 - Channel enable set register"]
    #[inline(always)]
    pub const fn chenset(&self) -> &Chenset {
        &self.chenset
    }
    #[doc = "0x508 - Channel enable clear register"]
    #[inline(always)]
    pub const fn chenclr(&self) -> &Chenclr {
        &self.chenclr
    }
    #[doc = "0x510..0x5b0 - PPI Channel"]
    #[inline(always)]
    pub const fn ch(&self, n: usize) -> &Ch {
        &self.ch[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x510..0x5b0 - PPI Channel"]
    #[inline(always)]
    pub fn ch_iter(&self) -> impl Iterator<Item = &Ch> {
        self.ch.iter()
    }
    #[doc = "0x800..0x818 - Description collection: Channel group n"]
    #[inline(always)]
    pub const fn chg(&self, n: usize) -> &Chg {
        &self.chg[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x800..0x818 - Description collection: Channel group n"]
    #[inline(always)]
    pub fn chg_iter(&self) -> impl Iterator<Item = &Chg> {
        self.chg.iter()
    }
    #[doc = "0x910..0x990 - Fork"]
    #[inline(always)]
    pub const fn fork(&self, n: usize) -> &Fork {
        &self.fork[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x910..0x990 - Fork"]
    #[inline(always)]
    pub fn fork_iter(&self) -> impl Iterator<Item = &Fork> {
        self.fork.iter()
    }
}
#[doc = "Channel group tasks"]
pub use self::tasks_chg::TasksChg;
#[doc = r"Cluster"]
#[doc = "Channel group tasks"]
pub mod tasks_chg;
#[doc = "CHEN (rw) register accessor: Channel enable register\n\nYou can [`read`](crate::Reg::read) this register and get [`chen::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chen::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chen`]
module"]
#[doc(alias = "CHEN")]
pub type Chen = crate::Reg<chen::ChenSpec>;
#[doc = "Channel enable register"]
pub mod chen;
#[doc = "CHENSET (rw) register accessor: Channel enable set register\n\nYou can [`read`](crate::Reg::read) this register and get [`chenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chenset`]
module"]
#[doc(alias = "CHENSET")]
pub type Chenset = crate::Reg<chenset::ChensetSpec>;
#[doc = "Channel enable set register"]
pub mod chenset;
#[doc = "CHENCLR (rw) register accessor: Channel enable clear register\n\nYou can [`read`](crate::Reg::read) this register and get [`chenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chenclr`]
module"]
#[doc(alias = "CHENCLR")]
pub type Chenclr = crate::Reg<chenclr::ChenclrSpec>;
#[doc = "Channel enable clear register"]
pub mod chenclr;
#[doc = "PPI Channel"]
pub use self::ch::Ch;
#[doc = r"Cluster"]
#[doc = "PPI Channel"]
pub mod ch;
#[doc = "CHG (rw) register accessor: Description collection: Channel group n\n\nYou can [`read`](crate::Reg::read) this register and get [`chg::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chg::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@chg`]
module"]
#[doc(alias = "CHG")]
pub type Chg = crate::Reg<chg::ChgSpec>;
#[doc = "Description collection: Channel group n"]
pub mod chg;
#[doc = "Fork"]
pub use self::fork::Fork;
#[doc = r"Cluster"]
#[doc = "Fork"]
pub mod fork;
