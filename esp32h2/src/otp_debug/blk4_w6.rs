#[doc = "Register `BLK4_W6` reader"]
pub struct R(crate::R<BLK4_W6_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK4_W6_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK4_W6_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK4_W6_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK4_W6` reader - Otp block4 word6 data."]
pub type BLOCK4_W6_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block4 word6 data."]
    #[inline(always)]
    pub fn block4_w6(&self) -> BLOCK4_W6_R {
        BLOCK4_W6_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK4_W6")
            .field("block4_w6", &format_args!("{}", self.block4_w6().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK4_W6_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Otp debuger block4 data register6.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk4_w6](index.html) module"]
pub struct BLK4_W6_SPEC;
impl crate::RegisterSpec for BLK4_W6_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk4_w6::R](R) reader structure"]
impl crate::Readable for BLK4_W6_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK4_W6 to value 0"]
impl crate::Resettable for BLK4_W6_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
