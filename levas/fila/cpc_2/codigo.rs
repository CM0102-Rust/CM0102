// ---------- %%LEVA%%a leva: o amistoso marcado do clube e a lista de partidas da data (friendly.cpp, fix_man.cpp) ----------

/// { 0x5c88f0 } (thiscall, ret 8, friendly.cpp): `find_friendly_match(data, clube)`: o amistoso do clube na lista
/// de amistosos (+0x34c8; nula: 0), do começo (cursor +0xa = +2). Com `data`: os da mesma data (0x4a7f20 com os dois
/// dwords dela) e do clube (+0xc); o primeiro com +0x10 e +0x14 os dois não nulos sai na hora, os outros ficam
/// guardados (o último) e um segundo achado sai na hora; no fim, o guardado (ou 0). Sem `data`: o primeiro do clube
/// com +0x10 ou +0x14 nulo e +8 não nulo. A lista é relida do gerenciador a cada passo, como na original. Na marcação
/// e nas telas dos amistosos (0x5c6f32, 0x5c86de, 0x5c9139), na mídia (0x77ce23, 0x782d9e, 0x784314) e em 0x4741f9.
/// Sem SEH, sem caixa, sem locais.
unsafe extern "thiscall" fn amistoso_do_clube(this: usize, data: usize, clube: usize) -> usize {
    let u = |a: usize| core::ptr::read_unaligned(a as *const usize);
    let mesma_data: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x4a7f20usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = u(this + 0x34c8);
    if l == 0 {
        return 0;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, u(l + 2));
    let mut guardado = 0;
    let mut a = proximo(l);
    while a != 0 {
        let do_clube = u(a + 0xc) == clube; // o clube do amistoso
        if data != 0 {
            if mesma_data(a, u(data) as u32, u(data + 4) as u32) != 0 && do_clube {
                if guardado != 0 || (u(a + 0x10) != 0 && u(a + 0x14) != 0) {
                    return a;
                }
                guardado = a;
            }
        } else if do_clube && (u(a + 0x10) == 0 || u(a + 0x14) == 0) && u(a + 8) != 0 {
            return a;
        }
        a = proximo(u(this + 0x34c8));
    }
    guardado
}

comparar_valor!(comparar_amistoso_do_clube, amistoso_do_clube, %%I0%%, (data: usize, clube: usize) -> usize);

/// { 0x5ae6a0 } (stdcall, ret 0xc, fix_man.cpp): `get_fixture_list_for_date(data, fatia, grupo)`: a lista de partidas
/// da 0x5abf10 do gerenciador [0xae2a58] para o ano (+2) e o dia (+0) da data. O argumento do ano leva na metade alta a
/// do ponteiro da data (`mov ax, [eax+2]`), reproduzida; o do dia leva a do ecx do chamador, que a 0x5abf10 descarta
/// (`and 0xffff`): a cópia passa zero. No dia de jogos (montar_o_dia 0x6b3c10, Rust, cada lista do dia), nos
/// amistosos (0x5cbf3e), no técnico humano (0x5fe83a), na mídia (0x7776a8) e em 0x7e6498, 0x7e6ac9.
unsafe extern "stdcall" fn lista_da_data(data: usize, fatia: u32, grupo: u32) -> usize {
    let lista: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> usize = core::mem::transmute(0x5abf10usize);
    let ano = (data as u32 & 0xffff_0000) | core::ptr::read_unaligned((data + 2) as *const u16) as u32;
    let dia = core::ptr::read_unaligned(data as *const u16) as u32;
    lista(*(0xae2a58 as *const usize), ano, dia, fatia, grupo) // o gerenciador de partidas
}

unsafe extern "stdcall" fn comparar_lista_da_data(data: usize, fatia: u32, grupo: u32) -> usize {
    let r = orig::<unsafe extern "stdcall" fn(usize, u32, u32) -> usize>(%%I1%%)(data, fatia, grupo);
    let n = lista_da_data(data, fatia, grupo);
    PLACAR.registrar(r == n, || format!("lista_da_data({:x}, fatia {}, grupo {}) original={:x} rust={:x}", data, fatia, grupo, r, n));
    r
}
