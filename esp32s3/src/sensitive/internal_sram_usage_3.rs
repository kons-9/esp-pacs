#[doc = "Register `INTERNAL_SRAM_USAGE_3` reader"]
pub struct R(crate::R<INTERNAL_SRAM_USAGE_3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INTERNAL_SRAM_USAGE_3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INTERNAL_SRAM_USAGE_3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INTERNAL_SRAM_USAGE_3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INTERNAL_SRAM_USAGE_3` writer"]
pub struct W(crate::W<INTERNAL_SRAM_USAGE_3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INTERNAL_SRAM_USAGE_3_SPEC>;
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
impl From<crate::W<INTERNAL_SRAM_USAGE_3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INTERNAL_SRAM_USAGE_3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `INTERNAL_SRAM_MAC_DUMP_USAGE` reader - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
pub type INTERNAL_SRAM_MAC_DUMP_USAGE_R = crate::FieldReader;
#[doc = "Field `INTERNAL_SRAM_MAC_DUMP_USAGE` writer - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
pub type INTERNAL_SRAM_MAC_DUMP_USAGE_W<'a, const O: u8> =
    crate::FieldWriter<'a, INTERNAL_SRAM_USAGE_3_SPEC, 4, O>;
impl R {
    #[doc = "Bits 0:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
    #[inline(always)]
    pub fn internal_sram_mac_dump_usage(&self) -> INTERNAL_SRAM_MAC_DUMP_USAGE_R {
        INTERNAL_SRAM_MAC_DUMP_USAGE_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERNAL_SRAM_USAGE_3")
            .field(
                "internal_sram_mac_dump_usage",
                &format_args!("{}", self.internal_sram_mac_dump_usage().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INTERNAL_SRAM_USAGE_3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:3 - Set 1 to someone bit means corresponding internal SRAM level can be accessed by mac dump."]
    #[inline(always)]
    #[must_use]
    pub fn internal_sram_mac_dump_usage(&mut self) -> INTERNAL_SRAM_MAC_DUMP_USAGE_W<0> {
        INTERNAL_SRAM_MAC_DUMP_USAGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Internal SRAM configuration register 3.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [internal_sram_usage_3](index.html) module"]
pub struct INTERNAL_SRAM_USAGE_3_SPEC;
impl crate::RegisterSpec for INTERNAL_SRAM_USAGE_3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [internal_sram_usage_3::R](R) reader structure"]
impl crate::Readable for INTERNAL_SRAM_USAGE_3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [internal_sram_usage_3::W](W) writer structure"]
impl crate::Writable for INTERNAL_SRAM_USAGE_3_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INTERNAL_SRAM_USAGE_3 to value 0"]
impl crate::Resettable for INTERNAL_SRAM_USAGE_3_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
