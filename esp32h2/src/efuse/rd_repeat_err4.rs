#[doc = "Register `RD_REPEAT_ERR4` reader"]
pub struct R(crate::R<RD_REPEAT_ERR4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RD_REPEAT_ERR4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RD_REPEAT_ERR4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RD_REPEAT_ERR4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `HYS_EN_PAD1_ERR` reader - Indicates a programming error of HYS_EN_PAD1."]
pub type HYS_EN_PAD1_ERR_R = crate::FieldReader<u32>;
#[doc = "Field `RPT4_RESERVED4_ERR_1` reader - Reserved."]
pub type RPT4_RESERVED4_ERR_1_R = crate::FieldReader;
#[doc = "Field `RPT4_RESERVED4_ERR_0` reader - Reserved."]
pub type RPT4_RESERVED4_ERR_0_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:21 - Indicates a programming error of HYS_EN_PAD1."]
    #[inline(always)]
    pub fn hys_en_pad1_err(&self) -> HYS_EN_PAD1_ERR_R {
        HYS_EN_PAD1_ERR_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 22:23 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved4_err_1(&self) -> RPT4_RESERVED4_ERR_1_R {
        RPT4_RESERVED4_ERR_1_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bits 24:31 - Reserved."]
    #[inline(always)]
    pub fn rpt4_reserved4_err_0(&self) -> RPT4_RESERVED4_ERR_0_R {
        RPT4_RESERVED4_ERR_0_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR4")
            .field(
                "hys_en_pad1_err",
                &format_args!("{}", self.hys_en_pad1_err().bits()),
            )
            .field(
                "rpt4_reserved4_err_1",
                &format_args!("{}", self.rpt4_reserved4_err_1().bits()),
            )
            .field(
                "rpt4_reserved4_err_0",
                &format_args!("{}", self.rpt4_reserved4_err_0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RD_REPEAT_ERR4_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Programming error record register 4 of BLOCK0.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rd_repeat_err4](index.html) module"]
pub struct RD_REPEAT_ERR4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rd_repeat_err4::R](R) reader structure"]
impl crate::Readable for RD_REPEAT_ERR4_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RD_REPEAT_ERR4 to value 0"]
impl crate::Resettable for RD_REPEAT_ERR4_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
