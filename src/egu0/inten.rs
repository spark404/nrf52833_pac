#[doc = "Register `INTEN` reader"]
pub type R = crate::R<IntenSpec>;
#[doc = "Register `INTEN` writer"]
pub type W = crate::W<IntenSpec>;
#[doc = "Enable or disable interrupt for event TRIGGERED\\[0\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered0 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered0> for bool {
    #[inline(always)]
    fn from(variant: Triggered0) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED0` reader - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
pub type Triggered0R = crate::BitReader<Triggered0>;
impl Triggered0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered0 {
        match self.bits {
            false => Triggered0::Disabled,
            true => Triggered0::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered0::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered0::Enabled
    }
}
#[doc = "Field `TRIGGERED0` writer - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
pub type Triggered0W<'a, REG> = crate::BitWriter<'a, REG, Triggered0>;
impl<'a, REG> Triggered0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered0::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered0::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[1\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered1 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered1> for bool {
    #[inline(always)]
    fn from(variant: Triggered1) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED1` reader - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
pub type Triggered1R = crate::BitReader<Triggered1>;
impl Triggered1R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered1 {
        match self.bits {
            false => Triggered1::Disabled,
            true => Triggered1::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered1::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered1::Enabled
    }
}
#[doc = "Field `TRIGGERED1` writer - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
pub type Triggered1W<'a, REG> = crate::BitWriter<'a, REG, Triggered1>;
impl<'a, REG> Triggered1W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered1::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered1::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[2\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered2 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered2> for bool {
    #[inline(always)]
    fn from(variant: Triggered2) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED2` reader - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
pub type Triggered2R = crate::BitReader<Triggered2>;
impl Triggered2R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered2 {
        match self.bits {
            false => Triggered2::Disabled,
            true => Triggered2::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered2::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered2::Enabled
    }
}
#[doc = "Field `TRIGGERED2` writer - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
pub type Triggered2W<'a, REG> = crate::BitWriter<'a, REG, Triggered2>;
impl<'a, REG> Triggered2W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered2::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered2::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[3\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered3 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered3> for bool {
    #[inline(always)]
    fn from(variant: Triggered3) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED3` reader - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
pub type Triggered3R = crate::BitReader<Triggered3>;
impl Triggered3R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered3 {
        match self.bits {
            false => Triggered3::Disabled,
            true => Triggered3::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered3::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered3::Enabled
    }
}
#[doc = "Field `TRIGGERED3` writer - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
pub type Triggered3W<'a, REG> = crate::BitWriter<'a, REG, Triggered3>;
impl<'a, REG> Triggered3W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered3::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered3::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[4\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered4 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered4> for bool {
    #[inline(always)]
    fn from(variant: Triggered4) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED4` reader - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
pub type Triggered4R = crate::BitReader<Triggered4>;
impl Triggered4R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered4 {
        match self.bits {
            false => Triggered4::Disabled,
            true => Triggered4::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered4::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered4::Enabled
    }
}
#[doc = "Field `TRIGGERED4` writer - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
pub type Triggered4W<'a, REG> = crate::BitWriter<'a, REG, Triggered4>;
impl<'a, REG> Triggered4W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered4::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered4::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[5\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered5 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered5> for bool {
    #[inline(always)]
    fn from(variant: Triggered5) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED5` reader - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
pub type Triggered5R = crate::BitReader<Triggered5>;
impl Triggered5R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered5 {
        match self.bits {
            false => Triggered5::Disabled,
            true => Triggered5::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered5::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered5::Enabled
    }
}
#[doc = "Field `TRIGGERED5` writer - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
pub type Triggered5W<'a, REG> = crate::BitWriter<'a, REG, Triggered5>;
impl<'a, REG> Triggered5W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered5::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered5::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[6\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered6 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered6> for bool {
    #[inline(always)]
    fn from(variant: Triggered6) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED6` reader - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
pub type Triggered6R = crate::BitReader<Triggered6>;
impl Triggered6R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered6 {
        match self.bits {
            false => Triggered6::Disabled,
            true => Triggered6::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered6::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered6::Enabled
    }
}
#[doc = "Field `TRIGGERED6` writer - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
pub type Triggered6W<'a, REG> = crate::BitWriter<'a, REG, Triggered6>;
impl<'a, REG> Triggered6W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered6::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered6::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[7\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered7 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered7> for bool {
    #[inline(always)]
    fn from(variant: Triggered7) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED7` reader - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
pub type Triggered7R = crate::BitReader<Triggered7>;
impl Triggered7R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered7 {
        match self.bits {
            false => Triggered7::Disabled,
            true => Triggered7::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered7::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered7::Enabled
    }
}
#[doc = "Field `TRIGGERED7` writer - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
pub type Triggered7W<'a, REG> = crate::BitWriter<'a, REG, Triggered7>;
impl<'a, REG> Triggered7W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered7::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered7::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[8\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered8 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered8> for bool {
    #[inline(always)]
    fn from(variant: Triggered8) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED8` reader - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
pub type Triggered8R = crate::BitReader<Triggered8>;
impl Triggered8R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered8 {
        match self.bits {
            false => Triggered8::Disabled,
            true => Triggered8::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered8::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered8::Enabled
    }
}
#[doc = "Field `TRIGGERED8` writer - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
pub type Triggered8W<'a, REG> = crate::BitWriter<'a, REG, Triggered8>;
impl<'a, REG> Triggered8W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered8::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered8::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[9\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered9 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered9> for bool {
    #[inline(always)]
    fn from(variant: Triggered9) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED9` reader - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
pub type Triggered9R = crate::BitReader<Triggered9>;
impl Triggered9R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered9 {
        match self.bits {
            false => Triggered9::Disabled,
            true => Triggered9::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered9::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered9::Enabled
    }
}
#[doc = "Field `TRIGGERED9` writer - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
pub type Triggered9W<'a, REG> = crate::BitWriter<'a, REG, Triggered9>;
impl<'a, REG> Triggered9W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered9::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered9::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[10\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered10 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered10> for bool {
    #[inline(always)]
    fn from(variant: Triggered10) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED10` reader - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
pub type Triggered10R = crate::BitReader<Triggered10>;
impl Triggered10R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered10 {
        match self.bits {
            false => Triggered10::Disabled,
            true => Triggered10::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered10::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered10::Enabled
    }
}
#[doc = "Field `TRIGGERED10` writer - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
pub type Triggered10W<'a, REG> = crate::BitWriter<'a, REG, Triggered10>;
impl<'a, REG> Triggered10W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered10::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered10::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[11\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered11 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered11> for bool {
    #[inline(always)]
    fn from(variant: Triggered11) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED11` reader - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
pub type Triggered11R = crate::BitReader<Triggered11>;
impl Triggered11R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered11 {
        match self.bits {
            false => Triggered11::Disabled,
            true => Triggered11::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered11::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered11::Enabled
    }
}
#[doc = "Field `TRIGGERED11` writer - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
pub type Triggered11W<'a, REG> = crate::BitWriter<'a, REG, Triggered11>;
impl<'a, REG> Triggered11W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered11::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered11::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[12\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered12 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered12> for bool {
    #[inline(always)]
    fn from(variant: Triggered12) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED12` reader - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
pub type Triggered12R = crate::BitReader<Triggered12>;
impl Triggered12R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered12 {
        match self.bits {
            false => Triggered12::Disabled,
            true => Triggered12::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered12::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered12::Enabled
    }
}
#[doc = "Field `TRIGGERED12` writer - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
pub type Triggered12W<'a, REG> = crate::BitWriter<'a, REG, Triggered12>;
impl<'a, REG> Triggered12W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered12::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered12::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[13\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered13 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered13> for bool {
    #[inline(always)]
    fn from(variant: Triggered13) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED13` reader - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
pub type Triggered13R = crate::BitReader<Triggered13>;
impl Triggered13R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered13 {
        match self.bits {
            false => Triggered13::Disabled,
            true => Triggered13::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered13::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered13::Enabled
    }
}
#[doc = "Field `TRIGGERED13` writer - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
pub type Triggered13W<'a, REG> = crate::BitWriter<'a, REG, Triggered13>;
impl<'a, REG> Triggered13W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered13::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered13::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[14\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered14 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered14> for bool {
    #[inline(always)]
    fn from(variant: Triggered14) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED14` reader - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
pub type Triggered14R = crate::BitReader<Triggered14>;
impl Triggered14R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered14 {
        match self.bits {
            false => Triggered14::Disabled,
            true => Triggered14::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered14::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered14::Enabled
    }
}
#[doc = "Field `TRIGGERED14` writer - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
pub type Triggered14W<'a, REG> = crate::BitWriter<'a, REG, Triggered14>;
impl<'a, REG> Triggered14W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered14::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered14::Enabled)
    }
}
#[doc = "Enable or disable interrupt for event TRIGGERED\\[15\\]\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Triggered15 {
    #[doc = "0: Disable"]
    Disabled = 0,
    #[doc = "1: Enable"]
    Enabled = 1,
}
impl From<Triggered15> for bool {
    #[inline(always)]
    fn from(variant: Triggered15) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TRIGGERED15` reader - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
pub type Triggered15R = crate::BitReader<Triggered15>;
impl Triggered15R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Triggered15 {
        match self.bits {
            false => Triggered15::Disabled,
            true => Triggered15::Enabled,
        }
    }
    #[doc = "Disable"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Triggered15::Disabled
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Triggered15::Enabled
    }
}
#[doc = "Field `TRIGGERED15` writer - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
pub type Triggered15W<'a, REG> = crate::BitWriter<'a, REG, Triggered15>;
impl<'a, REG> Triggered15W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Disable"]
    #[inline(always)]
    pub fn disabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered15::Disabled)
    }
    #[doc = "Enable"]
    #[inline(always)]
    pub fn enabled(self) -> &'a mut crate::W<REG> {
        self.variant(Triggered15::Enabled)
    }
}
impl R {
    #[doc = "Bit 0 - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
    #[inline(always)]
    pub fn triggered0(&self) -> Triggered0R {
        Triggered0R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
    #[inline(always)]
    pub fn triggered1(&self) -> Triggered1R {
        Triggered1R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
    #[inline(always)]
    pub fn triggered2(&self) -> Triggered2R {
        Triggered2R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
    #[inline(always)]
    pub fn triggered3(&self) -> Triggered3R {
        Triggered3R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
    #[inline(always)]
    pub fn triggered4(&self) -> Triggered4R {
        Triggered4R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
    #[inline(always)]
    pub fn triggered5(&self) -> Triggered5R {
        Triggered5R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
    #[inline(always)]
    pub fn triggered6(&self) -> Triggered6R {
        Triggered6R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
    #[inline(always)]
    pub fn triggered7(&self) -> Triggered7R {
        Triggered7R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
    #[inline(always)]
    pub fn triggered8(&self) -> Triggered8R {
        Triggered8R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
    #[inline(always)]
    pub fn triggered9(&self) -> Triggered9R {
        Triggered9R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
    #[inline(always)]
    pub fn triggered10(&self) -> Triggered10R {
        Triggered10R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
    #[inline(always)]
    pub fn triggered11(&self) -> Triggered11R {
        Triggered11R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
    #[inline(always)]
    pub fn triggered12(&self) -> Triggered12R {
        Triggered12R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
    #[inline(always)]
    pub fn triggered13(&self) -> Triggered13R {
        Triggered13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
    #[inline(always)]
    pub fn triggered14(&self) -> Triggered14R {
        Triggered14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
    #[inline(always)]
    pub fn triggered15(&self) -> Triggered15R {
        Triggered15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Enable or disable interrupt for event TRIGGERED\\[0\\]"]
    #[inline(always)]
    pub fn triggered0(&mut self) -> Triggered0W<IntenSpec> {
        Triggered0W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable or disable interrupt for event TRIGGERED\\[1\\]"]
    #[inline(always)]
    pub fn triggered1(&mut self) -> Triggered1W<IntenSpec> {
        Triggered1W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable or disable interrupt for event TRIGGERED\\[2\\]"]
    #[inline(always)]
    pub fn triggered2(&mut self) -> Triggered2W<IntenSpec> {
        Triggered2W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable or disable interrupt for event TRIGGERED\\[3\\]"]
    #[inline(always)]
    pub fn triggered3(&mut self) -> Triggered3W<IntenSpec> {
        Triggered3W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable or disable interrupt for event TRIGGERED\\[4\\]"]
    #[inline(always)]
    pub fn triggered4(&mut self) -> Triggered4W<IntenSpec> {
        Triggered4W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable or disable interrupt for event TRIGGERED\\[5\\]"]
    #[inline(always)]
    pub fn triggered5(&mut self) -> Triggered5W<IntenSpec> {
        Triggered5W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable or disable interrupt for event TRIGGERED\\[6\\]"]
    #[inline(always)]
    pub fn triggered6(&mut self) -> Triggered6W<IntenSpec> {
        Triggered6W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable or disable interrupt for event TRIGGERED\\[7\\]"]
    #[inline(always)]
    pub fn triggered7(&mut self) -> Triggered7W<IntenSpec> {
        Triggered7W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable or disable interrupt for event TRIGGERED\\[8\\]"]
    #[inline(always)]
    pub fn triggered8(&mut self) -> Triggered8W<IntenSpec> {
        Triggered8W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable or disable interrupt for event TRIGGERED\\[9\\]"]
    #[inline(always)]
    pub fn triggered9(&mut self) -> Triggered9W<IntenSpec> {
        Triggered9W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable or disable interrupt for event TRIGGERED\\[10\\]"]
    #[inline(always)]
    pub fn triggered10(&mut self) -> Triggered10W<IntenSpec> {
        Triggered10W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable or disable interrupt for event TRIGGERED\\[11\\]"]
    #[inline(always)]
    pub fn triggered11(&mut self) -> Triggered11W<IntenSpec> {
        Triggered11W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable or disable interrupt for event TRIGGERED\\[12\\]"]
    #[inline(always)]
    pub fn triggered12(&mut self) -> Triggered12W<IntenSpec> {
        Triggered12W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable or disable interrupt for event TRIGGERED\\[13\\]"]
    #[inline(always)]
    pub fn triggered13(&mut self) -> Triggered13W<IntenSpec> {
        Triggered13W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable or disable interrupt for event TRIGGERED\\[14\\]"]
    #[inline(always)]
    pub fn triggered14(&mut self) -> Triggered14W<IntenSpec> {
        Triggered14W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable or disable interrupt for event TRIGGERED\\[15\\]"]
    #[inline(always)]
    pub fn triggered15(&mut self) -> Triggered15W<IntenSpec> {
        Triggered15W::new(self, 15)
    }
}
#[doc = "Enable or disable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`inten::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`inten::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntenSpec;
impl crate::RegisterSpec for IntenSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`inten::R`](R) reader structure"]
impl crate::Readable for IntenSpec {}
#[doc = "`write(|w| ..)` method takes [`inten::W`](W) writer structure"]
impl crate::Writable for IntenSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTEN to value 0"]
impl crate::Resettable for IntenSpec {
    const RESET_VALUE: u32 = 0;
}
