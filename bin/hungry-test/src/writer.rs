use hungry::writer::QueuedWriter;
use crate::{Transport, Write};

pub(crate) async fn run_writer(mut writer: QueuedWriter<Write, Transport>) {
    loop {
        let buffer = (&mut writer).await.unwrap();

        println!(
            "WRITER: got back buffer ({:?}..{:?}, {})",
            buffer.as_ptr(),
            buffer.as_ptr().wrapping_add(buffer.capacity()),
            buffer.capacity()
        );
    }
}
