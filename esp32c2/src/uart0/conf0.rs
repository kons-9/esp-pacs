#[doc = "Register `CONF0` reader"]
pub struct R(crate::R<CONF0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CONF0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CONF0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CONF0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CONF0` writer"]
pub struct W(crate::W<CONF0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONF0_SPEC>;
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
impl From<crate::W<CONF0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONF0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PARITY` reader - This register is used to configure the parity check mode."]
pub type PARITY_R = crate::BitReader;
#[doc = "Field `PARITY` writer - This register is used to configure the parity check mode."]
pub type PARITY_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `PARITY_EN` reader - Set this bit to enable uart parity check."]
pub type PARITY_EN_R = crate::BitReader;
#[doc = "Field `PARITY_EN` writer - Set this bit to enable uart parity check."]
pub type PARITY_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `BIT_NUM` reader - This register is used to set the length of data."]
pub type BIT_NUM_R = crate::FieldReader;
#[doc = "Field `BIT_NUM` writer - This register is used to set the length of data."]
pub type BIT_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CONF0_SPEC, 2, O>;
#[doc = "Field `STOP_BIT_NUM` reader - This register is used to set the length of stop bit."]
pub type STOP_BIT_NUM_R = crate::FieldReader;
#[doc = "Field `STOP_BIT_NUM` writer - This register is used to set the length of stop bit."]
pub type STOP_BIT_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, CONF0_SPEC, 2, O>;
#[doc = "Field `SW_RTS` reader - This register is used to configure the software rts signal which is used in software flow control."]
pub type SW_RTS_R = crate::BitReader;
#[doc = "Field `SW_RTS` writer - This register is used to configure the software rts signal which is used in software flow control."]
pub type SW_RTS_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `SW_DTR` reader - This register is used to configure the software dtr signal which is used in software flow control."]
pub type SW_DTR_R = crate::BitReader;
#[doc = "Field `SW_DTR` writer - This register is used to configure the software dtr signal which is used in software flow control."]
pub type SW_DTR_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `TXD_BRK` reader - Set this bit to enbale transmitter to send NULL when the process of sending data is done."]
pub type TXD_BRK_R = crate::BitReader;
#[doc = "Field `TXD_BRK` writer - Set this bit to enbale transmitter to send NULL when the process of sending data is done."]
pub type TXD_BRK_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `IRDA_DPLX` reader - Set this bit to enable IrDA loopback mode."]
pub type IRDA_DPLX_R = crate::BitReader;
#[doc = "Field `IRDA_DPLX` writer - Set this bit to enable IrDA loopback mode."]
pub type IRDA_DPLX_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `IRDA_TX_EN` reader - This is the start enable bit for IrDA transmitter."]
pub type IRDA_TX_EN_R = crate::BitReader;
#[doc = "Field `IRDA_TX_EN` writer - This is the start enable bit for IrDA transmitter."]
pub type IRDA_TX_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `IRDA_WCTL` reader - 1'h1: The IrDA transmitter's 11th bit is the same as 10th bit. 1'h0: Set IrDA transmitter's 11th bit to 0."]
pub type IRDA_WCTL_R = crate::BitReader;
#[doc = "Field `IRDA_WCTL` writer - 1'h1: The IrDA transmitter's 11th bit is the same as 10th bit. 1'h0: Set IrDA transmitter's 11th bit to 0."]
pub type IRDA_WCTL_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `IRDA_TX_INV` reader - Set this bit to invert the level of IrDA transmitter."]
pub type IRDA_TX_INV_R = crate::BitReader;
#[doc = "Field `IRDA_TX_INV` writer - Set this bit to invert the level of IrDA transmitter."]
pub type IRDA_TX_INV_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `IRDA_RX_INV` reader - Set this bit to invert the level of IrDA receiver."]
pub type IRDA_RX_INV_R = crate::BitReader;
#[doc = "Field `IRDA_RX_INV` writer - Set this bit to invert the level of IrDA receiver."]
pub type IRDA_RX_INV_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `LOOPBACK` reader - Set this bit to enable uart loopback test mode."]
pub type LOOPBACK_R = crate::BitReader;
#[doc = "Field `LOOPBACK` writer - Set this bit to enable uart loopback test mode."]
pub type LOOPBACK_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `TX_FLOW_EN` reader - Set this bit to enable flow control function for transmitter."]
pub type TX_FLOW_EN_R = crate::BitReader;
#[doc = "Field `TX_FLOW_EN` writer - Set this bit to enable flow control function for transmitter."]
pub type TX_FLOW_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `IRDA_EN` reader - Set this bit to enable IrDA protocol."]
pub type IRDA_EN_R = crate::BitReader;
#[doc = "Field `IRDA_EN` writer - Set this bit to enable IrDA protocol."]
pub type IRDA_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `RXFIFO_RST` reader - Set this bit to reset the uart receive-FIFO."]
pub type RXFIFO_RST_R = crate::BitReader;
#[doc = "Field `RXFIFO_RST` writer - Set this bit to reset the uart receive-FIFO."]
pub type RXFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `TXFIFO_RST` reader - Set this bit to reset the uart transmit-FIFO."]
pub type TXFIFO_RST_R = crate::BitReader;
#[doc = "Field `TXFIFO_RST` writer - Set this bit to reset the uart transmit-FIFO."]
pub type TXFIFO_RST_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `RXD_INV` reader - Set this bit to inverse the level value of uart rxd signal."]
pub type RXD_INV_R = crate::BitReader;
#[doc = "Field `RXD_INV` writer - Set this bit to inverse the level value of uart rxd signal."]
pub type RXD_INV_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `CTS_INV` reader - Set this bit to inverse the level value of uart cts signal."]
pub type CTS_INV_R = crate::BitReader;
#[doc = "Field `CTS_INV` writer - Set this bit to inverse the level value of uart cts signal."]
pub type CTS_INV_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `DSR_INV` reader - Set this bit to inverse the level value of uart dsr signal."]
pub type DSR_INV_R = crate::BitReader;
#[doc = "Field `DSR_INV` writer - Set this bit to inverse the level value of uart dsr signal."]
pub type DSR_INV_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `TXD_INV` reader - Set this bit to inverse the level value of uart txd signal."]
pub type TXD_INV_R = crate::BitReader;
#[doc = "Field `TXD_INV` writer - Set this bit to inverse the level value of uart txd signal."]
pub type TXD_INV_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `RTS_INV` reader - Set this bit to inverse the level value of uart rts signal."]
pub type RTS_INV_R = crate::BitReader;
#[doc = "Field `RTS_INV` writer - Set this bit to inverse the level value of uart rts signal."]
pub type RTS_INV_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `DTR_INV` reader - Set this bit to inverse the level value of uart dtr signal."]
pub type DTR_INV_R = crate::BitReader;
#[doc = "Field `DTR_INV` writer - Set this bit to inverse the level value of uart dtr signal."]
pub type DTR_INV_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `CLK_EN` reader - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_R = crate::BitReader;
#[doc = "Field `CLK_EN` writer - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
pub type CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `ERR_WR_MASK` reader - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong."]
pub type ERR_WR_MASK_R = crate::BitReader;
#[doc = "Field `ERR_WR_MASK` writer - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong."]
pub type ERR_WR_MASK_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `AUTOBAUD_EN` reader - This is the enable bit for detecting baudrate."]
pub type AUTOBAUD_EN_R = crate::BitReader;
#[doc = "Field `AUTOBAUD_EN` writer - This is the enable bit for detecting baudrate."]
pub type AUTOBAUD_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
#[doc = "Field `MEM_CLK_EN` reader - UART memory clock gate enable signal."]
pub type MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `MEM_CLK_EN` writer - UART memory clock gate enable signal."]
pub type MEM_CLK_EN_W<'a, const O: u8> = crate::BitWriter<'a, CONF0_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This register is used to configure the parity check mode."]
    #[inline(always)]
    pub fn parity(&self) -> PARITY_R {
        PARITY_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable uart parity check."]
    #[inline(always)]
    pub fn parity_en(&self) -> PARITY_EN_R {
        PARITY_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - This register is used to set the length of data."]
    #[inline(always)]
    pub fn bit_num(&self) -> BIT_NUM_R {
        BIT_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - This register is used to set the length of stop bit."]
    #[inline(always)]
    pub fn stop_bit_num(&self) -> STOP_BIT_NUM_R {
        STOP_BIT_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - This register is used to configure the software rts signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_rts(&self) -> SW_RTS_R {
        SW_RTS_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This register is used to configure the software dtr signal which is used in software flow control."]
    #[inline(always)]
    pub fn sw_dtr(&self) -> SW_DTR_R {
        SW_DTR_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Set this bit to enbale transmitter to send NULL when the process of sending data is done."]
    #[inline(always)]
    pub fn txd_brk(&self) -> TXD_BRK_R {
        TXD_BRK_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Set this bit to enable IrDA loopback mode."]
    #[inline(always)]
    pub fn irda_dplx(&self) -> IRDA_DPLX_R {
        IRDA_DPLX_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - This is the start enable bit for IrDA transmitter."]
    #[inline(always)]
    pub fn irda_tx_en(&self) -> IRDA_TX_EN_R {
        IRDA_TX_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1'h1: The IrDA transmitter's 11th bit is the same as 10th bit. 1'h0: Set IrDA transmitter's 11th bit to 0."]
    #[inline(always)]
    pub fn irda_wctl(&self) -> IRDA_WCTL_R {
        IRDA_WCTL_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Set this bit to invert the level of IrDA transmitter."]
    #[inline(always)]
    pub fn irda_tx_inv(&self) -> IRDA_TX_INV_R {
        IRDA_TX_INV_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Set this bit to invert the level of IrDA receiver."]
    #[inline(always)]
    pub fn irda_rx_inv(&self) -> IRDA_RX_INV_R {
        IRDA_RX_INV_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Set this bit to enable uart loopback test mode."]
    #[inline(always)]
    pub fn loopback(&self) -> LOOPBACK_R {
        LOOPBACK_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Set this bit to enable flow control function for transmitter."]
    #[inline(always)]
    pub fn tx_flow_en(&self) -> TX_FLOW_EN_R {
        TX_FLOW_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Set this bit to enable IrDA protocol."]
    #[inline(always)]
    pub fn irda_en(&self) -> IRDA_EN_R {
        IRDA_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Set this bit to reset the uart receive-FIFO."]
    #[inline(always)]
    pub fn rxfifo_rst(&self) -> RXFIFO_RST_R {
        RXFIFO_RST_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Set this bit to reset the uart transmit-FIFO."]
    #[inline(always)]
    pub fn txfifo_rst(&self) -> TXFIFO_RST_R {
        TXFIFO_RST_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Set this bit to inverse the level value of uart rxd signal."]
    #[inline(always)]
    pub fn rxd_inv(&self) -> RXD_INV_R {
        RXD_INV_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Set this bit to inverse the level value of uart cts signal."]
    #[inline(always)]
    pub fn cts_inv(&self) -> CTS_INV_R {
        CTS_INV_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Set this bit to inverse the level value of uart dsr signal."]
    #[inline(always)]
    pub fn dsr_inv(&self) -> DSR_INV_R {
        DSR_INV_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Set this bit to inverse the level value of uart txd signal."]
    #[inline(always)]
    pub fn txd_inv(&self) -> TXD_INV_R {
        TXD_INV_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Set this bit to inverse the level value of uart rts signal."]
    #[inline(always)]
    pub fn rts_inv(&self) -> RTS_INV_R {
        RTS_INV_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Set this bit to inverse the level value of uart dtr signal."]
    #[inline(always)]
    pub fn dtr_inv(&self) -> DTR_INV_R {
        DTR_INV_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong."]
    #[inline(always)]
    pub fn err_wr_mask(&self) -> ERR_WR_MASK_R {
        ERR_WR_MASK_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    pub fn autobaud_en(&self) -> AUTOBAUD_EN_R {
        AUTOBAUD_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - UART memory clock gate enable signal."]
    #[inline(always)]
    pub fn mem_clk_en(&self) -> MEM_CLK_EN_R {
        MEM_CLK_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF0")
            .field("parity", &format_args!("{}", self.parity().bit()))
            .field("parity_en", &format_args!("{}", self.parity_en().bit()))
            .field("bit_num", &format_args!("{}", self.bit_num().bits()))
            .field(
                "stop_bit_num",
                &format_args!("{}", self.stop_bit_num().bits()),
            )
            .field("sw_rts", &format_args!("{}", self.sw_rts().bit()))
            .field("sw_dtr", &format_args!("{}", self.sw_dtr().bit()))
            .field("txd_brk", &format_args!("{}", self.txd_brk().bit()))
            .field("irda_dplx", &format_args!("{}", self.irda_dplx().bit()))
            .field("irda_tx_en", &format_args!("{}", self.irda_tx_en().bit()))
            .field("irda_wctl", &format_args!("{}", self.irda_wctl().bit()))
            .field("irda_tx_inv", &format_args!("{}", self.irda_tx_inv().bit()))
            .field("irda_rx_inv", &format_args!("{}", self.irda_rx_inv().bit()))
            .field("loopback", &format_args!("{}", self.loopback().bit()))
            .field("tx_flow_en", &format_args!("{}", self.tx_flow_en().bit()))
            .field("irda_en", &format_args!("{}", self.irda_en().bit()))
            .field("rxfifo_rst", &format_args!("{}", self.rxfifo_rst().bit()))
            .field("txfifo_rst", &format_args!("{}", self.txfifo_rst().bit()))
            .field("rxd_inv", &format_args!("{}", self.rxd_inv().bit()))
            .field("cts_inv", &format_args!("{}", self.cts_inv().bit()))
            .field("dsr_inv", &format_args!("{}", self.dsr_inv().bit()))
            .field("txd_inv", &format_args!("{}", self.txd_inv().bit()))
            .field("rts_inv", &format_args!("{}", self.rts_inv().bit()))
            .field("dtr_inv", &format_args!("{}", self.dtr_inv().bit()))
            .field("clk_en", &format_args!("{}", self.clk_en().bit()))
            .field("err_wr_mask", &format_args!("{}", self.err_wr_mask().bit()))
            .field("autobaud_en", &format_args!("{}", self.autobaud_en().bit()))
            .field("mem_clk_en", &format_args!("{}", self.mem_clk_en().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CONF0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to configure the parity check mode."]
    #[inline(always)]
    #[must_use]
    pub fn parity(&mut self) -> PARITY_W<0> {
        PARITY_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to enable uart parity check."]
    #[inline(always)]
    #[must_use]
    pub fn parity_en(&mut self) -> PARITY_EN_W<1> {
        PARITY_EN_W::new(self)
    }
    #[doc = "Bits 2:3 - This register is used to set the length of data."]
    #[inline(always)]
    #[must_use]
    pub fn bit_num(&mut self) -> BIT_NUM_W<2> {
        BIT_NUM_W::new(self)
    }
    #[doc = "Bits 4:5 - This register is used to set the length of stop bit."]
    #[inline(always)]
    #[must_use]
    pub fn stop_bit_num(&mut self) -> STOP_BIT_NUM_W<4> {
        STOP_BIT_NUM_W::new(self)
    }
    #[doc = "Bit 6 - This register is used to configure the software rts signal which is used in software flow control."]
    #[inline(always)]
    #[must_use]
    pub fn sw_rts(&mut self) -> SW_RTS_W<6> {
        SW_RTS_W::new(self)
    }
    #[doc = "Bit 7 - This register is used to configure the software dtr signal which is used in software flow control."]
    #[inline(always)]
    #[must_use]
    pub fn sw_dtr(&mut self) -> SW_DTR_W<7> {
        SW_DTR_W::new(self)
    }
    #[doc = "Bit 8 - Set this bit to enbale transmitter to send NULL when the process of sending data is done."]
    #[inline(always)]
    #[must_use]
    pub fn txd_brk(&mut self) -> TXD_BRK_W<8> {
        TXD_BRK_W::new(self)
    }
    #[doc = "Bit 9 - Set this bit to enable IrDA loopback mode."]
    #[inline(always)]
    #[must_use]
    pub fn irda_dplx(&mut self) -> IRDA_DPLX_W<9> {
        IRDA_DPLX_W::new(self)
    }
    #[doc = "Bit 10 - This is the start enable bit for IrDA transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn irda_tx_en(&mut self) -> IRDA_TX_EN_W<10> {
        IRDA_TX_EN_W::new(self)
    }
    #[doc = "Bit 11 - 1'h1: The IrDA transmitter's 11th bit is the same as 10th bit. 1'h0: Set IrDA transmitter's 11th bit to 0."]
    #[inline(always)]
    #[must_use]
    pub fn irda_wctl(&mut self) -> IRDA_WCTL_W<11> {
        IRDA_WCTL_W::new(self)
    }
    #[doc = "Bit 12 - Set this bit to invert the level of IrDA transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn irda_tx_inv(&mut self) -> IRDA_TX_INV_W<12> {
        IRDA_TX_INV_W::new(self)
    }
    #[doc = "Bit 13 - Set this bit to invert the level of IrDA receiver."]
    #[inline(always)]
    #[must_use]
    pub fn irda_rx_inv(&mut self) -> IRDA_RX_INV_W<13> {
        IRDA_RX_INV_W::new(self)
    }
    #[doc = "Bit 14 - Set this bit to enable uart loopback test mode."]
    #[inline(always)]
    #[must_use]
    pub fn loopback(&mut self) -> LOOPBACK_W<14> {
        LOOPBACK_W::new(self)
    }
    #[doc = "Bit 15 - Set this bit to enable flow control function for transmitter."]
    #[inline(always)]
    #[must_use]
    pub fn tx_flow_en(&mut self) -> TX_FLOW_EN_W<15> {
        TX_FLOW_EN_W::new(self)
    }
    #[doc = "Bit 16 - Set this bit to enable IrDA protocol."]
    #[inline(always)]
    #[must_use]
    pub fn irda_en(&mut self) -> IRDA_EN_W<16> {
        IRDA_EN_W::new(self)
    }
    #[doc = "Bit 17 - Set this bit to reset the uart receive-FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn rxfifo_rst(&mut self) -> RXFIFO_RST_W<17> {
        RXFIFO_RST_W::new(self)
    }
    #[doc = "Bit 18 - Set this bit to reset the uart transmit-FIFO."]
    #[inline(always)]
    #[must_use]
    pub fn txfifo_rst(&mut self) -> TXFIFO_RST_W<18> {
        TXFIFO_RST_W::new(self)
    }
    #[doc = "Bit 19 - Set this bit to inverse the level value of uart rxd signal."]
    #[inline(always)]
    #[must_use]
    pub fn rxd_inv(&mut self) -> RXD_INV_W<19> {
        RXD_INV_W::new(self)
    }
    #[doc = "Bit 20 - Set this bit to inverse the level value of uart cts signal."]
    #[inline(always)]
    #[must_use]
    pub fn cts_inv(&mut self) -> CTS_INV_W<20> {
        CTS_INV_W::new(self)
    }
    #[doc = "Bit 21 - Set this bit to inverse the level value of uart dsr signal."]
    #[inline(always)]
    #[must_use]
    pub fn dsr_inv(&mut self) -> DSR_INV_W<21> {
        DSR_INV_W::new(self)
    }
    #[doc = "Bit 22 - Set this bit to inverse the level value of uart txd signal."]
    #[inline(always)]
    #[must_use]
    pub fn txd_inv(&mut self) -> TXD_INV_W<22> {
        TXD_INV_W::new(self)
    }
    #[doc = "Bit 23 - Set this bit to inverse the level value of uart rts signal."]
    #[inline(always)]
    #[must_use]
    pub fn rts_inv(&mut self) -> RTS_INV_W<23> {
        RTS_INV_W::new(self)
    }
    #[doc = "Bit 24 - Set this bit to inverse the level value of uart dtr signal."]
    #[inline(always)]
    #[must_use]
    pub fn dtr_inv(&mut self) -> DTR_INV_W<24> {
        DTR_INV_W::new(self)
    }
    #[doc = "Bit 25 - 1'h1: Force clock on for register. 1'h0: Support clock only when application writes registers."]
    #[inline(always)]
    #[must_use]
    pub fn clk_en(&mut self) -> CLK_EN_W<25> {
        CLK_EN_W::new(self)
    }
    #[doc = "Bit 26 - 1'h1: Receiver stops storing data into FIFO when data is wrong. 1'h0: Receiver stores the data even if the received data is wrong."]
    #[inline(always)]
    #[must_use]
    pub fn err_wr_mask(&mut self) -> ERR_WR_MASK_W<26> {
        ERR_WR_MASK_W::new(self)
    }
    #[doc = "Bit 27 - This is the enable bit for detecting baudrate."]
    #[inline(always)]
    #[must_use]
    pub fn autobaud_en(&mut self) -> AUTOBAUD_EN_W<27> {
        AUTOBAUD_EN_W::new(self)
    }
    #[doc = "Bit 28 - UART memory clock gate enable signal."]
    #[inline(always)]
    #[must_use]
    pub fn mem_clk_en(&mut self) -> MEM_CLK_EN_W<28> {
        MEM_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "a\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [conf0](index.html) module"]
pub struct CONF0_SPEC;
impl crate::RegisterSpec for CONF0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [conf0::R](R) reader structure"]
impl crate::Readable for CONF0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [conf0::W](W) writer structure"]
impl crate::Writable for CONF0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONF0 to value 0x1000_001c"]
impl crate::Resettable for CONF0_SPEC {
    const RESET_VALUE: Self::Ux = 0x1000_001c;
}
