use super::prelude::*;

pub type qXferFeaturesRead<'a> = QXferReadBase<'a, FeaturesAnnex>;

#[derive(Debug)]
pub struct FeaturesAnnex;

impl ParseAnnex for FeaturesAnnex {
    fn from_buf(buf: &[u8]) -> Option<Self> {
        if buf != b"target.xml" {
            return None;
        }

        Some(FeaturesAnnex)
    }
}
