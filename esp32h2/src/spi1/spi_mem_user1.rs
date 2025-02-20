#[doc = "Register `SPI_MEM_USER1` reader"]
pub struct R(crate::R<SPI_MEM_USER1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_USER1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_USER1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_USER1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_USER1` writer"]
pub struct W(crate::W<SPI_MEM_USER1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_USER1_SPEC>;
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
impl From<crate::W<SPI_MEM_USER1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_USER1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_USR_DUMMY_CYCLELEN` reader - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub type SPI_MEM_USR_DUMMY_CYCLELEN_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_USR_DUMMY_CYCLELEN` writer - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
pub type SPI_MEM_USR_DUMMY_CYCLELEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_USER1_SPEC, 6, O>;
#[doc = "Field `SPI_MEM_USR_ADDR_BITLEN` reader - The length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_ADDR_BITLEN_R = crate::FieldReader;
#[doc = "Field `SPI_MEM_USR_ADDR_BITLEN` writer - The length in bits of address phase. The register value shall be (bit_num-1)."]
pub type SPI_MEM_USR_ADDR_BITLEN_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_USER1_SPEC, 6, O>;
impl R {
    #[doc = "Bits 0:5 - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_dummy_cyclelen(&self) -> SPI_MEM_USR_DUMMY_CYCLELEN_R {
        SPI_MEM_USR_DUMMY_CYCLELEN_R::new((self.bits & 0x3f) as u8)
    }
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    pub fn spi_mem_usr_addr_bitlen(&self) -> SPI_MEM_USR_ADDR_BITLEN_R {
        SPI_MEM_USR_ADDR_BITLEN_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_USER1")
            .field(
                "spi_mem_usr_dummy_cyclelen",
                &format_args!("{}", self.spi_mem_usr_dummy_cyclelen().bits()),
            )
            .field(
                "spi_mem_usr_addr_bitlen",
                &format_args!("{}", self.spi_mem_usr_addr_bitlen().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_USER1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:5 - The length in spi_mem_clk cycles of dummy phase. The register value shall be (cycle_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_dummy_cyclelen(&mut self) -> SPI_MEM_USR_DUMMY_CYCLELEN_W<0> {
        SPI_MEM_USR_DUMMY_CYCLELEN_W::new(self)
    }
    #[doc = "Bits 26:31 - The length in bits of address phase. The register value shall be (bit_num-1)."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_addr_bitlen(&mut self) -> SPI_MEM_USR_ADDR_BITLEN_W<26> {
        SPI_MEM_USR_ADDR_BITLEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 user1 register.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_user1](index.html) module"]
pub struct SPI_MEM_USER1_SPEC;
impl crate::RegisterSpec for SPI_MEM_USER1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_user1::R](R) reader structure"]
impl crate::Readable for SPI_MEM_USER1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_user1::W](W) writer structure"]
impl crate::Writable for SPI_MEM_USER1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_USER1 to value 0x5c00_0007"]
impl crate::Resettable for SPI_MEM_USER1_SPEC {
    const RESET_VALUE: Self::Ux = 0x5c00_0007;
}
