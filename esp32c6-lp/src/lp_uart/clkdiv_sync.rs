#[doc = "Register `CLKDIV_SYNC` reader"]
pub struct R(crate::R<CLKDIV_SYNC_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLKDIV_SYNC_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLKDIV_SYNC_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLKDIV_SYNC_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CLKDIV_SYNC` writer"]
pub struct W(crate::W<CLKDIV_SYNC_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CLKDIV_SYNC_SPEC>;
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
impl From<crate::W<CLKDIV_SYNC_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CLKDIV_SYNC_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CLKDIV` reader - The integral part of the frequency divider factor."]
pub type CLKDIV_R = crate::FieldReader<u16>;
#[doc = "Field `CLKDIV` writer - The integral part of the frequency divider factor."]
pub type CLKDIV_W<'a, const O: u8> = crate::FieldWriter<'a, CLKDIV_SYNC_SPEC, 12, O, u16>;
#[doc = "Field `CLKDIV_FRAG` reader - The decimal part of the frequency divider factor."]
pub type CLKDIV_FRAG_R = crate::FieldReader;
#[doc = "Field `CLKDIV_FRAG` writer - The decimal part of the frequency divider factor."]
pub type CLKDIV_FRAG_W<'a, const O: u8> = crate::FieldWriter<'a, CLKDIV_SYNC_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:11 - The integral part of the frequency divider factor."]
    #[inline(always)]
    pub fn clkdiv(&self) -> CLKDIV_R {
        CLKDIV_R::new((self.bits & 0x0fff) as u16)
    }
    #[doc = "Bits 20:23 - The decimal part of the frequency divider factor."]
    #[inline(always)]
    pub fn clkdiv_frag(&self) -> CLKDIV_FRAG_R {
        CLKDIV_FRAG_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLKDIV_SYNC")
            .field("clkdiv", &format_args!("{}", self.clkdiv().bits()))
            .field(
                "clkdiv_frag",
                &format_args!("{}", self.clkdiv_frag().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLKDIV_SYNC_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:11 - The integral part of the frequency divider factor."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv(&mut self) -> CLKDIV_W<0> {
        CLKDIV_W::new(self)
    }
    #[doc = "Bits 20:23 - The decimal part of the frequency divider factor."]
    #[inline(always)]
    #[must_use]
    pub fn clkdiv_frag(&mut self) -> CLKDIV_FRAG_W<20> {
        CLKDIV_FRAG_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Clock divider configuration\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clkdiv_sync](index.html) module"]
pub struct CLKDIV_SYNC_SPEC;
impl crate::RegisterSpec for CLKDIV_SYNC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clkdiv_sync::R](R) reader structure"]
impl crate::Readable for CLKDIV_SYNC_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [clkdiv_sync::W](W) writer structure"]
impl crate::Writable for CLKDIV_SYNC_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CLKDIV_SYNC to value 0x02b6"]
impl crate::Resettable for CLKDIV_SYNC_SPEC {
    const RESET_VALUE: Self::Ux = 0x02b6;
}
