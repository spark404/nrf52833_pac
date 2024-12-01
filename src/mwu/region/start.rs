#[doc = "Register `START` reader"]
pub type R = crate::R<StartSpec>;
#[doc = "Register `START` writer"]
pub type W = crate::W<StartSpec>;
#[doc = "Field `START` reader - Start address for region"]
pub type StartR = crate::FieldReader<u32>;
#[doc = "Field `START` writer - Start address for region"]
pub type StartW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Start address for region"]
    #[inline(always)]
    pub fn start(&self) -> StartR {
        StartR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Start address for region"]
    #[inline(always)]
    pub fn start(&mut self) -> StartW<StartSpec> {
        StartW::new(self, 0)
    }
}
#[doc = "Description cluster: Start address for region n\n\nYou can [`read`](crate::Reg::read) this register and get [`start::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`start::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct StartSpec;
impl crate::RegisterSpec for StartSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`start::R`](R) reader structure"]
impl crate::Readable for StartSpec {}
#[doc = "`write(|w| ..)` method takes [`start::W`](W) writer structure"]
impl crate::Writable for StartSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets START to value 0"]
impl crate::Resettable for StartSpec {
    const RESET_VALUE: u32 = 0;
}
