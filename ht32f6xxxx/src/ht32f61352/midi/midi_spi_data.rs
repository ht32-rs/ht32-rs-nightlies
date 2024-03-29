#[doc = "Register `MIDI_SPI_DATA` reader"]
pub struct R(crate::R<MIDI_SPI_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MIDI_SPI_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MIDI_SPI_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MIDI_SPI_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MIDI_SPI_DATA` writer"]
pub struct W(crate::W<MIDI_SPI_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MIDI_SPI_DATA_SPEC>;
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
impl From<crate::W<MIDI_SPI_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MIDI_SPI_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SPID` reader - SPID"]
pub type SPID_R = crate::FieldReader<u32>;
#[doc = "Field `SPID` writer - SPID"]
pub type SPID_W<'a, const O: u8> = crate::FieldWriter<'a, MIDI_SPI_DATA_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - SPID"]
    #[inline(always)]
    pub fn spid(&self) -> SPID_R {
        SPID_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - SPID"]
    #[inline(always)]
    #[must_use]
    pub fn spid(&mut self) -> SPID_W<0> {
        SPID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MIDI_SPI_DATA\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [midi_spi_data](index.html) module"]
pub struct MIDI_SPI_DATA_SPEC;
impl crate::RegisterSpec for MIDI_SPI_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [midi_spi_data::R](R) reader structure"]
impl crate::Readable for MIDI_SPI_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [midi_spi_data::W](W) writer structure"]
impl crate::Writable for MIDI_SPI_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MIDI_SPI_DATA to value 0"]
impl crate::Resettable for MIDI_SPI_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
