#[doc = "Register `B3` reader"]
pub type R = crate::R<B3Spec>;
#[doc = "Register `B3` writer"]
pub type W = crate::W<B3Spec>;
#[doc = "Field `B3` reader - y-intercept of fourth piecewise linear function"]
pub type B3R = crate::FieldReader<u16>;
#[doc = "Field `B3` writer - y-intercept of fourth piecewise linear function"]
pub type B3W<'a, REG> = crate::FieldWriter<'a, REG, 14, u16>;
impl R {
    #[doc = "Bits 0:13 - y-intercept of fourth piecewise linear function"]
    #[inline(always)]
    pub fn b3(&self) -> B3R {
        B3R::new((self.bits & 0x3fff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:13 - y-intercept of fourth piecewise linear function"]
    #[inline(always)]
    pub fn b3(&mut self) -> B3W<B3Spec> {
        B3W::new(self, 0)
    }
}
#[doc = "y-intercept of fourth piecewise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`b3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`b3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct B3Spec;
impl crate::RegisterSpec for B3Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`b3::R`](R) reader structure"]
impl crate::Readable for B3Spec {}
#[doc = "`write(|w| ..)` method takes [`b3::W`](W) writer structure"]
impl crate::Writable for B3Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets B3 to value 0x12"]
impl crate::Resettable for B3Spec {
    const RESET_VALUE: u32 = 0x12;
}
