#[doc = "Reader of register GPTM_CH0OCFR"]
pub type R = crate::R<u32, super::GPTM_CH0OCFR>;
#[doc = "Writer for register GPTM_CH0OCFR"]
pub type W = crate::W<u32, super::GPTM_CH0OCFR>;
#[doc = "Register GPTM_CH0OCFR `reset()`'s with value 0"]
impl crate::ResetValue for super::GPTM_CH0OCFR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CH0OM`"]
pub type CH0OM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CH0OM`"]
pub struct CH0OM_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0OM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
#[doc = "Reader of field `REF0CE`"]
pub type REF0CE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `REF0CE`"]
pub struct REF0CE_W<'a> {
    w: &'a mut W,
}
impl<'a> REF0CE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `CH0PRE`"]
pub type CH0PRE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0PRE`"]
pub struct CH0PRE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0PRE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `CH0IMAE`"]
pub type CH0IMAE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CH0IMAE`"]
pub struct CH0IMAE_W<'a> {
    w: &'a mut W,
}
impl<'a> CH0IMAE_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:2 - CH0OM"]
    #[inline(always)]
    pub fn ch0om(&self) -> CH0OM_R {
        CH0OM_R::new((self.bits & 0x07) as u8)
    }
    #[doc = "Bit 3 - REF0CE"]
    #[inline(always)]
    pub fn ref0ce(&self) -> REF0CE_R {
        REF0CE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 4 - CH0PRE"]
    #[inline(always)]
    pub fn ch0pre(&self) -> CH0PRE_R {
        CH0PRE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 5 - CH0IMAE"]
    #[inline(always)]
    pub fn ch0imae(&self) -> CH0IMAE_R {
        CH0IMAE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - CH0OM"]
    #[inline(always)]
    pub fn ch0om(&mut self) -> CH0OM_W {
        CH0OM_W { w: self }
    }
    #[doc = "Bit 3 - REF0CE"]
    #[inline(always)]
    pub fn ref0ce(&mut self) -> REF0CE_W {
        REF0CE_W { w: self }
    }
    #[doc = "Bit 4 - CH0PRE"]
    #[inline(always)]
    pub fn ch0pre(&mut self) -> CH0PRE_W {
        CH0PRE_W { w: self }
    }
    #[doc = "Bit 5 - CH0IMAE"]
    #[inline(always)]
    pub fn ch0imae(&mut self) -> CH0IMAE_W {
        CH0IMAE_W { w: self }
    }
}
