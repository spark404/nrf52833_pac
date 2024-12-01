#[doc = "Register `RXD` reader"]
pub type R = crate::R<RxdSpec>;
#[doc = "Field `RXD` reader - RX data received in previous transfers, double buffered"]
pub type RxdR = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - RX data received in previous transfers, double buffered"]
    #[inline(always)]
    pub fn rxd(&self) -> RxdR {
        RxdR::new((self.bits & 0xff) as u8)
    }
}
#[doc = "RXD register\n\nYou can [`read`](crate::Reg::read) this register and get [`rxd::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\n<div class=\"warning\">One or more dependent resources other than the current register are immediately affected by a read operation.</div>"]
pub struct RxdSpec;
impl crate::RegisterSpec for RxdSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rxd::R`](R) reader structure"]
impl crate::Readable for RxdSpec {}
#[doc = "`reset()` method sets RXD to value 0"]
impl crate::Resettable for RxdSpec {
    const RESET_VALUE: u32 = 0;
}
