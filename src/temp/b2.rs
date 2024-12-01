#[doc = "Register `B2` reader"]
pub type R = crate::R<B2Spec>;
#[doc = "Register `B2` writer"]
pub type W = crate::W<B2Spec>;
#[doc = "Field `B2` reader - y-intercept of third piecewise linear function"]
pub type B2R = crate::FieldReader<u16>;
#[doc = "Field `B2` writer - y-intercept of third piecewise linear function"]
pub type B2W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - y-intercept of third piecewise linear function"]
    #[inline(always)]
    pub fn b2(&self) -> B2R {
        B2R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of third piecewise linear function"]
    #[inline(always)]
    pub fn b2(&mut self) -> B2W<B2Spec> {
        B2W::new(self, 0)
    }
}
#[doc = "y-intercept of third piecewise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B2Spec;
impl crate::RegisterSpec for B2Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b2::R`](R) reader structure"]
impl crate::Readable for B2Spec {}
#[doc = "`write(|w| ..)` method takes [`b2::W`](W) writer structure"]
impl crate::Writable for B2Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B2 to value 0x3fbe"]
impl crate::Resettable for B2Spec {
    const RESET_VALUE: u32 = 0x3fbe;
}
