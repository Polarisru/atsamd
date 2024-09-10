#[doc = "Register `CTRLA` reader"]
pub type R = crate::R<CtrlaSpec>;
#[doc = "Register `CTRLA` writer"]
pub type W = crate::W<CtrlaSpec>;
#[doc = "Field `SWRST` reader - Software Reset"]
pub type SwrstR = crate::BitReader;
#[doc = "Field `SWRST` writer - Software Reset"]
pub type SwrstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ENABLE` reader - Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Operating Mode\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Modeselect {
    #[doc = "0: Mode 0: 32-bit Counter"]
    Count32 = 0,
    #[doc = "1: Mode 1: 16-bit Counter"]
    Count16 = 1,
    #[doc = "2: Mode 2: Clock/Calendar"]
    Clock = 2,
}
impl From<Modeselect> for u8 {
    #[inline(always)]
    fn from(variant: Modeselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Modeselect {
    type Ux = u8;
}
impl crate::IsEnum for Modeselect {}
#[doc = "Field `MODE` reader - Operating Mode"]
pub type ModeR = crate::FieldReader<Modeselect>;
impl ModeR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Modeselect> {
        match self.bits {
            0 => Some(Modeselect::Count32),
            1 => Some(Modeselect::Count16),
            2 => Some(Modeselect::Clock),
            _ => None,
        }
    }
    #[doc = "Mode 0: 32-bit Counter"]
    #[inline(always)]
    pub fn is_count32(&self) -> bool {
        *self == Modeselect::Count32
    }
    #[doc = "Mode 1: 16-bit Counter"]
    #[inline(always)]
    pub fn is_count16(&self) -> bool {
        *self == Modeselect::Count16
    }
    #[doc = "Mode 2: Clock/Calendar"]
    #[inline(always)]
    pub fn is_clock(&self) -> bool {
        *self == Modeselect::Clock
    }
}
#[doc = "Field `MODE` writer - Operating Mode"]
pub type ModeW<'a, REG> = crate::FieldWriter<'a, REG, 2, Modeselect>;
impl<'a, REG> ModeW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Mode 0: 32-bit Counter"]
    #[inline(always)]
    pub fn count32(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Count32)
    }
    #[doc = "Mode 1: 16-bit Counter"]
    #[inline(always)]
    pub fn count16(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Count16)
    }
    #[doc = "Mode 2: Clock/Calendar"]
    #[inline(always)]
    pub fn clock(self) -> &'a mut crate::W<REG> {
        self.variant(Modeselect::Clock)
    }
}
#[doc = "Field `CLKREP` reader - Clock Representation"]
pub type ClkrepR = crate::BitReader;
#[doc = "Field `CLKREP` writer - Clock Representation"]
pub type ClkrepW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MATCHCLR` reader - Clear on Match"]
pub type MatchclrR = crate::BitReader;
#[doc = "Field `MATCHCLR` writer - Clear on Match"]
pub type MatchclrW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Prescaler\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Prescalerselect {
    #[doc = "0: CLK_RTC_CNT = GCLK_RTC/1"]
    Off = 0,
    #[doc = "1: CLK_RTC_CNT = GCLK_RTC/1"]
    Div1 = 1,
    #[doc = "2: CLK_RTC_CNT = GCLK_RTC/2"]
    Div2 = 2,
    #[doc = "3: CLK_RTC_CNT = GCLK_RTC/4"]
    Div4 = 3,
    #[doc = "4: CLK_RTC_CNT = GCLK_RTC/8"]
    Div8 = 4,
    #[doc = "5: CLK_RTC_CNT = GCLK_RTC/16"]
    Div16 = 5,
    #[doc = "6: CLK_RTC_CNT = GCLK_RTC/32"]
    Div32 = 6,
    #[doc = "7: CLK_RTC_CNT = GCLK_RTC/64"]
    Div64 = 7,
    #[doc = "8: CLK_RTC_CNT = GCLK_RTC/128"]
    Div128 = 8,
    #[doc = "9: CLK_RTC_CNT = GCLK_RTC/256"]
    Div256 = 9,
    #[doc = "10: CLK_RTC_CNT = GCLK_RTC/512"]
    Div512 = 10,
    #[doc = "11: CLK_RTC_CNT = GCLK_RTC/1024"]
    Div1024 = 11,
}
impl From<Prescalerselect> for u8 {
    #[inline(always)]
    fn from(variant: Prescalerselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Prescalerselect {
    type Ux = u8;
}
impl crate::IsEnum for Prescalerselect {}
#[doc = "Field `PRESCALER` reader - Prescaler"]
pub type PrescalerR = crate::FieldReader<Prescalerselect>;
impl PrescalerR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Prescalerselect> {
        match self.bits {
            0 => Some(Prescalerselect::Off),
            1 => Some(Prescalerselect::Div1),
            2 => Some(Prescalerselect::Div2),
            3 => Some(Prescalerselect::Div4),
            4 => Some(Prescalerselect::Div8),
            5 => Some(Prescalerselect::Div16),
            6 => Some(Prescalerselect::Div32),
            7 => Some(Prescalerselect::Div64),
            8 => Some(Prescalerselect::Div128),
            9 => Some(Prescalerselect::Div256),
            10 => Some(Prescalerselect::Div512),
            11 => Some(Prescalerselect::Div1024),
            _ => None,
        }
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn is_off(&self) -> bool {
        *self == Prescalerselect::Off
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn is_div1(&self) -> bool {
        *self == Prescalerselect::Div1
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    #[inline(always)]
    pub fn is_div2(&self) -> bool {
        *self == Prescalerselect::Div2
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    #[inline(always)]
    pub fn is_div4(&self) -> bool {
        *self == Prescalerselect::Div4
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    #[inline(always)]
    pub fn is_div8(&self) -> bool {
        *self == Prescalerselect::Div8
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    #[inline(always)]
    pub fn is_div16(&self) -> bool {
        *self == Prescalerselect::Div16
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    #[inline(always)]
    pub fn is_div32(&self) -> bool {
        *self == Prescalerselect::Div32
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    #[inline(always)]
    pub fn is_div64(&self) -> bool {
        *self == Prescalerselect::Div64
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    #[inline(always)]
    pub fn is_div128(&self) -> bool {
        *self == Prescalerselect::Div128
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    #[inline(always)]
    pub fn is_div256(&self) -> bool {
        *self == Prescalerselect::Div256
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    #[inline(always)]
    pub fn is_div512(&self) -> bool {
        *self == Prescalerselect::Div512
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    #[inline(always)]
    pub fn is_div1024(&self) -> bool {
        *self == Prescalerselect::Div1024
    }
}
#[doc = "Field `PRESCALER` writer - Prescaler"]
pub type PrescalerW<'a, REG> = crate::FieldWriter<'a, REG, 4, Prescalerselect>;
impl<'a, REG> PrescalerW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn off(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Off)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1"]
    #[inline(always)]
    pub fn div1(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div1)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/2"]
    #[inline(always)]
    pub fn div2(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div2)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/4"]
    #[inline(always)]
    pub fn div4(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div4)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/8"]
    #[inline(always)]
    pub fn div8(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div8)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/16"]
    #[inline(always)]
    pub fn div16(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div16)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/32"]
    #[inline(always)]
    pub fn div32(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div32)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/64"]
    #[inline(always)]
    pub fn div64(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div64)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/128"]
    #[inline(always)]
    pub fn div128(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div128)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/256"]
    #[inline(always)]
    pub fn div256(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div256)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/512"]
    #[inline(always)]
    pub fn div512(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div512)
    }
    #[doc = "CLK_RTC_CNT = GCLK_RTC/1024"]
    #[inline(always)]
    pub fn div1024(self) -> &'a mut crate::W<REG> {
        self.variant(Prescalerselect::Div1024)
    }
}
#[doc = "Field `CLOCKSYNC` reader - Clock Read Synchronization Enable"]
pub type ClocksyncR = crate::BitReader;
#[doc = "Field `CLOCKSYNC` writer - Clock Read Synchronization Enable"]
pub type ClocksyncW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    pub fn swrst(&self) -> SwrstR {
        SwrstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline(always)]
    pub fn mode(&self) -> ModeR {
        ModeR::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bit 6 - Clock Representation"]
    #[inline(always)]
    pub fn clkrep(&self) -> ClkrepR {
        ClkrepR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Clear on Match"]
    #[inline(always)]
    pub fn matchclr(&self) -> MatchclrR {
        MatchclrR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    pub fn prescaler(&self) -> PrescalerR {
        PrescalerR::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bit 15 - Clock Read Synchronization Enable"]
    #[inline(always)]
    pub fn clocksync(&self) -> ClocksyncR {
        ClocksyncR::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Software Reset"]
    #[inline(always)]
    #[must_use]
    pub fn swrst(&mut self) -> SwrstW<CtrlaSpec> {
        SwrstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<CtrlaSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 2:3 - Operating Mode"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> ModeW<CtrlaSpec> {
        ModeW::new(self, 2)
    }
    #[doc = "Bit 6 - Clock Representation"]
    #[inline(always)]
    #[must_use]
    pub fn clkrep(&mut self) -> ClkrepW<CtrlaSpec> {
        ClkrepW::new(self, 6)
    }
    #[doc = "Bit 7 - Clear on Match"]
    #[inline(always)]
    #[must_use]
    pub fn matchclr(&mut self) -> MatchclrW<CtrlaSpec> {
        MatchclrW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Prescaler"]
    #[inline(always)]
    #[must_use]
    pub fn prescaler(&mut self) -> PrescalerW<CtrlaSpec> {
        PrescalerW::new(self, 8)
    }
    #[doc = "Bit 15 - Clock Read Synchronization Enable"]
    #[inline(always)]
    #[must_use]
    pub fn clocksync(&mut self) -> ClocksyncW<CtrlaSpec> {
        ClocksyncW::new(self, 15)
    }
}
#[doc = "MODE2 Control A\n\nYou can [`read`](crate::Reg::read) this register and get [`ctrla::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ctrla::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CtrlaSpec;
impl crate::RegisterSpec for CtrlaSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`ctrla::R`](R) reader structure"]
impl crate::Readable for CtrlaSpec {}
#[doc = "`write(|w| ..)` method takes [`ctrla::W`](W) writer structure"]
impl crate::Writable for CtrlaSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets CTRLA to value 0"]
impl crate::Resettable for CtrlaSpec {
    const RESET_VALUE: u16 = 0;
}
