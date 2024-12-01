#[doc = "Register `INTENSET` reader"]
pub type R = crate::R<IntensetSpec>;
#[doc = "Register `INTENSET` writer"]
pub type W = crate::W<IntensetSpec>;
#[doc = "Write '1' to enable interrupt for event RXPTRUPD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Rxptrupd {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Rxptrupd> for bool {
    #[inline(always)]
    fn from(variant: Rxptrupd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPTRUPD` reader - Write '1' to enable interrupt for event RXPTRUPD"]
pub type RxptrupdR = crate::BitReader<Rxptrupd>;
impl RxptrupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Rxptrupd {
        match self.bits {
            false => Rxptrupd::Disabled,
            true => Rxptrupd::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Rxptrupd::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Rxptrupd::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event RXPTRUPD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RxptrupdWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<RxptrupdWO> for bool {
    #[inline(always)]
    fn from(variant: RxptrupdWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `RXPTRUPD` writer - Write '1' to enable interrupt for event RXPTRUPD"]
pub type RxptrupdW<'a, REG> = crate::BitWriter<'a, REG, RxptrupdWO>;
impl<'a, REG> RxptrupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(RxptrupdWO::Set)
    }
}
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
#[doc = "Write '1' to enable interrupt for event TXPTRUPD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Txptrupd {
    #[doc = "0: Read: Disabled"]
    Disabled = 0,
    #[doc = "1: Read: Enabled"]
    Enabled = 1,
}
impl From<Txptrupd> for bool {
    #[inline(always)]
    fn from(variant: Txptrupd) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPTRUPD` reader - Write '1' to enable interrupt for event TXPTRUPD"]
pub type TxptrupdR = crate::BitReader<Txptrupd>;
impl TxptrupdR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Txptrupd {
        match self.bits {
            false => Txptrupd::Disabled,
            true => Txptrupd::Enabled,
        }
    }
    #[doc = "Read: Disabled"]
    #[inline(always)]
    pub fn is_disabled(&self) -> bool {
        *self == Txptrupd::Disabled
    }
    #[doc = "Read: Enabled"]
    #[inline(always)]
    pub fn is_enabled(&self) -> bool {
        *self == Txptrupd::Enabled
    }
}
#[doc = "Write '1' to enable interrupt for event TXPTRUPD\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TxptrupdWO {
    #[doc = "1: Enable"]
    Set = 1,
}
impl From<TxptrupdWO> for bool {
    #[inline(always)]
    fn from(variant: TxptrupdWO) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `TXPTRUPD` writer - Write '1' to enable interrupt for event TXPTRUPD"]
pub type TxptrupdW<'a, REG> = crate::BitWriter<'a, REG, TxptrupdWO>;
impl<'a, REG> TxptrupdW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "Enable"]
    #[inline(always)]
    pub fn set_(self) -> &'a mut crate::W<REG> {
        self.variant(TxptrupdWO::Set)
    }
}
impl R {
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RXPTRUPD"]
    #[inline(always)]
    pub fn rxptrupd(&self) -> RxptrupdR {
        RxptrupdR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&self) -> StoppedR {
        StoppedR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event TXPTRUPD"]
    #[inline(always)]
    pub fn txptrupd(&self) -> TxptrupdR {
        TxptrupdR::new(((self.bits >> 5) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - Write '1' to enable interrupt for event RXPTRUPD"]
    #[inline(always)]
    pub fn rxptrupd(&mut self) -> RxptrupdW<IntensetSpec> {
        RxptrupdW::new(self, 1)
    }
    #[doc = "Bit 2 - Write '1' to enable interrupt for event STOPPED"]
    #[inline(always)]
    pub fn stopped(&mut self) -> StoppedW<IntensetSpec> {
        StoppedW::new(self, 2)
    }
    #[doc = "Bit 5 - Write '1' to enable interrupt for event TXPTRUPD"]
    #[inline(always)]
    pub fn txptrupd(&mut self) -> TxptrupdW<IntensetSpec> {
        TxptrupdW::new(self, 5)
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
