|state,param|{
    let bs;
    match param {
        ParamTypes::MultiBytes(e) => bs = e,
        _=>panic!(".bytes takes a comma separated list of bytes")
    }
    println!("{:#?}",bs);
    for b in bs {
        state.push_byte(b);
    }
}