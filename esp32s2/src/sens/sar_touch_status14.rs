#[doc = "Register `SAR_TOUCH_STATUS14` reader"]
pub struct R(crate::R<SAR_TOUCH_STATUS14_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_TOUCH_STATUS14_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_TOUCH_STATUS14_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_TOUCH_STATUS14_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TOUCH_PAD14_DATA` reader - The data of touch pad 14, depending on the setting of SENS_TOUCH_DATA_SEL."]
pub type TOUCH_PAD14_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_PAD14_DEBOUNCE` reader - Touch pad 14 debounce value."]
pub type TOUCH_PAD14_DEBOUNCE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:21 - The data of touch pad 14, depending on the setting of SENS_TOUCH_DATA_SEL."]
    #[inline(always)]
    pub fn touch_pad14_data(&self) -> TOUCH_PAD14_DATA_R {
        TOUCH_PAD14_DATA_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 29:31 - Touch pad 14 debounce value."]
    #[inline(always)]
    pub fn touch_pad14_debounce(&self) -> TOUCH_PAD14_DEBOUNCE_R {
        TOUCH_PAD14_DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS14")
            .field(
                "touch_pad14_data",
                &format_args!("{}", self.touch_pad14_data().bits()),
            )
            .field(
                "touch_pad14_debounce",
                &format_args!("{}", self.touch_pad14_debounce().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_STATUS14_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Touch pad 14 status\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_touch_status14](index.html) module"]
pub struct SAR_TOUCH_STATUS14_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS14_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_touch_status14::R](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS14_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS14 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS14_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
