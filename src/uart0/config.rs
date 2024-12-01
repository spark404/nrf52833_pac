#[doc = "Register `CONFIG` reader"]
pub type R = crate::R<ConfigSpec>;
#[doc = "Register `CONFIG` writer"]
pub type W = crate::W<ConfigSpec>;
#[doc = "Hardware flow control\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Hwfc {
    #[doc = "0: Disabled"]
    Disabled = 0,
    #[doc = "1: Enabled"]
    Enabled = 1,
}
impl From<Hwfc> for bool {
    #[inline(always)]
    fn from(variant: Hwfc) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `HWFC` reader - Hardware flow control"]
pub type HwfcR = crate::BitReader<Hwfc>;
impl HwfcR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Hwfc {
        match self.bits {
            false => Hwfc::Disabled,
            true => Hwfc::Enabled,
        }
    }
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Hwfc::Disabled
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Hwfc::Enabled
    }
}
#[doc = "Field `HWFC` writer - Hardware flow control"]
pub type HwfcW<'a, REG> = crate::BitWriter<'a, REG, Hwfc>;
impl<'a, REG> HwfcW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disabled"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwfc::Disabled)
    }
    #[doc = "Enabled"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Hwfc::Enabled)
    }
}
#[doc = "Parity\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Parity {
    #[doc = "0: Exclude parity bit"]
    Excluded = 0,
    #[doc = "7: Include parity bit"]
    Included = 7,
}
impl From<Parity> for u8 {
    #[inline(always)]
    fn from(variant: Parity) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Parity {
    type Ux = u8;
}
impl crate::IsEnum for Parity {}
#[doc = "Field `PARITY` reader - Parity"]
pub type ParityR = crate::FieldReader<Parity>;
impl ParityR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Parity> {
        match self.bits {
            0 => Some(Parity::Excluded),
            7 => Some(Parity::Included),
            _ => None,
        }
    }
    #[doc = "Exclude parity bit"]
    #[inline(always)]
    pub fn is_excluded(&self) -> bool {
        *self == Parity::Excluded
    }
    #[doc = "Include parity bit"]
    #[inline(always)]
    pub fn is_included(&self) -> bool {
        *self == Parity::Included
    }
}
#[doc = "Field `PARITY` writer - Parity"]
pub type ParityW<'a, REG> = crate::FieldWriter<'a, REG, 3, Parity>;
impl<'a, REG> ParityW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Exclude parity bit"]
    #[inline(always)]
    pub fn excluded(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Excluded)
    }
    #[doc = "Include parity bit"]
    #[inline(always)]
    pub fn included(self) -> &'a mut crate::W<REG> {
        self.variant(Parity::Included)
    }
}
#[doc = "Stop bits\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stop {
    #[doc = "0: One stop bit"]
    One = 0,
    #[doc = "1: Two stop bits"]
    Two = 1,
}
impl From<Stop> for bool {
    #[inline(always)]
    fn from(variant: Stop) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOP` reader - Stop bits"]
pub type StopR = crate::BitReader<Stop>;
impl StopR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stop {
        match self.bits {
            false => Stop::One,
            true => Stop::Two,
        }
    }
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn is_one(&self) -> bool {
        *self == Stop::One
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn is_two(&self) -> bool {
        *self == Stop::Two
    }
}
#[doc = "Field `STOP` writer - Stop bits"]
pub type StopW<'a, REG> = crate::BitWriter<'a, REG, Stop>;
impl<'a, REG> StopW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "One stop bit"]
    #[inline(always)]
    pub fn one(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::One)
    }
    #[doc = "Two stop bits"]
    #[inline(always)]
    pub fn two(self) -> &'a mut crate::W<REG> {
        self.variant(Stop::Two)
    }
}
#[doc = "Even or odd parity type\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Paritytype {
    #[doc = "0: Even parity"]
    Even = 0,
    #[doc = "1: Odd parity"]
    Odd = 1,
}
impl From<Paritytype> for bool {
    #[inline(always)]
    fn from(variant: Paritytype) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PARITYTYPE` reader - Even or odd parity type"]
pub type ParitytypeR = crate::BitReader<Paritytype>;
impl ParitytypeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Paritytype {
        match self.bits {
            false => Paritytype::Even,
            true => Paritytype::Odd,
        }
    }
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn is_even(&self) -> bool {
        *self == Paritytype::Even
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn is_odd(&self) -> bool {
        *self == Paritytype::Odd
    }
}
#[doc = "Field `PARITYTYPE` writer - Even or odd parity type"]
pub type ParitytypeW<'a, REG> = crate::BitWriter<'a, REG, Paritytype>;
impl<'a, REG> ParitytypeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Even parity"]
    #[inline(always)]
    pub fn even(self) -> &'a mut crate::W<REG> {
        self.variant(Paritytype::Even)
    }
    #[doc = "Odd parity"]
    #[inline(always)]
    pub fn odd(self) -> &'a mut crate::W<REG> {
        self.variant(Paritytype::Odd)
    }
}
impl R {
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    pub fn hwfc(&self) -> HwfcR {
        HwfcR::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    pub fn parity(&self) -> ParityR {
        ParityR::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - Stop bits"]
    #[inline(always)]
    pub fn stop(&self) -> StopR {
        StopR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 8 - Even or odd parity type"]
    #[inline(always)]
    pub fn paritytype(&self) -> ParitytypeR {
        ParitytypeR::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Hardware flow control"]
    #[inline(always)]
    pub fn hwfc(&mut self) -> HwfcW<ConfigSpec> {
        HwfcW::new(self, 0)
    }
    #[doc = "Bits 1:3 - Parity"]
    #[inline(always)]
    pub fn parity(&mut self) -> ParityW<ConfigSpec> {
        ParityW::new(self, 1)
    }
    #[doc = "Bit 4 - Stop bits"]
    #[inline(always)]
    pub fn stop(&mut self) -> StopW<ConfigSpec> {
        StopW::new(self, 4)
    }
    #[doc = "Bit 8 - Even or odd parity type"]
    #[inline(always)]
    pub fn paritytype(&mut self) -> ParitytypeW<ConfigSpec> {
        ParitytypeW::new(self, 8)
    }
}
#[doc = "Configuration of parity and hardware flow control\n\nYou can [`read`](crate::Reg::read) this register and get [`config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ConfigSpec;
impl crate::RegisterSpec for ConfigSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`config::R`](R) reader structure"]
impl crate::Readable for ConfigSpec {}
#[doc = "`write(|w| ..)` method takes [`config::W`](W) writer structure"]
impl crate::Writable for ConfigSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CONFIG to value 0"]
impl crate::Resettable for ConfigSpec {
    const RESET_VALUE: u32 = 0;
}
