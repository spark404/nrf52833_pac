#[doc = "Register `TASKS_STOPRX` writer"]
pub type W = crate::W<TasksStoprxSpec>;
#[doc = "Stop UART receiver\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksStoprx {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksStoprx> for bool {
    #[inline(always)]
    fn from(variant: TasksStoprx) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_STOPRX` writer - Stop UART receiver"]
pub type TasksStoprxW<'a, REG> = crate::BitWriter<'a, REG, TasksStoprx>;
impl<'a, REG> TasksStoprxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksStoprx::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Stop UART receiver"]
    #[inline(always)]
    pub fn tasks_stoprx(&mut self) -> TasksStoprxW<TasksStoprxSpec> {
        TasksStoprxW::new(self, 0)
    }
}
#[doc = "Stop UART receiver\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_stoprx::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksStoprxSpec;
impl crate::RegisterSpec for TasksStoprxSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_stoprx::W`](W) writer structure"]
impl crate::Writable for TasksStoprxSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_STOPRX to value 0"]
impl crate::Resettable for TasksStoprxSpec {
    const RESET_VALUE: u32 = 0;
}
