#[doc = "Register `CMD_INFOR0` reader"]
pub struct R(crate::R<CMD_INFOR0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMD_INFOR0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMD_INFOR0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMD_INFOR0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CMD_CONTENT0` reader - "]
pub type CMD_CONTENT0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn cmd_content0(&self) -> CMD_CONTENT0_R {
        CMD_CONTENT0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CMD_INFOR0")
            .field(
                "cmd_content0",
                &format_args!("{}", self.cmd_content0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CMD_INFOR0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmd_infor0](index.html) module"]
pub struct CMD_INFOR0_SPEC;
impl crate::RegisterSpec for CMD_INFOR0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmd_infor0::R](R) reader structure"]
impl crate::Readable for CMD_INFOR0_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CMD_INFOR0 to value 0"]
impl crate::Resettable for CMD_INFOR0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
