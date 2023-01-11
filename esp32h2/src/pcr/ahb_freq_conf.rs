#[doc = "Register `AHB_FREQ_CONF` reader"]
pub struct R(crate::R<AHB_FREQ_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<AHB_FREQ_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<AHB_FREQ_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<AHB_FREQ_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `AHB_FREQ_CONF` writer"]
pub struct W(crate::W<AHB_FREQ_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<AHB_FREQ_CONF_SPEC>;
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
impl From<crate::W<AHB_FREQ_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<AHB_FREQ_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `AHB_DIV_NUM` reader - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
pub type AHB_DIV_NUM_R = crate::FieldReader<u8, u8>;
#[doc = "Field `AHB_DIV_NUM` writer - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
pub type AHB_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, AHB_FREQ_CONF_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
    #[inline(always)]
    pub fn ahb_div_num(&self) -> AHB_DIV_NUM_R {
        AHB_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set this field to generate clk_ahb drived by clk_hproot. The clk_ahb is div1(default)/div2/div4/div8 of clk_hproot. This field is only avaliable for low-speed clock-source such as XTAL/FOSC, and should be used together with PCR_CPU_DIV_NUM."]
    #[inline(always)]
    #[must_use]
    pub fn ahb_div_num(&mut self) -> AHB_DIV_NUM_W<0> {
        AHB_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "AHB_FREQ configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ahb_freq_conf](index.html) module"]
pub struct AHB_FREQ_CONF_SPEC;
impl crate::RegisterSpec for AHB_FREQ_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ahb_freq_conf::R](R) reader structure"]
impl crate::Readable for AHB_FREQ_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [ahb_freq_conf::W](W) writer structure"]
impl crate::Writable for AHB_FREQ_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets AHB_FREQ_CONF to value 0"]
impl crate::Resettable for AHB_FREQ_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
