


// fn main() {
//     let mut toml = String::new();
//     let mut deserializer = Deserializer::from_str(r#"{ "kana": {"ent_seq": "1000000"}, "kanji": [] }"#);
//     let mut serializer = Serializer::pretty(&mut toml);
//     serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();
// }

fn main() {
    let val: serde_json::Value = serde_json::from_str(r#"{ "kana": {"ent_seq": "1000000"}, "kanji": [] }"#).unwrap();
    toml::ser::to_string_pretty(&val).unwrap();
}
