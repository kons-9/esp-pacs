#[doc = "Register `DMA_INT_RAW` reader"]
pub struct R(crate::R<DMA_INT_RAW_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<DMA_INT_RAW_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<DMA_INT_RAW_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<DMA_INT_RAW_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `INLINK_DSCR_EMPTY_INT_RAW` reader - The raw bit for lack of enough inlink descriptors."]
pub type INLINK_DSCR_EMPTY_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUTLINK_DSCR_ERROR_INT_RAW` reader - The raw bit for outlink descriptor error."]
pub type OUTLINK_DSCR_ERROR_INT_RAW_R = crate::BitReader;
#[doc = "Field `INLINK_DSCR_ERROR_INT_RAW` reader - The raw bit for inlink descriptor error."]
pub type INLINK_DSCR_ERROR_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_DONE_INT_RAW` reader - The raw bit for completing usage of a inlink descriptor."]
pub type IN_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_ERR_EOF_INT_RAW` reader - The raw bit for receiving error."]
pub type IN_ERR_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF_INT_RAW` reader - The raw bit for completing receiving all the packets from host."]
pub type IN_SUC_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_DONE_INT_RAW` reader - The raw bit for completing usage of a outlink descriptor."]
pub type OUT_DONE_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_EOF_INT_RAW` reader - The raw bit for sending a packet to host done."]
pub type OUT_EOF_INT_RAW_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF_INT_RAW` reader - The raw bit for sending all the packets to host done."]
pub type OUT_TOTAL_EOF_INT_RAW_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw bit for lack of enough inlink descriptors."]
    #[inline(always)]
    pub fn inlink_dscr_empty_int_raw(&self) -> INLINK_DSCR_EMPTY_INT_RAW_R {
        INLINK_DSCR_EMPTY_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw bit for outlink descriptor error."]
    #[inline(always)]
    pub fn outlink_dscr_error_int_raw(&self) -> OUTLINK_DSCR_ERROR_INT_RAW_R {
        OUTLINK_DSCR_ERROR_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw bit for inlink descriptor error."]
    #[inline(always)]
    pub fn inlink_dscr_error_int_raw(&self) -> INLINK_DSCR_ERROR_INT_RAW_R {
        INLINK_DSCR_ERROR_INT_RAW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw bit for completing usage of a inlink descriptor."]
    #[inline(always)]
    pub fn in_done_int_raw(&self) -> IN_DONE_INT_RAW_R {
        IN_DONE_INT_RAW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - The raw bit for receiving error."]
    #[inline(always)]
    pub fn in_err_eof_int_raw(&self) -> IN_ERR_EOF_INT_RAW_R {
        IN_ERR_EOF_INT_RAW_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - The raw bit for completing receiving all the packets from host."]
    #[inline(always)]
    pub fn in_suc_eof_int_raw(&self) -> IN_SUC_EOF_INT_RAW_R {
        IN_SUC_EOF_INT_RAW_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - The raw bit for completing usage of a outlink descriptor."]
    #[inline(always)]
    pub fn out_done_int_raw(&self) -> OUT_DONE_INT_RAW_R {
        OUT_DONE_INT_RAW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - The raw bit for sending a packet to host done."]
    #[inline(always)]
    pub fn out_eof_int_raw(&self) -> OUT_EOF_INT_RAW_R {
        OUT_EOF_INT_RAW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - The raw bit for sending all the packets to host done."]
    #[inline(always)]
    pub fn out_total_eof_int_raw(&self) -> OUT_TOTAL_EOF_INT_RAW_R {
        OUT_TOTAL_EOF_INT_RAW_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DMA_INT_RAW")
            .field(
                "inlink_dscr_empty_int_raw",
                &format_args!("{}", self.inlink_dscr_empty_int_raw().bit()),
            )
            .field(
                "outlink_dscr_error_int_raw",
                &format_args!("{}", self.outlink_dscr_error_int_raw().bit()),
            )
            .field(
                "inlink_dscr_error_int_raw",
                &format_args!("{}", self.inlink_dscr_error_int_raw().bit()),
            )
            .field(
                "in_done_int_raw",
                &format_args!("{}", self.in_done_int_raw().bit()),
            )
            .field(
                "in_err_eof_int_raw",
                &format_args!("{}", self.in_err_eof_int_raw().bit()),
            )
            .field(
                "in_suc_eof_int_raw",
                &format_args!("{}", self.in_suc_eof_int_raw().bit()),
            )
            .field(
                "out_done_int_raw",
                &format_args!("{}", self.out_done_int_raw().bit()),
            )
            .field(
                "out_eof_int_raw",
                &format_args!("{}", self.out_eof_int_raw().bit()),
            )
            .field(
                "out_total_eof_int_raw",
                &format_args!("{}", self.out_total_eof_int_raw().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<DMA_INT_RAW_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [dma_int_raw](index.html) module"]
pub struct DMA_INT_RAW_SPEC;
impl crate::RegisterSpec for DMA_INT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [dma_int_raw::R](R) reader structure"]
impl crate::Readable for DMA_INT_RAW_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets DMA_INT_RAW to value 0"]
impl crate::Resettable for DMA_INT_RAW_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
