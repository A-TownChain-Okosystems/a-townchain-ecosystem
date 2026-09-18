//! Multimodal request contracts. Binary payloads stay outside the control-plane types.

use crate::AuroraError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Modality { Text, Image, Audio, Video, File }

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MediaInput {
    pub modality: Modality,
    pub media_type: String,
    pub digest: [u8; 32],
    pub bytes: Option<Vec<u8>>,
}

impl MediaInput {
    pub fn validate(&self, max_inline_bytes: usize) -> Result<(), AuroraError> {
        if self.media_type.trim().is_empty() { return Err(AuroraError::InvalidRequest); }
        if let Some(bytes)=&self.bytes {
            if bytes.len()>max_inline_bytes { return Err(AuroraError::ResourceExhausted); }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MultimodalInput { pub text: Option<String>, pub media: Vec<MediaInput> }

impl MultimodalInput {
    pub fn validate(&self, max_media: usize, max_inline_bytes: usize) -> Result<(), AuroraError> {
        if self.text.as_deref().map(str::trim).unwrap_or("").is_empty() && self.media.is_empty() {
            return Err(AuroraError::InvalidRequest);
        }
        if self.media.len()>max_media { return Err(AuroraError::ResourceExhausted); }
        for m in &self.media { m.validate(max_inline_bytes)?; }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn multimodal_input_enforces_limits() {
        let m=MultimodalInput{text:None,media:vec![MediaInput{modality:Modality::Image,media_type:"image/png".into(),digest:[0;32],bytes:Some(vec![0;8])}]};
        assert!(m.validate(1,8).is_ok());
        assert!(m.validate(1,7).is_err());
    }
}
