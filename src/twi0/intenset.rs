#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Stopped {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Stopped> for bool {
    #[inline(always)]
    fn from(variant: Stopped) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` reader - Write '1' to enable interrupt for event STOPPED"]
pub type StoppedR = crate::BitReader<Stopped>;
impl StoppedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Stopped {
        match self.bits {
            false => Stopped::Disabled,
            true => Stopped::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Stopped::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Stopped::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event STOPPED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoppedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<StoppedWO> for bool {
    #[inline(always)]
    fn from(variant: StoppedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `STOPPED` writer - Write '1' to enable interrupt for event STOPPED"]
pub type StoppedW<'a, REG> = crate::BitWriter<'a, REG, StoppedWO>;
impl<'a, REG> StoppedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(StoppedWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event RXDREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxdready {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxdready> for bool {
    #[inline(always)]
    fn from(variant: Rxdready) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDREADY` reader - Write '1' to enable interrupt for event RXDREADY"]
pub type RxdreadyR = crate::BitReader<Rxdready>;
impl RxdreadyR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxdready {
        match self.bits {
            false => Rxdready::Disabled,
            true => Rxdready::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxdready::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxdready::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RXDREADY\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxdreadyWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RxdreadyWO> for bool {
    #[inline(always)]
    fn from(variant: RxdreadyWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXDREADY` writer - Write '1' to enable interrupt for event RXDREADY"]
pub type RxdreadyW<'a, REG> = crate::BitWriter<'a, REG, RxdreadyWO>;
impl<'a, REG> RxdreadyW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxdreadyWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event TXDSENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txdsent {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txdsent> for bool {
    #[inline(always)]
    fn from(variant: Txdsent) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDSENT` reader - Write '1' to enable interrupt for event TXDSENT"]
pub type TxdsentR = crate::BitReader<Txdsent>;
impl TxdsentR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txdsent {
        match self.bits {
            false => Txdsent::Disabled,
            true => Txdsent::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txdsent::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txdsent::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event TXDSENT\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxdsentWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<TxdsentWO> for bool {
    #[inline(always)]
    fn from(variant: TxdsentWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXDSENT` writer - Write '1' to enable interrupt for event TXDSENT"]
pub type TxdsentW<'a, REG> = crate::BitWriter<'a, REG, TxdsentWO>;
impl<'a, REG> TxdsentW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TxdsentWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Error> for bool {
    #[inline(always)]
    fn from(variant: Error) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` reader - Write '1' to enable interrupt for event ERROR"]
pub type ErrorR = crate::BitReader<Error>;
impl ErrorR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Error {
        match self.bits {
            false => Error::Disabled,
            true => Error::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Error::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Error::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event ERROR\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ErrorWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<ErrorWO> for bool {
    #[inline(always)]
    fn from(variant: ErrorWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `ERROR` writer - Write '1' to enable interrupt for event ERROR"]
pub type ErrorW<'a, REG> = crate::BitWriter<'a, REG, ErrorWO>;
impl<'a, REG> ErrorW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(ErrorWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event BB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Bb {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Bb> for bool {
    #[inline(always)]
    fn from(variant: Bb) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB` reader - Write '1' to enable interrupt for event BB"]
pub type BbR = crate::BitReader<Bb>;
impl BbR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Bb {
        match self.bits {
            false => Bb::Disabled,
            true => Bb::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Bb::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Bb::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event BB\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BbWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<BbWO> for bool {
    #[inline(always)]
    fn from(variant: BbWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `BB` writer - Write '1' to enable interrupt for event BB"]
pub type BbW<'a, REG> = crate::BitWriter<'a, REG, BbWO>;
impl<'a, REG> BbW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(BbWO::Set)
    }
}
#[doc = "Write '1' to enable interrupt for event SUSPENDED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Suspended {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Suspended> for bool {
    #[inline(always)]
    fn from(variant: Suspended) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPENDED` reader - Write '1' to enable interrupt for event SUSPENDED"]
pub type SuspendedR = crate::BitReader<Suspended>;
impl SuspendedR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Suspended {
        match self.bits {
            false => Suspended::Disabled,
            true => Suspended::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Suspended::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Suspended::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event SUSPENDED\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SuspendedWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<SuspendedWO> for bool {
    #[inline(always)]
    fn from(variant: SuspendedWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `SUSPENDED` writer - Write '1' to enable interrupt for event SUSPENDED"]
pub type SuspendedW<'a, REG> = crate::BitWriter<'a, REG, SuspendedWO>;
impl<'a, REG> SuspendedW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(SuspendedWO::Set)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RXDREADY"]
    #[inline(always)]
    pub fn rxdready(&self) -> RxdreadyR {
        RxdreadyR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event TXDSENT"]
    #[inline(always)]
    pub fn txdsent(&self) -> TxdsentR {
        TxdsentR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&self) -> ErrorR {
        ErrorR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event BB"]
    #[inline(always)]
    pub fn bb(&self) -> BbR {
        BbR::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event SUSPENDED"]
    #[inline(always)]
    pub fn suspended(&self) -> SuspendedR {
        SuspendedR::new(((self.bits >> 18) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> StoppedW<IntensetSpec> {
        StoppedW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event RXDREADY"]
    #[inline(always)]
    pub fn rxdready(&mut self) -> RxdreadyW<IntensetSpec> {
        RxdreadyW::new(self, 2)
    }
    #[doc = "Bit 7 - Write '1' to enable interrupt for event TXDSENT"]
    #[inline(always)]
    pub fn txdsent(&mut self) -> TxdsentW<IntensetSpec> {
        TxdsentW::new(self, 7)
    }
    #[doc = "Bit 9 - Write '1' to enable interrupt for event ERROR"]
    #[inline(always)]
    pub fn error(&mut self) -> ErrorW<IntensetSpec> {
        ErrorW::new(self, 9)
    }
    #[doc = "Bit 14 - Write '1' to enable interrupt for event BB"]
    #[inline(always)]
    pub fn bb(&mut self) -> BbW<IntensetSpec> {
        BbW::new(self, 14)
    }
    #[doc = "Bit 18 - Write '1' to enable interrupt for event SUSPENDED"]
    #[inline(always)]
    pub fn suspended(&mut self) -> SuspendedW<IntensetSpec> {
        SuspendedW::new(self, 18)
    }
}
#[doc = "Enable interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`intenset::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intenset::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntensetSpec;
impl crate::RegisterSpec for IntensetSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intenset::R`](R) reader structure"]
impl crate::Readable for IntensetSpec {}
#[doc = "`write(|w| ..)` method takes [`intenset::W`](W) writer structure"]
impl crate::Writable for IntensetSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTENSET to value 0"]
impl crate::Resettable for IntensetSpec {
    const RESET_VALUE: u32 = 0;
}
