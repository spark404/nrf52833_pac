#[doc = "Register `APPROTECT` reader"]
pub type R = crate::R<ApprotectSpec>;
#[doc = "Register `APPROTECT` writer"]
pub type W = crate::W<ApprotectSpec>;
#[doc = "Enable or disable access port protection.\n\nValue on reset: 255"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Pall {
    #[doc = "255: Hardware disable of access port protection for devices where access port protection is controlled by hardware"]
    Disabled = 255,
    #[doc = "90: Hardware disable of access port protection for devices where access port protection is controlled by hardware and software"]
    HwDisabled = 90,
    #[doc = "0: Enable"]
    Enabled = 0,
}
impl From<Pall> for u8 {
    #[inline(always)]
    fn from(variant: Pall) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Pall {
    type Ux = u8;
}
impl crate::IsEnum for Pall {}
#[doc = "Field `PALL` reader - Enable or disable access port protection."]
pub type PallR = crate::FieldReader<Pall>;
impl PallR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Pall> {
        match self.bits {
            255 => Some(Pall::Disabled),
            90 => Some(Pall::HwDisabled),
            0 => Some(Pall::Enabled),
            _ => None,
        }
    }
    #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pall::Disabled
    }
    #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware and software"]
    #[inline(always)]
    pub fn is_hw_disabled(&self) -> bool {
        *self == Pall::HwDisabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pall::Enabled
    }
}
#[doc = "Field `PALL` writer - Enable or disable access port protection."]
pub type PallW<'a, REG> = crate::FieldWriter<'a, REG, 8, Pall>;
impl<'a, REG> PallW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Disabled)
    }
    #[doc = "Hardware disable of access port protection for devices where access port protection is controlled by hardware and software"]
    #[inline(always)]
    pub fn hw_disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::HwDisabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Pall::Enabled)
    }
}
impl R {
    #[doc = "Bits 0:7 - Enable or disable access port protection."]
    #[inline(always)]
    pub fn pall(&self) -> PallR {
        PallR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Enable or disable access port protection."]
    #[inline(always)]
    pub fn pall(&mut self) -> PallW<ApprotectSpec> {
        PallW::new(self, 0)
    }
}
#[doc = "Access port protection\n\nYou can [`read`](crate::Reg::read) this register and get [`approtect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`approtect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ApprotectSpec;
impl crate::RegisterSpec for ApprotectSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`approtect::R`](R) reader structure"]
impl crate::Readable for ApprotectSpec {}
#[doc = "`write(|w| ..)` method takes [`approtect::W`](W) writer structure"]
impl crate::Writable for ApprotectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APPROTECT to value 0xffff_ffff"]
impl crate::Resettable for ApprotectSpec {
    const RESET_VALUE: u32 = 0xffff_ffff;
}
