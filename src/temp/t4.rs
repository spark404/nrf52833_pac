#[doc = "Register `T4` reader"]
pub type R = crate::R<T4Spec>;
#[doc = "Register `T4` writer"]
pub type W = crate::W<T4Spec>;
#[doc = "Field `T4` reader - End point of fifth piecewise linear function"]
pub type T4R = crate::FieldReader;
#[doc = "Field `T4` writer - End point of fifth piecewise linear function"]
pub type T4W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - End point of fifth piecewise linear function"]
    #[inline(always)]
    pub fn t4(&self) -> T4R {
        T4R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - End point of fifth piecewise linear function"]
    #[inline(always)]
    pub fn t4(&mut self) -> T4W<T4Spec> {
        T4W::new(self, 0)
    }
}
#[doc = "End point of fifth piecewise linear function\n\nYou can [`read`](crate::Reg::read) this register and get [`t4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`t4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T4Spec;
impl crate::RegisterSpec for T4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t4::R`](R) reader structure"]
impl crate::Readable for T4Spec {}
#[doc = "`write(|w| ..)` method takes [`t4::W`](W) writer structure"]
impl crate::Writable for T4Spec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets T4 to value 0x50"]
impl crate::Resettable for T4Spec {
    const RESET_VALUE: u32 = 0x50;
}
