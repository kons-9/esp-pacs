#[doc = "Register `DCACHE_ATOMIC_OPERATE_ENA` reader"]
pub struct R(crate::R<DCACHE_ATOMIC_OPERATE_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DCACHE_ATOMIC_OPERATE_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DCACHE_ATOMIC_OPERATE_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DCACHE_ATOMIC_OPERATE_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `DCACHE_ATOMIC_OPERATE_ENA` writer"]
pub struct W(crate::W<DCACHE_ATOMIC_OPERATE_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<DCACHE_ATOMIC_OPERATE_ENA_SPEC>;
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
impl From<crate::W<DCACHE_ATOMIC_OPERATE_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<DCACHE_ATOMIC_OPERATE_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DCACHE_ATOMIC_OPERATE_ENA` reader - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub type DCACHE_ATOMIC_OPERATE_ENA_R = crate::BitReader;
#[doc = "Field `DCACHE_ATOMIC_OPERATE_ENA` writer - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
pub type DCACHE_ATOMIC_OPERATE_ENA_W<'a, const O: u8> =
    crate::BitWriter<'a, DCACHE_ATOMIC_OPERATE_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    pub fn dcache_atomic_operate_ena(&self) -> DCACHE_ATOMIC_OPERATE_ENA_R {
        DCACHE_ATOMIC_OPERATE_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DCACHE_ATOMIC_OPERATE_ENA")
            .field(
                "dcache_atomic_operate_ena",
                &format_args!("{}", self.dcache_atomic_operate_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DCACHE_ATOMIC_OPERATE_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - The bit is used to activate dcache atomic operation protection. In this case, sync/lock/occupy operation can not interrupt miss-work. This feature does not work during invalidateAll operation."]
    #[inline(always)]
    #[must_use]
    pub fn dcache_atomic_operate_ena(&mut self) -> DCACHE_ATOMIC_OPERATE_ENA_W<0> {
        DCACHE_ATOMIC_OPERATE_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "******* Description ***********\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dcache_atomic_operate_ena](index.html) module"]
pub struct DCACHE_ATOMIC_OPERATE_ENA_SPEC;
impl crate::RegisterSpec for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dcache_atomic_operate_ena::R](R) reader structure"]
impl crate::Readable for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [dcache_atomic_operate_ena::W](W) writer structure"]
impl crate::Writable for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets DCACHE_ATOMIC_OPERATE_ENA to value 0x01"]
impl crate::Resettable for DCACHE_ATOMIC_OPERATE_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
