use std::io::{Cursor, Read, Seek};

use byteorder::{ReadBytesExt, LE};
use crate::{PSUEntry, PSUEntryKind, PSUParser, DIR_ID, FILE_ID, PAGE_SIZE, PSU};
use crate::util::parse_cstring;

impl PSU {
    pub fn entries(&self) -> Vec<PSUEntry> {
        self.entries.clone()
    }
}

impl PSU {
    pub fn new(bytes: Vec<u8>) -> Self {
        Self {
            entries: PSUParser::new(bytes).parse().unwrap(),
        }
    }
}

impl PSUParser {
    fn new(bytes: Vec<u8>) -> PSUParser {
        let len = bytes.len() as u64;
        Self {
            c: Cursor::new(bytes),
            len,
        }
    }

    fn parse(&mut self) -> Result<Vec<PSUEntry>, std::io::Error> {
        let mut result = vec![];
        while self.c.position() < self.len {
            let entry = self.read_entry()?;
            result.push(entry);
        }

        Ok(result)
    }

    fn read_entry(&mut self) -> Result<PSUEntry, std::io::Error> {
        let id = self.c.read_u16::<LE>()?;
        let _ = self.c.read_u16::<LE>()?;
        let size = self.c.read_u32::<LE>()?;
        let created = self.read_timestamp()?;
        let sector = self.c.read_u16::<LE>()?;
        let _ = self.c.read_u16::<LE>()?;
        let _ = self.c.read_u32::<LE>()?;
        let modified = self.read_timestamp()?;
        self.c.seek_relative(32)?;

        let mut name = [0; 448];
        self.c.read_exact(&mut name)?;

        let contents = if id == FILE_ID {
            let mut contents = vec![0; size as usize];
            self.c.read_exact(&mut contents)?;
            let rem = 1024 - (size % 1024);

            self.c
                .seek_relative(if rem == PAGE_SIZE { 0 } else { rem as i64 })?;

            Some(contents)
        } else {
            None
        };

        Ok(PSUEntry {
            id,
            size,
            created,
            sector,
            modified,
            name: parse_cstring(&name),
            kind: if id == DIR_ID {
                PSUEntryKind::Directory
            } else {
                PSUEntryKind::File
            },
            contents,
        })
    }

    fn read_timestamp(&mut self) -> Result<chrono::NaiveDateTime, std::io::Error> {
        _ = self.c.read_u8()?;
        let seconds = self.c.read_u8()?;
        let minutes = self.c.read_u8()?;
        let hours = self.c.read_u8()?;
        let days = self.c.read_u8()?;
        let months = self.c.read_u8()?;
        let year = self.c.read_u16::<LE>()?;

        let date = chrono::NaiveDate::from_ymd_opt(year as i32, months as u32, days as u32)
            .unwrap()
            .and_hms_opt(hours as u32, minutes as u32, seconds as u32)
            .unwrap();
        Ok(date)
    }
}
