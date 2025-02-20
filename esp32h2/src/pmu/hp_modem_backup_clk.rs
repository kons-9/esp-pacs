#[doc = "Register `HP_MODEM_BACKUP_CLK` reader"]
pub struct R(crate::R<HP_MODEM_BACKUP_CLK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<HP_MODEM_BACKUP_CLK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<HP_MODEM_BACKUP_CLK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<HP_MODEM_BACKUP_CLK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `HP_MODEM_BACKUP_CLK` writer"]
pub struct W(crate::W<HP_MODEM_BACKUP_CLK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<HP_MODEM_BACKUP_CLK_SPEC>;
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
impl From<crate::W<HP_MODEM_BACKUP_CLK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<HP_MODEM_BACKUP_CLK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `HP_MODEM_BACKUP_ICG_FUNC_EN` reader - need_des"]
pub type HP_MODEM_BACKUP_ICG_FUNC_EN_R = crate::FieldReader<u32>;
#[doc = "Field `HP_MODEM_BACKUP_ICG_FUNC_EN` writer - need_des"]
pub type HP_MODEM_BACKUP_ICG_FUNC_EN_W<'a, const O: u8> =
    crate::FieldWriter<'a, HP_MODEM_BACKUP_CLK_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn hp_modem_backup_icg_func_en(&self) -> HP_MODEM_BACKUP_ICG_FUNC_EN_R {
        HP_MODEM_BACKUP_ICG_FUNC_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_MODEM_BACKUP_CLK")
            .field(
                "hp_modem_backup_icg_func_en",
                &format_args!("{}", self.hp_modem_backup_icg_func_en().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HP_MODEM_BACKUP_CLK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn hp_modem_backup_icg_func_en(&mut self) -> HP_MODEM_BACKUP_ICG_FUNC_EN_W<0> {
        HP_MODEM_BACKUP_ICG_FUNC_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [hp_modem_backup_clk](index.html) module"]
pub struct HP_MODEM_BACKUP_CLK_SPEC;
impl crate::RegisterSpec for HP_MODEM_BACKUP_CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [hp_modem_backup_clk::R](R) reader structure"]
impl crate::Readable for HP_MODEM_BACKUP_CLK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [hp_modem_backup_clk::W](W) writer structure"]
impl crate::Writable for HP_MODEM_BACKUP_CLK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets HP_MODEM_BACKUP_CLK to value 0"]
impl crate::Resettable for HP_MODEM_BACKUP_CLK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
