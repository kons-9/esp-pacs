#[doc = "Register `OUT_EOF_DES_ADDR_CH1` reader"]
pub struct R(crate::R<OUT_EOF_DES_ADDR_CH1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<OUT_EOF_DES_ADDR_CH1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<OUT_EOF_DES_ADDR_CH1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<OUT_EOF_DES_ADDR_CH1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `OUT_EOF_DES_ADDR` reader - This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1."]
pub type OUT_EOF_DES_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the address of the outlink descriptor when the EOF bit in this descriptor is 1."]
    #[inline(always)]
    pub fn out_eof_des_addr(&self) -> OUT_EOF_DES_ADDR_R {
        OUT_EOF_DES_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_EOF_DES_ADDR_CH1")
            .field(
                "out_eof_des_addr",
                &format_args!("{}", self.out_eof_des_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_EOF_DES_ADDR_CH1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "DMA_OUT_EOF_DES_ADDR_CH1_REG.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [out_eof_des_addr_ch1](index.html) module"]
pub struct OUT_EOF_DES_ADDR_CH1_SPEC;
impl crate::RegisterSpec for OUT_EOF_DES_ADDR_CH1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [out_eof_des_addr_ch1::R](R) reader structure"]
impl crate::Readable for OUT_EOF_DES_ADDR_CH1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets OUT_EOF_DES_ADDR_CH1 to value 0"]
impl crate::Resettable for OUT_EOF_DES_ADDR_CH1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
