/* 
A library describing popular video files extenstion as an enum allowing to parse it from a string.

*/ 

use std::fmt::Display;
use std::fmt::Formatter;
use std::fmt::Result;
use std::fmt::Debug;

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum VideoFormat {
    AVI,
    MKV,
    MP4,
    MOV,
    MPEG,
    MPG,
    OGG,
    WEBM,
    WMV,
    WMA,
    FLV,
}

impl VideoFormat { 

    pub fn from_extension(s: &str) -> Option<Self> {
        let ext = s.to_ascii_lowercase();

        Some(match ext.as_str() {
            "avi" => VideoFormat::AVI,
            "mkv" => VideoFormat::MKV,
            "mp4" => VideoFormat::MP4,
            "mov" => VideoFormat::MOV,
            "mpeg" => VideoFormat::MPEG,
            "mpg" => VideoFormat::MPG,
            "ogg" => VideoFormat::OGG,
            "webm" => VideoFormat::WEBM,
            "wmv" => VideoFormat::WMV,
            "wma" => VideoFormat::WMA,
            "flv" => VideoFormat::FLV,
            _ => return None,
        })
    }
}

impl Display for VideoFormat {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{:?}", self)
    }
}


#[cfg(test)]
mod tests { 
    use super::*;
    #[test]
    fn test_from_str() {
        assert_eq!(VideoFormat::from_extension("avi").unwrap(), VideoFormat::AVI);
        assert_eq!(VideoFormat::from_extension("mkv").unwrap(), VideoFormat::MKV);
        assert_eq!(VideoFormat::from_extension("mp4").unwrap(), VideoFormat::MP4);
        assert_eq!(VideoFormat::from_extension("mov").unwrap(), VideoFormat::MOV);
        assert_eq!(VideoFormat::from_extension("MOV").unwrap(), VideoFormat::MOV);
        assert_eq!(VideoFormat::from_extension("mpeg").unwrap(), VideoFormat::MPEG);
        assert_eq!(VideoFormat::from_extension("mpg").unwrap(), VideoFormat::MPG);
        assert_eq!(VideoFormat::from_extension("ogg").unwrap(), VideoFormat::OGG);
        assert_eq!(VideoFormat::from_extension("webm").unwrap(), VideoFormat::WEBM);
        assert_eq!(VideoFormat::from_extension("wmv").unwrap(), VideoFormat::WMV);
        assert_eq!(VideoFormat::from_extension("wma").unwrap(), VideoFormat::WMA);
        assert_eq!(VideoFormat::from_extension("flv").unwrap(), VideoFormat::FLV);
        assert_eq!(VideoFormat::from_extension("unknown").is_none(), true);
        assert_eq!(VideoFormat::from_extension("").is_none(), true);
        assert_eq!(VideoFormat::from_extension(" ").is_none(), true);
        assert_eq!(VideoFormat::from_extension("\t").is_none(), true);
        assert_eq!(VideoFormat::from_extension("\n").is_none(), true);
        assert_eq!(VideoFormat::from_extension("\r").is_none(), true);
        assert_eq!(VideoFormat::from_extension("\r\n").is_none(), true);
        assert_eq!(VideoFormat::from_extension("unknown").is_none(), true);
    }
    #[test]
    fn test_display() {
        assert_eq!(VideoFormat::AVI.to_string(), "AVI");
        assert_eq!(VideoFormat::MKV.to_string(), "MKV");
        assert_eq!(VideoFormat::MP4.to_string(), "MP4");
        assert_eq!(VideoFormat::MOV.to_string(), "MOV");
        assert_eq!(VideoFormat::MPEG.to_string(), "MPEG");
        assert_eq!(VideoFormat::MPG.to_string(), "MPG");
        assert_eq!(VideoFormat::OGG.to_string(), "OGG");
        assert_eq!(VideoFormat::WEBM.to_string(), "WEBM");
        assert_eq!(VideoFormat::WMV.to_string(), "WMV");
        assert_eq!(VideoFormat::WMA.to_string(), "WMA");
        assert_eq!(VideoFormat::FLV.to_string(), "FLV");
    }
    #[test]
    fn test_debug() {
        assert_eq!(format!("{:?}", VideoFormat::AVI), "AVI");
        assert_eq!(format!("{:?}", VideoFormat::MKV), "MKV");
        assert_eq!(format!("{:?}", VideoFormat::MP4), "MP4");
        assert_eq!(format!("{:?}", VideoFormat::MOV), "MOV");
        assert_eq!(format!("{:?}", VideoFormat::MPEG), "MPEG");
        assert_eq!(format!("{:?}", VideoFormat::MPG), "MPG");
        assert_eq!(format!("{:?}", VideoFormat::OGG), "OGG");
        assert_eq!(format!("{:?}", VideoFormat::WEBM), "WEBM");
        assert_eq!(format!("{:?}", VideoFormat::WMV), "WMV");
        assert_eq!(format!("{:?}", VideoFormat::WMA), "WMA");
        assert_eq!(format!("{:?}", VideoFormat::FLV), "FLV");
    }
}


