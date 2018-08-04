extern crate gpt;
extern crate tempfile;

use gpt::mbr;
use std::io::Read;

#[test]
fn test_mbr_partrecord() {
    let pr0 = mbr::PartRecord::zero();
    let data0 = pr0.as_bytes().unwrap();
    assert_eq!(data0.len(), 16);
    assert_eq!(data0, [0x00; 16]);

    let pr1 = mbr::PartRecord::new_protective(None);
    let data1 = pr1.as_bytes().unwrap();
    assert_eq!(data0.len(), data1.len());
    assert_ne!(data0, data1);
}

#[test]
fn test_mbr_protective() {
    let m0 = mbr::ProtectiveMBR::new();
    let data0 = m0.as_bytes().unwrap();
    assert_eq!(data0.len(), 512);
    assert_eq!(data0[510], 0x55);
    assert_eq!(data0[511], 0xAA);

    let m1 = mbr::ProtectiveMBR::with_lb_size(0x01);
    let data1 = m1.as_bytes().unwrap();
    assert_eq!(data0.len(), data1.len());
    assert_ne!(data0, data1);
    assert_eq!(data1[510], 0x55);
    assert_eq!(data1[511], 0xAA);
}

#[test]
fn test_mbr_write() {
    let mut tempdisk = tempfile::tempfile().unwrap();
    let m0 = mbr::ProtectiveMBR::new();
    let data0 = m0.as_bytes().unwrap();
    m0.overwrite_lba0(&mut tempdisk).unwrap();
    m0.update_conservative(&mut tempdisk).unwrap();

    let mut buf = Vec::new();
    let size = tempdisk.read_to_end(&mut buf).unwrap();
    assert!(size != 0);
    assert_eq!(buf, data0);
}
