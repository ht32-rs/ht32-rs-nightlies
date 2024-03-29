#[doc = "Register `BFTM_SR` reader"]
pub struct R(crate::R<BFTM_SR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BFTM_SR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BFTM_SR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BFTM_SR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `BFTM_SR` writer"]
pub struct W(crate::W<BFTM_SR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<BFTM_SR_SPEC>;
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
impl From<crate::W<BFTM_SR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<BFTM_SR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MIF` reader - MIF"]
pub type MIF_R = crate::BitReader;
#[doc = "Field `MIF` writer - MIF"]
pub type MIF_W<'a, const O: u8> = crate::BitWriter<'a, BFTM_SR_SPEC, O>;
impl R {
    #[doc = "Bit 0 - MIF"]
    #[inline(always)]
    pub fn mif(&self) -> MIF_R {
        MIF_R::new((self.bits & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - MIF"]
    #[inline(always)]
    #[must_use]
    pub fn mif(&mut self) -> MIF_W<0> {
        MIF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "BFTM_SR\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bftm_sr](index.html) module"]
pub struct BFTM_SR_SPEC;
impl crate::RegisterSpec for BFTM_SR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bftm_sr::R](R) reader structure"]
impl crate::Readable for BFTM_SR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [bftm_sr::W](W) writer structure"]
impl crate::Writable for BFTM_SR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets BFTM_SR to value 0"]
impl crate::Resettable for BFTM_SR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
