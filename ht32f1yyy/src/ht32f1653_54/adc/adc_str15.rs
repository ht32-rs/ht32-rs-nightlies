#[doc = "Reader of register ADC_STR15"]
pub type R = crate::R<u32, super::ADC_STR15>;
#[doc = "Writer for register ADC_STR15"]
pub type W = crate::W<u32, super::ADC_STR15>;
#[doc = "Register ADC_STR15 `reset()`'s with value 0"]
impl crate::ResetValue for super::ADC_STR15 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `ADST15`"]
pub type ADST15_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `ADST15`"]
pub struct ADST15_W<'a> {
    w: &'a mut W,
}
impl<'a> ADST15_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - ADST15"]
    #[inline(always)]
    pub fn adst15(&self) -> ADST15_R {
        ADST15_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - ADST15"]
    #[inline(always)]
    pub fn adst15(&mut self) -> ADST15_W {
        ADST15_W { w: self }
    }
}
