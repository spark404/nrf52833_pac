#[doc = "Register `SHORTS` reader"]
pub type R = crate::R<ShortsSpec>;
#[doc = "Register `SHORTS` writer"]
pub type W = crate::W<ShortsSpec>;
#[doc = "Shortcut between event END and task ACQUIRE\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum EndAcquire {
    #[doc = "0: Disable shortcut"]
    Disabled = 0,
    #[doc = "1: Enable shortcut"]
    Enabled = 1,
}
impl From<EndAcquire> for bool {
    #[inline(always)]
    fn from(variant: EndAcquire) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `END_ACQUIRE` reader - Shortcut between event END and task ACQUIRE"]
pub type EndAcquireR = crate::BitReader<EndAcquire>;
impl EndAcquireR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> EndAcquire {
        match self.bits {
            false => EndAcquire::Disabled,
            true => EndAcquire::Enabled,
        }
    }
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == EndAcquire::Disabled
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == EndAcquire::Enabled
    }
}
#[doc = "Field `END_ACQUIRE` writer - Shortcut between event END and task ACQUIRE"]
pub type EndAcquireW<'a, REG> = crate::BitWriter<'a, REG, EndAcquire>;
impl<'a, REG> EndAcquireW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable shortcut"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndAcquire::Disabled)
    }
    #[doc = "Enable shortcut"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(EndAcquire::Enabled)
    }
}
impl R {
    #[doc = "Bit 2 - Shortcut between event END and task ACQUIRE"]
    #[inline(always)]
    pub fn end_acquire(&self) -> EndAcquireR {
        EndAcquireR::new(((self.bits >> 2) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 2 - Shortcut between event END and task ACQUIRE"]
    #[inline(always)]
    pub fn end_acquire(&mut self) -> EndAcquireW<ShortsSpec> {
        EndAcquireW::new(self, 2)
    }
}
#[doc = "Shortcuts between local events and tasks\n\nYou can [`read`](crate::Reg::read) this register and get [`shorts::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`shorts::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ShortsSpec;
impl crate::RegisterSpec for ShortsSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`shorts::R`](R) reader structure"]
impl crate::Readable for ShortsSpec {}
#[doc = "`write(|w| ..)` method takes [`shorts::W`](W) writer structure"]
impl crate::Writable for ShortsSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SHORTS to value 0"]
impl crate::Resettable for ShortsSpec {
    const RESET_VALUE: u32 = 0;
}
