// ---------- %%LEVA%%a leva: os convidados da Merconorte e a fase seguinte da Mercosul e da Copa América (con_merc_cup.cpp, conmebol_merc.cpp, goldcup.cpp) ----------

unsafe fn cqs11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqs11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Os clubes de uma nação para a Merconorte: até 30, na ordem da tabela de clubes.
struct Cqs11Nacao {
    clubes: [usize; 30],
    n: u8,
}

impl Cqs11Nacao {
    fn novo() -> Self {
        Cqs11Nacao { clubes: [0; 30], n: 0 }
    }

    fn por(&mut self, c: usize) {
        if self.n < 0x1e {
            self.clubes[self.n as usize] = c;
            self.n += 1;
        }
    }
}

/// { 0x4c2560 } (thiscall, ret 0, con_merc_cup.cpp): `ConMercCup::select_invited_clubs()`: +0xf2 = 16 e a lista +0xee
/// (0x944e46(0x40): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x262 e devolve 0). Os clubes da tabela
/// ([0xae23b4], 0x245 bytes, [0xae235c] deles; a primeira seleção, 0x53b900 database_is_national_team: push clube; add
/// esp, 4 -> cdecl, encerra a busca) com nação (+0x53) vão, pelo id da nação (+0 dela), para as listas do México
/// ([0x9cf3d0]), da Colômbia ([0x9cf29c]), do Peru ([0x9cf428]) e da Bolívia ([0x9cf254]), até 30 em cada; os de outra
/// nação cuja liga (+0x57) é a [0x9cf590] (a MLS) vão para a lista da liga americana, até 12. Cada lista é ordenada pela
/// reputação e tem a faixa de cima embaralhada (0x4afbd0 comp_util_sort_and_shuffle_top_clubs: push quantos; push a
/// conta (word); push lista; add esp, 0xc -> cdecl; sorteia): México 4, Colômbia 5, Peru 3, Bolívia 2, a liga americana 2.
/// A lista: nas vagas 2..5 os 4 primeiros do México, 6..10 os 5 da Colômbia, 11..13 os 3 do Peru, 14..15 os 2 da
/// Bolívia; nas vagas 0 e 1, sem a competição da liga americana ([[0xadadfc] + 4 × [0x9cf728]] = 0) os 2 primeiros da
/// lista dela; com ela, o campeão do último registro da competição [0x9cf590] (0xae23d0 + 0x6b × id) e o da competição
/// +4 dela (0x4afd80 comp_util_get_latest_placed_club: push lugar; push competição; add esp -> cdecl), e, se forem o
/// mesmo clube, o vice da primeira na vaga 0. A lista embaralhada (0x4afb40(lista, 16): push; push; add esp, 8 -> cdecl;
/// 3 × 16 trocas sorteadas). Devolve 1.
/// A original guarda as listas na pilha sem zerá-las: com menos clubes que as vagas, ela lê sobra; a cópia lê zero
/// [inferência: nesta carreira as quatro nações e a MLS têm clubes que bastam; o comparador confere a lista].
unsafe fn cqs11_convidados(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    let ordenar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    cqs11_pd(this + 0xf2, 0x10);
    let americana = cqs11_d(cqs11_d(0xadadfc) + cqs11_d(0x9cf728) * 4);
    let lista = alocar(0x40);
    cqs11_pd(this + 0xee, lista);
    if lista == 0 {
        aviso_em(0x9a7d6c, 0x262);
        return 0;
    }
    let (mut mexico, mut colombia, mut peru, mut bolivia) = (Cqs11Nacao::novo(), Cqs11Nacao::novo(), Cqs11Nacao::novo(), Cqs11Nacao::novo());
    let mut mls = [0usize; 12];
    let mut n_mls = 0i8;
    let mut i = 0i32;
    let mut c = cqs11_d(0xae23b4);
    while i < cqs11_d(0xae235c) as i32 {
        if selecao(c) != 0 {
            break;
        }
        let nacao = cqs11_d(c + 0x53);
        if nacao != 0 {
            let id = cqs11_d(nacao);
            if id == cqs11_d(0x9cf3d0) {
                mexico.por(c);
            } else if id == cqs11_d(0x9cf29c) {
                colombia.por(c);
            } else if id == cqs11_d(0x9cf428) {
                peru.por(c);
            } else if id == cqs11_d(0x9cf254) {
                bolivia.por(c);
            } else {
                let liga = cqs11_d(c + 0x57);
                if liga != 0 && cqs11_d(liga) == cqs11_d(0x9cf590) && n_mls < 0xc {
                    mls[n_mls as usize] = c;
                    n_mls += 1;
                }
            }
        }
        i += 1;
        c += 0x245;
    }
    crate::comparacao::ramo("cqs_11 convidados da merconorte");
    ordenar(mexico.clubes.as_mut_ptr() as usize, mexico.n as u32, 4); // cqs11: os 4 do México
    ordenar(colombia.clubes.as_mut_ptr() as usize, colombia.n as u32, 5);
    ordenar(peru.clubes.as_mut_ptr() as usize, peru.n as u32, 3);
    ordenar(bolivia.clubes.as_mut_ptr() as usize, bolivia.n as u32, 2);
    ordenar(mls.as_mut_ptr() as usize, n_mls as i16 as u16 as u32, 2);
    let vaga = |k: usize, v: usize| cqs11_pd(cqs11_d(this + 0xee) + k * 4, v);
    vaga(2, mexico.clubes[0]);
    vaga(3, mexico.clubes[1]);
    vaga(4, mexico.clubes[2]);
    vaga(5, mexico.clubes[3]);
    for k in 0..5 {
        vaga(6 + k, colombia.clubes[k]);
    }
    for k in 0..3 {
        vaga(11 + k, peru.clubes[k]);
    }
    vaga(14, bolivia.clubes[0]);
    vaga(15, bolivia.clubes[1]);
    if americana == 0 {
        vaga(0, mls[0]);
        vaga(1, mls[1]);
    } else {
        let registro = cqs11_d(0xae23d0) + cqs11_d(0x9cf590) * 0x6b;
        crate::comparacao::ramo("cqs_11 campeao da liga americana");
        vaga(0, colocado(registro, 0)); // cqs11: o campeão da liga americana
        vaga(1, colocado(cqs11_d(americana + 4), 0));
        let l = cqs11_d(this + 0xee);
        if cqs11_d(l) == cqs11_d(l + 4) {
            vaga(0, colocado(registro, 1));
        }
    }
    embaralhar(cqs11_d(this + 0xee), 0x10);
    1
}

unsafe extern "thiscall" fn cqs11_merconorte_convidados(this: usize) -> u32 {
    cqs11_convidados(this)
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nas 16 vagas e liberada. Os sorteios das listas
/// das nações ficam na pilha de cada uma: só a lista final e o sorteador contam.
unsafe extern "thiscall" fn comparar_cqs11_merconorte_convidados(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(cqs11_d(this + 0xee));
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqs11_convidados(this));
        lr.set(cqs11_d(this + 0xee));
        cqs11_pd(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x40) == core::slice::from_raw_parts(lr as *const u8, 0x40)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqs11_merconorte_convidados({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// A fase seguinte de um torneio com grupos e mata-mata (`advance_stage`, a virtual +0x28): na última fase (+0x30 ≥
/// +0x2c − 1, com sinal), a caixa da linha e volta; senão +0x30 sobe e, chegando a `mata`, o mata-mata (call direto com
/// o ecx da copa -> thiscall, ret 0).
unsafe fn cqs11_fase_seguinte(this: usize, arquivo: usize, linha: u32, mata: i32, rotina: usize, ramo: &'static str) {
    let f = cqs11_d(this + 0x30) as i32;
    if f >= (cqs11_d(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(arquivo, linha);
    }
    let f = f + 1;
    cqs11_pd(this + 0x30, f as usize);
    if f == mata {
        crate::comparacao::ramo(ramo);
        let m: unsafe extern "thiscall" fn(usize) = core::mem::transmute(rotina);
        m(this);
    }
}

// ponytail: o mata-mata aloca e sorteia: não roda duas vezes. Só a original; o plano: +0x30 sobe um (fora da última
// fase, que dá a caixa e não muda nada).
unsafe fn cqs11_comparar_fase_seguinte(nome: &str, this: usize, original: impl FnOnce()) {
    let f = cqs11_d(this + 0x30) as i32;
    let ultima = f >= (cqs11_d(this + 0x2c) as i32).wrapping_sub(1);
    original();
    let depois = cqs11_d(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("{}({:#x}) fase {} -> {}", nome, this, f, depois));
}

/// { 0x4c5140 } (thiscall, ret 0, conmebol_merc.cpp; a virtual +0x28): `ConmebolMerc::advance_stage()`, a fase seguinte
/// da Mercosul: a caixa da linha 0x1d9 na última; chegando a 4 (depois dos 4 grupos-fase), as quartas (0x4c53d0).
unsafe extern "thiscall" fn cqs11_mercosul_fase_seguinte(this: usize) {
    cqs11_fase_seguinte(this, 0x9a7e5c, 0x1d9, 4, 0x4c53d0, "cqs_11 quartas da mercosul");
}

unsafe extern "thiscall" fn comparar_cqs11_mercosul_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqs11_comparar_fase_seguinte("cqs11_mercosul_fase_seguinte", this, || o(this))
}

/// { 0x5e1620 } (thiscall, ret 0, goldcup.cpp; a virtual +0x28): `GoldCup::advance_stage()`, a fase seguinte da Copa
/// América: a caixa da linha 0x29d na última; chegando a 2 (depois dos 2 grupos-fase), o mata-mata (0x5e18a0).
unsafe extern "thiscall" fn cqs11_copa_america_fase_seguinte(this: usize) {
    cqs11_fase_seguinte(this, 0x9cd640, 0x29d, 2, 0x5e18a0, "cqs_11 mata-mata da copa america");
}

unsafe extern "thiscall" fn comparar_cqs11_copa_america_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    cqs11_comparar_fase_seguinte("cqs11_copa_america_fase_seguinte", this, || o(this))
}
