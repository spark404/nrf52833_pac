#[doc = "Register `TXD` reader"]
pub type R = crate::R<TxdSpec>;
#[doc = "Register `TXD` writer"]
pub type W = crate::W<TxdSpec>;
#[doc = "Field `TXD` reader - TXD register"]
pub type TxdR = crate::FieldReader;
#[doc = "Field `TXD` writer - TXD register"]
pub type TxdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - TXD register"]
    #[inline(always)]
    pub fn txd(&self) -> TxdR {
        TxdR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - TXD register"]
    #[inline(always)]
    pub fn txd(&mut self) -> TxdW<TxdSpec> {
        TxdW::new(self, 0)
    }
}
#[doc = "TXD register\n\nYou can [`read`](crate::Reg::read) this register and get [`txd::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txd::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdSpec;
impl crate::RegisterSpec for TxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`txd::R`](R) reader structure"]
impl crate::Readable for TxdSpec {}
#[doc = "`write(|w| ..)` method takes [`txd::W`](W) writer structure"]
impl crate::Writable for TxdSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TXD to value 0"]
impl crate::Resettable for TxdSpec {
    const RESET_VALUE: u32 = 0;
}
