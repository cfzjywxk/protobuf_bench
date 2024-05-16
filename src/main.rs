use protobuf_bench::protos::quick::benchmark::TestMessage as QuickTestMessage;
use quick_protobuf::{BytesReader, MessageRead, Writer};

fn main() {
    let quick_message = QuickTestMessage {
        term: 5,
        index: 5,
        data: b"abcdefg".into(),
        context: b"xyz".into(),
    };
    let mut out = Vec::new();
    {
        let mut writer = Writer::new(&mut out);
        writer.write_message(&quick_message).unwrap();
    }
    let mut reader = BytesReader::from_bytes(&out);
    println!("out={:?}", out);
    let _ = reader.read_u8(&out);
    let err = QuickTestMessage::from_reader(&mut reader, &out);
    println!("Hello, world! res={:?}", err.unwrap());
}
