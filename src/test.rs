use crate::*;

pub(crate) trait TestConfigExt: ConfigSource + Sized + 'static {
    fn new_config(self) -> Configuration {
        Configuration::new().register_source(self)
    }
}

impl<C: ConfigSource + 'static> TestConfigExt for C {}

type R<V> = Result<V, ConfigError>;
use std::collections::HashMap;

#[derive(Debug, FromConfig)]
struct ConfigSuit {
    #[config(name = "val")]
    int: IntSuit,
    arr: Vec<String>,
    brr: Vec<Vec<String>>,
    #[config(name = "val")]
    map: HashMap<String, usize>,
    #[config(name = "map")]
    bap: HashMap<String, Vec<bool>>,
    crr: Vec<FloatSuit>,
    err: R<u8>,
}
#[derive(Debug, FromConfig)]
struct FloatSuit {
    v1: f32,
    v2: f64,
}

#[derive(Debug, FromConfig)]
struct IntSuit {
    v1: u8,
    v2: u16,
    v3: u32,
    v4: u64,
    v5: u128,
    v6: usize,
}

#[allow(dead_code)]
pub(crate) fn source_test_suit(src: impl ConfigSource + 'static) -> Result<(), ConfigError> {
    let config = src.new_config();
    let v: ConfigSuit = config.get("suit")?;
    assert_eq!(vec!["a0", "a1", "a2"], v.arr);
    assert_eq!(Some(&vec![true]), v.bap.get("b1"));
    assert_eq!(Some(&vec![true, false]), v.bap.get("b2"));
    let brr = vec!["b00"];
    assert_eq!(vec![brr], v.brr);
    for i in 1..=6 {
        assert_eq!(Some(&i), v.map.get(&format!("v{}", i)));
    }
    assert_eq!(1, v.int.v1);
    assert_eq!(2, v.int.v2);
    assert_eq!(3, v.int.v3);
    assert_eq!(4, v.int.v4);
    assert_eq!(5, v.int.v5);
    assert_eq!(6, v.int.v6);

    assert_eq!(1, v.crr.len());
    let crr = &v.crr[0];
    assert_eq!(1.0, crr.v1);
    assert_eq!(2.0, crr.v2);
    assert_eq!(true, v.err.is_err());
    Ok(())
}

#[test]
fn in_memory_test() {
    use crate::source::memory::MemorySource;
    source_test_suit(
        MemorySource::default()
            .set("suit.val.v1", "1")
            .set("suit.val.v2", "2")
            .set("suit.val.v3", "3")
            .set("suit.val.v4", "4")
            .set("suit.val.v5", "5")
            .set("suit.val.v6", "6")
            .set("suit.arr[0]", "a0")
            .set("suit.arr[1]", "a1")
            .set("suit.arr[2]", "a2")
            .set("suit.map.b1[0]", "true")
            .set("suit.map.b2[0]", "true")
            .set("suit.map.b2[1]", "false")
            .set("suit.crr[0].v1", "1.0")
            .set("suit.crr[0].v2", "2.0")
            .set("suit.brr[0][0]", "b00"),
    )
    .unwrap();
}
