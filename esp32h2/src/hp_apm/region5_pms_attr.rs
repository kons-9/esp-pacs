#[doc = "Register `REGION5_PMS_ATTR` reader"]
pub struct R(crate::R<REGION5_PMS_ATTR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<REGION5_PMS_ATTR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<REGION5_PMS_ATTR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<REGION5_PMS_ATTR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `REGION5_PMS_ATTR` writer"]
pub struct W(crate::W<REGION5_PMS_ATTR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REGION5_PMS_ATTR_SPEC>;
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
impl From<crate::W<REGION5_PMS_ATTR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REGION5_PMS_ATTR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `REGION5_R0_PMS_X` reader - Region execute authority in REE_MODE0"]
pub type REGION5_R0_PMS_X_R = crate::BitReader<bool>;
#[doc = "Field `REGION5_R0_PMS_X` writer - Region execute authority in REE_MODE0"]
pub type REGION5_R0_PMS_X_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REGION5_PMS_ATTR_SPEC, bool, O>;
#[doc = "Field `REGION5_R0_PMS_W` reader - Region write authority in REE_MODE0"]
pub type REGION5_R0_PMS_W_R = crate::BitReader<bool>;
#[doc = "Field `REGION5_R0_PMS_W` writer - Region write authority in REE_MODE0"]
pub type REGION5_R0_PMS_W_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REGION5_PMS_ATTR_SPEC, bool, O>;
#[doc = "Field `REGION5_R0_PMS_R` reader - Region read authority in REE_MODE0"]
pub type REGION5_R0_PMS_R_R = crate::BitReader<bool>;
#[doc = "Field `REGION5_R0_PMS_R` writer - Region read authority in REE_MODE0"]
pub type REGION5_R0_PMS_R_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REGION5_PMS_ATTR_SPEC, bool, O>;
#[doc = "Field `REGION5_R1_PMS_X` reader - Region execute authority in REE_MODE1"]
pub type REGION5_R1_PMS_X_R = crate::BitReader<bool>;
#[doc = "Field `REGION5_R1_PMS_X` writer - Region execute authority in REE_MODE1"]
pub type REGION5_R1_PMS_X_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REGION5_PMS_ATTR_SPEC, bool, O>;
#[doc = "Field `REGION5_R1_PMS_W` reader - Region write authority in REE_MODE1"]
pub type REGION5_R1_PMS_W_R = crate::BitReader<bool>;
#[doc = "Field `REGION5_R1_PMS_W` writer - Region write authority in REE_MODE1"]
pub type REGION5_R1_PMS_W_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REGION5_PMS_ATTR_SPEC, bool, O>;
#[doc = "Field `REGION5_R1_PMS_R` reader - Region read authority in REE_MODE1"]
pub type REGION5_R1_PMS_R_R = crate::BitReader<bool>;
#[doc = "Field `REGION5_R1_PMS_R` writer - Region read authority in REE_MODE1"]
pub type REGION5_R1_PMS_R_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REGION5_PMS_ATTR_SPEC, bool, O>;
#[doc = "Field `REGION5_R2_PMS_X` reader - Region execute authority in REE_MODE2"]
pub type REGION5_R2_PMS_X_R = crate::BitReader<bool>;
#[doc = "Field `REGION5_R2_PMS_X` writer - Region execute authority in REE_MODE2"]
pub type REGION5_R2_PMS_X_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REGION5_PMS_ATTR_SPEC, bool, O>;
#[doc = "Field `REGION5_R2_PMS_W` reader - Region write authority in REE_MODE2"]
pub type REGION5_R2_PMS_W_R = crate::BitReader<bool>;
#[doc = "Field `REGION5_R2_PMS_W` writer - Region write authority in REE_MODE2"]
pub type REGION5_R2_PMS_W_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REGION5_PMS_ATTR_SPEC, bool, O>;
#[doc = "Field `REGION5_R2_PMS_R` reader - Region read authority in REE_MODE2"]
pub type REGION5_R2_PMS_R_R = crate::BitReader<bool>;
#[doc = "Field `REGION5_R2_PMS_R` writer - Region read authority in REE_MODE2"]
pub type REGION5_R2_PMS_R_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, REGION5_PMS_ATTR_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Region execute authority in REE_MODE0"]
    #[inline(always)]
    pub fn region5_r0_pms_x(&self) -> REGION5_R0_PMS_X_R {
        REGION5_R0_PMS_X_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Region write authority in REE_MODE0"]
    #[inline(always)]
    pub fn region5_r0_pms_w(&self) -> REGION5_R0_PMS_W_R {
        REGION5_R0_PMS_W_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Region read authority in REE_MODE0"]
    #[inline(always)]
    pub fn region5_r0_pms_r(&self) -> REGION5_R0_PMS_R_R {
        REGION5_R0_PMS_R_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - Region execute authority in REE_MODE1"]
    #[inline(always)]
    pub fn region5_r1_pms_x(&self) -> REGION5_R1_PMS_X_R {
        REGION5_R1_PMS_X_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Region write authority in REE_MODE1"]
    #[inline(always)]
    pub fn region5_r1_pms_w(&self) -> REGION5_R1_PMS_W_R {
        REGION5_R1_PMS_W_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Region read authority in REE_MODE1"]
    #[inline(always)]
    pub fn region5_r1_pms_r(&self) -> REGION5_R1_PMS_R_R {
        REGION5_R1_PMS_R_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 8 - Region execute authority in REE_MODE2"]
    #[inline(always)]
    pub fn region5_r2_pms_x(&self) -> REGION5_R2_PMS_X_R {
        REGION5_R2_PMS_X_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Region write authority in REE_MODE2"]
    #[inline(always)]
    pub fn region5_r2_pms_w(&self) -> REGION5_R2_PMS_W_R {
        REGION5_R2_PMS_W_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Region read authority in REE_MODE2"]
    #[inline(always)]
    pub fn region5_r2_pms_r(&self) -> REGION5_R2_PMS_R_R {
        REGION5_R2_PMS_R_R::new(((self.bits >> 10) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Region execute authority in REE_MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn region5_r0_pms_x(&mut self) -> REGION5_R0_PMS_X_W<0> {
        REGION5_R0_PMS_X_W::new(self)
    }
    #[doc = "Bit 1 - Region write authority in REE_MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn region5_r0_pms_w(&mut self) -> REGION5_R0_PMS_W_W<1> {
        REGION5_R0_PMS_W_W::new(self)
    }
    #[doc = "Bit 2 - Region read authority in REE_MODE0"]
    #[inline(always)]
    #[must_use]
    pub fn region5_r0_pms_r(&mut self) -> REGION5_R0_PMS_R_W<2> {
        REGION5_R0_PMS_R_W::new(self)
    }
    #[doc = "Bit 4 - Region execute authority in REE_MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn region5_r1_pms_x(&mut self) -> REGION5_R1_PMS_X_W<4> {
        REGION5_R1_PMS_X_W::new(self)
    }
    #[doc = "Bit 5 - Region write authority in REE_MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn region5_r1_pms_w(&mut self) -> REGION5_R1_PMS_W_W<5> {
        REGION5_R1_PMS_W_W::new(self)
    }
    #[doc = "Bit 6 - Region read authority in REE_MODE1"]
    #[inline(always)]
    #[must_use]
    pub fn region5_r1_pms_r(&mut self) -> REGION5_R1_PMS_R_W<6> {
        REGION5_R1_PMS_R_W::new(self)
    }
    #[doc = "Bit 8 - Region execute authority in REE_MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn region5_r2_pms_x(&mut self) -> REGION5_R2_PMS_X_W<8> {
        REGION5_R2_PMS_X_W::new(self)
    }
    #[doc = "Bit 9 - Region write authority in REE_MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn region5_r2_pms_w(&mut self) -> REGION5_R2_PMS_W_W<9> {
        REGION5_R2_PMS_W_W::new(self)
    }
    #[doc = "Bit 10 - Region read authority in REE_MODE2"]
    #[inline(always)]
    #[must_use]
    pub fn region5_r2_pms_r(&mut self) -> REGION5_R2_PMS_R_W<10> {
        REGION5_R2_PMS_R_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Region access authority attribute register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [region5_pms_attr](index.html) module"]
pub struct REGION5_PMS_ATTR_SPEC;
impl crate::RegisterSpec for REGION5_PMS_ATTR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [region5_pms_attr::R](R) reader structure"]
impl crate::Readable for REGION5_PMS_ATTR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [region5_pms_attr::W](W) writer structure"]
impl crate::Writable for REGION5_PMS_ATTR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REGION5_PMS_ATTR to value 0"]
impl crate::Resettable for REGION5_PMS_ATTR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
