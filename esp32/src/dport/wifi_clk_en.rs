#[doc = "Register `WIFI_CLK_EN` reader"]
pub struct R(crate::R<WIFI_CLK_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WIFI_CLK_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WIFI_CLK_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WIFI_CLK_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WIFI_CLK_EN` writer"]
pub struct W(crate::W<WIFI_CLK_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WIFI_CLK_EN_SPEC>;
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
impl From<crate::W<WIFI_CLK_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WIFI_CLK_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WIFI_CLK_EN` reader - "]
pub type WIFI_CLK_EN_R = crate::FieldReader<u32>;
#[doc = "Field `WIFI_CLK_EN` writer - "]
pub type WIFI_CLK_EN_W<'a, const O: u8> = crate::FieldWriter<'a, WIFI_CLK_EN_SPEC, 32, O, u32>;
#[doc = "Field `WIFI_CLK_WIFI_EN` reader - "]
pub type WIFI_CLK_WIFI_EN_R = crate::FieldReader;
#[doc = "Field `WIFI_CLK_WIFI_EN` writer - "]
pub type WIFI_CLK_WIFI_EN_W<'a, const O: u8> = crate::FieldWriter<'a, WIFI_CLK_EN_SPEC, 3, O>;
#[doc = "Field `WIFI_CLK_WIFI_BT_COMMON` reader - "]
pub type WIFI_CLK_WIFI_BT_COMMON_R = crate::FieldReader;
#[doc = "Field `WIFI_CLK_WIFI_BT_COMMON` writer - "]
pub type WIFI_CLK_WIFI_BT_COMMON_W<'a, const O: u8> =
    crate::FieldWriter<'a, WIFI_CLK_EN_SPEC, 6, O>;
#[doc = "Field `WIFI_CLK_BT_EN` reader - "]
pub type WIFI_CLK_BT_EN_R = crate::FieldReader;
#[doc = "Field `WIFI_CLK_BT_EN` writer - "]
pub type WIFI_CLK_BT_EN_W<'a, const O: u8> = crate::FieldWriter<'a, WIFI_CLK_EN_SPEC, 3, O>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn wifi_clk_en(&self) -> WIFI_CLK_EN_R {
        WIFI_CLK_EN_R::new(self.bits)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn wifi_clk_wifi_en(&self) -> WIFI_CLK_WIFI_EN_R {
        WIFI_CLK_WIFI_EN_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn wifi_clk_wifi_bt_common(&self) -> WIFI_CLK_WIFI_BT_COMMON_R {
        WIFI_CLK_WIFI_BT_COMMON_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn wifi_clk_bt_en(&self) -> WIFI_CLK_BT_EN_R {
        WIFI_CLK_BT_EN_R::new(((self.bits >> 11) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WIFI_CLK_EN")
            .field(
                "wifi_clk_en",
                &format_args!("{}", self.wifi_clk_en().bits()),
            )
            .field(
                "wifi_clk_wifi_en",
                &format_args!("{}", self.wifi_clk_wifi_en().bits()),
            )
            .field(
                "wifi_clk_wifi_bt_common",
                &format_args!("{}", self.wifi_clk_wifi_bt_common().bits()),
            )
            .field(
                "wifi_clk_bt_en",
                &format_args!("{}", self.wifi_clk_bt_en().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WIFI_CLK_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_clk_en(&mut self) -> WIFI_CLK_EN_W<0> {
        WIFI_CLK_EN_W::new(self)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_clk_wifi_en(&mut self) -> WIFI_CLK_WIFI_EN_W<0> {
        WIFI_CLK_WIFI_EN_W::new(self)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_clk_wifi_bt_common(&mut self) -> WIFI_CLK_WIFI_BT_COMMON_W<0> {
        WIFI_CLK_WIFI_BT_COMMON_W::new(self)
    }
    #[doc = "Bits 11:13"]
    #[inline(always)]
    #[must_use]
    pub fn wifi_clk_bt_en(&mut self) -> WIFI_CLK_BT_EN_W<11> {
        WIFI_CLK_BT_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wifi_clk_en](index.html) module"]
pub struct WIFI_CLK_EN_SPEC;
impl crate::RegisterSpec for WIFI_CLK_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wifi_clk_en::R](R) reader structure"]
impl crate::Readable for WIFI_CLK_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wifi_clk_en::W](W) writer structure"]
impl crate::Writable for WIFI_CLK_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WIFI_CLK_EN to value 0xfffc_e030"]
impl crate::Resettable for WIFI_CLK_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0xfffc_e030;
}
