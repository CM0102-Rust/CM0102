// ---------- %%LEVA%%a leva: a Copa América: as 12 seleções, as posições previstas e a temporada nova (goldcup.cpp) ----------

unsafe fn cqs12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqs12_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// A seleção (o clube da seleção principal) da nação de id [`global`]: 0x53b3d0 database_get_national_team (push nação;
/// add esp -> cdecl) com a nação da tabela [0xae23a8] (0x122 bytes cada).
unsafe fn national_team_of_nation_id(id: usize) -> usize {
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    selecao(cqs12_d(0xae23a8) + id * 0x122)
}

/// As 12 da Copa América de 2001, na ordem da lista (os grupos intercalados: A nas vagas 0, 3, 6 e 9; B nas 1, 4, 7 e
/// 10; C nas 2, 5, 8 e 11): os globais dos ids das nações.
const COPA_AMERICA_2001_NATIONS: [usize; 12] = [
    0x9cf29c, 0x9cf260, 0x9cf2a8, 0x9cf294, // cqs12: as vagas 0 a 3 (a Colômbia na 0)
    0x9cf3d0, 0x9cf33c, 0x9cf2d4, 0x9cf428, // cqs12: as vagas 4 a 7 (o México na 4, o Peru na 7)
    0x9cf4f4, 0x9cf504, 0x9cf420, 0x9cf254, // cqs12: as vagas 8 a 11 (a Bolívia na 11)
];

/// { 0x5e1cd0 } (thiscall, ret 0, goldcup.cpp): `GoldCup::set_up_team_list()`: +0xf2 = 12 e a lista +0xee (0x944e46(0x30):
/// push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x37b e devolve 0). Em 2001 (+0x40 = 0x7d1): as 12 fixas
/// (COPA_AMERICA_2001_NATIONS, cada uma pela 0x53b3d0) e devolve 1, sem ordenar. Nos outros anos: as sedes da Copa América
/// ([0x9cf7a8]) no ano (0x5fa730 HostCountry::find_hosts em [0xb5e134]: push 1; push &segunda; push &primeira; push ano;
/// push competição -> thiscall, ret 0x14; as duas começam em −1; devolve quantas, byte com sinal) na frente; depois,
/// pela ordem das nações ([0xae23a8], [0xae2350] delas, i como palavra) e enquanto a lista não enche (+0xf2), as do
/// continente [0x9cfa1c] (+0x71, o +0 dele) e as convidadas: em 1999 (0x7cf) a [0x9cf36c], nos outros anos a [0x9cf280];
/// e sempre o México ([0x9cf3d0]); cada uma entra se não é a nação de uma sede (+0x53 da seleção). Sem 12: a caixa da
/// linha 0x3bc e devolve 0; com 12, a lista ordenada pela reputação (qsort 0x945333 com 0x4b07d0: 4 pushes; add esp ->
/// cdecl) e devolve 1.
unsafe fn build_gold_cup_team_list(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let selecao: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    cqs12_pd(this + 0xf2, 0xc);
    let lista = alocar(0x30);
    cqs12_pd(this + 0xee, lista);
    if lista == 0 {
        show_error_at(0x9cd640, 0x37b);
        return 0;
    }
    let ano = cqs12_w(this + 0x40) as u16;
    if ano == 0x7d1 {
        crate::comparacao::ramo("cqs_12 selecoes de 2001");
        for (k, &g) in COPA_AMERICA_2001_NATIONS.iter().enumerate() {
            cqs12_pd(cqs12_d(this + 0xee) + k * 4, national_team_of_nation_id(cqs12_d(g)));
        }
        return 1;
    }
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u32 = core::mem::transmute(0x5fa730usize);
    let mut ids = [-1i32; 2];
    let (a, b) = ids.split_at_mut(1);
    // ponytail: a original empurra o eax com o ano nos 16 bits baixos e sobra nos altos; a 0x5fa730 só lê o word
    let quantas = sedes(cqs12_d(0xb5e134), cqs12_d(0x9cf7a8), ano as u32, a.as_mut_ptr(), b.as_mut_ptr(), 1) as u8 as i8;
    let mut k = 0i16;
    while k < quantas as i16 {
        cqs12_pd(cqs12_d(this + 0xee) + k as usize * 4, national_team_of_nation_id(ids[k as usize] as usize));
        k += 1;
    }
    let mut i = 0i16;
    let mut nacao = cqs12_d(0xae23a8);
    while (i as i32) < cqs12_d(0xae2350) as i32 && (k as i32) < cqs12_d(this + 0xf2) as i32 {
        let continente = cqs12_d(nacao + 0x71);
        let id = cqs12_d(nacao);
        let entra = (continente != 0 && cqs12_d(continente) == cqs12_d(0x9cfa1c))
            || (ano == 0x7cf && id == cqs12_d(0x9cf36c))
            || (ano != 0x7cf && id == cqs12_d(0x9cf280))
            || id == cqs12_d(0x9cf3d0);
        if entra {
            let l = cqs12_d(this + 0xee);
            let sede = (0..quantas.max(0) as usize).any(|j| cqs12_d(cqs12_d(l + j * 4) + 0x53) == nacao);
            if !sede {
                cqs12_pd(cqs12_d(this + 0xee) + k as usize * 4, selecao(nacao));
                k += 1;
            }
        }
        i = i.wrapping_add(1);
        nacao += 0x122;
    }
    if k as i32 != cqs12_d(this + 0xf2) as i32 {
        show_error_at(0x9cd640, 0x3bc);
        return 0;
    }
    ordenar(cqs12_d(this + 0xee), k as i32 as u32, 4, 0x4b07d0);
    1
}

unsafe extern "thiscall" fn set_up_team_list(this: usize) -> u32 {
    build_gold_cup_team_list(this)
}

/// Lado a lado: o +0xf2 e o [0xb67a34] a partir do mesmo estado (nada sorteia); a original deixa a lista dela em +0xee, a
/// cópia faz a sua, comparada nas 12 vagas e liberada.
unsafe extern "thiscall" fn comparar_set_up_team_list(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqs12_d(this + 0xee));
        r
    }, || {
        let r = build_gold_cup_team_list(this);
        lr.set(cqs12_d(this + 0xee));
        cqs12_pd(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        a == 0 || core::slice::from_raw_parts(lo as *const u8, 0x30) == core::slice::from_raw_parts(lr as *const u8, 0x30)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("set_up_team_list({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// As posições previstas da Copa América, pela ordem de reputação das 12.
const COPA_AMERICA_PREDICTED_PLACES: [u32; 12] = [1, 2, 3, 3, 5, 5, 5, 5, 9, 0xa, 0xa, 0xa];

/// { 0x5e2340 } (thiscall, ret 0, goldcup.cpp; a virtual +0x5c): `GoldCup::set_predicted_positions()`: sem estatística
/// (+8), a caixa da linha 0x419 e volta; uma cópia da lista (0x944e46(+0xf2 × 4); sem memória, a caixa da linha 0x421 e
/// volta), ordenada pela reputação (qsort 0x945333 com 0x4b07d0, +0xf2 itens), e a posição prevista de cada uma na
/// estatística (0x4a2540 Comp_Stats::set_club_final_position: push posição; push clube; mov ecx, [+8] -> thiscall,
/// ret 8): 1, 2, 3, 3, quatro 5, 9 e três 10 (as 12 primeiras da cópia); a cópia liberada (0x9452ca).
unsafe extern "thiscall" fn goldcup_set_predicted_positions(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x4a2540usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqs12_d(this + 8) == 0 {
        return show_error_at(0x9cd640, 0x419);
    }
    let v = alocar((cqs12_d(this + 0xf2) << 2) as u32);
    if v == 0 {
        return show_error_at(0x9cd640, 0x421);
    }
    let n = cqs12_d(this + 0xf2) << 2;
    core::ptr::copy_nonoverlapping(cqs12_d(this + 0xee) as *const u8, v as *mut u8, n);
    ordenar(v, cqs12_d(this + 0xf2) as u32, 4, 0x4b07d0);
    crate::comparacao::ramo("cqs_12 previstas da copa america");
    for (k, &p) in COPA_AMERICA_PREDICTED_PLACES.iter().enumerate() {
        posicao(cqs12_d(this + 8), cqs12_d(v + k * 4), p);
    }
    soltar(v);
}

// ponytail: a estatística acha ou cria a entrada de cada seleção (aloca): não roda duas vezes. Só a original; o plano:
// a lista ordenada pela reputação antes, e depois a posição (+0x72 da entrada, 0x4a4850 Comp_Stats::find_club_entry) de
// cada uma das 12 igual à da faixa.
unsafe extern "thiscall" fn comparar_goldcup_set_predicted_positions(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    if cqs12_d(this + 8) == 0 || cqs12_d(this + 0xee) == 0 || cqs12_d(this + 0xf2) != 0xc {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut v: Vec<usize> = (0..12).map(|i| cqs12_d(cqs12_d(this + 0xee) + i * 4)).collect();
    ordenar(v.as_mut_ptr(), 12, 4, 0x4b07d0);
    o(this);
    let errado = (0..12).find(|&k| {
        let e = if v[k] != 0 { achar(cqs12_d(this + 8), v[k]) } else { 0 };
        e == 0 || *((e + 0x72) as *const u8) as u32 != COPA_AMERICA_PREDICTED_PLACES[k]
    });
    PLACAR.registrar(errado.is_none(), || format!("goldcup_set_predicted_positions: a posição prevista da vaga {:?} não bate", errado));
}

/// { 0x5e1530 } (thiscall, ret 0, goldcup.cpp; a virtual +8): `GoldCup::start_new_season()`, a Copa América seguinte:
/// +0x4c = 0, a tabela da copa liberada (0x687970 League::free_table: push 0; mov ecx, this -> thiscall, ret 4), a
/// tabela das rodadas +0xba liberada (0x9452ca) e zerada, a temporada arquivada na estatística (0x4a1c50
/// Comp_Stats::archive_season_and_reset: push 1; mov ecx, [+8] -> thiscall, ret 4), as fases 0..+0x30 destruídas (a
/// virtual 0 de cada uma com 1: o destrutor que libera), o ano +0x40 sobe 2, +0x45 = 0, +0x30 = −1 e as regras de novo
/// (a virtual +0x8c, thiscall, ret 0; 0: a caixa da linha 0x28b).
unsafe extern "thiscall" fn goldcup_start_new_season(this: usize) {
    let soltar_tabela: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x687970usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let arquivar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x4a1c50usize);
    cqs12_pd(this + 0x4c, 0);
    soltar_tabela(this, 0);
    if cqs12_d(this + 0xba) != 0 {
        soltar(cqs12_d(this + 0xba));
        cqs12_pd(this + 0xba, 0);
    }
    if cqs12_d(this + 8) != 0 {
        arquivar(cqs12_d(this + 8), 1);
    }
    if (cqs12_d(this + 0x30) as i32) >= 0 {
        let mut i = 0i32;
        loop {
            let f = cqs12_d(cqs12_d(this + 0xc) + i as usize * 4);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqs12_d(cqs12_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqs12_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    crate::comparacao::ramo("cqs_12 temporada nova da copa america");
    core::ptr::write_unaligned((this + 0x40) as *mut i16, cqs12_w(this + 0x40).wrapping_add(2)); // cqs12: a Copa América a cada 2 anos
    cqs12_pd(this + 0x45, 0);
    cqs12_pd(this + 0x30, usize::MAX);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(cqs12_d(cqs12_d(this) + 0x8c));
    if regras(this) == 0 {
        show_error_at(0x9cd640, 0x28b);
    }
}

// ponytail: libera as tabelas e destrói as fases: não roda duas vezes. Só a original; o plano: o ano sobe 2, +0x45 = 0,
// +0x30 = −1, +0xba = 0 e +0x4c = 0 (as regras gravam a tabela das rodadas de novo em +0xba: conferida só que existe
// quando havia antes).
unsafe extern "thiscall" fn comparar_goldcup_start_new_season(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let ano = cqs12_w(this + 0x40);
    o(this);
    let ok = cqs12_w(this + 0x40) == ano.wrapping_add(2) && cqs12_d(this + 0x45) == 0 && cqs12_d(this + 0x30) as i32 == -1 && cqs12_d(this + 0x4c) == 0;
    PLACAR.registrar(ok, || format!("goldcup_start_new_season({:#x}): ano {} -> {}, +0x45 {}, +0x30 {}", this, ano, cqs12_w(this + 0x40), cqs12_d(this + 0x45), cqs12_d(this + 0x30) as i32));
}
