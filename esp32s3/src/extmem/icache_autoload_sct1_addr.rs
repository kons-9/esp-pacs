#[doc = "Register `ICACHE_AUTOLOAD_SCT1_ADDR` reader"]
pub struct R(crate::R<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ICACHE_AUTOLOAD_SCT1_ADDR` writer"]
pub struct W(crate::W<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>;
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
impl From<crate::W<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ICACHE_AUTOLOAD_SCT1_ADDR` reader - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena."]
pub type ICACHE_AUTOLOAD_SCT1_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `ICACHE_AUTOLOAD_SCT1_ADDR` writer - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena."]
pub type ICACHE_AUTOLOAD_SCT1_ADDR_W<'a, const O: u8> =
    crate::FieldWriter<'a, ICACHE_AUTOLOAD_SCT1_ADDR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena."]
    #[inline(always)]
    pub fn icache_autoload_sct1_addr(&self) -> ICACHE_AUTOLOAD_SCT1_ADDR_R {
        ICACHE_AUTOLOAD_SCT1_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICACHE_AUTOLOAD_SCT1_ADDR")
            .field(
                "icache_autoload_sct1_addr",
                &format_args!("{}", self.icache_autoload_sct1_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ICACHE_AUTOLOAD_SCT1_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - The bits are used to configure the start virtual address of the second section for autoload operation. It should be combined with icache_autoload_sct1_ena."]
    #[inline(always)]
    #[must_use]
    pub fn icache_autoload_sct1_addr(&mut self) -> ICACHE_AUTOLOAD_SCT1_ADDR_W<0> {
        ICACHE_AUTOLOAD_SCT1_ADDR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [icache_autoload_sct1_addr](index.html) module"]
pub struct ICACHE_AUTOLOAD_SCT1_ADDR_SPEC;
impl crate::RegisterSpec for ICACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [icache_autoload_sct1_addr::R](R) reader structure"]
impl crate::Readable for ICACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [icache_autoload_sct1_addr::W](W) writer structure"]
impl crate::Writable for ICACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ICACHE_AUTOLOAD_SCT1_ADDR to value 0"]
impl crate::Resettable for ICACHE_AUTOLOAD_SCT1_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
