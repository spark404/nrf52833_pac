#[doc = "Register `PREFIX0` reader"]
pub type R = crate::R<Prefix0Spec>;
#[doc = "Register `PREFIX0` writer"]
pub type W = crate::W<Prefix0Spec>;
#[doc = "Field `AP0` reader - Address prefix 0."]
pub type Ap0R = crate::FieldReader;
#[doc = "Field `AP0` writer - Address prefix 0."]
pub type Ap0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AP1` reader - Address prefix 1."]
pub type Ap1R = crate::FieldReader;
#[doc = "Field `AP1` writer - Address prefix 1."]
pub type Ap1W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AP2` reader - Address prefix 2."]
pub type Ap2R = crate::FieldReader;
#[doc = "Field `AP2` writer - Address prefix 2."]
pub type Ap2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `AP3` reader - Address prefix 3."]
pub type Ap3R = crate::FieldReader;
#[doc = "Field `AP3` writer - Address prefix 3."]
pub type Ap3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Address prefix 0."]
    #[inline(always)]
    pub fn ap0(&self) -> Ap0R {
        Ap0R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Address prefix 1."]
    #[inline(always)]
    pub fn ap1(&self) -> Ap1R {
        Ap1R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Address prefix 2."]
    #[inline(always)]
    pub fn ap2(&self) -> Ap2R {
        Ap2R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - Address prefix 3."]
    #[inline(always)]
    pub fn ap3(&self) -> Ap3R {
        Ap3R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Address prefix 0."]
    #[inline(always)]
    pub fn ap0(&mut self) -> Ap0W<Prefix0Spec> {
        Ap0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Address prefix 1."]
    #[inline(always)]
    pub fn ap1(&mut self) -> Ap1W<Prefix0Spec> {
        Ap1W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Address prefix 2."]
    #[inline(always)]
    pub fn ap2(&mut self) -> Ap2W<Prefix0Spec> {
        Ap2W::new(self, 16)
    }
    #[doc = "Bits 24:31 - Address prefix 3."]
    #[inline(always)]
    pub fn ap3(&mut self) -> Ap3W<Prefix0Spec> {
        Ap3W::new(self, 24)
    }
}
#[doc = "Prefixes bytes for logical addresses 0-3\n\nYou can [`read`](crate::Reg::read) this register and get [`prefix0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`prefix0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Prefix0Spec;
impl crate::RegisterSpec for Prefix0Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`prefix0::R`](R) reader structure"]
impl crate::Readable for Prefix0Spec {}
#[doc = "`write(|w| ..)` method takes [`prefix0::W`](W) writer structure"]
impl crate::Writable for Prefix0Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PREFIX0 to value 0"]
impl crate::Resettable for Prefix0Spec {
    const RESET_VALUE: u32 = 0;
}
