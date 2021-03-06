#[doc = "Reader of register RTC_CMP"]
pub type R = crate::R<u32, super::RTC_CMP>;
#[doc = "Writer for register RTC_CMP"]
pub type W = crate::W<u32, super::RTC_CMP>;
#[doc = "Register RTC_CMP `reset()`'s with value 0"]
impl crate::ResetValue for super::RTC_CMP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RTCCMP`"]
pub type RTCCMP_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RTCCMP`"]
pub struct RTCCMP_W<'a> {
    w: &'a mut W,
}
impl<'a> RTCCMP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - RTCCMP"]
    #[inline(always)]
    pub fn rtccmp(&self) -> RTCCMP_R {
        RTCCMP_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - RTCCMP"]
    #[inline(always)]
    pub fn rtccmp(&mut self) -> RTCCMP_W {
        RTCCMP_W { w: self }
    }
}
