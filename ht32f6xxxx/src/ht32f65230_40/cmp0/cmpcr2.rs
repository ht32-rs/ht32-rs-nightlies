#[doc = "Register `CMPCR2` reader"]
pub struct R(crate::R<CMPCR2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CMPCR2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CMPCR2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CMPCR2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CMPCR2` writer"]
pub struct W(crate::W<CMPCR2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CMPCR2_SPEC>;
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
impl From<crate::W<CMPCR2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CMPCR2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CMPEN` reader - CMPEN"]
pub type CMPEN_R = crate::BitReader;
#[doc = "Field `CMPEN` writer - CMPEN"]
pub type CMPEN_W<'a, const O: u8> = crate::BitWriter<'a, CMPCR2_SPEC, O>;
#[doc = "Field `CMPSM` reader - CMPSM"]
pub type CMPSM_R = crate::BitReader;
#[doc = "Field `CMPSM` writer - CMPSM"]
pub type CMPSM_W<'a, const O: u8> = crate::BitWriter<'a, CMPCR2_SPEC, O>;
#[doc = "Field `CMPHM` reader - CMPHM"]
pub type CMPHM_R = crate::FieldReader;
#[doc = "Field `CMPHM` writer - CMPHM"]
pub type CMPHM_W<'a, const O: u8> = crate::FieldWriter<'a, CMPCR2_SPEC, 2, O>;
#[doc = "Field `CMPINSEL` reader - CMPINSEL"]
pub type CMPINSEL_R = crate::FieldReader;
#[doc = "Field `CMPINSEL` writer - CMPINSEL"]
pub type CMPINSEL_W<'a, const O: u8> = crate::FieldWriter<'a, CMPCR2_SPEC, 2, O>;
#[doc = "Field `CMPPOL` reader - CMPPOL"]
pub type CMPPOL_R = crate::BitReader;
#[doc = "Field `CMPPOL` writer - CMPPOL"]
pub type CMPPOL_W<'a, const O: u8> = crate::BitWriter<'a, CMPCR2_SPEC, O>;
#[doc = "Field `SYNCSEL` reader - SYNCSEL"]
pub type SYNCSEL_R = crate::BitReader;
#[doc = "Field `SYNCSEL` writer - SYNCSEL"]
pub type SYNCSEL_W<'a, const O: u8> = crate::BitWriter<'a, CMPCR2_SPEC, O>;
#[doc = "Field `CVREN` reader - CVREN"]
pub type CVREN_R = crate::BitReader;
#[doc = "Field `CVREN` writer - CVREN"]
pub type CVREN_W<'a, const O: u8> = crate::BitWriter<'a, CMPCR2_SPEC, O>;
#[doc = "Field `CVROE` reader - CVROE"]
pub type CVROE_R = crate::BitReader;
#[doc = "Field `CVROE` writer - CVROE"]
pub type CVROE_W<'a, const O: u8> = crate::BitWriter<'a, CMPCR2_SPEC, O>;
#[doc = "Field `CMPOSEL` reader - CMPOSEL"]
pub type CMPOSEL_R = crate::FieldReader;
#[doc = "Field `CMPOSEL` writer - CMPOSEL"]
pub type CMPOSEL_W<'a, const O: u8> = crate::FieldWriter<'a, CMPCR2_SPEC, 3, O>;
#[doc = "Field `CMPWPEN` reader - CMPWPEN"]
pub type CMPWPEN_R = crate::BitReader;
#[doc = "Field `CMPWPEN` writer - CMPWPEN"]
pub type CMPWPEN_W<'a, const O: u8> = crate::BitWriter<'a, CMPCR2_SPEC, O>;
#[doc = "Field `CMPSTS` reader - CMPSTS"]
pub type CMPSTS_R = crate::BitReader;
#[doc = "Field `CMPSTS` writer - CMPSTS"]
pub type CMPSTS_W<'a, const O: u8> = crate::BitWriter<'a, CMPCR2_SPEC, O>;
#[doc = "Field `PROTECT` reader - PROTECT"]
pub type PROTECT_R = crate::FieldReader<u16>;
#[doc = "Field `PROTECT` writer - PROTECT"]
pub type PROTECT_W<'a, const O: u8> = crate::FieldWriter<'a, CMPCR2_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bit 0 - CMPEN"]
    #[inline(always)]
    pub fn cmpen(&self) -> CMPEN_R {
        CMPEN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - CMPSM"]
    #[inline(always)]
    pub fn cmpsm(&self) -> CMPSM_R {
        CMPSM_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - CMPHM"]
    #[inline(always)]
    pub fn cmphm(&self) -> CMPHM_R {
        CMPHM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - CMPINSEL"]
    #[inline(always)]
    pub fn cmpinsel(&self) -> CMPINSEL_R {
        CMPINSEL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - CMPPOL"]
    #[inline(always)]
    pub fn cmppol(&self) -> CMPPOL_R {
        CMPPOL_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - SYNCSEL"]
    #[inline(always)]
    pub fn syncsel(&self) -> SYNCSEL_R {
        SYNCSEL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - CVREN"]
    #[inline(always)]
    pub fn cvren(&self) -> CVREN_R {
        CVREN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - CVROE"]
    #[inline(always)]
    pub fn cvroe(&self) -> CVROE_R {
        CVROE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 11:13 - CMPOSEL"]
    #[inline(always)]
    pub fn cmposel(&self) -> CMPOSEL_R {
        CMPOSEL_R::new(((self.bits >> 11) & 7) as u8)
    }
    #[doc = "Bit 14 - CMPWPEN"]
    #[inline(always)]
    pub fn cmpwpen(&self) -> CMPWPEN_R {
        CMPWPEN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - CMPSTS"]
    #[inline(always)]
    pub fn cmpsts(&self) -> CMPSTS_R {
        CMPSTS_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:31 - PROTECT"]
    #[inline(always)]
    pub fn protect(&self) -> PROTECT_R {
        PROTECT_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bit 0 - CMPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmpen(&mut self) -> CMPEN_W<0> {
        CMPEN_W::new(self)
    }
    #[doc = "Bit 1 - CMPSM"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsm(&mut self) -> CMPSM_W<1> {
        CMPSM_W::new(self)
    }
    #[doc = "Bits 2:3 - CMPHM"]
    #[inline(always)]
    #[must_use]
    pub fn cmphm(&mut self) -> CMPHM_W<2> {
        CMPHM_W::new(self)
    }
    #[doc = "Bits 4:5 - CMPINSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cmpinsel(&mut self) -> CMPINSEL_W<4> {
        CMPINSEL_W::new(self)
    }
    #[doc = "Bit 6 - CMPPOL"]
    #[inline(always)]
    #[must_use]
    pub fn cmppol(&mut self) -> CMPPOL_W<6> {
        CMPPOL_W::new(self)
    }
    #[doc = "Bit 7 - SYNCSEL"]
    #[inline(always)]
    #[must_use]
    pub fn syncsel(&mut self) -> SYNCSEL_W<7> {
        SYNCSEL_W::new(self)
    }
    #[doc = "Bit 8 - CVREN"]
    #[inline(always)]
    #[must_use]
    pub fn cvren(&mut self) -> CVREN_W<8> {
        CVREN_W::new(self)
    }
    #[doc = "Bit 9 - CVROE"]
    #[inline(always)]
    #[must_use]
    pub fn cvroe(&mut self) -> CVROE_W<9> {
        CVROE_W::new(self)
    }
    #[doc = "Bits 11:13 - CMPOSEL"]
    #[inline(always)]
    #[must_use]
    pub fn cmposel(&mut self) -> CMPOSEL_W<11> {
        CMPOSEL_W::new(self)
    }
    #[doc = "Bit 14 - CMPWPEN"]
    #[inline(always)]
    #[must_use]
    pub fn cmpwpen(&mut self) -> CMPWPEN_W<14> {
        CMPWPEN_W::new(self)
    }
    #[doc = "Bit 15 - CMPSTS"]
    #[inline(always)]
    #[must_use]
    pub fn cmpsts(&mut self) -> CMPSTS_W<15> {
        CMPSTS_W::new(self)
    }
    #[doc = "Bits 16:31 - PROTECT"]
    #[inline(always)]
    #[must_use]
    pub fn protect(&mut self) -> PROTECT_W<16> {
        PROTECT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CMPCR2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cmpcr2](index.html) module"]
pub struct CMPCR2_SPEC;
impl crate::RegisterSpec for CMPCR2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cmpcr2::R](R) reader structure"]
impl crate::Readable for CMPCR2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cmpcr2::W](W) writer structure"]
impl crate::Writable for CMPCR2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CMPCR2 to value 0"]
impl crate::Resettable for CMPCR2_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}