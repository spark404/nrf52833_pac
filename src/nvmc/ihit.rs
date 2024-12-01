#[doc = "Register `IHIT` reader"]
pub type R = crate::R<IhitSpec>;
#[doc = "Register `IHIT` writer"]
pub type W = crate::W<IhitSpec>;
#[doc = "Field `HITS` reader - Number of cache hits. Register is writable, but only to '0'."]
pub type HitsR = crate::FieldReader<u32>;
#[doc = "Field `HITS` writer - Number of cache hits. Register is writable, but only to '0'."]
pub type HitsW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - Number of cache hits. Register is writable, but only to '0'."]
    #[inline(always)]
    pub fn hits(&self) -> HitsR {
        HitsR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - Number of cache hits. Register is writable, but only to '0'."]
    #[inline(always)]
    pub fn hits(&mut self) -> HitsW<IhitSpec> {
        HitsW::new(self, 0)
    }
}
#[doc = "I-code cache hit counter\n\nYou can [`read`](crate::Reg::read) this register and get [`ihit::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ihit::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IhitSpec;
impl crate::RegisterSpec for IhitSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ihit::R`](R) reader structure"]
impl crate::Readable for IhitSpec {}
#[doc = "`write(|w| ..)` method takes [`ihit::W`](W) writer structure"]
impl crate::Writable for IhitSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IHIT to value 0"]
impl crate::Resettable for IhitSpec {
    const RESET_VALUE: u32 = 0;
}
