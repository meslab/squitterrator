use std::fmt::{self, Debug};

use log::debug;

use super::*;

#[derive(Debug)]
pub enum DF {
    SRT(Srt),
    EXT(Ext),
    MDS(Mds),
}

pub fn get_downlink(message: &[u32]) -> Option<DF> {
    df(message).map(|df| match df {
        0..=16 => DF::SRT(Srt::from_message(message)),
        17 => DF::EXT(Ext::from_message(message)),
        20 | 21 => DF::MDS(Mds::from_message(message)),
        _ => {
            debug!("Cannot create DF:{}", df);
            DF::SRT(Srt::new())
        }
    })
}

impl fmt::Display for DF {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DF::SRT(v) => write!(f, "{}", v),
            DF::EXT(v) => write!(f, "{}", v),
            DF::MDS(v) => write!(f, "{}", v),
        }
    }
}
