// ---------- 283a leva: a fase de mata-mata das copas (Cup_Stage): o construtor (a tabela das rodadas copiada, os times com as sementes e a árvore dos confrontos) e o destrutor (cup_stage.cpp) ----------






/// { 0x522e00 } (thiscall, ret 0x3c, cup_stage.cpp; quadro SEH sem destrutor no caminho normal): `Cup_Stage::Cup_Stage(pai,
/// n, times, rodadas, a5, modelo, ano, fase, b2, id, b3, arvore, por_ordem, a14, sementes)`: a base (Cup::Cup 0x518640,
/// mov ecx -> thiscall, ret 0); +4 = a5; +0x40 = o ano (word); +0x44 = a fase, +0x42 = b2 (bytes); +0x30 = +0x2c = −1;
/// +0x51 = b3; a vtable 0x969468; +0x43 = 1; +0xb1 = +0xab = +0xac = 0; +0x3c = as rodadas e +0x3a = o id (words);
/// +0x49 e +0x4a copiados do pai; +0x28 = a14. A tabela das rodadas (0x944e46 de rodadas × 0x68 em +0xa3; sem memória, a
/// caixa da linha 0x48) copiada do modelo; +0x3e = n e a lista dos times (calloc 0x945501(n, 6) em +0xa7; sem memória, a
/// caixa da linha 0x53): cada time com a situação (+5) 0 e a semente (+4): a do vetor `sementes`, ou, sem ele, a posição
/// na lista se `por_ordem`, ou 0. Por fim a árvore dos confrontos: com `arvore`, Cup_Stage::map_fixture_tree (0x5241e0),
/// senão Cup::map_fixture_tree (0x518790), as duas thiscall, ret 0 (sorteiam); negativa, a caixa da linha 0x62 ou 0x6a.
/// Devolve this.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn cqs7_fase_de_copa(this: usize, pai: usize, n: u32, times: usize, rodadas: u32, a5: u32, modelo: usize, ano: u32, fase: u32, b2: u32,
    id: u32, b3: u32, arvore: u32, por_ordem: u32, a14: u32, sementes: usize) -> usize {
    const F: usize = 0x9b5944;
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_CUP_CUP);
    base(this);
    write_u32_from_usize(this + 4, a5 as usize);
    write_u16(this + 0x40, ano as u16); // cqs7: o ano da fase
    write_u8(this + 0x42, b2 as u8);
    write_u8(this + 0x44, fase as u8);
    write_u32_from_usize(this + 0x30, usize::MAX);
    write_u32_from_usize(this + 0x2c, usize::MAX);
    write_u8(this + 0x51, b3 as u8);
    write_u32_from_usize(this, 0x969468);
    write_u8(this + 0x43, 1);
    write_u8(this + 0xb1, 0);
    write_u8(this + 0xab, 0);
    write_u8(this + 0xac, 0);
    write_u16(this + 0x3c, rodadas as u16);
    write_u16(this + 0x3a, id as u16);
    write_u8(this + 0x49, *((pai + 0x49) as *const u8));
    write_u8(this + 0x4a, *((pai + 0x4a) as *const u8));
    write_u32_from_usize(this + 0x28, a14 as usize);
    let tamanho = (rodadas as u16 as i16 as i32 * 0x68) as u32;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(tamanho);
    write_u32_from_usize(this + 0xa3, t);
    if t == 0 {
        aviso_em(F, 0x48);
        return this;
    }
    core::ptr::copy_nonoverlapping(modelo as *const u8, t as *mut u8, tamanho as usize);
    write_u16(this + 0x3e, n as u16);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let l = zerada(n as u16 as i16 as i32 as u32, 6);
    write_u32_from_usize(this + 0xa7, l);
    if l == 0 {
        aviso_em(F, 0x53);
        return this;
    }
    crate::comparacao::ramo("cqs_7 fase de copa");
    let m = read_i16(this + 0x3e) as i32;
    let mut i = 0i32;
    while i < m {
        let e = i as usize * 6;
        write_u32_from_usize(read_u32_as_usize(this + 0xa7) + e, read_u32_as_usize(times + i as usize * 4)); // cqs7: o time i
        write_u8(read_u32_as_usize(this + 0xa7) + e + 5, 0);
        let s = if sementes != 0 {
            *((sementes + i as usize) as *const u8) // cqs7: a semente do vetor
        } else if por_ordem != 0 {
            i as u8
        } else {
            0
        };
        write_u8(read_u32_as_usize(this + 0xa7) + e + 4, s);
        i += 1;
    }
    let (mapa, linha) = if arvore != 0 { (0x5241e0usize, 0x62) } else { (0x518790usize, 0x6a) };
    let mapa: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(mapa);
    if mapa(this) < 0 {
        aviso_em(F, linha);
    }
    this
}

/// Só a original (a base, as alocações e a árvore, que sorteia: não roda duas vezes), conferindo o plano feito antes: devolve
/// this, +0x3e = n, +0x3c = as rodadas, +0x40 = o ano, as duas tabelas montadas e, na lista, os mesmos clubes.
#[allow(clippy::too_many_arguments)]
unsafe extern "thiscall" fn comparar_cqs7_fase_de_copa(this: usize, pai: usize, n: u32, times: usize, rodadas: u32, a5: u32, modelo: usize, ano: u32, fase: u32, b2: u32,
    id: u32, b3: u32, arvore: u32, por_ordem: u32, a14: u32, sementes: usize) -> usize {
    #[allow(clippy::type_complexity)]
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, u32, usize, u32, u32, u32, u32, u32, u32, u32, u32, usize) -> usize>(2035);
    let k = (n as u16 as i16).max(0) as usize;
    let plano: Vec<(usize, u8)> = (0..k).map(|i| {
        let s = if sementes != 0 { *((sementes + i) as *const u8) } else if por_ordem != 0 { i as u8 } else { 0 };
        (read_u32_as_usize(times + i * 4), s)
    }).collect();
    let tamanho = (rodadas as u16 as i16 as i32 * 0x68).max(0) as usize;
    let modelo_antes: Vec<u8> = core::slice::from_raw_parts(modelo as *const u8, tamanho).to_vec();
    let r = o(this, pai, n, times, rodadas, a5, modelo, ano, fase, b2, id, b3, arvore, por_ordem, a14, sementes);
    let l = read_u32_as_usize(this + 0xa7);
    let t = read_u32_as_usize(this + 0xa3);
    // a árvore (chamada no fim) reescreve partes da tabela das rodadas e da lista (na comparação de 06/11 as duas
    // diferiam do que o construtor grava): confere os campos fixos, a tabela montada e o conjunto dos clubes da lista
    let _ = &modelo_antes;
    let ok = r == this && read_i16(this + 0x3e) as u16 == n as u16 && read_i16(this + 0x3c) as u16 == rodadas as u16 && read_i16(this + 0x40) as u16 == ano as u16
        && t != 0 && l != 0 && {
            let mut a: Vec<usize> = (0..plano.len()).map(|i| read_u32_as_usize(l + i * 6)).collect();
            let mut b: Vec<usize> = plano.iter().map(|x| x.0).collect();
            a.sort_unstable();
            b.sort_unstable();
            a == b
        };
    PLACAR.registrar(ok, || format!("cqs7_fase_de_copa({:#x}) n={} rodadas={} ano={}", this, n as u16 as i16, rodadas as u16 as i16, ano as u16));
    r
}

/// { 0x523300 } (thiscall, ret 0, cup_stage.cpp; o destrutor, chamado pelo destrutor virtual): `Cup_Stage::~Cup_Stage()`:
/// a vtable 0x969468; a lista dos times (+0xa7), a tabela das rodadas (+0xa3) e, com os confrontos (+0xad), cada vetor de
/// confrontos das +0x3c rodadas e o próprio vetor liberados (0x9452ca, cdecl; os ponteiros ficam); por fim a base
/// (Cup::~Cup 0x518690, mov ecx -> thiscall, ret 0), cujo eax ela devolve.
unsafe extern "thiscall" fn cqs7_fim_da_fase_de_copa(this: usize) -> usize {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    write_u32_from_usize(this, 0x969468);
    if read_u32_as_usize(this + 0xa7) != 0 {
        soltar(read_u32_as_usize(this + 0xa7));
    }
    if read_u32_as_usize(this + 0xa3) != 0 {
        soltar(read_u32_as_usize(this + 0xa3));
    }
    if read_u32_as_usize(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < read_i16(this + 0x3c) as i32 {
            let p = read_u32_as_usize(read_u32_as_usize(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
            }
            i += 1;
        }
        soltar(read_u32_as_usize(this + 0xad));
    }
    crate::comparacao::ramo("cqs_7 fim da fase de copa");
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_CUP_DESTRUCTOR_CUP);
    base(this)
}

// ponytail: libera as tabelas e destrói a base; não pode rodar duas vezes: só a original
unsafe extern "thiscall" fn comparar_cqs7_fim_da_fase_de_copa(this: usize) -> usize {
    orig::<unsafe extern "thiscall" fn(usize) -> usize>(2036)(this)
}

