mod reader;
mod writer;

use crate::reader::run_reader;
use crate::writer::run_writer;

use bytes::BytesMut;

use hungry::tl::{Serialize, Int128};
use hungry::{Envelope, mtproto};
use hungry::reader::{Dump, PlainDeserializer};
use hungry::writer::QueuedWriter;

const ADDR: &str = "149.154.167.40:443";

type Transport = hungry::transport::Full;
type Read = tokio::net::tcp::OwnedReadHalf;
type Write = tokio::net::tcp::OwnedWriteHalf;

fn queue_request(writer: &mut QueuedWriter<Write, Transport>) {
    let mut buffer = BytesMut::with_capacity(1024 * 1024);

    let transport_envelope = Envelope::split(&mut buffer);
    let mtp_envelope = Envelope::split(&mut buffer);

    let mut nonce = Int128::default();
    rand::fill(&mut nonce);

    let request = mtproto::tl::funcs::ReqPqMulti { nonce };

    unsafe {
        buffer.set_len(request.serialized_len());
        request.serialize_unchecked(buffer.as_mut_ptr());
    }

    writer.queue_plain(buffer, transport_envelope, mtp_envelope, 0);
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    let stream = tokio::net::TcpStream::connect(ADDR).await?;
    let (r, w) = stream.into_split();

    let behaviour = Dump(PlainDeserializer::new());

    let buffer = BytesMut::with_capacity(1024);

    let (reader, writer) = hungry::new::<Transport, _, _, _>(r, w, behaviour, buffer);

    let mut writer = QueuedWriter::new(writer);

    queue_request(&mut writer);

    let _writer = tokio::task::spawn(async move { run_writer(writer).await });
    let _reader = tokio::task::spawn(async move { run_reader(reader).await });

    tokio::signal::ctrl_c().await?;

    Ok(())
}
