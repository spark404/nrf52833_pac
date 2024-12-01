#[doc = "Register `TXD` writer"]
pub type W = crate::W<TxdSpec>;
#[doc = "Field `TXD` writer - TX data to be transferred"]
pub type TxdW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl W {
    #[doc = "Bits 0:7 - TX data to be transferred"]
    #[inline(always)]
    pub fn txd(&mut self) -> TxdW<TxdSpec> {
        TxdW::new(self, 0)
    }
}
#[doc = "TXD register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`txd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TxdSpec;
impl crate::RegisterSpec for TxdSpec {
    type Ux = u32;
}
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
