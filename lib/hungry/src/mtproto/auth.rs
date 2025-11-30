use crate::{mtproto, tl};

pub fn req_pq_multi(nonce: tl::Int128) -> mtproto::tl::funcs::ReqPqMulti {
    mtproto::tl::funcs::ReqPqMulti { nonce }
}

// pub fn req_dh_params(res_pq: mtproto::tl::enums::ResPq) -> mtproto::tl::funcs:: {}
