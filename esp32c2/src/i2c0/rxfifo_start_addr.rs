#[doc = "Register `RXFIFO_START_ADDR` reader"]
pub struct R(crate::R<RXFIFO_START_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RXFIFO_START_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RXFIFO_START_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RXFIFO_START_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RXFIFO_START_ADDR` reader - This is the I2C rxfifo first address."]
pub type RXFIFO_START_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This is the I2C rxfifo first address."]
    #[inline(always)]
    pub fn rxfifo_start_addr(&self) -> RXFIFO_START_ADDR_R {
        RXFIFO_START_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RXFIFO_START_ADDR")
            .field(
                "rxfifo_start_addr",
                &format_args!("{}", self.rxfifo_start_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RXFIFO_START_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "I2C RXFIFO base address register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rxfifo_start_addr](index.html) module"]
pub struct RXFIFO_START_ADDR_SPEC;
impl crate::RegisterSpec for RXFIFO_START_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rxfifo_start_addr::R](R) reader structure"]
impl crate::Readable for RXFIFO_START_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets RXFIFO_START_ADDR to value 0"]
impl crate::Resettable for RXFIFO_START_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
