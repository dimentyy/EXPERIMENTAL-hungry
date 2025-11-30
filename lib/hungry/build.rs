fn main() {
    let mtproto_schema = std::fs::read_to_string("tl/mtproto.tl").unwrap();

    let mtproto_config = hungry_tl::Config {
        derive_clone: false,
        derive_debug: true,
        schema_title: "mtproto".to_owned(),
        main_tl_path: "crate::tl".to_owned(),
        include_path: "crate::mtproto::tl".to_owned(),
    };

    hungry_tl::generate(mtproto_config, &mtproto_schema);

    let api_schema = std::fs::read_to_string("tl/api.tl").unwrap();

    let api_config = hungry_tl::Config {
        derive_clone: true,
        derive_debug: true,
        schema_title: "api".to_owned(),
        main_tl_path: "crate::tl".to_owned(),
        include_path: "crate::api::tl".to_owned(),
    };

    hungry_tl::generate(api_config, &api_schema);
}
