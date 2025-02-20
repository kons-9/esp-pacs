#[doc = "Register `MODE` reader"]
pub struct R(crate::R<MODE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MODE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MODE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MODE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MODE` writer"]
pub struct W(crate::W<MODE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MODE_SPEC>;
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
impl From<crate::W<MODE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MODE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RESET_MODE` reader - This bit is used to configure the operating mode of the TWAI Controller. 1: Reset mode; 0: Operating mode."]
pub type RESET_MODE_R = crate::BitReader;
#[doc = "Field `RESET_MODE` writer - This bit is used to configure the operating mode of the TWAI Controller. 1: Reset mode; 0: Operating mode."]
pub type RESET_MODE_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O>;
#[doc = "Field `LISTEN_ONLY_MODE` reader - 1: Listen only mode. In this mode the nodes will only receive messages from the bus, without generating the acknowledge signal nor updating the RX error counter."]
pub type LISTEN_ONLY_MODE_R = crate::BitReader;
#[doc = "Field `LISTEN_ONLY_MODE` writer - 1: Listen only mode. In this mode the nodes will only receive messages from the bus, without generating the acknowledge signal nor updating the RX error counter."]
pub type LISTEN_ONLY_MODE_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O>;
#[doc = "Field `SELF_TEST_MODE` reader - 1: Self test mode. In this mode the TX nodes can perform a successful transmission without receiving the acknowledge signal. This mode is often used to test a single node with the self reception request command."]
pub type SELF_TEST_MODE_R = crate::BitReader;
#[doc = "Field `SELF_TEST_MODE` writer - 1: Self test mode. In this mode the TX nodes can perform a successful transmission without receiving the acknowledge signal. This mode is often used to test a single node with the self reception request command."]
pub type SELF_TEST_MODE_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O>;
#[doc = "Field `RX_FILTER_MODE` reader - This bit is used to configure the filter mode. 0: Dual filter mode; 1: Single filter mode."]
pub type RX_FILTER_MODE_R = crate::BitReader;
#[doc = "Field `RX_FILTER_MODE` writer - This bit is used to configure the filter mode. 0: Dual filter mode; 1: Single filter mode."]
pub type RX_FILTER_MODE_W<'a, const O: u8> = crate::BitWriter<'a, MODE_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This bit is used to configure the operating mode of the TWAI Controller. 1: Reset mode; 0: Operating mode."]
    #[inline(always)]
    pub fn reset_mode(&self) -> RESET_MODE_R {
        RESET_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Listen only mode. In this mode the nodes will only receive messages from the bus, without generating the acknowledge signal nor updating the RX error counter."]
    #[inline(always)]
    pub fn listen_only_mode(&self) -> LISTEN_ONLY_MODE_R {
        LISTEN_ONLY_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - 1: Self test mode. In this mode the TX nodes can perform a successful transmission without receiving the acknowledge signal. This mode is often used to test a single node with the self reception request command."]
    #[inline(always)]
    pub fn self_test_mode(&self) -> SELF_TEST_MODE_R {
        SELF_TEST_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This bit is used to configure the filter mode. 0: Dual filter mode; 1: Single filter mode."]
    #[inline(always)]
    pub fn rx_filter_mode(&self) -> RX_FILTER_MODE_R {
        RX_FILTER_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MODE")
            .field("reset_mode", &format_args!("{}", self.reset_mode().bit()))
            .field(
                "listen_only_mode",
                &format_args!("{}", self.listen_only_mode().bit()),
            )
            .field(
                "self_test_mode",
                &format_args!("{}", self.self_test_mode().bit()),
            )
            .field(
                "rx_filter_mode",
                &format_args!("{}", self.rx_filter_mode().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MODE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This bit is used to configure the operating mode of the TWAI Controller. 1: Reset mode; 0: Operating mode."]
    #[inline(always)]
    #[must_use]
    pub fn reset_mode(&mut self) -> RESET_MODE_W<0> {
        RESET_MODE_W::new(self)
    }
    #[doc = "Bit 1 - 1: Listen only mode. In this mode the nodes will only receive messages from the bus, without generating the acknowledge signal nor updating the RX error counter."]
    #[inline(always)]
    #[must_use]
    pub fn listen_only_mode(&mut self) -> LISTEN_ONLY_MODE_W<1> {
        LISTEN_ONLY_MODE_W::new(self)
    }
    #[doc = "Bit 2 - 1: Self test mode. In this mode the TX nodes can perform a successful transmission without receiving the acknowledge signal. This mode is often used to test a single node with the self reception request command."]
    #[inline(always)]
    #[must_use]
    pub fn self_test_mode(&mut self) -> SELF_TEST_MODE_W<2> {
        SELF_TEST_MODE_W::new(self)
    }
    #[doc = "Bit 3 - This bit is used to configure the filter mode. 0: Dual filter mode; 1: Single filter mode."]
    #[inline(always)]
    #[must_use]
    pub fn rx_filter_mode(&mut self) -> RX_FILTER_MODE_W<3> {
        RX_FILTER_MODE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Mode Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mode](index.html) module"]
pub struct MODE_SPEC;
impl crate::RegisterSpec for MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mode::R](R) reader structure"]
impl crate::Readable for MODE_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mode::W](W) writer structure"]
impl crate::Writable for MODE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MODE to value 0x01"]
impl crate::Resettable for MODE_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
