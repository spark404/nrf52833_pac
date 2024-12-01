#[doc = "Register `NMIENSET` reader"]
pub type R = crate::R<NmiensetSpec>;
#[doc = "Register `NMIENSET` writer"]
pub type W = crate::W<NmiensetSpec>;
#[doc = "Write '1' to enable interrupt for event REGION0WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region0wa> for bool {
    #[inline(always)]
    fn from(variant: Region0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0WA` reader - Write '1' to enable interrupt for event REGION0WA"]
pub type Region0waR = crate::BitReader<Region0wa>;
impl Region0waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region0wa {
        match self.bits {
            false => Region0wa::Disabled,
            true => Region0wa::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region0wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region0wa::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event REGION0WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0waWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Region0waWO> for bool {
    #[inline(always)]
    fn from(variant: Region0waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0WA` writer - Write '1' to enable interrupt for event REGION0WA"]
pub type Region0waW<'a, REG> = crate::BitWriter<'a, REG, Region0waWO>;
impl<'a, REG> Region0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Region0waWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event REGION0RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region0ra> for bool {
    #[inline(always)]
    fn from(variant: Region0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0RA` reader - Write '1' to enable interrupt for event REGION0RA"]
pub type Region0raR = crate::BitReader<Region0ra>;
impl Region0raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region0ra {
        match self.bits {
            false => Region0ra::Disabled,
            true => Region0ra::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region0ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region0ra::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event REGION0RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region0raWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Region0raWO> for bool {
    #[inline(always)]
    fn from(variant: Region0raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION0RA` writer - Write '1' to enable interrupt for event REGION0RA"]
pub type Region0raW<'a, REG> = crate::BitWriter<'a, REG, Region0raWO>;
impl<'a, REG> Region0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Region0raWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event REGION1WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region1wa> for bool {
    #[inline(always)]
    fn from(variant: Region1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1WA` reader - Write '1' to enable interrupt for event REGION1WA"]
pub type Region1waR = crate::BitReader<Region1wa>;
impl Region1waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region1wa {
        match self.bits {
            false => Region1wa::Disabled,
            true => Region1wa::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region1wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region1wa::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event REGION1WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1waWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Region1waWO> for bool {
    #[inline(always)]
    fn from(variant: Region1waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1WA` writer - Write '1' to enable interrupt for event REGION1WA"]
pub type Region1waW<'a, REG> = crate::BitWriter<'a, REG, Region1waWO>;
impl<'a, REG> Region1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Region1waWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event REGION1RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region1ra> for bool {
    #[inline(always)]
    fn from(variant: Region1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1RA` reader - Write '1' to enable interrupt for event REGION1RA"]
pub type Region1raR = crate::BitReader<Region1ra>;
impl Region1raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region1ra {
        match self.bits {
            false => Region1ra::Disabled,
            true => Region1ra::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region1ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region1ra::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event REGION1RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region1raWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Region1raWO> for bool {
    #[inline(always)]
    fn from(variant: Region1raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION1RA` writer - Write '1' to enable interrupt for event REGION1RA"]
pub type Region1raW<'a, REG> = crate::BitWriter<'a, REG, Region1raWO>;
impl<'a, REG> Region1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Region1raWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event REGION2WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region2wa> for bool {
    #[inline(always)]
    fn from(variant: Region2wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2WA` reader - Write '1' to enable interrupt for event REGION2WA"]
pub type Region2waR = crate::BitReader<Region2wa>;
impl Region2waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region2wa {
        match self.bits {
            false => Region2wa::Disabled,
            true => Region2wa::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region2wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region2wa::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event REGION2WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2waWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Region2waWO> for bool {
    #[inline(always)]
    fn from(variant: Region2waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2WA` writer - Write '1' to enable interrupt for event REGION2WA"]
pub type Region2waW<'a, REG> = crate::BitWriter<'a, REG, Region2waWO>;
impl<'a, REG> Region2waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Region2waWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event REGION2RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region2ra> for bool {
    #[inline(always)]
    fn from(variant: Region2ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2RA` reader - Write '1' to enable interrupt for event REGION2RA"]
pub type Region2raR = crate::BitReader<Region2ra>;
impl Region2raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region2ra {
        match self.bits {
            false => Region2ra::Disabled,
            true => Region2ra::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region2ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region2ra::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event REGION2RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region2raWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Region2raWO> for bool {
    #[inline(always)]
    fn from(variant: Region2raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION2RA` writer - Write '1' to enable interrupt for event REGION2RA"]
pub type Region2raW<'a, REG> = crate::BitWriter<'a, REG, Region2raWO>;
impl<'a, REG> Region2raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Region2raWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event REGION3WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region3wa> for bool {
    #[inline(always)]
    fn from(variant: Region3wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3WA` reader - Write '1' to enable interrupt for event REGION3WA"]
pub type Region3waR = crate::BitReader<Region3wa>;
impl Region3waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region3wa {
        match self.bits {
            false => Region3wa::Disabled,
            true => Region3wa::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region3wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region3wa::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event REGION3WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3waWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Region3waWO> for bool {
    #[inline(always)]
    fn from(variant: Region3waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3WA` writer - Write '1' to enable interrupt for event REGION3WA"]
pub type Region3waW<'a, REG> = crate::BitWriter<'a, REG, Region3waWO>;
impl<'a, REG> Region3waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Region3waWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event REGION3RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Region3ra> for bool {
    #[inline(always)]
    fn from(variant: Region3ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3RA` reader - Write '1' to enable interrupt for event REGION3RA"]
pub type Region3raR = crate::BitReader<Region3ra>;
impl Region3raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Region3ra {
        match self.bits {
            false => Region3ra::Disabled,
            true => Region3ra::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Region3ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Region3ra::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event REGION3RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Region3raWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Region3raWO> for bool {
    #[inline(always)]
    fn from(variant: Region3raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `REGION3RA` writer - Write '1' to enable interrupt for event REGION3RA"]
pub type Region3raW<'a, REG> = crate::BitWriter<'a, REG, Region3raWO>;
impl<'a, REG> Region3raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Region3raWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event PREGION0WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pregion0wa> for bool {
    #[inline(always)]
    fn from(variant: Pregion0wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0WA` reader - Write '1' to enable interrupt for event PREGION0WA"]
pub type Pregion0waR = crate::BitReader<Pregion0wa>;
impl Pregion0waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pregion0wa {
        match self.bits {
            false => Pregion0wa::Disabled,
            true => Pregion0wa::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion0wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion0wa::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event PREGION0WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0waWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Pregion0waWO> for bool {
    #[inline(always)]
    fn from(variant: Pregion0waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0WA` writer - Write '1' to enable interrupt for event PREGION0WA"]
pub type Pregion0waW<'a, REG> = crate::BitWriter<'a, REG, Pregion0waWO>;
impl<'a, REG> Pregion0waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion0waWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event PREGION0RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pregion0ra> for bool {
    #[inline(always)]
    fn from(variant: Pregion0ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0RA` reader - Write '1' to enable interrupt for event PREGION0RA"]
pub type Pregion0raR = crate::BitReader<Pregion0ra>;
impl Pregion0raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pregion0ra {
        match self.bits {
            false => Pregion0ra::Disabled,
            true => Pregion0ra::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion0ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion0ra::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event PREGION0RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion0raWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Pregion0raWO> for bool {
    #[inline(always)]
    fn from(variant: Pregion0raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION0RA` writer - Write '1' to enable interrupt for event PREGION0RA"]
pub type Pregion0raW<'a, REG> = crate::BitWriter<'a, REG, Pregion0raWO>;
impl<'a, REG> Pregion0raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion0raWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event PREGION1WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1wa {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pregion1wa> for bool {
    #[inline(always)]
    fn from(variant: Pregion1wa) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1WA` reader - Write '1' to enable interrupt for event PREGION1WA"]
pub type Pregion1waR = crate::BitReader<Pregion1wa>;
impl Pregion1waR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pregion1wa {
        match self.bits {
            false => Pregion1wa::Disabled,
            true => Pregion1wa::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion1wa::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion1wa::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event PREGION1WA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1waWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Pregion1waWO> for bool {
    #[inline(always)]
    fn from(variant: Pregion1waWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1WA` writer - Write '1' to enable interrupt for event PREGION1WA"]
pub type Pregion1waW<'a, REG> = crate::BitWriter<'a, REG, Pregion1waWO>;
impl<'a, REG> Pregion1waW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion1waWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event PREGION1RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1ra {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Pregion1ra> for bool {
    #[inline(always)]
    fn from(variant: Pregion1ra) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1RA` reader - Write '1' to enable interrupt for event PREGION1RA"]
pub type Pregion1raR = crate::BitReader<Pregion1ra>;
impl Pregion1raR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Pregion1ra {
        match self.bits {
            false => Pregion1ra::Disabled,
            true => Pregion1ra::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Pregion1ra::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Pregion1ra::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event PREGION1RA\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Pregion1raWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<Pregion1raWO> for bool {
    #[inline(always)]
    fn from(variant: Pregion1raWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `PREGION1RA` writer - Write '1' to enable interrupt for event PREGION1RA"]
pub type Pregion1raW<'a, REG> = crate::BitWriter<'a, REG, Pregion1raWO>;
impl<'a, REG> Pregion1raW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(Pregion1raWO::Set)
    }
}
impl R {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event REGION0WA"]
    #[inline(always)]
    pub fn region0wa(&self) -> Region0waR {
        Region0waR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event REGION0RA"]
    #[inline(always)]
    pub fn region0ra(&self) -> Region0raR {
        Region0raR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event REGION1WA"]
    #[inline(always)]
    pub fn region1wa(&self) -> Region1waR {
        Region1waR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event REGION1RA"]
    #[inline(always)]
    pub fn region1ra(&self) -> Region1raR {
        Region1raR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event REGION2WA"]
    #[inline(always)]
    pub fn region2wa(&self) -> Region2waR {
        Region2waR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event REGION2RA"]
    #[inline(always)]
    pub fn region2ra(&self) -> Region2raR {
        Region2raR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event REGION3WA"]
    #[inline(always)]
    pub fn region3wa(&self) -> Region3waR {
        Region3waR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event REGION3RA"]
    #[inline(always)]
    pub fn region3ra(&self) -> Region3raR {
        Region3raR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 24 - Write '1' to enable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub fn pregion0wa(&self) -> Pregion0waR {
        Pregion0waR::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - Write '1' to enable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub fn pregion0ra(&self) -> Pregion0raR {
        Pregion0raR::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - Write '1' to enable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub fn pregion1wa(&self) -> Pregion1waR {
        Pregion1waR::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - Write '1' to enable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub fn pregion1ra(&self) -> Pregion1raR {
        Pregion1raR::new(((self.bits >> 27) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Write '1' to enable interrupt for event REGION0WA"]
    #[inline(always)]
    pub fn region0wa(&mut self) -> Region0waW<NmiensetSpec> {
        Region0waW::new(self, 0)
    }
    #[doc = "Bit 1 - Write '1' to enable interrupt for event REGION0RA"]
    #[inline(always)]
    pub fn region0ra(&mut self) -> Region0raW<NmiensetSpec> {
        Region0raW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event REGION1WA"]
    #[inline(always)]
    pub fn region1wa(&mut self) -> Region1waW<NmiensetSpec> {
        Region1waW::new(self, 2)
    }
    #[doc = "Bit 3 - Write '1' to enable interrupt for event REGION1RA"]
    #[inline(always)]
    pub fn region1ra(&mut self) -> Region1raW<NmiensetSpec> {
        Region1raW::new(self, 3)
    }
    #[doc = "Bit 4 - Write '1' to enable interrupt for event REGION2WA"]
    #[inline(always)]
    pub fn region2wa(&mut self) -> Region2waW<NmiensetSpec> {
        Region2waW::new(self, 4)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event REGION2RA"]
    #[inline(always)]
    pub fn region2ra(&mut self) -> Region2raW<NmiensetSpec> {
        Region2raW::new(self, 5)
    }
    #[doc = "Bit 6 - Write '1' to enable interrupt for event REGION3WA"]
    #[inline(always)]
    pub fn region3wa(&mut self) -> Region3waW<NmiensetSpec> {
        Region3waW::new(self, 6)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event REGION3RA"]
    #[inline(always)]
    pub fn region3ra(&mut self) -> Region3raW<NmiensetSpec> {
        Region3raW::new(self, 7)
    }
    #[doc = "Bit 24 - Write '1' to enable interrupt for event PREGION0WA"]
    #[inline(always)]
    pub fn pregion0wa(&mut self) -> Pregion0waW<NmiensetSpec> {
        Pregion0waW::new(self, 24)
    }
    #[doc = "Bit 25 - Write '1' to enable interrupt for event PREGION0RA"]
    #[inline(always)]
    pub fn pregion0ra(&mut self) -> Pregion0raW<NmiensetSpec> {
        Pregion0raW::new(self, 25)
    }
    #[doc = "Bit 26 - Write '1' to enable interrupt for event PREGION1WA"]
    #[inline(always)]
    pub fn pregion1wa(&mut self) -> Pregion1waW<NmiensetSpec> {
        Pregion1waW::new(self, 26)
    }
    #[doc = "Bit 27 - Write '1' to enable interrupt for event PREGION1RA"]
    #[inline(always)]
    pub fn pregion1ra(&mut self) -> Pregion1raW<NmiensetSpec> {
        Pregion1raW::new(self, 27)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`nmienset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`nmienset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct NmiensetSpec;
impl crate::RegisterSpec for NmiensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`nmienset::R`](R) reader structure"]
impl crate::Readable for NmiensetSpec {}
#[doc = "`write(|w| ..)` method takes [`nmienset::W`](W) writer structure"]
impl crate::Writable for NmiensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets NMIENSET to value 0"]
impl crate::Resettable for NmiensetSpec {
    const RESET_VALUE: u32 = 0;
}
