use std::{error::Error, fs, io::Read};
use crate::{bytes::Bytes, json::Json};
use super::*;
use test_case::test_case;

#[test_case("examples/rd300nx/_DEFAULT.RDS")]
#[test_case("examples/rd300nx/ASSIGN.RDS")]
#[test_case("examples/rd300nx/CHO-REV.RDS")]
#[test_case("examples/rd300nx/COMMON.RDS")]
#[test_case("examples/rd300nx/COMP_127.RDS")]
#[test_case("examples/rd300nx/COMP_ON.RDS")]
#[test_case("examples/rd300nx/CONTROL.RDS")]
#[test_case("examples/rd300nx/DEFAULT2.RDS")]
#[test_case("examples/rd300nx/DEFAULT3.RDS")]
#[test_case("examples/rd300nx/DRV-ORIG.RDS")]
#[test_case("examples/rd300nx/FOCUS.RDS")]
#[test_case("examples/rd300nx/EMPTY.RDS")]
#[test_case("examples/rd300nx/EXTERNAL.RDS")]
#[test_case("examples/rd300nx/FLAGS.RDS")]
#[test_case("examples/rd300nx/HAM-CHAR.RDS")]
#[test_case("examples/rd300nx/JUNK.RDS")]
#[test_case("examples/rd300nx/MEM-M1.RDS")]
#[test_case("examples/rd300nx/MFX.RDS")]
#[test_case("examples/rd300nx/MFX75.RDS")]
#[test_case("examples/rd300nx/MFX-ALL.RDS")]
#[test_case("examples/rd300nx/PART+PERF.RDS")]
#[test_case("examples/rd300nx/PIANO.RDS")]
#[test_case("examples/rd300nx/PRESET1.RDS")]
#[test_case("examples/rd300nx/PRESET2.RDS")]
#[test_case("examples/rd300nx/PRESET3.RDS")]
#[test_case("examples/rd300nx/PRESET4.RDS")]
#[test_case("examples/rd300nx/RELEASE.RDS")]
#[test_case("examples/rd300nx/RX-ON.RDS")]
#[test_case("examples/rd300nx/RX-RESET.RDS")]
#[test_case("examples/rd300nx/SONG.RDS")]
#[test_case("examples/rd300nx/SPONGE.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE1-1.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE1-2.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE1-4.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE1-8.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE2-1.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE2-2.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE2-4.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE2-8.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE3-1.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE3-2.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE3-4.RDS")]
#[test_case("examples/rd300nx/SWASS-PRE3-8.RDS")]
#[test_case("examples/rd300nx/SYS-COM1.RDS")]
#[test_case("examples/rd300nx/SYS-COM2.RDS")]
#[test_case("examples/rd300nx/SYS-COMP1.RDS")]
#[test_case("examples/rd300nx/SYS-COMP2.RDS")]
#[test_case("examples/rd300nx/SYS-COMP3.RDS")]
#[test_case("examples/rd300nx/SYS-FAV1.RDS")]
#[test_case("examples/rd300nx/SYS-FAV2.RDS")]
#[test_case("examples/rd300nx/SYS-FAV3.RDS")]
#[test_case("examples/rd300nx/SYS-FAV4.RDS")]
#[test_case("examples/rd300nx/SYS-FAV5.RDS")]
#[test_case("examples/rd300nx/SYS-VLNK1.RDS")]
#[test_case("examples/rd300nx/SYS-VLNK2.RDS")]
#[test_case("examples/rd300nx/TMP-1.RDS")]
#[test_case("examples/rd300nx/TMP-7.RDS")]
#[test_case("examples/rd300nx/TMP-7B.RDS")]
#[test_case("examples/rd300nx/TMP-7CS.RDS")]
#[test_case("examples/rd300nx/TONE_VOLUME.RDS")]
#[test_case("examples/rd300nx/TOUCH.RDS")]
#[test_case("examples/rd300nx/VLINK.RDS")]
#[test_case("examples/rd300nx/VOL.RDS")]
fn encode_decode(rds_filename: &str) -> Result<(), Box<dyn Error>> {
    let mut rds_bytes = Vec::new();
    let mut f = fs::File::options().read(true).open(&rds_filename)?;
    f.read_to_end(&mut rds_bytes)?;
    let decode = rd300nx::RD300NX::from_bytes(rds_bytes.clone().try_into().unwrap())?;
    let json = decode.to_json();
    let encode = rd300nx::RD300NX::from_json(json)?;
    assert_eq!(rds_bytes, encode.to_bytes()?.to_vec());
    Ok(())
}
