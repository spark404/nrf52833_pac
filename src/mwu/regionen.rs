#[doc = "Register `REGIONEN` reader"]
pub type R = crate::R<RegionenSpec>;
#[doc = "Register `REGIONEN` writer"]
pub type W = crate::W<RegionenSpec>;
#[doc = "Enable/disable write access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn0wa {
    #[doc = "0: Disable write access watch in this region"]
    Disable = 0,
    #[doc = "1: Enable write access watch in this region"]
    Enable = 1,
}
impl From<Rgn0wa> for bool {
    #[inline(always)]
    fn from(variant: Rgn0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN0WA` reader - Enable/disable write access watch in region\\[0\\]"]
pub type Rgn0waR = crate::BitReader<Rgn0wa>;
impl Rgn0waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn0wa {
        match self.bits {
            false => Rgn0wa::Disable,
            true => Rgn0wa::Enable,
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rgn0wa::Disable
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rgn0wa::Enable
    }
}
#[doc = "Field `RGN0WA` writer - Enable/disable write access watch in region\\[0\\]"]
pub type Rgn0waW<'a, REG> = crate::BitWriter<'a, REG, Rgn0wa>;
impl<'a, REG> Rgn0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn0wa::Disable)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn0wa::Enable)
    }
}
#[doc = "Enable/disable read access watch in region\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn0ra {
    #[doc = "0: Disable read access watch in this region"]
    Disable = 0,
    #[doc = "1: Enable read access watch in this region"]
    Enable = 1,
}
impl From<Rgn0ra> for bool {
    #[inline(always)]
    fn from(variant: Rgn0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN0RA` reader - Enable/disable read access watch in region\\[0\\]"]
pub type Rgn0raR = crate::BitReader<Rgn0ra>;
impl Rgn0raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn0ra {
        match self.bits {
            false => Rgn0ra::Disable,
            true => Rgn0ra::Enable,
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rgn0ra::Disable
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rgn0ra::Enable
    }
}
#[doc = "Field `RGN0RA` writer - Enable/disable read access watch in region\\[0\\]"]
pub type Rgn0raW<'a, REG> = crate::BitWriter<'a, REG, Rgn0ra>;
impl<'a, REG> Rgn0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn0ra::Disable)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn0ra::Enable)
    }
}
#[doc = "Enable/disable write access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn1wa {
    #[doc = "0: Disable write access watch in this region"]
    Disable = 0,
    #[doc = "1: Enable write access watch in this region"]
    Enable = 1,
}
impl From<Rgn1wa> for bool {
    #[inline(always)]
    fn from(variant: Rgn1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN1WA` reader - Enable/disable write access watch in region\\[1\\]"]
pub type Rgn1waR = crate::BitReader<Rgn1wa>;
impl Rgn1waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn1wa {
        match self.bits {
            false => Rgn1wa::Disable,
            true => Rgn1wa::Enable,
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rgn1wa::Disable
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rgn1wa::Enable
    }
}
#[doc = "Field `RGN1WA` writer - Enable/disable write access watch in region\\[1\\]"]
pub type Rgn1waW<'a, REG> = crate::BitWriter<'a, REG, Rgn1wa>;
impl<'a, REG> Rgn1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn1wa::Disable)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn1wa::Enable)
    }
}
#[doc = "Enable/disable read access watch in region\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn1ra {
    #[doc = "0: Disable read access watch in this region"]
    Disable = 0,
    #[doc = "1: Enable read access watch in this region"]
    Enable = 1,
}
impl From<Rgn1ra> for bool {
    #[inline(always)]
    fn from(variant: Rgn1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN1RA` reader - Enable/disable read access watch in region\\[1\\]"]
pub type Rgn1raR = crate::BitReader<Rgn1ra>;
impl Rgn1raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn1ra {
        match self.bits {
            false => Rgn1ra::Disable,
            true => Rgn1ra::Enable,
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rgn1ra::Disable
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rgn1ra::Enable
    }
}
#[doc = "Field `RGN1RA` writer - Enable/disable read access watch in region\\[1\\]"]
pub type Rgn1raW<'a, REG> = crate::BitWriter<'a, REG, Rgn1ra>;
impl<'a, REG> Rgn1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn1ra::Disable)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn1ra::Enable)
    }
}
#[doc = "Enable/disable write access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn2wa {
    #[doc = "0: Disable write access watch in this region"]
    Disable = 0,
    #[doc = "1: Enable write access watch in this region"]
    Enable = 1,
}
impl From<Rgn2wa> for bool {
    #[inline(always)]
    fn from(variant: Rgn2wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN2WA` reader - Enable/disable write access watch in region\\[2\\]"]
pub type Rgn2waR = crate::BitReader<Rgn2wa>;
impl Rgn2waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn2wa {
        match self.bits {
            false => Rgn2wa::Disable,
            true => Rgn2wa::Enable,
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rgn2wa::Disable
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rgn2wa::Enable
    }
}
#[doc = "Field `RGN2WA` writer - Enable/disable write access watch in region\\[2\\]"]
pub type Rgn2waW<'a, REG> = crate::BitWriter<'a, REG, Rgn2wa>;
impl<'a, REG> Rgn2waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn2wa::Disable)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn2wa::Enable)
    }
}
#[doc = "Enable/disable read access watch in region\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn2ra {
    #[doc = "0: Disable read access watch in this region"]
    Disable = 0,
    #[doc = "1: Enable read access watch in this region"]
    Enable = 1,
}
impl From<Rgn2ra> for bool {
    #[inline(always)]
    fn from(variant: Rgn2ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN2RA` reader - Enable/disable read access watch in region\\[2\\]"]
pub type Rgn2raR = crate::BitReader<Rgn2ra>;
impl Rgn2raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn2ra {
        match self.bits {
            false => Rgn2ra::Disable,
            true => Rgn2ra::Enable,
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rgn2ra::Disable
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rgn2ra::Enable
    }
}
#[doc = "Field `RGN2RA` writer - Enable/disable read access watch in region\\[2\\]"]
pub type Rgn2raW<'a, REG> = crate::BitWriter<'a, REG, Rgn2ra>;
impl<'a, REG> Rgn2raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn2ra::Disable)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn2ra::Enable)
    }
}
#[doc = "Enable/disable write access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn3wa {
    #[doc = "0: Disable write access watch in this region"]
    Disable = 0,
    #[doc = "1: Enable write access watch in this region"]
    Enable = 1,
}
impl From<Rgn3wa> for bool {
    #[inline(always)]
    fn from(variant: Rgn3wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN3WA` reader - Enable/disable write access watch in region\\[3\\]"]
pub type Rgn3waR = crate::BitReader<Rgn3wa>;
impl Rgn3waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn3wa {
        match self.bits {
            false => Rgn3wa::Disable,
            true => Rgn3wa::Enable,
        }
    }
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rgn3wa::Disable
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rgn3wa::Enable
    }
}
#[doc = "Field `RGN3WA` writer - Enable/disable write access watch in region\\[3\\]"]
pub type Rgn3waW<'a, REG> = crate::BitWriter<'a, REG, Rgn3wa>;
impl<'a, REG> Rgn3waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable write access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn3wa::Disable)
    }
    #[doc = "Enable write access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn3wa::Enable)
    }
}
#[doc = "Enable/disable read access watch in region\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rgn3ra {
    #[doc = "0: Disable read access watch in this region"]
    Disable = 0,
    #[doc = "1: Enable read access watch in this region"]
    Enable = 1,
}
impl From<Rgn3ra> for bool {
    #[inline(always)]
    fn from(variant: Rgn3ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RGN3RA` reader - Enable/disable read access watch in region\\[3\\]"]
pub type Rgn3raR = crate::BitReader<Rgn3ra>;
impl Rgn3raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rgn3ra {
        match self.bits {
            false => Rgn3ra::Disable,
            true => Rgn3ra::Enable,
        }
    }
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Rgn3ra::Disable
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Rgn3ra::Enable
    }
}
#[doc = "Field `RGN3RA` writer - Enable/disable read access watch in region\\[3\\]"]
pub type Rgn3raW<'a, REG> = crate::BitWriter<'a, REG, Rgn3ra>;
impl<'a, REG> Rgn3raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable read access watch in this region"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn3ra::Disable)
    }
    #[doc = "Enable read access watch in this region"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Rgn3ra::Enable)
    }
}
#[doc = "Enable/disable write access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn0wa {
    #[doc = "0: Disable write access watch in this PREGION"]
    Disable = 0,
    #[doc = "1: Enable write access watch in this PREGION"]
    Enable = 1,
}
impl From<Prgn0wa> for bool {
    #[inline(always)]
    fn from(variant: Prgn0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN0WA` reader - Enable/disable write access watch in PREGION\\[0\\]"]
pub type Prgn0waR = crate::BitReader<Prgn0wa>;
impl Prgn0waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prgn0wa {
        match self.bits {
            false => Prgn0wa::Disable,
            true => Prgn0wa::Enable,
        }
    }
    #[doc = "Disable write access watch in this PREGION"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prgn0wa::Disable
    }
    #[doc = "Enable write access watch in this PREGION"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Prgn0wa::Enable
    }
}
#[doc = "Field `PRGN0WA` writer - Enable/disable write access watch in PREGION\\[0\\]"]
pub type Prgn0waW<'a, REG> = crate::BitWriter<'a, REG, Prgn0wa>;
impl<'a, REG> Prgn0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable write access watch in this PREGION"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn0wa::Disable)
    }
    #[doc = "Enable write access watch in this PREGION"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn0wa::Enable)
    }
}
#[doc = "Enable/disable read access watch in PREGION\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn0ra {
    #[doc = "0: Disable read access watch in this PREGION"]
    Disable = 0,
    #[doc = "1: Enable read access watch in this PREGION"]
    Enable = 1,
}
impl From<Prgn0ra> for bool {
    #[inline(always)]
    fn from(variant: Prgn0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN0RA` reader - Enable/disable read access watch in PREGION\\[0\\]"]
pub type Prgn0raR = crate::BitReader<Prgn0ra>;
impl Prgn0raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prgn0ra {
        match self.bits {
            false => Prgn0ra::Disable,
            true => Prgn0ra::Enable,
        }
    }
    #[doc = "Disable read access watch in this PREGION"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prgn0ra::Disable
    }
    #[doc = "Enable read access watch in this PREGION"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Prgn0ra::Enable
    }
}
#[doc = "Field `PRGN0RA` writer - Enable/disable read access watch in PREGION\\[0\\]"]
pub type Prgn0raW<'a, REG> = crate::BitWriter<'a, REG, Prgn0ra>;
impl<'a, REG> Prgn0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable read access watch in this PREGION"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn0ra::Disable)
    }
    #[doc = "Enable read access watch in this PREGION"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn0ra::Enable)
    }
}
#[doc = "Enable/disable write access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn1wa {
    #[doc = "0: Disable write access watch in this PREGION"]
    Disable = 0,
    #[doc = "1: Enable write access watch in this PREGION"]
    Enable = 1,
}
impl From<Prgn1wa> for bool {
    #[inline(always)]
    fn from(variant: Prgn1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN1WA` reader - Enable/disable write access watch in PREGION\\[1\\]"]
pub type Prgn1waR = crate::BitReader<Prgn1wa>;
impl Prgn1waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prgn1wa {
        match self.bits {
            false => Prgn1wa::Disable,
            true => Prgn1wa::Enable,
        }
    }
    #[doc = "Disable write access watch in this PREGION"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prgn1wa::Disable
    }
    #[doc = "Enable write access watch in this PREGION"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Prgn1wa::Enable
    }
}
#[doc = "Field `PRGN1WA` writer - Enable/disable write access watch in PREGION\\[1\\]"]
pub type Prgn1waW<'a, REG> = crate::BitWriter<'a, REG, Prgn1wa>;
impl<'a, REG> Prgn1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable write access watch in this PREGION"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn1wa::Disable)
    }
    #[doc = "Enable write access watch in this PREGION"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn1wa::Enable)
    }
}
#[doc = "Enable/disable read access watch in PREGION\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Prgn1ra {
    #[doc = "0: Disable read access watch in this PREGION"]
    Disable = 0,
    #[doc = "1: Enable read access watch in this PREGION"]
    Enable = 1,
}
impl From<Prgn1ra> for bool {
    #[inline(always)]
    fn from(variant: Prgn1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PRGN1RA` reader - Enable/disable read access watch in PREGION\\[1\\]"]
pub type Prgn1raR = crate::BitReader<Prgn1ra>;
impl Prgn1raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Prgn1ra {
        match self.bits {
            false => Prgn1ra::Disable,
            true => Prgn1ra::Enable,
        }
    }
    #[doc = "Disable read access watch in this PREGION"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Prgn1ra::Disable
    }
    #[doc = "Enable read access watch in this PREGION"]
    #[inline(always)]
    pub fn is_enable(&self) -> bool {
        *self == Prgn1ra::Enable
    }
}
#[doc = "Field `PRGN1RA` writer - Enable/disable read access watch in PREGION\\[1\\]"]
pub type Prgn1raW<'a, REG> = crate::BitWriter<'a, REG, Prgn1ra>;
impl<'a, REG> Prgn1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable read access watch in this PREGION"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn1ra::Disable)
    }
    #[doc = "Enable read access watch in this PREGION"]
    #[inline(always)]
    pub fn enable(self) -> &'a mut crate::W<REG> {
        self.variant(Prgn1ra::Enable)
    }
}
impl R {
    #[doc = "Bit 0 - Enable/disable write access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0wa(&self) -> Rgn0waR {
        Rgn0waR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable/disable read access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0ra(&self) -> Rgn0raR {
        Rgn0raR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable/disable write access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1wa(&self) -> Rgn1waR {
        Rgn1waR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable/disable read access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1ra(&self) -> Rgn1raR {
        Rgn1raR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable/disable write access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2wa(&self) -> Rgn2waR {
        Rgn2waR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable/disable read access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2ra(&self) -> Rgn2raR {
        Rgn2raR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable/disable write access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3wa(&self) -> Rgn3waR {
        Rgn3waR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable/disable read access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3ra(&self) -> Rgn3raR {
        Rgn3raR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable/disable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0wa(&self) -> Prgn0waR {
        Prgn0waR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Enable/disable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0ra(&self) -> Prgn0raR {
        Prgn0raR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Enable/disable write access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1wa(&self) -> Prgn1waR {
        Prgn1waR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Enable/disable read access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1ra(&self) -> Prgn1raR {
        Prgn1raR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable/disable write access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0wa(&mut self) -> Rgn0waW<RegionenSpec> {
        Rgn0waW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable/disable read access watch in region\\[0\\]"]
    #[inline(always)]
    pub fn rgn0ra(&mut self) -> Rgn0raW<RegionenSpec> {
        Rgn0raW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable/disable write access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1wa(&mut self) -> Rgn1waW<RegionenSpec> {
        Rgn1waW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable/disable read access watch in region\\[1\\]"]
    #[inline(always)]
    pub fn rgn1ra(&mut self) -> Rgn1raW<RegionenSpec> {
        Rgn1raW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable/disable write access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2wa(&mut self) -> Rgn2waW<RegionenSpec> {
        Rgn2waW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable/disable read access watch in region\\[2\\]"]
    #[inline(always)]
    pub fn rgn2ra(&mut self) -> Rgn2raW<RegionenSpec> {
        Rgn2raW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable/disable write access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3wa(&mut self) -> Rgn3waW<RegionenSpec> {
        Rgn3waW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable/disable read access watch in region\\[3\\]"]
    #[inline(always)]
    pub fn rgn3ra(&mut self) -> Rgn3raW<RegionenSpec> {
        Rgn3raW::new(self, 7)
    }
    #[doc = "Bit 24 - Enable/disable write access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0wa(&mut self) -> Prgn0waW<RegionenSpec> {
        Prgn0waW::new(self, 24)
    }
    #[doc = "Bit 25 - Enable/disable read access watch in PREGION\\[0\\]"]
    #[inline(always)]
    pub fn prgn0ra(&mut self) -> Prgn0raW<RegionenSpec> {
        Prgn0raW::new(self, 25)
    }
    #[doc = "Bit 26 - Enable/disable write access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1wa(&mut self) -> Prgn1waW<RegionenSpec> {
        Prgn1waW::new(self, 26)
    }
    #[doc = "Bit 27 - Enable/disable read access watch in PREGION\\[1\\]"]
    #[inline(always)]
    pub fn prgn1ra(&mut self) -> Prgn1raW<RegionenSpec> {
        Prgn1raW::new(self, 27)
    }
}
#[doc = "Enable/disable regions watch\n\nYou can [`read`](crate::Reg::read) this register and get [`regionen::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regionen::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RegionenSpec;
impl crate::RegisterSpec for RegionenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regionen::R`](R) reader structure"]
impl crate::Readable for RegionenSpec {}
#[doc = "`write(|w| ..)` method takes [`regionen::W`](W) writer structure"]
impl crate::Writable for RegionenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets REGIONEN to value 0"]
impl crate::Resettable for RegionenSpec {
    const RESET_VALUE: u32 = 0;
}
