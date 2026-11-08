// ---------- %%LEVA%%a leva: a lista de alvos dos clubes: as vagas da formação, refazer a lista do clube e suspender os clubes que miram o jogador (shortlist_manager.cpp) ----------

unsafe fn cpp5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp5_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// A entrada de 9 bytes do clube na Shortlist_Manager (+0xdc; +0 as 40 vagas de 0x25 bytes, +8 o estado).
unsafe fn cpp5_entrada(this: usize, clube: usize) -> usize {
    cpp5_d(this + 0xdc).wrapping_add(cpp5_d(clube).wrapping_mul(9))
}

/// A primeira diferença entre duas leituras de memória (endereço, original, Rust), para a mensagem.
fn cpp5_dif(a: &crate::comparacao::Memoria, b: &crate::comparacao::Memoria) -> Option<(usize, u8, u8)> {
    for ((ea, va), (_, vb)) in a.0.iter().zip(b.0.iter()) {
        if let Some(k) = (0..va.len()).find(|&k| va[k] != vb[k]) {
            return Some((ea + k, va[k], vb[k]));
        }
    }
    None
}

/// As máscaras de posição (+1, 16 bits) das vagas 1..24 de jogador, por formação (+7 da entrada); a vaga 0 é sempre
/// o goleiro (1). A linha 0 vale para formação fora de 1..9 (o `default` do switch).
static CPP5_MASCARAS: [[u16; 24]; 10] = [
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x8, 0x10, 0x10, 0x20, 0x40, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // fora de 1..9
    [0x808, 0x88, 0x404, 0x2, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x2, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formação 1
    [0x808, 0x88, 0x404, 0x204, 0x104, 0x410, 0x220, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formação 2
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x210, 0x1, 0x40, 0x4, 0x8, 0x10, 0x1], // formação 3
    [0x804, 0x84, 0x404, 0x104, 0x210, 0x90, 0x810, 0x140, 0x440, 0x240, 0x4, 0x4, 0x10, 0x10, 0x140, 0x440, 0x4, 0x10, 0x1, 0x40, 0x4, 0x10, 0x40, 0x1], // formação 4
    [0x804, 0x84, 0x404, 0x104, 0x820, 0xa0, 0x410, 0x110, 0x440, 0x140, 0x4, 0x4, 0x10, 0x20, 0x440, 0x140, 0x204, 0x10, 0x1, 0x40, 0x4, 0x20, 0x10, 0x1], // formação 5
    [0x810, 0x90, 0x404, 0x204, 0x104, 0x410, 0x208, 0x110, 0x440, 0x140, 0x8, 0x210, 0x90, 0x810, 0x240, 0x220, 0x204, 0x8, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 6
    [0x804, 0x84, 0x404, 0x104, 0x208, 0x220, 0x410, 0x110, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 7
    [0x804, 0x84, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x240, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 8
    [0x204, 0x404, 0x104, 0x810, 0x90, 0x410, 0x110, 0x210, 0x140, 0x440, 0x4, 0x4, 0x10, 0x10, 0x220, 0x240, 0x204, 0x10, 0x1, 0x40, 0x4, 0x10, 0x10, 0x1], // formação 9
];

/// { 0x83c090 } (stdcall, ret 4, shortlist_manager.cpp): `shortlist_set_up_formation_slots(entrada)`: nas 40 vagas de
/// 0x25 bytes da entrada (+0), as 25 primeiras viram vagas de jogador (+0 = 0xb) e as 15 seguintes recebem o papel
/// (6, 7, 8 ×5, 10, 9 ×7) com +0x24 = (+0x24 & 0xe3) | 2; depois grava a máscara de posição (+1, 16 bits) das 25 vagas
/// de jogador pela formação (+7, byte com sinal; fora de 1..9 a linha padrão). A tabela saiu da emulação do switch
/// (cada caso grava as 25 vagas uma vez só).
unsafe extern "stdcall" fn shortlist_set_up_formation_slots(entrada: usize) {
    for k in 0..40usize {
        let s = cpp5_d(entrada) + k * 0x25;
        if k < 25 {
            *(s as *mut u8) = 0xb;
        } else {
            let b = (s + 0x24) as *mut u8;
            *b = (*b & 0xe3) | 2;
            *(s as *mut u8) = match k - 25 {
                0 => 6,
                1 => 7,
                2..=6 => 8,
                7 => 10,
                _ => 9,
            };
        }
    }
    let f = *((entrada + 7) as *const i8) as i32;
    let linha = if (1..=9).contains(&f) { f as usize } else { 0 };
    crate::comparacao::ramo("cpp_5 vagas da formacao");
    let v = cpp5_d(entrada);
    cpp5_w(v + 1, 1); // o goleiro
    for k in 1..25usize {
        cpp5_w(v + 1 + k * 0x25, CPP5_MASCARAS[linha][k - 1]);
    }
}

/// { 0x83a730 } (thiscall, ret 4, shortlist_manager.cpp): `Shortlist_Manager::update_club_shortlist(clube)`: solta os
/// alvos velhos das vagas do clube (0x844660 com a entrada), reavalia o elenco (0x83cb40 com a entrada recalculada,
/// o clube, diário 1, refazer 0) e redistribui a verba de transferências (0x83b6e0).
unsafe extern "thiscall" fn update_club_shortlist(this: usize, clube: usize) {
    let soltar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x844660usize);
    let avaliar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) = core::mem::transmute(0x83cb40usize);
    let verba: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x83b6e0usize);
    soltar(this, clube, cpp5_entrada(this, clube));
    avaliar(this, cpp5_entrada(this, clube), clube, 1, 0);
    crate::comparacao::ramo("cpp_5 refazer lista");
    verba(this, clube); // a verba de transferências
}

/// { 0x842b00 } (thiscall, ret 8, shortlist_manager.cpp, quadro SEH só para a lista nova):
/// `Shortlist_Manager::suspend_clubs_shortlisting_player(jogador, limpar)`: numa LLIST nova (0x944cf1(0xe) e 0x68d020)
/// junta os clubes que miram o jogador (0x842180 com ignorar 1, modo 3, todos); para cada item (0x68d1b0, depois de
/// recomeçar o cursor +0xa = +2) põe o estado da entrada do clube (+8) em 100 e, com `limpar`, tira a marca de lance
/// (+0x17 = −1 quando > −1) das vagas que têm o jogador (+0xf = id). Destrói e libera a lista (0x68d040, 0x944c94).
/// A contagem (+0, 16 bits com sinal) é relida a cada volta, como na original.
unsafe extern "thiscall" fn suspend_clubs_shortlisting_player(this: usize, jogador: usize, limpar: u8) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let p = novo(0xe);
    let lista = if p != 0 { lista_nova(p) } else { 0 };
    interessados(this, jogador, lista, 1, 3, 0);
    let n = *(lista as *const i16);
    if n != 0 {
        core::ptr::write_unaligned((lista + 0xa) as *mut usize, cpp5_d(lista + 2));
    }
    if n > 0 {
        let mut i = 0i32;
        while i < *(lista as *const i16) as i32 {
            let item = proximo(lista);
            if item != 0 {
                let e = cpp5_d(this + 0xdc).wrapping_add(cpp5_d(item + 4).wrapping_mul(9));
                crate::comparacao::ramo("cpp_5 suspender clube");
                *((e + 8) as *mut u8) = 0x64; // o clube suspende o jogador
                if limpar != 0 {
                    for k in 0..0x28usize {
                        let s = cpp5_d(e) + k * 0x25;
                        if cpp5_d(s + 0xf) == cpp5_d(jogador) && core::ptr::read_unaligned((s + 0x17) as *const i32) > -1 {
                            core::ptr::write_unaligned((s + 0x17) as *mut i32, -1);
                        }
                    }
                }
            }
            i += 1;
        }
    }
    if lista != 0 {
        destruir(lista);
        liberar(lista);
    }
}

/// Comparador: lado a lado pelas 40 vagas da entrada (sem chamadas).
unsafe extern "stdcall" fn comparar_shortlist_set_up_formation_slots(entrada: usize) {
    let o = orig::<unsafe extern "stdcall" fn(usize)>(%%I0%%);
    let reg = [(cpp5_d(entrada), 40 * 0x25)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(entrada), || shortlist_set_up_formation_slots(entrada));
    PLACAR.registrar(igual, || format!("shortlist_set_up_formation_slots(formacao {}) primeira diferença {:x?}", *((entrada + 7) as *const i8), cpp5_dif(&mo, &mr)));
}

/// Comparador: a 0x83cb40 reavalia o elenco (sorteios, ofertas, vagas de muitos clubes): roda só a original.
// ponytail: só a original; a cópia é provada pelo ramo e pela mutação na carreira.
unsafe extern "thiscall" fn comparar_update_club_shortlist(this: usize, clube: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, clube)
}

/// Comparador: lado a lado pelas entradas (9 bytes) e, com `limpar`, pelas vagas (40 × 0x25) dos clubes do plano: os
/// clubes que a 0x842180 (pura) junta antes, numa lista própria. Cada lado cria e libera a sua lista.
unsafe extern "thiscall" fn comparar_suspend_clubs_shortlisting_player(this: usize, jogador: usize, limpar: u8) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u8)>(%%I2%%);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let p = novo(0xe);
    if p == 0 {
        return o(this, jogador, limpar);
    }
    let lista = lista_nova(p);
    interessados(this, jogador, lista, 1, 3, 0);
    // os itens (8 bytes: id do jogador, id do clube) pelos nós (+2 o primeiro; nó: +0 o dado, +6 o próximo)
    let mut reg: Vec<(usize, usize)> = Vec::new();
    let mut no = cpp5_d(lista + 2);
    while no != 0 {
        let e = cpp5_d(this + 0xdc).wrapping_add(cpp5_d(cpp5_d(no) + 4).wrapping_mul(9));
        reg.push((e, 9));
        if limpar != 0 {
            reg.push((cpp5_d(e), 0x28 * 0x25));
        }
        no = cpp5_d(no + 6);
    }
    destruir(lista);
    liberar(lista);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador, limpar), || suspend_clubs_shortlisting_player(this, jogador, limpar));
    PLACAR.registrar(igual, || format!("suspend_clubs_shortlisting_player(jogador {:x}, limpar {}, {} regiões) primeira diferença {:x?}", cpp5_d(jogador), limpar, reg.len(), cpp5_dif(&mo, &mr)));
}
