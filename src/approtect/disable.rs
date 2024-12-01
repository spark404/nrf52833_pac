#[doc = "Register `DISABLE` reader"]
pub type R = crate::R<DisableSpec>;
#[doc = "Register `DISABLE` writer"]
pub type W = crate::W<DisableSpec>;
#[doc = "Software disable APPROTECT mechanism\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Disable {
    #[doc = "90: Software disable APPROTECT mechanism"]
    SwDisable = 90,
}
impl From<Disable> for u8 {
    #[inline(always)]
    fn from(variant: Disable) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Disable {
    type Ux = u8;
}
impl crate::IsEnum for Disable {}
#[doc = "Field `DISABLE` reader - Software disable APPROTECT mechanism"]
pub type DisableR = crate::FieldReader<Disable>;
impl DisableR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Disable> {
        match self.bits {
            90 => Some(Disable::SwDisable),
            _ => None,
        }
    }
    #[doc = "Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn is_sw_disable(&self) -> bool {
        *self == Disable::SwDisable
    }
}
#[doc = "Field `DISABLE` writer - Software disable APPROTECT mechanism"]
pub type DisableW<'a, REG> = crate::FieldWriter<'a, REG, 8, Disable>;
impl<'a, REG> DisableW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn sw_disable(self) -> &'a mut crate::W<REG> {
        self.variant(Disable::SwDisable)
    }
}
impl R {
    #[doc = "Bits 0:7 - Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&self) -> DisableR {
        DisableR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Software disable APPROTECT mechanism"]
    #[inline(always)]
    pub fn disable(&mut self) -> DisableW<DisableSpec> {
        DisableW::new(self, 0)
    }
}
#[doc = "Software disable APPROTECT mechanism\n\nYou can [`read`](crate::Reg::read) this register and get [`disable::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`disable::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DisableSpec;
impl crate::RegisterSpec for DisableSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`disable::R`](R) reader structure"]
impl crate::Readable for DisableSpec {}
#[doc = "`write(|w| ..)` method takes [`disable::W`](W) writer structure"]
impl crate::Writable for DisableSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DISABLE to value 0"]
impl crate::Resettable for DisableSpec {
    const RESET_VALUE: u32 = 0;
}
