#[doc = "Register `OUT_STATE_CH2` reader"]
pub struct R(crate::R<OUT_STATE_CH2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_STATE_CH2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_STATE_CH2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_STATE_CH2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUTLINK_DSCR_ADDR` reader - This register stores the current outlink descriptor's address."]
pub type OUTLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUT_DSCR_STATE` reader - reserved"]
pub type OUT_DSCR_STATE_R = crate::FieldReader;
#[doc = "Field `OUT_STATE` reader - reserved"]
pub type OUT_STATE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:17 - This register stores the current outlink descriptor's address."]
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OUTLINK_DSCR_ADDR_R {
        OUTLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:19 - reserved"]
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OUT_DSCR_STATE_R {
        OUT_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:22 - reserved"]
    #[inline(always)]
    pub fn out_state(&self) -> OUT_STATE_R {
        OUT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_STATE_CH2")
            .field(
                "outlink_dscr_addr",
                &format_args!("{}", self.outlink_dscr_addr().bits()),
            )
            .field(
                "out_dscr_state",
                &format_args!("{}", self.out_dscr_state().bits()),
            )
            .field("out_state", &format_args!("{}", self.out_state().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_STATE_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA_OUT_STATE_CH2_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_state_ch2](index.html) module"]
pub struct OUT_STATE_CH2_SPEC;
impl crate::RegisterSpec for OUT_STATE_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_state_ch2::R](R) reader structure"]
impl crate::Readable for OUT_STATE_CH2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_STATE_CH2 to value 0"]
impl crate::Resettable for OUT_STATE_CH2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
