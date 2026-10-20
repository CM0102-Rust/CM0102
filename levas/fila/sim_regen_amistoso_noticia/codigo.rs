// ---------- %%LEVA%%a leva: a entrada da aposentadoria, as competições do amistoso e a vaga da notícia (player_regen.cpp, friendly.cpp, news.cpp) ----------

/// { 0x7a7ed0 } (thiscall, ret 8, player_regen.cpp): a entrada da pessoa `*id` na lista das
/// aposentadorias ([+4], 0x1a bytes cada, [+8] quantas): com `por_lesao`, só se o motivo (+0x18) for 8;
/// senão só se já anunciada (+0x19). Fora isso, 0.
unsafe extern "thiscall" fn entrada_da_aposentadoria(regen: usize, id: *const i32, por_lesao: u32) -> usize {
    let lista = *((regen + 4) as *const usize);
    let n = *((regen + 8) as *const i32);
    for i in 0..n.max(0) as usize {
        let e = lista + i * 0x1a;
        if core::ptr::read_unaligned(e as *const i32) == *id {
            let ok = if por_lesao != 0 { *((e + 0x18) as *const u8) == 8 } else { *((e + 0x19) as *const u8) != 0 };
            return if ok { e } else { 0 };
        }
    }
    0
}

/// { 0x5ca030 } (thiscall, friendly.cpp): grava na tabela das competições ([0xadadfc]) o objeto dos
/// amistosos ([0xae2c74]) na chave [0x9cf668] e os torneios do amistoso (127 vagas de 0x69 bytes a partir
/// de +0xb1; com +0xb1 e +0x116 não nulos, +0x116 vai para a chave [+0xaf] + vaga).
unsafe extern "thiscall" fn registrar_competicoes_do_amistoso(amistoso: usize) {
    let tabela = || *(0xadadfc as *const usize) as isize;
    *((tabela() + *(0x9cf668 as *const i32) as isize * 4) as *mut usize) = *(0xae2c74 as *const usize);
    for i in 0..0x7fusize {
        let p = amistoso + 0x116 + i * 0x69;
        let objeto = core::ptr::read_unaligned(p as *const usize);
        if core::ptr::read_unaligned((p - 0x65) as *const usize) != 0 && objeto != 0 {
            let chave = core::ptr::read_unaligned((amistoso + 0xaf) as *const i16) as isize + i as isize;
            *((tabela() + chave * 4) as *mut usize) = objeto;
        }
    }
}

/// { 0x785830 } (thiscall, ret 4, news.cpp): o valor da vaga `vaga` (0 a 49) da notícia (+5, 4 bytes
/// cada). Fora da faixa: a original (caixa de erro, devolve 0).
unsafe extern "thiscall" fn vaga_da_noticia(noticia: usize, vaga: i8) -> u32 {
    if !(0..0x32).contains(&vaga) {
        return orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I2%%)(noticia, vaga);
    }
    core::ptr::read_unaligned((noticia + 5 + vaga as usize * 4) as *const u32)
}

unsafe extern "thiscall" fn comparar_entrada_da_aposentadoria(regen: usize, id: *const i32, por_lesao: u32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const i32, u32) -> usize>(%%I0%%)(regen, id, por_lesao);
    let n = entrada_da_aposentadoria(regen, id, por_lesao);
    PLACAR.registrar(r == n, || format!("entrada_da_aposentadoria({}, {}) original={:#x} rust={:#x}", *id, por_lesao, r, n));
    r
}

unsafe extern "thiscall" fn comparar_registrar_competicoes_do_amistoso(amistoso: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let tabela = *(0xadadfc as *const usize) as isize;
    let base = core::ptr::read_unaligned((amistoso + 0xaf) as *const i16) as isize;
    let reg = [((tabela + *(0x9cf668 as *const i32) as isize * 4) as usize, 4), ((tabela + base * 4) as usize, 0x7f * 4)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(amistoso), || registrar_competicoes_do_amistoso(amistoso));
    PLACAR.registrar(igual, || format!("registrar_competicoes_do_amistoso {:x?} x {:x?}", mo, mr));
}

unsafe extern "thiscall" fn comparar_vaga_da_noticia(noticia: usize, vaga: i8) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, i8) -> u32>(%%I2%%)(noticia, vaga);
    if !(0..0x32).contains(&vaga) {
        return r;
    }
    let n = vaga_da_noticia(noticia, vaga);
    PLACAR.registrar(r == n, || format!("vaga_da_noticia({}) original={:#x} rust={:#x}", vaga, r, n));
    r
}
