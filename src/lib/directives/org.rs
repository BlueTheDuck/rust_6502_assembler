|state,param|{
    let param = match param {
        ParamTypes::Addr(addr) => addr,
        _ => panic!(".org only takes one address as parameter")
    };
    println!("Setting PC to {:X?}",param);
    state.pc = param as usize;
};