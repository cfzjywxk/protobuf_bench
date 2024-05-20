use criterion::{black_box, criterion_group, criterion_main, Criterion};
use prost::Message as ProstMessage;
use protobuf::Message as ProtobufMessage;
use quick_protobuf::{BytesReader, MessageRead, Writer};

use protobuf_bench::protos::prost::benchmark::TestMessage as ProstTestMessage;
use protobuf_bench::protos::protobuf::benchmark::TestMessage as RustProtobufTestMessage;
use protobuf_bench::protos::quick::benchmark::TestMessage as QuickTestMessage;

fn criterion_benchmark(c: &mut Criterion) {
    let test_message = RustProtobufTestMessage {
        term: 5,
        index: 5,
        data: b"abcdefg".to_vec(),
        context: b"xyz".to_vec(),
        ..Default::default()
    };

    // rust-protobuf
    c.bench_function("rust-protobuf serialize", |b| {
        b.iter(|| {
            let bytes = test_message.write_to_bytes().unwrap();
            black_box(bytes);
        })
    });

    c.bench_function("rust-protobuf deserialize", |b| {
        let bytes = test_message.write_to_bytes().unwrap();
        b.iter(|| {
            let mut res = RustProtobufTestMessage::new();
            let _ = res.merge_from_bytes(&bytes).unwrap();
            black_box(res);
        })
    });

    // prost
    let prost_message = ProstTestMessage {
        term: 5,
        index: 5,
        data: b"abcdefg".into(),
        context: b"xyz".into(),
        ..Default::default()
    };

    c.bench_function("prost serialize", |b| {
        b.iter(|| {
            let mut buf = Vec::new();
            prost_message.encode(&mut buf).unwrap();
            black_box(buf);
        })
    });

    c.bench_function("prost deserialize", |b| {
        let mut buf = Vec::new();
        prost_message.encode(&mut buf).unwrap();
        b.iter(|| {
            let msg = ProstTestMessage::decode(&buf[..]).unwrap();
            black_box(msg);
        })
    });

    // quick-protobuf
    let quick_message = QuickTestMessage {
        term: 5,
        index: 5,
        data: b"abcdefg".into(),
        context: b"xyz".into(),
        ..Default::default()
    };

    c.bench_function("quick-protobuf serialize", |b| {
        b.iter(|| {
            let mut out = Vec::new();
            let mut writer = Writer::new(&mut out);
            writer.write_message(&quick_message).unwrap();
            black_box(out);
        })
    });

    c.bench_function("quick-protobuf deserialize", |b| {
        let mut out = Vec::new();
        {
            let mut writer = Writer::new(&mut out);
            writer.write_message(&quick_message).unwrap();
        }
        b.iter(|| {
            let mut reader = BytesReader::from_bytes(&out);
            // TODO: it's weird the reader would not skip the first byte representing the length
            // of the `out` buffer. Without the skipping operation the deserialize fails.
            let _ = reader.read_u8(&out);
            let msg = QuickTestMessage::from_reader(&mut reader, &out).unwrap();
            black_box(msg);
        })
    });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
