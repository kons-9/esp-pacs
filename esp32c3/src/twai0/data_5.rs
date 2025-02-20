#[doc = "Register `DATA_5` reader"]
pub struct R(crate::R<DATA_5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DATA_5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DATA_5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DATA_5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DATA_5` writer"]
pub struct W(crate::W<DATA_5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DATA_5_SPEC>;
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
impl From<crate::W<DATA_5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DATA_5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `TX_BYTE_5` reader - In reset mode, it is acceptance code register 5 with R/W Permission. In operation mode, it stores the 5th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_5_R = crate::FieldReader;
#[doc = "Field `TX_BYTE_5` writer - In reset mode, it is acceptance code register 5 with R/W Permission. In operation mode, it stores the 5th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
pub type TX_BYTE_5_W<'a, const O: u8> = crate::FieldWriter<'a, DATA_5_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 5 with R/W Permission. In operation mode, it stores the 5th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    pub fn tx_byte_5(&self) -> TX_BYTE_5_R {
        TX_BYTE_5_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DATA_5")
            .field("tx_byte_5", &format_args!("{}", self.tx_byte_5().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DATA_5_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - In reset mode, it is acceptance code register 5 with R/W Permission. In operation mode, it stores the 5th byte of the data to be transmitted or received. In operation mode, writing writes to the transmit buffer while reading reads from the receive buffer."]
    #[inline(always)]
    #[must_use]
    pub fn tx_byte_5(&mut self) -> TX_BYTE_5_W<0> {
        TX_BYTE_5_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [data_5](index.html) module"]
pub struct DATA_5_SPEC;
impl crate::RegisterSpec for DATA_5_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [data_5::R](R) reader structure"]
impl crate::Readable for DATA_5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [data_5::W](W) writer structure"]
impl crate::Writable for DATA_5_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DATA_5 to value 0"]
impl crate::Resettable for DATA_5_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
