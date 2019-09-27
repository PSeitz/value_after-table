use serde_json::Deserializer;
use toml::ser::Serializer;

fn main() {
    println!("Hello, world!");
    let mut toml = String::new();

    let mut deserializer = Deserializer::from_str(r#"{ "kana": {"ent_seq": "1000000"}, "kanji": [] }"#);
    let mut serializer = Serializer::pretty(&mut toml);
    serde_transcode::transcode(&mut deserializer, &mut serializer).unwrap();

}
