#[doc = "Register `CH%s_TX_LIM` reader"]
pub struct R(crate::R<CH_TX_LIM_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_TX_LIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_TX_LIM_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_TX_LIM_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CH%s_TX_LIM` writer"]
pub struct W(crate::W<CH_TX_LIM_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CH_TX_LIM_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CH_TX_LIM_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CH_TX_LIM_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_LIM` reader - reg_rmt_tx_lim_ch0."]
pub type TX_LIM_R = crate::FieldReader<u16>;
#[doc = "Field `TX_LIM` writer - reg_rmt_tx_lim_ch0."]
pub type TX_LIM_W<'a, const O: u8> = crate::FieldWriter<'a, CH_TX_LIM_SPEC, 9, O, u16>;
#[doc = "Field `TX_LOOP_NUM` reader - reg_rmt_tx_loop_num_ch0."]
pub type TX_LOOP_NUM_R = crate::FieldReader<u16>;
#[doc = "Field `TX_LOOP_NUM` writer - reg_rmt_tx_loop_num_ch0."]
pub type TX_LOOP_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CH_TX_LIM_SPEC, 10, O, u16>;
#[doc = "Field `TX_LOOP_CNT_EN` reader - reg_rmt_tx_loop_cnt_en_ch0."]
pub type TX_LOOP_CNT_EN_R = crate::BitReader;
#[doc = "Field `TX_LOOP_CNT_EN` writer - reg_rmt_tx_loop_cnt_en_ch0."]
pub type TX_LOOP_CNT_EN_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_LIM_SPEC, O>;
#[doc = "Field `LOOP_COUNT_RESET` writer - reg_loop_count_reset_ch0."]
pub type LOOP_COUNT_RESET_W<'a, const O: u8> = crate::BitWriter<'a, CH_TX_LIM_SPEC, O>;
impl R {
    #[doc = "Bits 0:8 - reg_rmt_tx_lim_ch0."]
    #[inline(always)]
    pub fn tx_lim(&self) -> TX_LIM_R {
        TX_LIM_R::new((self.bits & 0x01ff) as u16)
    }
    #[doc = "Bits 9:18 - reg_rmt_tx_loop_num_ch0."]
    #[inline(always)]
    pub fn tx_loop_num(&self) -> TX_LOOP_NUM_R {
        TX_LOOP_NUM_R::new(((self.bits >> 9) & 0x03ff) as u16)
    }
    #[doc = "Bit 19 - reg_rmt_tx_loop_cnt_en_ch0."]
    #[inline(always)]
    pub fn tx_loop_cnt_en(&self) -> TX_LOOP_CNT_EN_R {
        TX_LOOP_CNT_EN_R::new(((self.bits >> 19) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_TX_LIM")
            .field("tx_lim", &format_args!("{}", self.tx_lim().bits()))
            .field(
                "tx_loop_num",
                &format_args!("{}", self.tx_loop_num().bits()),
            )
            .field(
                "tx_loop_cnt_en",
                &format_args!("{}", self.tx_loop_cnt_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_TX_LIM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:8 - reg_rmt_tx_lim_ch0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_lim(&mut self) -> TX_LIM_W<0> {
        TX_LIM_W::new(self)
    }
    #[doc = "Bits 9:18 - reg_rmt_tx_loop_num_ch0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_loop_num(&mut self) -> TX_LOOP_NUM_W<9> {
        TX_LOOP_NUM_W::new(self)
    }
    #[doc = "Bit 19 - reg_rmt_tx_loop_cnt_en_ch0."]
    #[inline(always)]
    #[must_use]
    pub fn tx_loop_cnt_en(&mut self) -> TX_LOOP_CNT_EN_W<19> {
        TX_LOOP_CNT_EN_W::new(self)
    }
    #[doc = "Bit 20 - reg_loop_count_reset_ch0."]
    #[inline(always)]
    #[must_use]
    pub fn loop_count_reset(&mut self) -> LOOP_COUNT_RESET_W<20> {
        LOOP_COUNT_RESET_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT_CH%s_TX_LIM_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_tx_lim](index.html) module"]
pub struct CH_TX_LIM_SPEC;
impl crate::RegisterSpec for CH_TX_LIM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_tx_lim::R](R) reader structure"]
impl crate::Readable for CH_TX_LIM_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ch_tx_lim::W](W) writer structure"]
impl crate::Writable for CH_TX_LIM_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CH%s_TX_LIM to value 0x80"]
impl crate::Resettable for CH_TX_LIM_SPEC {
    const RESET_VALUE: Self::Ux = 0x80;
}
