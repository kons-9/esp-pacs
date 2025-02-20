#[doc = "Register `WDTCONFIG2` reader"]
pub struct R(crate::R<WDTCONFIG2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG2` writer"]
pub struct W(crate::W<WDTCONFIG2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG2_SPEC>;
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
impl From<crate::W<WDTCONFIG2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_STG1_HOLD` reader - Configures the hold time of RTC watchdog at level 2."]
pub type WDT_STG1_HOLD_R = crate::FieldReader<u32>;
#[doc = "Field `WDT_STG1_HOLD` writer - Configures the hold time of RTC watchdog at level 2."]
pub type WDT_STG1_HOLD_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Configures the hold time of RTC watchdog at level 2."]
    #[inline(always)]
    pub fn wdt_stg1_hold(&self) -> WDT_STG1_HOLD_R {
        WDT_STG1_HOLD_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG2")
            .field(
                "wdt_stg1_hold",
                &format_args!("{}", self.wdt_stg1_hold().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configures the hold time of RTC watchdog at level 2."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_stg1_hold(&mut self) -> WDT_STG1_HOLD_W<0> {
        WDT_STG1_HOLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configures the hold time of RTC watchdog at level 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig2](index.html) module"]
pub struct WDTCONFIG2_SPEC;
impl crate::RegisterSpec for WDTCONFIG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig2::R](R) reader structure"]
impl crate::Readable for WDTCONFIG2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig2::W](W) writer structure"]
impl crate::Writable for WDTCONFIG2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCONFIG2 to value 0x0001_3880"]
impl crate::Resettable for WDTCONFIG2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_3880;
}
