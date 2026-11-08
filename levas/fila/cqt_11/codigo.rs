// ---------- %%LEVA%%a leva: o Campeonato de Clubes da Oceania: os 8 clubes, a fase seguinte e as semifinais (oceania_club_champ.cpp) ----------

unsafe fn cqt11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqt11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqt11_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Os clubes de uma origem para a Oceania: até 20, na ordem da tabela de clubes.
struct SourceClubList {
    clubes: [usize; 20],
    n: i16,
}

impl SourceClubList {
    fn novo() -> Self {
        SourceClubList { clubes: [0; 20], n: 0 }
    }

    /// Entra se cabe (a conta é palavra com sinal, < 20).
    fn cabe(&self) -> bool {
        self.n < 0x14
    }

    fn por(&mut self, c: usize) {
        self.clubes[self.n as usize] = c;
        self.n += 1;
    }

    /// Ordena pela reputação e embaralha a faixa de cima (0x4afbd0 comp_util_sort_and_shuffle_top_clubs: push quantos;
    /// push a conta; push lista; add esp -> cdecl; sorteia).
    unsafe fn ordenar(&mut self, quantos: u32) {
        let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
        f(self.clubes.as_mut_ptr() as usize, self.n as u16 as u32, quantos);
    }

    /// O primeiro da lista e o primeiro depois dele de outra nação (+0x53), como a original: percorre as `n` vagas; vaga
    /// vazia não conta como primeiro.
    unsafe fn dois_de_nacoes_diferentes(&self) -> (usize, usize) {
        let (mut a, mut b) = (0usize, 0usize);
        for &c in self.clubes.iter().take(self.n.max(0) as usize) {
            if a == 0 {
                a = c;
            } else if b == 0 && cqt11_d(c + 0x53) != cqt11_d(a + 0x53) {
                b = c;
            }
        }
        (a, b)
    }
}

/// O par de um pote vai para os grupos A e B: com o sorteio de 2 (0x90cfc0(2): push 2; add esp -> cdecl) diferente de
/// zero, o primeiro na vaga `k` (grupo A) e o segundo na `k` + 4 (grupo B); com zero, ao contrário.
unsafe fn place_pot_pair(this: usize, k: usize, primeiro: usize, segundo: usize, sorteio: i32) {
    let l = cqt11_d(this + 0xee);
    if sorteio != 0 {
        cqt11_pd(l + k * 4, primeiro);
        cqt11_pd(cqt11_d(this + 0xee) + (k + 4) * 4, segundo);
    } else {
        cqt11_pd(l + (k + 4) * 4, primeiro);
        cqt11_pd(cqt11_d(this + 0xee) + k * 4, segundo);
    }
}

/// { 0x798220 } (thiscall, ret 0, oceania_club_champ.cpp): `OceaniaClubChamp::select_qualified_clubs()`: +0xf2 = 8 e a
/// lista +0xee (0x944e46(0x20): push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x307 e devolve 0). O campeão
/// do último registro da liga australiana (0x4afd80 comp_util_get_latest_placed_club: push 0; push competição
/// 0xae23d0 + 0x6b × [0x9cf89c]; add esp, 8 -> cdecl). Os clubes ([0xae23b4], 0x245 bytes, [0xae235c] − 2 × [0xae2350]
/// deles) de nação da Oceania ([0x9cfa18]: +0x71 da nação, o +0 dele) vão, pelo id da nação, para as listas da Nova
/// Zelândia ([0x9cf3f4]), da Austrália ([0x9cf224]), do Taiti ([0x9cf4b4]) e de Fiji ([0x9cf2f8]), até 20 em cada (a
/// conta é palavra com sinal); os outros (ou com a lista cheia) para a melanésia (0x53d900 database_is_melanesian_nation:
/// push nação; add esp -> cdecl; cheia, o clube fica de fora) ou, senão, a polinésia (0x53d940 com a nação de novo), até
/// 20. Sem campeão, o melhor
/// australiano (a lista com a faixa de 1 sorteada) no lugar. Os pares, cada um sorteado entre os grupos A (vagas 0..3)
/// e B (4..7) por 0x90cfc0(2): o campeão e o melhor neozelandês (vagas 0/4); o melhor de Fiji e o do Taiti (1/5); os dois
/// primeiros melanésios de nações diferentes (2/6, a faixa de 2); os dois polinésios (3/7). Devolve 1.
/// A original guarda as listas na pilha sem zerá-las; com menos clubes que o lido ela lê sobra; a cópia lê zero
/// [inferência: nesta carreira as listas têm clubes que bastam; o comparador confere a lista].
unsafe fn select_qualified_clubs(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x4afd80usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let melanesia: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d900usize);
    let polinesia: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53d940usize);
    cqt11_pd(this + 0xf2, 8);
    let lista = alocar(0x20);
    cqt11_pd(this + 0xee, lista);
    if lista == 0 {
        show_error_at(0xa6a068, 0x307);
        return 0;
    }
    let mut campeao = colocado(cqt11_d(0xae23d0) + cqt11_d(0x9cf89c) * 0x6b, 0);
    let (mut nz, mut aus, mut tai, mut fij, mut mel, mut pol) =
        (SourceClubList::novo(), SourceClubList::novo(), SourceClubList::novo(), SourceClubList::novo(), SourceClubList::novo(), SourceClubList::novo());
    let mut i = 0i32;
    let mut c = cqt11_d(0xae23b4);
    while i < (cqt11_d(0xae235c) as i32).wrapping_sub((cqt11_d(0xae2350) as i32).wrapping_mul(2)) {
        let nacao = cqt11_d(c + 0x53);
        if nacao != 0 && cqt11_d(nacao + 0x71) != 0 && cqt11_d(cqt11_d(nacao + 0x71)) == cqt11_d(0x9cfa18) {
            let id = cqt11_d(nacao);
            if id == cqt11_d(0x9cf3f4) && nz.cabe() {
                nz.por(c);
            } else if id == cqt11_d(0x9cf224) && aus.cabe() {
                aus.por(c);
            } else if id == cqt11_d(0x9cf4b4) && tai.cabe() {
                tai.por(c);
            } else if id == cqt11_d(0x9cf2f8) && fij.cabe() {
                fij.por(c);
            } else if melanesia(nacao) != 0 {
                if mel.cabe() {
                    mel.por(c);
                }
            } else if polinesia(cqt11_d(c + 0x53)) != 0 && pol.cabe() {
                pol.por(c);
            }
        }
        i += 1;
        c += 0x245;
    }
    crate::comparison::branch("cqt_11 clubes da oceania");
    if campeao == 0 {
        aus.ordenar(1);
        campeao = aus.clubes[0];
    }
    nz.ordenar(1);
    place_pot_pair(this, 0, campeao, nz.clubes[0], sorteio(2)); // cqt11: o campeão australiano e o neozelandês
    fij.ordenar(1);
    tai.ordenar(1);
    place_pot_pair(this, 1, fij.clubes[0], tai.clubes[0], sorteio(2)); // cqt11: Fiji e Taiti
    mel.ordenar(2);
    let (a, b) = mel.dois_de_nacoes_diferentes();
    place_pot_pair(this, 2, a, b, sorteio(2)); // cqt11: os dois melanésios
    pol.ordenar(2);
    let (a, b) = pol.dois_de_nacoes_diferentes();
    place_pot_pair(this, 3, a, b, sorteio(2));
    1
}

unsafe extern "thiscall" fn oceania_club_champ_select_qualified_clubs(this: usize) -> u32 {
    select_qualified_clubs(this)
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nas 8 vagas e liberada.
unsafe extern "thiscall" fn compare_oceania_club_champ_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::rng::STATE_REGIONS;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(cqt11_d(this + 0xee));
        r
    }, || {
        let r = crate::rng::without_counting(|| select_qualified_clubs(this));
        lr.set(cqt11_d(this + 0xee));
        cqt11_pd(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x20) == core::slice::from_raw_parts(lr as *const u8, 0x20)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    SCOREBOARD.record(igual && a == b && lista, || format!("oceania_club_champ_select_qualified_clubs({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x797df0 } (thiscall, ret 0, oceania_club_champ.cpp; a virtual +0x28): `OceaniaClubChamp::advance_stage()`: na
/// última fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x274; senão +0x30 sobe e, chegando a 1 (depois dos
/// grupos), as semifinais (0x798050: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn oceania_club_champ_advance_stage(this: usize) {
    let f = cqt11_d(this + 0x30) as i32;
    if f >= (cqt11_d(this + 0x2c) as i32).wrapping_sub(1) {
        return show_error_at(0xa6a068, 0x274);
    }
    let f = f + 1;
    cqt11_pd(this + 0x30, f as usize);
    if f == 1 {
        crate::comparison::branch("cqt_11 semifinais da oceania");
        let semifinais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x798050usize);
        semifinais(this);
    }
}

// ponytail: as semifinais alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um (fora da
// última fase, que dá a caixa e não muda nada).
unsafe extern "thiscall" fn compare_oceania_club_champ_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let f = cqt11_d(this + 0x30) as i32;
    let ultima = f >= (cqt11_d(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = cqt11_d(this + 0x30) as i32;
    SCOREBOARD.record(depois == if ultima { f } else { f + 1 }, || format!("oceania_club_champ_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// Os 4 das semifinais, na ordem da lista: o 1º do grupo A (a tabela da copa, +0xb1), o 2º do B (a tabela da fase
/// [+0xc][0]), o 2º do A e o 1º do B (os cruzamentos: as chaves são os pares da lista).
unsafe fn semifinalists(this: usize) -> [usize; 4] {
    let a = |l: usize| cqt11_d(cqt11_d(this + 0xb1) + l * 0x3b);
    let b = |l: usize| cqt11_d(cqt11_d(cqt11_d(cqt11_d(this + 0xc)) + 0xb1) + l * 0x3b);
    [a(0), b(1), a(1), b(0)] // cqt11: 1º A × 2º B, 2º A × 1º B
}

/// { 0x798050 } (thiscall, ret 0, oceania_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `OceaniaClubChamp::create_knockout_stage()`, as semifinais: a lista dos 4 (0x944e46(0x10): push; add esp, 4
/// -> cdecl; sem memória, a caixa da linha 0x2cb e volta), os cruzamentos de semifinalists; a tabela das rodadas
/// pela virtual +0x3c (push 0; push &id; push &rodadas; push 1; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2)
/// push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a
/// tabela, o ano +0x40, 1, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo
/// nos 16 bits altos, que o construtor não lê) em [+0xc][1]; a tabela e a lista liberadas (0x9452ca). Sem fase: a caixa
/// da linha 0x2e3 e volta. Com ela: a rodada montada (0x51c800 Cup::set_up_round: push 0; mov ecx, fase -> thiscall,
/// ret 4).
unsafe extern "thiscall" fn oceania_club_champ_create_knockout_stage(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
    let lista = alocar(0x10);
    if lista == 0 {
        return show_error_at(0xa6a068, 0x2cb);
    }
    crate::comparison::branch("cqt_11 cruzamentos da oceania");
    for (k, &c) in semifinalists(this).iter().enumerate() {
        cqt11_pd(lista + k * 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(cqt11_d(cqt11_d(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 1, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(0x522e00usize);
        construir(p, this, 4, lista, rodadas as u16 as u32, cqt11_d(this + 4), tabela, cqt11_w(this + 0x40) as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    cqt11_pd(cqt11_d(this + 0xc) + 4, fase);
    soltar(tabela);
    soltar(lista);
    let f = cqt11_d(cqt11_d(this + 0xc) + 4);
    if f == 0 {
        return show_error_at(0xa6a068, 0x2e3);
    }
    montar(f, 0);
}

// ponytail: aloca a fase e monta a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][1] com 4 times,
// o +4 da copa e os 4 cruzados (lidos antes) na lista dela (+0xa7, 6 bytes; como conjunto: a rodada pode reordenar).
unsafe extern "thiscall" fn compare_oceania_club_champ_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    if cqt11_d(this + 0xb1) == 0 || cqt11_d(this + 0xc) == 0 || cqt11_d(cqt11_d(this + 0xc)) == 0 {
        return o(this);
    }
    let mut plano = semifinalists(this).to_vec();
    o(this);
    let fase = cqt11_d(cqt11_d(this + 0xc) + 4);
    if fase == 0 {
        SCOREBOARD.record(false, || format!("oceania_club_champ_create_knockout_stage({:#x}) sem fase", this));
        return;
    }
    let lista = cqt11_d(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| cqt11_d(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && cqt11_w(fase + 0x3e) == 4 && cqt11_d(fase + 4) == cqt11_d(this + 4);
    SCOREBOARD.record(igual, || format!("oceania_club_champ_create_knockout_stage({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}
