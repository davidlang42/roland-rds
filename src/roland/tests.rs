use std::{error::Error, fs, io::Read};
use crate::{bytes::Bytes, json::Json};
use super::{*, rd300nx::RD300NX};
use schemars::schema_for;
use test_case::test_case;
use validator::Validate;

#[test_case("examples/rd300nx/AJGS-2016.RDS")]
#[test_case("examples/rd300nx/AM.RDS")]
#[test_case("examples/rd300nx/ASSIGN.RDS")]
#[test_case("examples/rd300nx/BLANK.RDS")]
#[test_case("examples/rd300nx/BMC-1.RDS")]
#[test_case("examples/rd300nx/BMC-2.RDS")]
#[test_case("examples/rd300nx/C1-MAX.RDS")]
#[test_case("examples/rd300nx/CGS-2014.RDS")]
#[test_case("examples/rd300nx/CGS-2017-v1.RDS")]
#[test_case("examples/rd300nx/CGS-20170704.RDS")]
#[test_case("examples/rd300nx/CH-D.RDS")]
#[test_case("examples/rd300nx/CH.RDS")]
#[test_case("examples/rd300nx/CHO-REV.RDS")]
#[test_case("examples/rd300nx/CHO-REV-DFLTS.RDS")]
#[test_case("examples/rd300nx/COMMON.RDS")]
#[test_case("examples/rd300nx/COMP_127.RDS")]
#[test_case("examples/rd300nx/COMP_ON.RDS")]
#[test_case("examples/rd300nx/CONTROL.RDS")]
#[test_case("examples/rd300nx/D1.RDS")]
#[test_case("examples/rd300nx/DEFAULT.RDS")]
#[test_case("examples/rd300nx/DEFAULT1.RDS")]
#[test_case("examples/rd300nx/DEFAULT2.RDS")]
#[test_case("examples/rd300nx/DEFAULT3.RDS")]
#[test_case("examples/rd300nx/DEFAULT_AGAIN.RDS")]
#[test_case("examples/rd300nx/DEFAULT_RESAVED.RDS")]
#[test_case("examples/rd300nx/DRV-ORIG.RDS")]
#[test_case("examples/rd300nx/DUST-1.RDS")]
#[test_case("examples/rd300nx/DUST-2.RDS")]
#[test_case("examples/rd300nx/EFFECT-AM.RDS")]
#[test_case("examples/rd300nx/EFFECTS.RDS")]
#[test_case("examples/rd300nx/EMPTY.RDS")]
#[test_case("examples/rd300nx/EXTERNAL.RDS")]
#[test_case("examples/rd300nx/FLAGS.RDS")]
#[test_case("examples/rd300nx/FOCUS.RDS")]
#[test_case("examples/rd300nx/GS-2015-V1.RDS")]
#[test_case("examples/rd300nx/GS-2015-V2.RDS")]
#[test_case("examples/rd300nx/HAIR.RDS")]
#[test_case("examples/rd300nx/HAM-CHAR.RDS")]
#[test_case("examples/rd300nx/LEASE-2017-05-10.RDS")]
#[test_case("examples/rd300nx/LEASE-2017-05-28.RDS")]
#[test_case("examples/rd300nx/LEASE-2017-05-30.RDS")]
#[test_case("examples/rd300nx/LEASE-2017-05-31.RDS")]
#[test_case("examples/rd300nx/LEASE.RDS")]
#[test_case("examples/rd300nx/MAMG-1.RDS")]
#[test_case("examples/rd300nx/MAMG-2.RDS")]
#[test_case("examples/rd300nx/MFX-NEW.RDS")]
#[test_case("examples/rd300nx/MFX.RDS")]
#[test_case("examples/rd300nx/MFX0-59.RDS")]
#[test_case("examples/rd300nx/MFX0-59_MAX.RDS")]
#[test_case("examples/rd300nx/MFX0-59_MIN.RDS")]
#[test_case("examples/rd300nx/MFX60-78.RDS")]
#[test_case("examples/rd300nx/MFX60-78_MAX.RDS")]
#[test_case("examples/rd300nx/MFX60-78_MIN.RDS")]
#[test_case("examples/rd300nx/MFX75.RDS")]
#[test_case("examples/rd300nx/PART+PERF.RDS")]
#[test_case("examples/rd300nx/PIANO.RDS")]
#[test_case("examples/rd300nx/PRESET1.RDS")]
#[test_case("examples/rd300nx/PRESET2.RDS")]
#[test_case("examples/rd300nx/PRESET3.RDS")]
#[test_case("examples/rd300nx/PRESET4.RDS")]
#[test_case("examples/rd300nx/R1.RDS")]
#[test_case("examples/rd300nx/R2.RDS")]
#[test_case("examples/rd300nx/RAIN.RDS")]
#[test_case("examples/rd300nx/RELEASE.RDS")]
#[test_case("examples/rd300nx/RX-ON.RDS")]
#[test_case("examples/rd300nx/RX-RESET.RDS")]
#[test_case("examples/rd300nx/SB-2022-10-02.RDS")]
#[test_case("examples/rd300nx/SB-2022-10-03.RDS")]
#[test_case("examples/rd300nx/SB.RDS")]
#[test_case("examples/rd300nx/SB2.RDS")]
#[test_case("examples/rd300nx/SOM.RDS")]
#[test_case("examples/rd300nx/SONG.RDS")]
#[test_case("examples/rd300nx/SOUL-V1.RDS")]
#[test_case("examples/rd300nx/SOUL-V2.RDS")]
#[test_case("examples/rd300nx/SOUL-V3.RDS")]
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
#[test_case("examples/rd300nx/SYS-FAV3.RDS")]
#[test_case("examples/rd300nx/SYS-FAV4.RDS")]
#[test_case("examples/rd300nx/SYS-FAV5.RDS")]
#[test_case("examples/rd300nx/SYS_PEDAL_EXPRESSION.RDS")]
#[test_case("examples/rd300nx/SYS_PEDAL_MODE.RDS")]
#[test_case("examples/rd300nx/TMM.RDS")]
#[test_case("examples/rd300nx/TMP-1.RDS")]
#[test_case("examples/rd300nx/TMP-7.RDS")]
#[test_case("examples/rd300nx/TMP-7B.RDS")]
#[test_case("examples/rd300nx/TMP-7CS.RDS")]
#[test_case("examples/rd300nx/TONE_VOLUME.RDS")]
#[test_case("examples/rd300nx/TOUCH.RDS")]
#[test_case("examples/rd300nx/VLINK.RDS")]
#[test_case("examples/rd300nx/VLink-Channel1.RDS")]
#[test_case("examples/rd300nx/VLink-Channel10.RDS")]
#[test_case("examples/rd300nx/VLink-Channel11.RDS")]
#[test_case("examples/rd300nx/VLink-Channel12.RDS")]
#[test_case("examples/rd300nx/VLink-Channel13.RDS")]
#[test_case("examples/rd300nx/VLink-Channel14.RDS")]
#[test_case("examples/rd300nx/VLink-Channel15.RDS")]
#[test_case("examples/rd300nx/VLink-Channel16.RDS")]
#[test_case("examples/rd300nx/VLink-Channel2.RDS")]
#[test_case("examples/rd300nx/VLink-Channel3.RDS")]
#[test_case("examples/rd300nx/VLink-Channel4.RDS")]
#[test_case("examples/rd300nx/VLink-Channel5.RDS")]
#[test_case("examples/rd300nx/VLink-Channel6.RDS")]
#[test_case("examples/rd300nx/VLink-Channel7.RDS")]
#[test_case("examples/rd300nx/VLink-Channel8.RDS")]
#[test_case("examples/rd300nx/VLink-Channel9.RDS")]
#[test_case("examples/rd300nx/VLink-Local1.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel1.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel10.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel11.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel12.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel13.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel14.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel15.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel16.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel2.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel3.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel4.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel5.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel6.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel7.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel8.RDS")]
#[test_case("examples/rd300nx/VLink-Mode1-Channel9.RDS")]
#[test_case("examples/rd300nx/VOL.RDS")]
#[test_case("examples/rd300nx/WOZ.RDS")]
#[test_case("examples/rd300nx/system-common-back.RDS")]
#[test_case("examples/rd300nx/system-common-then-write.RDS")]
#[test_case("examples/rd300nx/system-common.RDS")]
#[test_case("examples/rd300nx/system-userset1.RDS")]
#[test_case("examples/rd300nx/system-vlink3.RDS")]
#[test_case("examples/rd300nx/vlink-mode-local0.RDS")]
#[test_case("examples/rd300nx/vlink-mode-local2.RDS")]
fn decode_validate_encode(rds_filename: &str) -> Result<(), Box<dyn Error>> {
    let mut rds_bytes = Vec::new();
    let mut f = fs::File::options().read(true).open(&rds_filename)?;
    f.read_to_end(&mut rds_bytes)?;
    let decode = rd300nx::RD300NX::from_bytes(rds_bytes.clone().try_into().unwrap())?;
    assert!(decode.validate().is_ok());
    let json = decode.to_json();
    let encode = rd300nx::RD300NX::from_json(json)?;
    assert_eq!(rds_bytes, encode.to_bytes()?.to_vec());
    Ok(())
}

#[test_case("examples/rd300nx/MFX0-59.RDS", 60)]
#[test_case("examples/rd300nx/MFX60-78.RDS", 19)]
fn mfx_default_values(rds_filename: &str, user_set_count: usize) -> Result<(), Box<dyn Error>> {
    let mut rds_bytes = Vec::new();
    let mut f = fs::File::options().read(true).open(&rds_filename)?;
    f.read_to_end(&mut rds_bytes)?;
    let rds = rd300nx::RD300NX::from_bytes(rds_bytes.try_into().unwrap())?;
    for ls in rds.user_sets.iter().take(user_set_count) {
        let found = &ls.mfx.mfx_type;
        let expected = found.default();
        let f = found.parameters();
        let e = expected.parameters();
        assert_eq!(f.len(), e.len());
        for i in 0..f.len() {
            assert_eq!(f[i], e[i], "MFX{}({}), Parameter #{}", found.number(), found.name(), i + 1);
        }
    }
    Ok(())
}

#[test_case("examples/rd300nx/CHO-REV-DFLTS.RDS", 7)]
fn reverb_default_values(rds_filename: &str, user_set_count: usize) -> Result<(), Box<dyn Error>> {
    let mut rds_bytes = Vec::new();
    let mut f = fs::File::options().read(true).open(&rds_filename)?;
    f.read_to_end(&mut rds_bytes)?;
    let rds = rd300nx::RD300NX::from_bytes(rds_bytes.try_into().unwrap())?;
    for ls in rds.user_sets.iter().take(user_set_count) {
        let found = &ls.reverb.reverb_type;
        let expected = found.default();
        let f = found.parameters();
        let e = expected.parameters();
        assert_eq!(f.len(), e.len());
        for i in 0..f.len() {
            assert_eq!(f[i], e[i], "Reverb{}({}), Parameter #{}", found.number(), found.name(), i + 1);
        }
    }
    Ok(())
}

#[test_case("examples/rd300nx/CHO-REV-DFLTS.RDS", 4)]
fn chorus_default_values(rds_filename: &str, user_set_count: usize) -> Result<(), Box<dyn Error>> {
    let mut rds_bytes = Vec::new();
    let mut f = fs::File::options().read(true).open(&rds_filename)?;
    f.read_to_end(&mut rds_bytes)?;
    let rds = rd300nx::RD300NX::from_bytes(rds_bytes.try_into().unwrap())?;
    for ls in rds.user_sets.iter().take(user_set_count) {
        let found = &ls.chorus.chorus_type;
        let expected = found.default();
        let f = found.parameters();
        let e = expected.parameters();
        assert_eq!(f.len(), e.len());
        for i in 0..f.len() {
            assert_eq!(f[i], e[i], "Chorus{}({}), Parameter #{}", found.number(), found.name(), i + 1);
        }
    }
    Ok(())
}

#[test_case("schema/rd300nx.json")]
fn no_changes_to_schema(schema_filename: &str) -> Result<(), Box<dyn Error>> {
    let mut bytes = Vec::new();
    let mut f = fs::File::options().read(true).open(&schema_filename)?;
    f.read_to_end(&mut bytes)?;
    let expected_schema: String = bytes.into_iter().map(|u| u as char).collect();
    let found_schema = serde_json::to_string_pretty(&schema_for!(RD300NX)).unwrap();
    assert_eq!(expected_schema, found_schema);
    Ok(())
}