#[doc = "Register `TASKS_STOP` writer"]
pub type W = crate::W<TasksStopSpec>;
#[doc = "Stop temperature measurement\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStop {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStop> for bool {
    #[inline(always)]
    fn from(variant: TasksStop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STOP` writer - Stop temperature measurement"]
pub type TasksStopW<'a, REG> = crate::BitWriter<'a, REG, TasksStop>;
impl<'a, REG> TasksStopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStop::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop temperature measurement"]
    #[inline(always)]
    pub fn tasks_stop(&mut self) -> TasksStopW<TasksStopSpec> {
        TasksStopW::new(self, 0)
    }
}
#[doc = "Stop temperature measurement\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stop::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStopSpec;
impl crate::RegisterSpec for TasksStopSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_stop::W`](W) writer structure"]
impl crate::Writable for TasksStopSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_STOP to value 0"]
impl crate::Resettable for TasksStopSpec {
    const RESET_VALUE: u32 = 0;
}
