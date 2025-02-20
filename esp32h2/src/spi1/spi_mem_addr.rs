#[doc = "Register `SPI_MEM_ADDR` reader"]
pub struct R(crate::R<SPI_MEM_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SPI_MEM_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SPI_MEM_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SPI_MEM_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SPI_MEM_ADDR` writer"]
pub struct W(crate::W<SPI_MEM_ADDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SPI_MEM_ADDR_SPEC>;
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
impl From<crate::W<SPI_MEM_ADDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SPI_MEM_ADDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPI_MEM_USR_ADDR_VALUE` reader - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
pub type SPI_MEM_USR_ADDR_VALUE_R = crate::FieldReader<u32>;
#[doc = "Field `SPI_MEM_USR_ADDR_VALUE` writer - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
pub type SPI_MEM_USR_ADDR_VALUE_W<'a, const O: u8> =
    crate::FieldWriter<'a, SPI_MEM_ADDR_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
    #[inline(always)]
    pub fn spi_mem_usr_addr_value(&self) -> SPI_MEM_USR_ADDR_VALUE_R {
        SPI_MEM_USR_ADDR_VALUE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SPI_MEM_ADDR")
            .field(
                "spi_mem_usr_addr_value",
                &format_args!("{}", self.spi_mem_usr_addr_value().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SPI_MEM_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - In user mode, it is the memory address. other then the bit0-bit23 is the memory address, the bit24-bit31 are the byte length of a transfer."]
    #[inline(always)]
    #[must_use]
    pub fn spi_mem_usr_addr_value(&mut self) -> SPI_MEM_USR_ADDR_VALUE_W<0> {
        SPI_MEM_USR_ADDR_VALUE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SPI1 address register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [spi_mem_addr](index.html) module"]
pub struct SPI_MEM_ADDR_SPEC;
impl crate::RegisterSpec for SPI_MEM_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [spi_mem_addr::R](R) reader structure"]
impl crate::Readable for SPI_MEM_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [spi_mem_addr::W](W) writer structure"]
impl crate::Writable for SPI_MEM_ADDR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SPI_MEM_ADDR to value 0"]
impl crate::Resettable for SPI_MEM_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
