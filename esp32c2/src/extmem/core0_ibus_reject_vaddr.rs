#[doc = "Register `CORE0_IBUS_REJECT_VADDR` reader"]
pub struct R(crate::R<CORE0_IBUS_REJECT_VADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE0_IBUS_REJECT_VADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE0_IBUS_REJECT_VADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE0_IBUS_REJECT_VADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CORE0_IBUS_VADDR` reader - The bits are used to indicate the virtual address of CPU access ibus when authentication fail."]
pub type CORE0_IBUS_VADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - The bits are used to indicate the virtual address of CPU access ibus when authentication fail."]
    #[inline(always)]
    pub fn core0_ibus_vaddr(&self) -> CORE0_IBUS_VADDR_R {
        CORE0_IBUS_VADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CORE0_IBUS_REJECT_VADDR")
            .field(
                "core0_ibus_vaddr",
                &format_args!("{}", self.core0_ibus_vaddr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE0_IBUS_REJECT_VADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "This description will be updated in the near future.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core0_ibus_reject_vaddr](index.html) module"]
pub struct CORE0_IBUS_REJECT_VADDR_SPEC;
impl crate::RegisterSpec for CORE0_IBUS_REJECT_VADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core0_ibus_reject_vaddr::R](R) reader structure"]
impl crate::Readable for CORE0_IBUS_REJECT_VADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CORE0_IBUS_REJECT_VADDR to value 0xffff_ffff"]
impl crate::Resettable for CORE0_IBUS_REJECT_VADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
