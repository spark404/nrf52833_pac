#[doc = "Register `TASKS_SAMPLE` writer"]
pub type W = crate::W<TasksSampleSpec>;
#[doc = "Takes one SAADC sample\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TasksSample {
    #[doc = "1: Trigger task"]
    Trigger = 1,
}
impl From<TasksSample> for bool {
    #[inline(always)]
    fn from(variant: TasksSample) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TASKS_SAMPLE` writer - Takes one SAADC sample"]
pub type TasksSampleW<'a, REG> = crate::BitWriter<'a, REG, TasksSample>;
impl<'a, REG> TasksSampleW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Trigger task"]
    #[inline(always)]
    pub fn trigger(self) -> &'a mut crate::W<REG> {
        self.variant(TasksSample::Trigger)
    }
}
impl W {
    #[doc = "Bit 0 - Takes one SAADC sample"]
    #[inline(always)]
    pub fn tasks_sample(&mut self) -> TasksSampleW<TasksSampleSpec> {
        TasksSampleW::new(self, 0)
    }
}
#[doc = "Takes one SAADC sample\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tasks_sample::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TasksSampleSpec;
impl crate::RegisterSpec for TasksSampleSpec {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`tasks_sample::W`](W) writer structure"]
impl crate::Writable for TasksSampleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TASKS_SAMPLE to value 0"]
impl crate::Resettable for TasksSampleSpec {
    const RESET_VALUE: u32 = 0;
}
