use crate::{Read, Transport};
use hungry::tl;
use hungry::reader::{Dump, PlainDeserializer, Reader};

type Behaviour = Dump<PlainDeserializer<tl::mtproto::enums::ResPq>>;

pub(crate) async fn run_reader(mut reader: Reader<Read, Behaviour, Transport>) {
    loop {
        dbg!((&mut reader).await.unwrap().unwrap());
    }
}
