#[repr(C)]
#[doc = "Register block"]
pub struct RegisterBlock {
    tasks_trigger: [TasksTrigger; 16],
    _reserved1: [u8; 0xc0],
    events_triggered: [EventsTriggered; 16],
    _reserved2: [u8; 0x01c0],
    inten: Inten,
    intenset: Intenset,
    intenclr: Intenclr,
}
impl RegisterBlock {
    #[doc = "0x00..0x40 - Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\]
event"]
    #[inline(always)]
    pub const fn tasks_trigger(&self, n: usize) -> &TasksTrigger {
        &self.tasks_trigger[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x00..0x40 - Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\]
event"]
    #[inline(always)]
    pub fn tasks_trigger_iter(&self) -> impl Iterator<Item = &TasksTrigger> {
        self.tasks_trigger.iter()
    }
    #[doc = "0x100..0x140 - Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\]
task"]
    #[inline(always)]
    pub const fn events_triggered(&self, n: usize) -> &EventsTriggered {
        &self.events_triggered[n]
    }
    #[doc = "Iterator for array of:"]
    #[doc = "0x100..0x140 - Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\]
task"]
    #[inline(always)]
    pub fn events_triggered_iter(&self) -> impl Iterator<Item = &EventsTriggered> {
        self.events_triggered.iter()
    }
    #[doc = "0x300 - Enable or disable interrupt"]
    #[inline(always)]
    pub const fn inten(&self) -> &Inten {
        &self.inten
    }
    #[doc = "0x304 - Enable interrupt"]
    #[inline(always)]
    pub const fn intenset(&self) -> &Intenset {
        &self.intenset
    }
    #[doc = "0x308 - Disable interrupt"]
    #[inline(always)]
    pub const fn intenclr(&self) -> &Intenclr {
        &self.intenclr
    }
}
#[doc = "TASKS_TRIGGER (w) register accessor: Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\]
event\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_trigger::W`]. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@tasks_trigger`]
module"]
#[doc(alias = "TASKS_TRIGGER")]
pub type TasksTrigger = crate::Reg<tasks_trigger::TasksTriggerSpec>;
#[doc = "Description collection: Trigger n for triggering the corresponding TRIGGERED\\[n\\]
event"]
pub mod tasks_trigger;
#[doc = "EVENTS_TRIGGERED (rw) register accessor: Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\]
task\n\nYou can [`read`](crate::Reg::read) this register and get [`events_triggered::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`events_triggered::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@events_triggered`]
module"]
#[doc(alias = "EVENTS_TRIGGERED")]
pub type EventsTriggered = crate::Reg<events_triggered::EventsTriggeredSpec>;
#[doc = "Description collection: Event number n generated by triggering the corresponding TRIGGER\\[n\\]
task"]
pub mod events_triggered;
#[doc = "INTEN (rw) register accessor: Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@inten`]
module"]
#[doc(alias = "INTEN")]
pub type Inten = crate::Reg<inten::IntenSpec>;
#[doc = "Enable or disable interrupt"]
pub mod inten;
#[doc = "INTENSET (rw) register accessor: Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenset`]
module"]
#[doc(alias = "INTENSET")]
pub type Intenset = crate::Reg<intenset::IntensetSpec>;
#[doc = "Enable interrupt"]
pub mod intenset;
#[doc = "INTENCLR (rw) register accessor: Disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenclr::R`]. You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenclr::W`]. You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [`mod@intenclr`]
module"]
#[doc(alias = "INTENCLR")]
pub type Intenclr = crate::Reg<intenclr::IntenclrSpec>;
#[doc = "Disable interrupt"]
pub mod intenclr;
