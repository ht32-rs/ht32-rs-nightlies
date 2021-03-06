#[doc = "Reader of register RTC_CNT"]
pub type R = crate::R<u32, super::RTC_CNT>;
#[doc = "Writer for register RTC_CNT"]
pub type W = crate::W<u32, super::RTC_CNT>;
#[doc = "Register RTC_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCCNT`"]
pub type RTCCNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTCCNT`"]
pub struct RTCCNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RTCCNT"]
    #[inline(always)]
    pub fn rtccnt(&self) -> RTCCNT_R {
        RTCCNT_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTCCNT"]
    #[inline(always)]
    pub fn rtccnt(&mut self) -> RTCCNT_W {
        RTCCNT_W { w: self }
    }
}
