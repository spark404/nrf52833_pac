#[doc = "Register `A1` reader"]
pub type R = crate::R<A1Spec>;
#[doc = "Register `A1` writer"]
pub type W = crate::W<A1Spec>;
#[doc = "Field `A1` reader - Slope of second piecewise linear function"]
pub type A1R = crate::FieldReader<u16>;
#[doc = "Field `A1` writer - Slope of second piecewise linear function"]
pub type A1W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
impl R {
    #[doc = "Bits 0:11 - Slope of second piecewise linear function"]
    #[inline(always)]
    pub fn a1(&self) -> A1R {
        A1R::new((self.bits & 0x0fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:11 - Slope of second piecewise linear function"]
    #[inline(always)]
    pub fn a1(&mut self) -> A1W<A1Spec> {
        A1W::new(self, 0)
    }
}
#[doc = "Slope of second piecewise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`a1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`a1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct A1Spec;
impl crate::RegisterSpec for A1Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`a1::R`](R) reader structure"]
impl crate::Readable for A1Spec {}
#[doc = "`write(|w| ..)` method takes [`a1::W`](W) writer structure"]
impl crate::Writable for A1Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets A1 to value 0x0348"]
impl crate::Resettable for A1Spec {
    const RESET_VALUE: u32 = 0x0348;
}
