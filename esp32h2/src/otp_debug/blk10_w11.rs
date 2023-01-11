#[doc = "Register `BLK10_W11` reader"]
pub struct R(crate::R<BLK10_W11_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BLK10_W11_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BLK10_W11_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BLK10_W11_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `BLOCK10_W11` reader - Otp block10 word11 data."]
pub type BLOCK10_W11_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - Otp block10 word11 data."]
    #[inline(always)]
    pub fn block10_w11(&self) -> BLOCK10_W11_R {
        BLOCK10_W11_R::new(self.bits)
    }
}
#[doc = "Otp debuger block10 data register11.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [blk10_w11](index.html) module"]
pub struct BLK10_W11_SPEC;
impl crate::RegisterSpec for BLK10_W11_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [blk10_w11::R](R) reader structure"]
impl crate::Readable for BLK10_W11_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BLK10_W11 to value 0"]
impl crate::Resettable for BLK10_W11_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
