#[doc = "Register `TASKS_TRIGOVRFLW` writer"]
pub type W = crate::W<TasksTrigovrflwSpec>;
#[doc = "Set COUNTER to 0xFFFFF0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksTrigovrflw {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksTrigovrflw> for bool {
    #[inline(always)]
    fn from(variant: TasksTrigovrflw) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_TRIGOVRFLW` writer - Set COUNTER to 0xFFFFF0"]
pub type TasksTrigovrflwW<'a, REG> = crate::BitWriter<'a, REG, TasksTrigovrflw>;
impl<'a, REG> TasksTrigovrflwW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksTrigovrflw::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Set COUNTER to 0xFFFFF0"]
    #[inline(always)]
    pub fn tasks_trigovrflw(&mut self) -> TasksTrigovrflwW<TasksTrigovrflwSpec> {
        TasksTrigovrflwW::new(self, 0)
    }
}
#[doc = "Set COUNTER to 0xFFFFF0\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_trigovrflw::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksTrigovrflwSpec;
impl crate::RegisterSpec for TasksTrigovrflwSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_trigovrflw::W`](W) writer structure"]
impl crate::Writable for TasksTrigovrflwSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_TRIGOVRFLW to value 0"]
impl crate::Resettable for TasksTrigovrflwSpec {
    const RESET_VALUE: u32 = 0;
}
