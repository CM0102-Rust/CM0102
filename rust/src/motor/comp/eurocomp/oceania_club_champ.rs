// ---------- 283a leva: o Campeonato de Clubes da Oceania: o dia do sorteio e as tabelas dos grupos A e B (oceania_club_champ.cpp) ----------




/// O clube k dos 8 escolhidos (+0xee, ponteiros: o grupo A nas vagas 0..3, o B nas 4..7), a que as duas tabelas leem.
unsafe fn cqt10_escolhido(this: usize, k: usize) -> usize {
    read_u32_as_usize(read_u32_as_usize(this + 0xee) + k * 4) // cqt10: o escolhido k
}

/// { 0x797b40 } (thiscall, ret 8, oceania_club_champ.cpp; a virtual +0x14; quadro SEH só das datas locais, sem
/// destrutor): `OceaniaClubChamp::update_for_date(hoje, completo)`, o dia do Campeonato de Clubes da Oceania. A data do
/// sorteio é a de 0x549ef0(+0xb6, +0xb8 + +0x40) (lea ecx, local; push ano; push dia -> thiscall, ret 8, pura). Antes
/// dela (o ano de hoje menor, sem sinal; ou o mesmo ano e o dia de hoje menor, com sinal) a rotina volta 0 sem o dia de
/// liga. No dia do sorteio, fora do sorteio feito (+0x45 = 0) e SEM `completo`: os 8 clubes (0x798220: mov ecx ->
/// thiscall, ret 0), as posições previstas (a virtual +0x5c, thiscall, ret 0), a tabela do grupo A (0x7978c0, thiscall,
/// ret 0; 0: a caixa da linha 0x21d), os jogos (0x6835c0 League::create_fixtures, thiscall, ret 0; 0: a caixa da linha
/// 0x224), a tabela ordenada (0x6827d0 push 0; mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x22b) (um erro volta 1,
/// sem o dia de liga), a fase do grupo B (0x797e80, thiscall, ret 0), a lista +0xee liberada (0x9452ca: push; add esp,
/// 4 -> cdecl) e zerada, +0xf2 = 0 e +0x45 = 1. Depois o dia de liga (0x6847c0: push completo; push hoje; mov ecx, this
/// -> thiscall, ret 8), cuja devolução ela repassa.
unsafe extern "thiscall" fn cqt10_oceania_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let clubes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_OCEANIACLUBCHAMP_SELECT_QUALIFIED_CLUBS);
    let grupo_a: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_OCEANIACLUBCHAMP_CREATE_GROUP_A_TABLE);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let grupo_b: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_OCEANIACLUBCHAMP_CREATE_GROUP_B_STAGE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let dia = read_i16(this + 0xb6) as u16;
    let ano = (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16);
    let dia_de_hoje = core::ptr::read_unaligned(hoje as *const u16);
    let ano_de_hoje = core::ptr::read_unaligned((hoje + 2) as *const u16);
    // ponytail: nas voltas antes do sorteio e nos erros a original devolve só o al (0 ou 1; o resto do eax é sobra das
    // chamadas de antes); quem chama lê o byte
    if ano_de_hoje < ano || (ano_de_hoje == ano && (dia_de_hoje as i16) < dia as i16) {
        return 0;
    }
    if ano_de_hoje == ano && dia_de_hoje == dia && read_u32_as_usize(this + 0x45) == 0 && completo == 0 {
        crate::comparacao::ramo("cqt_10 sorteio da oceania");
        clubes(this);
        let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x5c));
        previstas(this);
        if grupo_a(this) == 0 {
            aviso_em(0xa6a068, 0x21d);
            return 1;
        }
        if jogos(this) == 0 {
            aviso_em(0xa6a068, 0x224);
            return 1;
        }
        if ordenar(this, 0) == 0 {
            aviso_em(0xa6a068, 0x22b);
            return 1;
        }
        grupo_b(this);
        soltar(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, 0);
        write_u32_from_usize(this + 0xf2, 0);
        write_u32_from_usize(this + 0x45, 1);
    }
    liga(this, hoje, completo)
}

// ponytail: os clubes, as previsões, as tabelas, os jogos e o dia de liga não rodam duas vezes. Só a original; o plano
// no dia do sorteio (a condição da cópia): +0x45 = 1, a lista +0xee e +0xf2 zeradas, a tabela do grupo A e a fase do
// grupo B criadas e a fase atual (+0x30) = 0.
unsafe extern "thiscall" fn comparar_cqt10_oceania_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(2059);
    let sorteia = read_u32_as_usize(this + 0x45) == 0
        && completo == 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_u32_as_usize(this + 0xf2) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && fases != 0
            && read_u32_as_usize(fases) != 0
            && read_u32_as_usize(this + 0x30) == 0;
        PLACAR.registrar(ok, || format!("cqt10_oceania_dia({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// { 0x7978c0 } (thiscall, ret 0, oceania_club_champ.cpp): `OceaniaClubChamp::create_group_a_table()`: 4 times (+0x3e), a
/// tabela +0xb1 (0x944e46(0xec): push 0xec; add esp, 4 -> cdecl; 4 linhas de 0x3b bytes; sem memória, a caixa da linha
/// 0x182 e devolve 0), cada linha i com o escolhido i (+0xee) (0x687430 League::init_table_row: push 0; push 0; push
/// clube; push i; mov ecx, this -> thiscall, ret 0x10), a notícia do sorteio do grupo (0x684230: mov ecx, this ->
/// thiscall, ret 0) e devolve 1. O mesmo corpo da 0x4019c0 da Copa Africana (só o arquivo e a linha da caixa mudam).
unsafe extern "thiscall" fn cqt10_oceania_grupo_a(this: usize) -> u32 {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let linha: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 4);
    let t = novo(0xec);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xa6a068, 0x182);
        return 0;
    }
    let mut i = 0i8;
    while (i as i16) < read_i16(this + 0x3e) {
        crate::comparacao::ramo("cqt_10 linha do grupo a");
        // ponytail: o índice vai num dword cujos 3 bytes altos são sobra de pilha; a 0x687430 lê só o byte (char)
        linha(this, i as u8 as u32, cqt10_escolhido(this, i as usize), 0, 0); // cqt10: a linha i do grupo A
        i = i.wrapping_add(1);
    }
    noticia(this);
    1
}

// ponytail: a tabela é alocada (e cada linha aloca o histórico) e a notícia vai aos técnicos: não roda duas vezes. Só a
// original; o plano: devolve 1, 4 times e na linha i (0x3b bytes) o escolhido i com a posição i (+4).
unsafe extern "thiscall" fn comparar_cqt10_oceania_grupo_a(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2060);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| read_u32_as_usize(lista + i * 4));
    let r = o(this);
    let t = read_u32_as_usize(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (read_u32_as_usize(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && read_i16(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("cqt10_oceania_grupo_a({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// { 0x797e80 } (thiscall, ret 0, oceania_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `OceaniaClubChamp::create_group_b_stage()`: os times num vetor de 4 ponteiros (0x944e46(0x10): push; add esp,
/// 4 -> cdecl; sem memória, a caixa da linha 0x296 e 0): os escolhidos 4 a 7 (+0xee); a tabela das rodadas pela virtual
/// +0x3c (push 0; push &id; push &rodadas; push 0; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xee) push; add
/// esp, 4 -> cdecl; o construtor League_Stage 0x68b100, thiscall, ret 0x58, 22 argumentos: a copa, 4, o vetor, 1, +4, a
/// tabela das rodadas, as rodadas (word), +0xc2, +0xc3, +0xc4, &+0xc5, &+0xbe, o ano +0x40 (word), 0, o id (word),
/// +0x51, 2, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes altos, que o construtor
/// não lê: a cópia passa os valores limpos) em [+0xc][0]; o vetor e a tabela das rodadas liberados (0x9452ca). Sem fase:
/// a caixa da linha 0x2ad e 0. Com ela: +0x30 = 0, a notícia do grupo (0x684230 na fase) e 1.
unsafe extern "thiscall" fn cqt10_oceania_grupo_b(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let times = alocar(0x10);
    if times == 0 {
        aviso_em(0xa6a068, 0x296);
        return 0;
    }
    for n in 0..4usize {
        crate::comparacao::ramo("cqt_10 time do grupo b");
        write_u32_from_usize(times + n * 4, cqt10_escolhido(this, 4 + n)); // cqt10: o time na vaga do grupo B
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 0, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        let b = |d: usize| *((this + d) as *const u8) as u32;
        construir(
            p,
            this,
            4,
            times,
            1,
            read_u32_as_usize(this + 4),
            tabela,
            rodadas as u16 as u32,
            b(0xc2),
            b(0xc3),
            b(0xc4),
            this + 0xc5,
            this + 0xbe,
            read_i16(this + 0x40) as u16 as u32,
            0,
            id as u16 as u32,
            b(0x51),
            2,
            0,
            read_i16(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc), fase);
    soltar(times);
    soltar(tabela);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc));
    if f == 0 {
        aviso_em(0xa6a068, 0x2ad);
        return 0;
    }
    write_u32_from_usize(this + 0x30, 0);
    noticia(f);
    1
}

// ponytail: a fase é alocada e a notícia vai aos técnicos: não roda duas vezes. Só a original; o plano: devolve 1 com a
// fase em [+0xc][0], +0x30 = 0, a fase com 4 times e o ano da copa e, na tabela dela (+0xb1, linhas de 0x3b bytes), os
// escolhidos 4 a 7 (em qualquer ordem: o construtor ordena a tabela).
unsafe extern "thiscall" fn comparar_cqt10_oceania_grupo_b(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2061);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let mut plano: Vec<usize> = (4..8).map(|k| read_u32_as_usize(lista + k * 4)).collect();
    let r = o(this);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc));
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && read_u32_as_usize(this + 0x30) == 0 && read_i16(f + 0x3e) == 4 && read_i16(f + 0x40) == read_i16(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("cqt10_oceania_grupo_b: a fase não bate com o plano ({:x?} lidos {:x?})", plano, a));
    }
    r
}

// ---------- 283a leva: o Campeonato de Clubes da Oceania: os 8 clubes, a fase seguinte e as semifinais (oceania_club_champ.cpp) ----------




/// Os clubes de uma origem para a Oceania: até 20, na ordem da tabela de clubes.
struct Cqt11Lista {
    clubes: [usize; 20],
    n: i16,
}

impl Cqt11Lista {
    fn novo() -> Self {
        Cqt11Lista { clubes: [0; 20], n: 0 }
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
        let f: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(FN_COMP_UTIL_SORT_AND_SHUFFLE_TOP_CLUBS);
        f(self.clubes.as_mut_ptr() as usize, self.n as u16 as u32, quantos);
    }

    /// O primeiro da lista e o primeiro depois dele de outra nação (+0x53), como a original: percorre as `n` vagas; vaga
    /// vazia não conta como primeiro.
    unsafe fn dois_de_nacoes_diferentes(&self) -> (usize, usize) {
        let (mut a, mut b) = (0usize, 0usize);
        for &c in self.clubes.iter().take(self.n.max(0) as usize) {
            if a == 0 {
                a = c;
            } else if b == 0 && read_u32_as_usize(c + 0x53) != read_u32_as_usize(a + 0x53) {
                b = c;
            }
        }
        (a, b)
    }
}

/// O par de um pote vai para os grupos A e B: com o sorteio de 2 (0x90cfc0(2): push 2; add esp -> cdecl) diferente de
/// zero, o primeiro na vaga `k` (grupo A) e o segundo na `k` + 4 (grupo B); com zero, ao contrário.
unsafe fn cqt11_par(this: usize, k: usize, primeiro: usize, segundo: usize, sorteio: i32) {
    let l = read_u32_as_usize(this + 0xee);
    if sorteio != 0 {
        write_u32_from_usize(l + k * 4, primeiro);
        write_u32_from_usize(read_u32_as_usize(this + 0xee) + (k + 4) * 4, segundo);
    } else {
        write_u32_from_usize(l + (k + 4) * 4, primeiro);
        write_u32_from_usize(read_u32_as_usize(this + 0xee) + k * 4, segundo);
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
unsafe fn cqt11_clubes(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let colocado: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let melanesia: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_MELANESIAN_NATION);
    let polinesia: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_POLYNESIAN_NATION);
    write_u32_from_usize(this + 0xf2, 8);
    let lista = alocar(0x20);
    write_u32_from_usize(this + 0xee, lista);
    if lista == 0 {
        aviso_em(0xa6a068, 0x307);
        return 0;
    }
    let mut campeao = colocado(read_u32_as_usize(0xae23d0) + read_u32_as_usize(KEY_CLUB_COMP_AUSTRALIAN_NATIONAL_SOCCER_LEAGUE) * 0x6b, 0);
    let (mut nz, mut aus, mut tai, mut fij, mut mel, mut pol) =
        (Cqt11Lista::novo(), Cqt11Lista::novo(), Cqt11Lista::novo(), Cqt11Lista::novo(), Cqt11Lista::novo(), Cqt11Lista::novo());
    let mut i = 0i32;
    let mut c = read_u32_as_usize(0xae23b4);
    while i < (read_u32_as_usize(0xae235c) as i32).wrapping_sub((read_u32_as_usize(0xae2350) as i32).wrapping_mul(2)) {
        let nacao = read_u32_as_usize(c + 0x53);
        if nacao != 0 && read_u32_as_usize(nacao + 0x71) != 0 && read_u32_as_usize(read_u32_as_usize(nacao + 0x71)) == read_u32_as_usize(KEY_CONTINENT_OCEANIA) {
            let id = read_u32_as_usize(nacao);
            if id == read_u32_as_usize(KEY_NATION_NEW_ZEALAND) && nz.cabe() {
                nz.por(c);
            } else if id == read_u32_as_usize(KEY_NATION_AUSTRALIA) && aus.cabe() {
                aus.por(c);
            } else if id == read_u32_as_usize(KEY_NATION_TAHITI) && tai.cabe() {
                tai.por(c);
            } else if id == read_u32_as_usize(KEY_NATION_FIJI) && fij.cabe() {
                fij.por(c);
            } else if melanesia(nacao) != 0 {
                if mel.cabe() {
                    mel.por(c);
                }
            } else if polinesia(read_u32_as_usize(c + 0x53)) != 0 && pol.cabe() {
                pol.por(c);
            }
        }
        i += 1;
        c += 0x245;
    }
    crate::comparacao::ramo("cqt_11 clubes da oceania");
    if campeao == 0 {
        aus.ordenar(1);
        campeao = aus.clubes[0];
    }
    nz.ordenar(1);
    cqt11_par(this, 0, campeao, nz.clubes[0], sorteio(2)); // cqt11: o campeão australiano e o neozelandês
    fij.ordenar(1);
    tai.ordenar(1);
    cqt11_par(this, 1, fij.clubes[0], tai.clubes[0], sorteio(2)); // cqt11: Fiji e Taiti
    mel.ordenar(2);
    let (a, b) = mel.dois_de_nacoes_diferentes();
    cqt11_par(this, 2, a, b, sorteio(2)); // cqt11: os dois melanésios
    pol.ordenar(2);
    let (a, b) = pol.dois_de_nacoes_diferentes();
    cqt11_par(this, 3, a, b, sorteio(2));
    1
}

unsafe extern "thiscall" fn cqt11_oceania_clubes(this: usize) -> u32 {
    cqt11_clubes(this)
}

/// Lado a lado: o +0xf2, o [0xb67a34] e o estado do sorteador a partir do mesmo estado (a cópia sorteia sem contar); a
/// original deixa a lista dela em +0xee, a cópia faz a sua, comparada nas 8 vagas e liberada.
unsafe extern "thiscall" fn comparar_cqt11_oceania_clubes(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(2062);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let a_ = crate::aleatorio::REGIOES_DO_ESTADO;
    let reg = [(this + 0xf2, 4usize), (0xb67a34, 4), a_[0], a_[1]];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o(this);
        lo.set(read_u32_as_usize(this + 0xee));
        r
    }, || {
        let r = crate::aleatorio::sem_contar(|| cqt11_clubes(this));
        lr.set(read_u32_as_usize(this + 0xee));
        write_u32_from_usize(this + 0xee, lo.get());
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, 0x20) == core::slice::from_raw_parts(lr as *const u8, 0x20)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("cqt11_oceania_clubes({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x797df0 } (thiscall, ret 0, oceania_club_champ.cpp; a virtual +0x28): `OceaniaClubChamp::advance_stage()`: na
/// última fase (+0x30 ≥ +0x2c − 1, com sinal), a caixa da linha 0x274; senão +0x30 sobe e, chegando a 1 (depois dos
/// grupos), as semifinais (0x798050: mov ecx, this -> thiscall, ret 0).
unsafe extern "thiscall" fn cqt11_oceania_fase_seguinte(this: usize) {
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return aviso_em(0xa6a068, 0x274);
    }
    let f = f + 1;
    write_u32_from_usize(this + 0x30, f as usize);
    if f == 1 {
        crate::comparacao::ramo("cqt_11 semifinais da oceania");
        let semifinais: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_OCEANIACLUBCHAMP_CREATE_KNOCKOUT_STAGE);
        semifinais(this);
    }
}

// ponytail: as semifinais alocam e sorteiam: não rodam duas vezes. Só a original; o plano: +0x30 sobe um (fora da
// última fase, que dá a caixa e não muda nada).
unsafe extern "thiscall" fn comparar_cqt11_oceania_fase_seguinte(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2063);
    let f = read_u32_as_usize(this + 0x30) as i32;
    let ultima = f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1);
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == if ultima { f } else { f + 1 }, || format!("cqt11_oceania_fase_seguinte({:#x}) fase {} -> {}", this, f, depois));
}

/// Os 4 das semifinais, na ordem da lista: o 1º do grupo A (a tabela da copa, +0xb1), o 2º do B (a tabela da fase
/// [+0xc][0]), o 2º do A e o 1º do B (os cruzamentos: as chaves são os pares da lista).
unsafe fn cqt11_semifinalistas(this: usize) -> [usize; 4] {
    let a = |l: usize| read_u32_as_usize(read_u32_as_usize(this + 0xb1) + l * 0x3b);
    let b = |l: usize| read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(this + 0xc)) + 0xb1) + l * 0x3b);
    [a(0), b(1), a(1), b(0)] // cqt11: 1º A × 2º B, 2º A × 1º B
}

/// { 0x798050 } (thiscall, ret 0, oceania_club_champ.cpp, quadro SEH só para o new da fase: sem destrutor no caminho
/// normal): `OceaniaClubChamp::create_knockout_stage()`, as semifinais: a lista dos 4 (0x944e46(0x10): push; add esp, 4
/// -> cdecl; sem memória, a caixa da linha 0x2cb e volta), os cruzamentos de cqt11_semifinalistas; a tabela das rodadas
/// pela virtual +0x3c (push 0; push &id; push &rodadas; push 1; mov ecx, this -> thiscall, ret 0x10); a fase (0x944cf1(0xb2)
/// push; add esp, 4 -> cdecl; Cup_Stage::Cup_Stage 0x522e00, thiscall, ret 0x3c: a copa, 4, a lista, as rodadas, +4, a
/// tabela, o ano +0x40, 1, 2, o id, 0x14, 0, 0, 0 e nenhuma semente; a original empurra as rodadas, o id e o ano com lixo
/// nos 16 bits altos, que o construtor não lê) em [+0xc][1]; a tabela e a lista liberadas (0x9452ca). Sem fase: a caixa
/// da linha 0x2e3 e volta. Com ela: a rodada montada (0x51c800 Cup::set_up_round: push 0; mov ecx, fase -> thiscall,
/// ret 4).
unsafe extern "thiscall" fn cqt11_oceania_semifinais(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let montar: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_CUP_SET_UP_ROUND);
    let lista = alocar(0x10);
    if lista == 0 {
        return aviso_em(0xa6a068, 0x2cb);
    }
    crate::comparacao::ramo("cqt_11 cruzamentos da oceania");
    for (k, &c) in cqt11_semifinalistas(this).iter().enumerate() {
        write_u32_from_usize(lista + k * 4, c);
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, 1, &mut rodadas, &mut id, 0);
    let p = novo(0xb2);
    let fase = if p != 0 {
        #[allow(clippy::type_complexity)]
        let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, u32, *const u8) -> usize =
            core::mem::transmute(FN_CUP_STAGE_CUP_STAGE);
        construir(p, this, 4, lista, rodadas as u16 as u32, read_u32_as_usize(this + 4), tabela, read_i16(this + 0x40) as u16 as u32, 1, 2, id as u16 as u32, 0x14, 0, 0, 0, core::ptr::null())
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + 4, fase);
    soltar(tabela);
    soltar(lista);
    let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4);
    if f == 0 {
        return aviso_em(0xa6a068, 0x2e3);
    }
    montar(f, 0);
}

// ponytail: aloca a fase e monta a rodada: não roda duas vezes. Só a original; o plano: a fase em [+0xc][1] com 4 times,
// o +4 da copa e os 4 cruzados (lidos antes) na lista dela (+0xa7, 6 bytes; como conjunto: a rodada pode reordenar).
unsafe extern "thiscall" fn comparar_cqt11_oceania_semifinais(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(2064);
    if read_u32_as_usize(this + 0xb1) == 0 || read_u32_as_usize(this + 0xc) == 0 || read_u32_as_usize(read_u32_as_usize(this + 0xc)) == 0 {
        return o(this);
    }
    let mut plano = cqt11_semifinalistas(this).to_vec();
    o(this);
    let fase = read_u32_as_usize(read_u32_as_usize(this + 0xc) + 4);
    if fase == 0 {
        PLACAR.registrar(false, || format!("cqt11_oceania_semifinais({:#x}) sem fase", this));
        return;
    }
    let lista = read_u32_as_usize(fase + 0xa7);
    let mut achou: Vec<usize> = (0..4).map(|k| read_u32_as_usize(lista + 6 * k)).collect();
    let (pl, ac) = (plano.clone(), achou.clone());
    plano.sort_unstable();
    achou.sort_unstable();
    let igual = plano == achou && read_i16(fase + 0x3e) == 4 && read_u32_as_usize(fase + 4) == read_u32_as_usize(this + 4);
    PLACAR.registrar(igual, || format!("cqt11_oceania_semifinais({:#x}) plano={:x?} lista={:x?}", this, pl, ac));
}

// ---------- 281a leva: as copas de clubes da Oceania e da Ásia: o calendário do Campeonato de Clubes da Oceania e os times da Liga dos Campeões e da Recopa asiáticas (oceania_club_champ.cpp, asia_club_champ.cpp, asia_cup_winner.cpp) ----------

/// Uma operação da montagem da tabela das rodadas, na ordem da original: `D` a data de uma rodada de grupo (0x68a160:
/// rodada, dia, mês com 0 = janeiro, anos a somar, dia da semana, código, valor; registros de 0x41 bytes), `L` um jogo
/// da rodada de grupo (0x68a1c0: rodada, jogo, a, b, c, valor), `S` o sorteio de uma rodada de copa (0x521e60: rodada,
/// dia, mês, anos, dia da semana; registros de 0x68 bytes), `J` o jogo da rodada de copa (0x521eb0: rodada, dia, mês,
/// anos, dia da semana, horário, mando), `E` um dia do confronto (0x521f10: rodada, vaga, dia da semana, horário,
/// código, mando) e `B`/`W`/`X` a gravação de um byte/palavra/dword no deslocamento dado da tabela.
#[derive(Clone, Copy)]
enum Cqq12Op {
    D(i32, i32, i32, i32, i32, i32, i32),
    L(i32, i32, i32, i32, i32, u32),
    S(u32, u32, u32, u32, u32),
    J(u32, u32, u32, u32, u32, u32, u32),
    E(i32, i32, i32, i32, i32, u32),
    B(usize, u8),
    W(usize, u16),
    X(usize, u32),
}
use Cqq12Op::{B as Cqq12B, D as Cqq12D, E as Cqq12E, J as Cqq12J, L as Cqq12L, S as Cqq12S, W as Cqq12W, X as Cqq12X};

/// Uma fase da virtual +0x3c (`create_schedule(fase, &rodadas, &extra, &é_liga)`): o é_liga gravado (se pedido), as
/// rodadas, o extra (só se o ponteiro não é nulo, quando `extra_checa`), o tamanho da tabela, a linha da caixa sem
/// memória, as operações comuns e as da fase (os jogos de cada grupo) e o nome do contador. `None`: a fase devolve 0
/// sem gravar nada.
#[derive(Clone, Copy)]
struct Cqq12Fase {
    liga: i32,
    rodadas: i16,
    extra: i16,
    extra_checa: bool,
    tamanho: u32,
    linha: u32,
    ops: &'static [Cqq12Op],
    ops2: &'static [Cqq12Op],
    nome: &'static str,
}

/// Executa a fase `f` (ou devolve 0 sem ela): as saídas, a tabela (0x944e46; sem memória, a caixa `linha` do fonte
/// `arquivo` e 0) e as operações na ordem; o ano da copa (+0x40) vai em 16 bits limpos para a 0x68a160 (a original
/// empurra o registrador com lixo nos 16 bits altos; a 0x68a160 só usa a palavra) e com sinal para as de copa.
unsafe fn cqq12_tabela(this: usize, f: Option<Cqq12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32, arquivo: usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => return 0,
    };
    if !liga.is_null() {
        *liga = f.liga;
    }
    *rodadas = f.rodadas;
    if !f.extra_checa || !extra.is_null() {
        *extra = f.extra;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar(f.tamanho);
    if t == 0 {
        aviso_em(arquivo, f.linha);
        return 0;
    }
    crate::comparacao::ramo(f.nome);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let jogo_l: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    let sorteio: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_DRAW_DATE);
    let jogo_c: unsafe extern "C" fn(usize, u32, u32, u32, u32, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DATE);
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(FN_CUP_SET_ROUND_MATCH_DAY);
    let ano16 = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for op in f.ops.iter().chain(f.ops2.iter()) {
        match *op {
            Cqq12D(r, dia, mes, anos, sem, cod, v) => {
                data(t, r, dia, mes, anos, sem, cod, ano16, v);
            }
            Cqq12L(r, j, a, b, c, v) => jogo_l(t, r, j, a, b, c, v),
            Cqq12S(r, dia, mes, anos, sem) => {
                sorteio(t, r, dia, mes, anos, sem, ano);
            }
            Cqq12J(r, dia, mes, anos, sem, hora, mando) => {
                jogo_c(t, r, dia, mes, anos, sem, hora, ano, mando);
            }
            Cqq12E(r, vaga, sem, hora, cod, mando) => {
                elo(t, r, vaga, sem, hora, cod, mando);
            }
            Cqq12B(o, v) => *((t + o) as *mut u8) = v,
            Cqq12W(o, v) => core::ptr::write_unaligned((t + o) as *mut u16, v),
            Cqq12X(o, v) => core::ptr::write_unaligned((t + o) as *mut u32, v),
        }
    }
    t
}

/// Os bytes que as operações gravam na tabela `t` (o resto fica como veio da alocação), na ordem das operações.
unsafe fn cqq12_gravados(ops: &[Cqq12Op], ops2: &[Cqq12Op], t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    let mut pega = |a: usize, n: usize| v.extend_from_slice(core::slice::from_raw_parts((t + a) as *const u8, n));
    for op in ops.iter().chain(ops2.iter()) {
        match *op {
            Cqq12D(r, ..) => {
                pega(r as usize * 0x41, 5);
                pega(r as usize * 0x41 + 0x3d, 4);
            }
            Cqq12L(r, j, ..) => pega(r as usize * 0x41 + 5 + j as usize * 7, 7),
            Cqq12S(r, ..) => pega(r as usize * 0x68, 3),
            Cqq12J(r, ..) => {
                pega(r as usize * 0x68 + 3, 4);
                pega(r as usize * 0x68 + 0x13, 4);
            }
            Cqq12E(r, vaga, ..) => pega(r as usize * 0x68 + 0x23 + vaga as usize * 7, 7),
            Cqq12B(o, _) => pega(o, 1),
            Cqq12W(o, _) => pega(o, 2),
            Cqq12X(o, _) => pega(o, 4),
        }
    }
    v
}

/// Lado a lado: a original com as saídas de quem chama, a cópia com saídas próprias que começam com os mesmos valores, a
/// partir do mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas têm de ser iguais; a tabela da cópia é
/// liberada (0x9452ca). Na fase sem tabela só a original (nenhuma grava nada; as saídas podem nem existir).
unsafe fn cqq12_comparar(nome: &str, fase: u32, f: Option<Cqq12Fase>, rodadas: *mut i16, extra: *mut i16, liga: *mut i32,
    o: impl FnOnce() -> usize, c: impl FnOnce(*mut i16, *mut i16, *mut i32) -> usize) -> usize {
    let f = match f {
        Some(f) => f,
        None => {
            let a = o();
            PLACAR.registrar(a == 0, || format!("{}(fase {}) devolveu {:#x}", nome, fase as u8 as i8, a));
            return a;
        }
    };
    let (mut r2, mut e2) = (*rodadas, if extra.is_null() { 0 } else { *extra });
    let mut l2 = if liga.is_null() { 0 } else { *liga };
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, o, || c(&mut r2, if extra.is_null() { core::ptr::null_mut() } else { &mut e2 },
        if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }));
    let saidas = *rodadas == r2 && (extra.is_null() || *extra == e2) && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqq12_gravados(f.ops, f.ops2, a) == cqq12_gravados(f.ops, f.ops2, b) } else { a == 0 && b == 0 };
    if b != 0 {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("{}(fase {}): saídas {} ({} x {}) tabela {}", nome, fase as u8 as i8, saidas, *rodadas, r2, tabela));
    a
}

/// O Campeonato de Clubes da Oceania, as três rodadas de cada grupo (10, 12 e 14 de janeiro do ano seguinte, valor 1) com o jogo 2 vazio.
const CQQ12_OCEANIA_GRUPOS: [Cqq12Op; 6] = [
    Cqq12D(0, 0xa, 0, 1, 4, 2, 1), Cqq12L(0, 2, -1, -1, -1, 0), Cqq12D(1, 0xc, 0, 1, 6, 1, 1),
    Cqq12L(1, 2, -1, -1, -1, 0), Cqq12D(2, 0xe, 0, 1, 1, 2, 1), Cqq12L(2, 2, -1, -1, -1, 0),
];
/// Os jogos do grupo da fase −1.
const CQQ12_OCEANIA_GRUPO_A: [Cqq12Op; 6] = [
    Cqq12L(0, 0, 4, 1, 3, 4), Cqq12L(0, 1, 4, 2, 3, 0x40), Cqq12L(1, 0, 6, 1, 3, 4), Cqq12L(1, 1, 6, 2, 3, 0x40),
    Cqq12L(2, 0, 1, 1, 3, 4), Cqq12L(2, 1, 1, 1, 3, 0x40),
];
/// Os jogos do grupo da fase 0.
const CQQ12_OCEANIA_GRUPO_B: [Cqq12Op; 6] = [
    Cqq12L(0, 0, 5, 1, 3, 0x40), Cqq12L(0, 1, 5, 2, 3, 4), Cqq12L(1, 0, 0, 1, 3, 0x40), Cqq12L(1, 1, 0, 2, 3, 4),
    Cqq12L(2, 0, 2, 1, 3, 4), Cqq12L(2, 1, 2, 1, 3, 0x40),
];
/// O mata-mata em jogo único: semifinal em 17/1, final e terceiro lugar em 19/1 do ano seguinte.
const CQQ12_OCEANIA_MATA: [Cqq12Op; 54] = [
    Cqq12S(0, 0x10, 0, 1, 3), Cqq12J(0, 0x11, 0, 1, 4, 2, 0x40), Cqq12W(0x7, 0x82), Cqq12W(0x9, 0), Cqq12W(0xb, 0),
    Cqq12W(0xd, 0x283), Cqq12B(0x17, 8), Cqq12W(0x18, 4), Cqq12W(0x1a, 2), Cqq12W(0x1c, 4), Cqq12W(0xf, 0),
    Cqq12W(0x1e, 0), Cqq12B(0x20, 0), Cqq12B(0x21, 1), Cqq12B(0x22, 0), Cqq12X(0x5c, 0), Cqq12X(0x60, 0),
    Cqq12X(0x64, 0), Cqq12S(1, 0x12, 0, 1, 5), Cqq12J(1, 0x13, 0, 1, 6, 1, 4), Cqq12W(0x6f, 0x96), Cqq12W(0x71, 1),
    Cqq12W(0x73, 0), Cqq12W(0x75, 0x83), Cqq12B(0x7f, 8), Cqq12W(0x80, 2), Cqq12W(0x82, 1), Cqq12W(0x84, 0),
    Cqq12W(0x77, 0), Cqq12W(0x86, 0), Cqq12B(0x88, 0), Cqq12B(0x89, 1), Cqq12B(0x8a, 0), Cqq12X(0xc4, 0),
    Cqq12X(0xc8, 0), Cqq12X(0xcc, 0), Cqq12S(2, 0x12, 0, 1, 5), Cqq12J(2, 0x13, 0, 1, 6, 1, 0x40), Cqq12W(0xd7, 0x8c),
    Cqq12W(0xd9, 2), Cqq12W(0xdb, 0), Cqq12W(0xdd, 0x83), Cqq12B(0xe7, 8), Cqq12W(0xe8, 2), Cqq12W(0xea, 1),
    Cqq12W(0xec, 0), Cqq12W(0xdf, 0), Cqq12W(0xee, 0), Cqq12B(0xf0, 0), Cqq12B(0xf1, 1), Cqq12B(0xf2, 0),
    Cqq12X(0x12c, 0), Cqq12X(0x130, 0), Cqq12X(0x134, 0),
];

/// A fase do Campeonato de Clubes da Oceania: abaixo de 1 os grupos (id 0x420 + fase; os jogos de cada grupo para −1
/// e 0), 1 o mata-mata, acima nada.
fn cqq12_oceania_fase(fase: u32) -> Option<Cqq12Fase> {
    let f = fase as u8 as i8;
    if f < 1 {
        let jogos: &'static [Cqq12Op] = match f {
            -1 => &CQQ12_OCEANIA_GRUPO_A,
            0 => &CQQ12_OCEANIA_GRUPO_B,
            _ => &[],
        };
        Some(Cqq12Fase { liga: 1, rodadas: 3, extra: (f as i16).wrapping_add(0x420), extra_checa: false, tamanho: 0xc3, linha: 0x93,
            ops: &CQQ12_OCEANIA_GRUPOS, ops2: jogos, nome: "cqq_12 grupos da oceania" })
    } else if f == 1 {
        Some(Cqq12Fase { liga: 0, rodadas: 3, extra: 0, extra_checa: false, tamanho: 0x138, linha: 0xc5,
            ops: &CQQ12_OCEANIA_MATA, ops2: &[], nome: "cqq_12 mata-mata da oceania" })
    } else {
        None
    }
}

/// { 0x797230 } (thiscall, ret 0x10, oceania_club_champ.cpp; a virtual +0x3c): `OceaniaClubChamp::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: a fase de `cqq12_oceania_fase`.
unsafe extern "thiscall" fn cqq12_oceania_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    cqq12_tabela(this, cqq12_oceania_fase(fase), rodadas, extra, liga, 0xa6a068)
}

unsafe extern "thiscall" fn comparar_cqq12_oceania_tabela(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1983);
    cqq12_comparar("cqq12_oceania_tabela", fase, cqq12_oceania_fase(fase), rodadas, extra, liga, || o(this, fase, rodadas, extra, liga),
        |r, e, l| cqq12_oceania_tabela(this, fase, r, e, l))
}


/// A tabela dos 16 times das copas asiáticas (AsiaClubChamp::build_team_table 0x40b620 e AsiaCupWinner::
/// create_team_entries 0x40d4d0, o mesmo corpo): +0x3e = 16 e a tabela (0x944e46 de 0x60 em +0xa7; sem memória, a caixa
/// `linha` de `arquivo` e −1); a vaga i recebe o clube i dos classificados (+0xb2, ponteiros; `ordem` dá o índice) com
/// classe e situação 0. Devolve 1.
unsafe fn cqq12_times(this: usize, arquivo: usize, linha: u32, nome: &'static str, ordem: impl Fn(i16) -> i16) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x10);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let l = alocar(0x60);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        aviso_em(arquivo, linha);
        return -1;
    }
    crate::comparacao::ramo(nome);
    let mut i: i16 = 0;
    while i < core::ptr::read_unaligned((this + 0x3e) as *const i16) {
        let c = read_u32_as_usize(read_u32_as_usize(this + 0xb2) + ordem(i) as usize * 4);
        let e = read_u32_as_usize(this + 0xa7) + i as usize * 6;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 4) as *mut u8) = 0;
        *((e + 5) as *mut u8) = 0;
        i = i.wrapping_add(1);
    }
    1
}

/// Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a tabela dela em +0xa7, a cópia faz a
/// sua, comparada nas 16 vagas (com a devolução 1) e liberada.
unsafe fn cqq12_comparar_times(nome: &str, this: usize, o: impl FnOnce() -> i32, c: impl FnOnce() -> i32) -> i32 {
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || {
        let r = o();
        lo.set(read_u32_as_usize(this + 0xa7));
        r
    }, || {
        let r = c();
        lr.set(read_u32_as_usize(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { 0x60 } else { 0 };
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && lista, || format!("{}({:#x}) devolveu {} x {} memória {} lista {}", nome, this, a, b, igual, lista));
    a
}

/// { 0x40b620 } (thiscall, ret 0, asia_club_champ.cpp): `AsiaClubChamp::build_team_table()`: os 16 times (a caixa da
/// linha 0x1cd sem memória).
unsafe extern "thiscall" fn cqq12_asia_clubes_times(this: usize) -> i32 {
    cqq12_times(this, 0x987a40, 0x1cd, "cqq_12 times da liga dos campeoes asiatica", |i| i) // cqq12: os times da Liga dos Campeões asiática
}

unsafe extern "thiscall" fn comparar_cqq12_asia_clubes_times(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1984);
    cqq12_comparar_times("cqq12_asia_clubes_times", this, || o(this), || cqq12_asia_clubes_times(this))
}

/// { 0x40d4d0 } (thiscall, ret 0, asia_cup_winner.cpp): `AsiaCupWinner::create_team_entries()`: os 16 times (a caixa
/// da linha 0x185 sem memória).
unsafe extern "thiscall" fn cqq12_asia_recopa_times(this: usize) -> i32 {
    cqq12_times(this, 0x987b60, 0x185, "cqq_12 times da recopa asiatica", |i| i) // cqq12: os times da Recopa asiática
}

unsafe extern "thiscall" fn comparar_cqq12_asia_recopa_times(this: usize) -> i32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> i32>(1985);
    cqq12_comparar_times("cqq12_asia_recopa_times", this, || o(this), || cqq12_asia_recopa_times(this))
}

