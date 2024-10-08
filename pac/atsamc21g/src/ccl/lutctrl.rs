#[doc = "Register `LUTCTRL[%s]` reader"]
pub type R = crate::R<LutctrlSpec>;
#[doc = "Register `LUTCTRL[%s]` writer"]
pub type W = crate::W<LutctrlSpec>;
#[doc = "Field `ENABLE` reader - LUT Enable"]
pub type EnableR = crate::BitReader;
#[doc = "Field `ENABLE` writer - LUT Enable"]
pub type EnableW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Filter Selection\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Filtselselect {
    #[doc = "0: Filter disabled"]
    Disable = 0,
    #[doc = "1: Synchronizer enabled"]
    Synch = 1,
    #[doc = "2: Filter enabled"]
    Filter = 2,
}
impl From<Filtselselect> for u8 {
    #[inline(always)]
    fn from(variant: Filtselselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Filtselselect {
    type Ux = u8;
}
impl crate::IsEnum for Filtselselect {}
#[doc = "Field `FILTSEL` reader - Filter Selection"]
pub type FiltselR = crate::FieldReader<Filtselselect>;
impl FiltselR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Filtselselect> {
        match self.bits {
            0 => Some(Filtselselect::Disable),
            1 => Some(Filtselselect::Synch),
            2 => Some(Filtselselect::Filter),
            _ => None,
        }
    }
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn is_disable(&self) -> bool {
        *self == Filtselselect::Disable
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn is_synch(&self) -> bool {
        *self == Filtselselect::Synch
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn is_filter(&self) -> bool {
        *self == Filtselselect::Filter
    }
}
#[doc = "Field `FILTSEL` writer - Filter Selection"]
pub type FiltselW<'a, REG> = crate::FieldWriter<'a, REG, 2, Filtselselect>;
impl<'a, REG> FiltselW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Filter disabled"]
    #[inline(always)]
    pub fn disable(self) -> &'a mut crate::W<REG> {
        self.variant(Filtselselect::Disable)
    }
    #[doc = "Synchronizer enabled"]
    #[inline(always)]
    pub fn synch(self) -> &'a mut crate::W<REG> {
        self.variant(Filtselselect::Synch)
    }
    #[doc = "Filter enabled"]
    #[inline(always)]
    pub fn filter(self) -> &'a mut crate::W<REG> {
        self.variant(Filtselselect::Filter)
    }
}
#[doc = "Field `EDGESEL` reader - Edge Selection"]
pub type EdgeselR = crate::BitReader;
#[doc = "Field `EDGESEL` writer - Edge Selection"]
pub type EdgeselW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Input Selection 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Insel0select {
    #[doc = "0: Masked input"]
    Mask = 0,
    #[doc = "1: Feedback input source"]
    Feedback = 1,
    #[doc = "2: Linked LUT input source"]
    Link = 2,
    #[doc = "3: Event in put source"]
    Event = 3,
    #[doc = "4: I/O pin input source"]
    Io = 4,
    #[doc = "5: AC input source"]
    Ac = 5,
    #[doc = "6: TC input source"]
    Tc = 6,
    #[doc = "7: Alternate TC input source"]
    Alttc = 7,
    #[doc = "8: TCC input source"]
    Tcc = 8,
    #[doc = "9: SERCOM inout source"]
    Sercom = 9,
}
impl From<Insel0select> for u8 {
    #[inline(always)]
    fn from(variant: Insel0select) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Insel0select {
    type Ux = u8;
}
impl crate::IsEnum for Insel0select {}
#[doc = "Field `INSEL0` reader - Input Selection 0"]
pub type Insel0R = crate::FieldReader<Insel0select>;
impl Insel0R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Insel0select> {
        match self.bits {
            0 => Some(Insel0select::Mask),
            1 => Some(Insel0select::Feedback),
            2 => Some(Insel0select::Link),
            3 => Some(Insel0select::Event),
            4 => Some(Insel0select::Io),
            5 => Some(Insel0select::Ac),
            6 => Some(Insel0select::Tc),
            7 => Some(Insel0select::Alttc),
            8 => Some(Insel0select::Tcc),
            9 => Some(Insel0select::Sercom),
            _ => None,
        }
    }
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn is_mask(&self) -> bool {
        *self == Insel0select::Mask
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn is_feedback(&self) -> bool {
        *self == Insel0select::Feedback
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn is_link(&self) -> bool {
        *self == Insel0select::Link
    }
    #[doc = "Event in put source"]
    #[inline(always)]
    pub fn is_event(&self) -> bool {
        *self == Insel0select::Event
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn is_io(&self) -> bool {
        *self == Insel0select::Io
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn is_ac(&self) -> bool {
        *self == Insel0select::Ac
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn is_tc(&self) -> bool {
        *self == Insel0select::Tc
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn is_alttc(&self) -> bool {
        *self == Insel0select::Alttc
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn is_tcc(&self) -> bool {
        *self == Insel0select::Tcc
    }
    #[doc = "SERCOM inout source"]
    #[inline(always)]
    pub fn is_sercom(&self) -> bool {
        *self == Insel0select::Sercom
    }
}
#[doc = "Field `INSEL0` writer - Input Selection 0"]
pub type Insel0W<'a, REG> = crate::FieldWriter<'a, REG, 4, Insel0select>;
impl<'a, REG> Insel0W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Masked input"]
    #[inline(always)]
    pub fn mask(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Mask)
    }
    #[doc = "Feedback input source"]
    #[inline(always)]
    pub fn feedback(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Feedback)
    }
    #[doc = "Linked LUT input source"]
    #[inline(always)]
    pub fn link(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Link)
    }
    #[doc = "Event in put source"]
    #[inline(always)]
    pub fn event(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Event)
    }
    #[doc = "I/O pin input source"]
    #[inline(always)]
    pub fn io(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Io)
    }
    #[doc = "AC input source"]
    #[inline(always)]
    pub fn ac(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Ac)
    }
    #[doc = "TC input source"]
    #[inline(always)]
    pub fn tc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Tc)
    }
    #[doc = "Alternate TC input source"]
    #[inline(always)]
    pub fn alttc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Alttc)
    }
    #[doc = "TCC input source"]
    #[inline(always)]
    pub fn tcc(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Tcc)
    }
    #[doc = "SERCOM inout source"]
    #[inline(always)]
    pub fn sercom(self) -> &'a mut crate::W<REG> {
        self.variant(Insel0select::Sercom)
    }
}
#[doc = "Field `INSEL1` reader - Input Selection 1"]
pub type Insel1R = crate::FieldReader;
#[doc = "Field `INSEL1` writer - Input Selection 1"]
pub type Insel1W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INSEL2` reader - Input Selection 2"]
pub type Insel2R = crate::FieldReader;
#[doc = "Field `INSEL2` writer - Input Selection 2"]
pub type Insel2W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `INVEI` reader - Input Event Invert"]
pub type InveiR = crate::BitReader;
#[doc = "Field `INVEI` writer - Input Event Invert"]
pub type InveiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUTEI` reader - Event Input Enable"]
pub type LuteiR = crate::BitReader;
#[doc = "Field `LUTEI` writer - Event Input Enable"]
pub type LuteiW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LUTEO` reader - Event Output Enable"]
pub type LuteoR = crate::BitReader;
#[doc = "Field `LUTEO` writer - Event Output Enable"]
pub type LuteoW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRUTH` reader - Truth Value"]
pub type TruthR = crate::FieldReader;
#[doc = "Field `TRUTH` writer - Truth Value"]
pub type TruthW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 1 - LUT Enable"]
    #[inline(always)]
    pub fn enable(&self) -> EnableR {
        EnableR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    pub fn filtsel(&self) -> FiltselR {
        FiltselR::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    pub fn edgesel(&self) -> EdgeselR {
        EdgeselR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline(always)]
    pub fn insel0(&self) -> Insel0R {
        Insel0R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline(always)]
    pub fn insel1(&self) -> Insel1R {
        Insel1R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline(always)]
    pub fn insel2(&self) -> Insel2R {
        Insel2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bit 20 - Input Event Invert"]
    #[inline(always)]
    pub fn invei(&self) -> InveiR {
        InveiR::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Event Input Enable"]
    #[inline(always)]
    pub fn lutei(&self) -> LuteiR {
        LuteiR::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Event Output Enable"]
    #[inline(always)]
    pub fn luteo(&self) -> LuteoR {
        LuteoR::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline(always)]
    pub fn truth(&self) -> TruthR {
        TruthR::new(((self.bits >> 24) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 1 - LUT Enable"]
    #[inline(always)]
    #[must_use]
    pub fn enable(&mut self) -> EnableW<LutctrlSpec> {
        EnableW::new(self, 1)
    }
    #[doc = "Bits 4:5 - Filter Selection"]
    #[inline(always)]
    #[must_use]
    pub fn filtsel(&mut self) -> FiltselW<LutctrlSpec> {
        FiltselW::new(self, 4)
    }
    #[doc = "Bit 7 - Edge Selection"]
    #[inline(always)]
    #[must_use]
    pub fn edgesel(&mut self) -> EdgeselW<LutctrlSpec> {
        EdgeselW::new(self, 7)
    }
    #[doc = "Bits 8:11 - Input Selection 0"]
    #[inline(always)]
    #[must_use]
    pub fn insel0(&mut self) -> Insel0W<LutctrlSpec> {
        Insel0W::new(self, 8)
    }
    #[doc = "Bits 12:15 - Input Selection 1"]
    #[inline(always)]
    #[must_use]
    pub fn insel1(&mut self) -> Insel1W<LutctrlSpec> {
        Insel1W::new(self, 12)
    }
    #[doc = "Bits 16:19 - Input Selection 2"]
    #[inline(always)]
    #[must_use]
    pub fn insel2(&mut self) -> Insel2W<LutctrlSpec> {
        Insel2W::new(self, 16)
    }
    #[doc = "Bit 20 - Input Event Invert"]
    #[inline(always)]
    #[must_use]
    pub fn invei(&mut self) -> InveiW<LutctrlSpec> {
        InveiW::new(self, 20)
    }
    #[doc = "Bit 21 - Event Input Enable"]
    #[inline(always)]
    #[must_use]
    pub fn lutei(&mut self) -> LuteiW<LutctrlSpec> {
        LuteiW::new(self, 21)
    }
    #[doc = "Bit 22 - Event Output Enable"]
    #[inline(always)]
    #[must_use]
    pub fn luteo(&mut self) -> LuteoW<LutctrlSpec> {
        LuteoW::new(self, 22)
    }
    #[doc = "Bits 24:31 - Truth Value"]
    #[inline(always)]
    #[must_use]
    pub fn truth(&mut self) -> TruthW<LutctrlSpec> {
        TruthW::new(self, 24)
    }
}
#[doc = "LUT Control x\n\nYou can [`read`](crate::Reg::read) this register and get [`lutctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lutctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LutctrlSpec;
impl crate::RegisterSpec for LutctrlSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lutctrl::R`](R) reader structure"]
impl crate::Readable for LutctrlSpec {}
#[doc = "`write(|w| ..)` method takes [`lutctrl::W`](W) writer structure"]
impl crate::Writable for LutctrlSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LUTCTRL[%s]
to value 0"]
impl crate::Resettable for LutctrlSpec {
    const RESET_VALUE: u32 = 0;
}
