#[doc = "Register `W8` reader"]
pub struct R(crate::R<W8_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<W8_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<W8_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<W8_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `W8` writer"]
pub struct W(crate::W<W8_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<W8_SPEC>;
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
impl From<crate::W<W8_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<W8_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `BUF8` reader - 32 bits data buffer 8, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF8_R = crate::FieldReader<u32>;
#[doc = "Field `BUF8` writer - 32 bits data buffer 8, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
pub type BUF8_W<'a, const O: u8> = crate::FieldWriter<'a, W8_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - 32 bits data buffer 8, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    pub fn buf8(&self) -> BUF8_R {
        BUF8_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("W8")
            .field("buf8", &format_args!("{}", self.buf8().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<W8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - 32 bits data buffer 8, transferred in the unit of byte. Byte addressable in slave half-duplex mode."]
    #[inline(always)]
    #[must_use]
    pub fn buf8(&mut self) -> BUF8_W<0> {
        BUF8_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Data buffer 8\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [w8](index.html) module"]
pub struct W8_SPEC;
impl crate::RegisterSpec for W8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [w8::R](R) reader structure"]
impl crate::Readable for W8_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [w8::W](W) writer structure"]
impl crate::Writable for W8_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets W8 to value 0"]
impl crate::Resettable for W8_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
