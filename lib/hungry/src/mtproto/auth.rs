use crate::tl;

pub fn req_pq_multi(nonce: tl::Int128) -> tl::mtproto::funcs::ReqPqMulti {
    tl::mtproto::funcs::ReqPqMulti { nonce }
}

// pub fn req_dh_params(res_pq: mtproto::schema::enums::ResPq) -> mtproto::schema::funcs:: {}
