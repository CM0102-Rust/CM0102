use super::*;

// ---------- 280a leva: o construtor da primeira divisão francesa: as duas competições-marcador, os clubes da CFA e da divisão de baixo levados à divisão genérica e as duas ligas de fundo montadas e soltas (fra_first.cpp) ----------



/// O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes; a original faz id × 3 × 9 × 4 − id).
pub(crate) unsafe fn cqq6_registro(g: usize) -> usize {
    read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(read_u32_as_usize(g).wrapping_mul(0x6b))
}

/// Uma competição-marcador (operator new 0x944cf1 de 0xa3: push; add esp, 4 -> cdecl; sem memória: 0): Comp::Comp
/// (0x48c580: mov ecx -> thiscall, ret 0), a vtable `vtabela`, +0x42 = 1, o registro de id [g] em +4 e sem disciplina
/// (+0x50 = 0xff).
pub(crate) unsafe fn cqq6_marcador(vtabela: usize, g: usize) -> usize {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let comp: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_COMP_COMP);
    let c = novo(0xa3);
    if c == 0 {
        return 0;
    }
    comp(c);
    write_u32_from_usize(c, vtabela);
    *((c + 0x42) as *mut u8) = 1;
    write_u32_from_usize(c + 4, cqq6_registro(g));
    *((c + 0x50) as *mut u8) = 0xff;
    c
}

/// builds the French first division: discipline 0xc; two placeholder competitions (the CFA and the lower division),
/// with no matches of their own; in a new career, every club whose league in the database is the CFA moves to the
/// generic division with the CFA as reserve competition [inference: the B sides of the professional clubs], and every
/// lower division club moves to the generic division with no reserve; then the rules, the table, the fixtures, the
/// order, the statistics and the predicted positions; at last it builds the CFA (4 groups of 18) and the lower division
/// (2 groups of 20), which leave their matches in the calendar, and frees both objects
/// Original: `FraFirst::FraFirst` (0x5bb160, fra_first.cpp, thiscall, ret 8).
///
/// [pt] { 0x5bb160 } (thiscall, ret 8, fra_first.cpp, 988 bytes, quadro SEH só para os operator new): `FraFirst::FraFirst(ano,
/// registro)`: a base (League::League 0x682200: mov ecx -> thiscall, ret 0), a vtable 0x96aa24, o ano (+0x40, palavra),
/// o registro (+4) e a disciplina 0xc (+0x50); em +0xee e +0xf2 as competições-marcador da CFA (vtable 0x96a994, id
/// [0x9cf5d8]) e da divisão de baixo (0x96a904, id [0x9cf548]). Tenta carregar do arquivo temporário
/// (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4): carregou, pronto. Senão, a liga nova:
/// cada clube (os [0xae235c] − 2 × [0xae2350] primeiros de [0xae23b4], 0x245 bytes; a conta refeita a cada volta) cuja
/// competição (+0x57) é a CFA passa à divisão genérica (registro de id [0x9cf764]) com a CFA como competição de reserva
/// (+0x60); o da divisão de baixo, à genérica sem reserva (+0x60 = 0). Depois fase 0xff (+0x44), fase atual −1 (+0x30),
/// sem fases (+0x2c, +0xc); as regras (0x5bc410), a tabela (0x5bc2b0), os jogos (League::create_fixtures 0x6835c0), a
/// ordem (League::sort_table 0x6827d0: push 0 -> thiscall, ret 4), as estatísticas (operator new de 0x5ce e
/// Comp_Stats::Comp_Stats 0x49ee70: push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas
/// (0x68a850: mov ecx -> thiscall, ret 0); cada passo que falha mostra a caixa da sua linha (0x65, 0x6c, 0x73, 0x7c,
/// 0x83) e encerra. Por fim, as duas ligas de fundo: a CFA (operator new de 0xee; FraCfa::FraCfa 0x5b91c0: push ano;
/// mov ecx -> thiscall, ret 4) e a divisão de baixo (0x5bd600, o mesmo), que deixam os seus jogos no calendário do jogo,
/// e são destruídas (a virtual 0 com 1: push 1; mov ecx -> thiscall, ret 4). Devolve this.
pub(crate) unsafe extern "thiscall" fn construir_primeira_francesa(this: usize, ano: u32, registro: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LEAGUE_LEAGUE);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_LOAD_FROM_TEMP_FILE);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_FRAFIRST_SET_UP_RULES);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_FRAFIRST_CREATE_TABLE);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_COMP_STATS);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SET_PREDICTED_POSITIONS);
    let cfa: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_FRACFA_FRACFA);
    let baixo: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(FN_FRALOWER_FRALOWER);
    base(this);
    write_u32_from_usize(this, 0x96aa24);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    write_u32_from_usize(this + 4, registro);
    *((this + 0x50) as *mut u8) = 0xc; // cqq6: a disciplina da primeira francesa
    let m = cqq6_marcador(0x96a994, KEY_CLUB_COMP_FRENCH_CFA);
    write_u32_from_usize(this + 0xee, m);
    let m = cqq6_marcador(0x96a904, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION);
    write_u32_from_usize(this + 0xf2, m);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqq_6 primeira francesa nova");
    let n = || (read_u32_as_usize(G_CLUB_COUNT) as i32).wrapping_sub((read_u32_as_usize(G_NATION_COUNT) as i32).wrapping_mul(2));
    if n() > 0 {
        let mut p = read_u32_as_usize(G_CLUB_TABLE) + 0x57;
        let mut i: i32 = 0;
        loop {
            let comp = read_u32_as_usize(p);
            if comp != 0 {
                if read_u32_as_usize(comp) == read_u32_as_usize(KEY_CLUB_COMP_FRENCH_CFA) {
                    write_u32_from_usize(p + 9, comp); // cqq6: a CFA vira a competição de reserva
                    write_u32_from_usize(p, cqq6_registro(KEY_CLUB_COMP_A_LOWER_DIVISION));
                } else if read_u32_as_usize(comp) == read_u32_as_usize(KEY_CLUB_COMP_FRENCH_LOWER_DIVISION) {
                    write_u32_from_usize(p + 9, 0); // cqq6: a divisão de baixo fica sem reserva
                    write_u32_from_usize(p, cqq6_registro(KEY_CLUB_COMP_A_LOWER_DIVISION));
                }
            }
            i += 1;
            p += 0x245;
            if i >= n() {
                break;
            }
        }
    }
    *((this + 0x44) as *mut u8) = 0xff;
    write_u32_from_usize(this + 0x30, 0xffff_ffff);
    write_u32_from_usize(this + 0x2c, 0);
    write_u32_from_usize(this + 0xc, 0);
    if regras(this) == 0 {
        aviso_em(0x9ccb5c, 0x65);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(0x9ccb5c, 0x6c);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(0x9ccb5c, 0x73);
        return this;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(0x9ccb5c, 0x7c);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    write_u32_from_usize(this + 8, s);
    if s == 0 {
        aviso_em(0x9ccb5c, 0x83);
        return this;
    }
    previstas(this);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let a = novo(0xee);
    let ligas_cfa = if a != 0 { cfa(a, ano) } else { 0 };
    let b = novo(0xee);
    let ligas_baixo = if b != 0 { baixo(b, ano) } else { 0 };
    for l in [ligas_cfa, ligas_baixo] {
        if l != 0 {
            let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(l)));
            destruir(l, 1);
        }
    }
    this
}

/// Sorteia (as ligas de fundo), aloca e mexe nos clubes: só a original, conferindo a devolução, a vtable, o ano, o
/// registro, a disciplina e os dois marcadores (vtable e registro) e, na liga nova, os campos do começo e as
/// estatísticas.
pub(crate) unsafe extern "thiscall" fn comparar_construir_primeira_francesa(this: usize, ano: u32, registro: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(1946);
    let r = o(this, ano, registro);
    let marcador = |m: usize, vt: usize, g: usize| m == 0 || (read_u32_as_usize(m) == vt && read_u32_as_usize(m + 4) == cqq6_registro(g) && *((m + 0x50) as *const u8) == 0xff);
    let mut ok = r == this && read_u32_as_usize(this) == 0x96aa24 && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && read_u32_as_usize(this + 4) == registro && *((this + 0x50) as *const u8) == 0xc
        && marcador(read_u32_as_usize(this + 0xee), 0x96a994, KEY_CLUB_COMP_FRENCH_CFA) && marcador(read_u32_as_usize(this + 0xf2), 0x96a904, KEY_CLUB_COMP_FRENCH_LOWER_DIVISION);
    if *((this + 0x44) as *const u8) == 0xff && read_u32_as_usize(this + 8) != 0 {
        ok = ok && read_u32_as_usize(this + 0x2c) == 0 && read_u32_as_usize(this + 0xc) == 0;
    }
    PLACAR.registrar(ok, || format!("construir_primeira_francesa({:#x}): devolveu {:#x}, vtable {:#x}, +0xee = {:#x}, +0xf2 = {:#x}, +8 = {:#x}", this, r, read_u32_as_usize(this), read_u32_as_usize(this + 0xee), read_u32_as_usize(this + 0xf2), read_u32_as_usize(this + 8)));
    r
}

// ---------- 278a leva: o calendário real de 2001/02 dos dois grupos da Série C1 italiana (os jogos fixos pelo nome dos clubes) e os dois tocos que devolvem 1 (ita_ser_c1a.cpp, ita_ser_c1b.cpp, fra_first.cpp) ----------



/// Quantos jogos a liga tem: (clubes +0x3e / 2, com sinal, para zero) × rodadas +0xa9.
pub(crate) unsafe fn cqn6_jogos(liga: usize) -> i32 {
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
    let rodadas = core::ptr::read_unaligned((liga + 0xa9) as *const i16) as i32;
    (clubes / 2).wrapping_mul(rodadas)
}

/// O corpo das duas (set_up_fixed_fixtures; thiscall, ret 0, quadro SEH só para o construtor de vetor): o vetor dos
/// jogos fixos (operator new 0x944cf1 de n × 16 + 4: push; add esp, 4 -> cdecl; a conta n no primeiro dword e cada jogo
/// pelo construtor 0x645800, com o destrutor 0x5a53c0 para a exceção: `eh vector constructor iterator` 0x9463af, push
/// 0x5a53c0; push 0x645800; push n; push 0x10; push vetor -> stdcall, ret 0x14) em +0xea; sem memória, a caixa da
/// linha `linha` do fonte `arquivo` e fim. Depois cada jogo da `tabela`, na ordem, no seu lugar do vetor
/// (fixed_fixture_set_by_club_names 0x68aff0: push o 6º (fora, pelo mapa); push o 5º (casa); push ano; push mês (0 =
/// janeiro); push dia; push liga; mov ecx, [+0xea] + 16 × k -> thiscall, ret 0x18: a data e os clubes achados pelo
/// nome, −1 em +8/+0xc se não acha). Por fim, se algum dos n jogos ficou com um clube sem achar (+8 ou +0xc = −1), o
/// vetor é destruído (`eh vector destructor iterator` 0x9462bb: push 0x5a53c0; push conta; push 0x10; push vetor ->
/// stdcall, ret 0x10; operator delete 0x944c94 do bloco: push; add esp, 4 -> cdecl) e +0xea = 0: a liga volta ao
/// calendário sorteado.
pub(crate) unsafe fn cqn6_fixos(this: usize, tabela: &[(u8, u8, u16, u32, u32)], arquivo: usize, linha: u32) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let construir: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(FN_CRT_VECTOR_CONSTRUCTOR_ITERATOR);
    let destruir: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_VECTOR_DESTRUCTOR_ITERATOR);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_OPERATOR_DELETE);
    let jogo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(FN_FIXED_FIXTURE_SET_BY_CLUB_NAMES);
    let n = cqn6_jogos(this);
    let bloco = novo((n as u32).wrapping_mul(16).wrapping_add(4));
    let vetor = if bloco != 0 {
        write_u32_from_usize(bloco, n as usize);
        construir(bloco + 4, 0x10, n as u32, 0x645800, 0x5a53c0);
        bloco + 4
    } else {
        0
    };
    write_u32_from_usize(this + 0xea, vetor);
    if vetor == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, &(dia, mes, ano, casa, fora)) in tabela.iter().enumerate() {
        jogo(read_u32_as_usize(this + 0xea) + k * 0x10, this, dia as u32, mes as u32, ano as u32, casa, fora); // cqn6: cada jogo fixo
    }
    crate::comparacao::ramo("cqn_6 calendario fixo");
    let n = cqn6_jogos(this);
    if n <= 0 {
        return;
    }
    let v = read_u32_as_usize(this + 0xea);
    for i in 0..n as usize {
        let f = v + i * 0x10;
        if read_u32_as_usize(f + 8) == 0xffff_ffff || read_u32_as_usize(f + 0xc) == 0xffff_ffff { // cqn6: um clube sem achar derruba a lista
            if v != 0 {
                destruir(v, 0x10, read_u32_as_usize(v - 4) as u32, 0x5a53c0);
                apagar(v - 4);
            }
            write_u32_from_usize(this + 0xea, 0);
            return;
        }
    }
}


/// the 2001/02 C1/B, with the same rule
/// Original: `ItaSerC1b::set_up_fixed_fixtures` (0x650100, ita_ser_c1b.cpp, thiscall, ret 0).
///
/// [pt] { 0x650100 } (thiscall, ret 0, ita_ser_c1b.cpp, 11.650 bytes): `ItaSerC1b::set_up_fixed_fixtures()`: os 306 jogos
/// da C1/B 2001/02. Fonte 0x9ea558, linha 0x404.
pub(crate) unsafe extern "thiscall" fn calendario_fixo_da_c1b(this: usize) {
    cqn6_fixos(this, &CQN6_C1B, 0x9ea558, 0x404)
}
/// O comparador das duas: alocam: só a original, conferindo o que ela garante: ou a lista ficou (+0xea com a conta n e
/// cada jogo com os dois clubes achados), ou foi derrubada (+0xea = 0).
pub(crate) unsafe fn cqn6_comparar(nome: &str, this: usize, original: impl FnOnce()) {
    original();
    let v = read_u32_as_usize(this + 0xea);
    let n = cqn6_jogos(this);
    let ok = v == 0 || (read_u32_as_usize(v - 4) as i32 == n && (0..n.max(0) as usize).all(|i| read_u32_as_usize(v + i * 0x10 + 8) != 0xffff_ffff && read_u32_as_usize(v + i * 0x10 + 0xc) != 0xffff_ffff));
    PLACAR.registrar(ok, || format!("{}({:#x}): a lista dos jogos fixos (+0xea = {:#x}) fora do esperado", nome, this, v));
}


pub(crate) unsafe extern "thiscall" fn comparar_calendario_fixo_da_c1b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1884);
    cqn6_comparar("calendario_fixo_da_c1b", this, || o(this))
}

/// the add_result stub of competitions that play no matches: returns 1
/// Original: `stub_return_one_ret8` (0x5bb540, fra_first.cpp, thiscall, ret 8).
///
/// [pt] { 0x5bb540 } (thiscall, ret 8, fra_first.cpp, 8 bytes): `stub_return_one_ret8(a, b)`, o toco das competições que não
/// jogam partidas no lugar do add_result (o slot +4): ignora os dois e devolve 1.
pub(crate) unsafe extern "thiscall" fn toco_um_com_dois(_this: usize, _a: u32, _b: u32) -> u32 {
    1 // cqn6: o toco devolve 1
}

/// the default of competition slot 25 and ruling body slot 6: returns 1
/// Original: `stub_return_one_ret4` (0x5bb550, fra_first.cpp, thiscall, ret 4).
///
/// [pt] { 0x5bb550 } (thiscall, ret 4, fra_first.cpp, 8 bytes): `stub_return_one_ret4(a)`, o padrão do slot 25 das
/// competições e do slot 6 do órgão disciplinar: ignora o argumento e devolve 1.
pub(crate) unsafe extern "thiscall" fn toco_um_com_um(_this: usize, _a: u32) -> u32 {
    1
}

/// Os dois tocos: compara a devolução.
pub(crate) unsafe extern "thiscall" fn comparar_toco_um_com_dois(this: usize, a: u32, b: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(1885)(this, a, b);
    let n = toco_um_com_dois(this, a, b);
    PLACAR.registrar(r == n, || format!("toco_um_com_dois: original={} rust={}", r, n));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_toco_um_com_um(this: usize, a: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(1886)(this, a);
    let n = toco_um_com_um(this, a);
    PLACAR.registrar(r == n, || format!("toco_um_com_um: original={} rust={}", r, n));
    r
}

/// Os jogos fixos (dia, mês com 0 = janeiro, ano, o 5º argumento (casa, pelo mapa), o 6º (fora)), os nomes no .rdata do
/// exe; na ordem da original (extraídos da desmontagem e conferidos: os lugares do vetor vão de 16 em 16).
pub(crate) const CQN6_C1A: [(u8, u8, u16, u32, u32); 306] = [
    (2, 8, 2001, 0x9ea540, 0x9ea54c), // Albinoleffe × Carrarese
    (2, 8, 2001, 0x9ea530, 0x9ea538), // Alzano × Arezzo
    (2, 8, 2001, 0x9ea520, 0x9ea528), // Cesena × Livorno
    (2, 8, 2001, 0x9ea50c, 0x9ea518), // Lumezzane × Lecco
    (2, 8, 2001, 0x9ea4fc, 0x9ea504), // Pisa × Treviso
    (2, 8, 2001, 0x9ea4e4, 0x9ea4f0), // Reggiana × Triestina
    (2, 8, 2001, 0x9ea4d4, 0x9ea4dc), // Spal × Padova
    (2, 8, 2001, 0x9ea4c4, 0x9ea4cc), // Spezia × Monza
    (2, 8, 2001, 0x9ea4b0, 0x9ea4b8), // Varese × Lucchese
    (9, 8, 2001, 0x9ea538, 0x9ea4fc), // Arezzo × Pisa
    (9, 8, 2001, 0x9ea54c, 0x9ea4d4), // Carrarese × Spal
    (9, 8, 2001, 0x9ea518, 0x9ea540), // Lecco × Albinoleffe
    (9, 8, 2001, 0x9ea528, 0x9ea4b0), // Livorno × Varese
    (9, 8, 2001, 0x9ea4b8, 0x9ea50c), // Lucchese × Lumezzane
    (9, 8, 2001, 0x9ea4cc, 0x9ea520), // Monza × Cesena
    (9, 8, 2001, 0x9ea4dc, 0x9ea4e4), // Padova × Reggiana
    (9, 8, 2001, 0x9ea504, 0x9ea4c4), // Treviso × Spezia
    (9, 8, 2001, 0x9ea4f0, 0x9ea530), // Triestina × Alzano
    (16, 8, 2001, 0x9ea540, 0x9ea4f0), // Albinoleffe × Triestina
    (16, 8, 2001, 0x9ea530, 0x9ea4c4), // Alzano × Spezia
    (16, 8, 2001, 0x9ea538, 0x9ea528), // Arezzo × Livorno
    (16, 8, 2001, 0x9ea54c, 0x9ea4dc), // Carrarese × Padova
    (16, 8, 2001, 0x9ea520, 0x9ea4e4), // Cesena × Reggiana
    (16, 8, 2001, 0x9ea50c, 0x9ea4cc), // Lumezzane × Monza
    (16, 8, 2001, 0x9ea4fc, 0x9ea4b8), // Pisa × Lucchese
    (16, 8, 2001, 0x9ea4d4, 0x9ea518), // Spal × Lecco
    (16, 8, 2001, 0x9ea4b0, 0x9ea504), // Varese × Treviso
    (23, 8, 2001, 0x9ea518, 0x9ea520), // Lecco × Cesena
    (23, 8, 2001, 0x9ea528, 0x9ea4d4), // Livorno × Spal
    (23, 8, 2001, 0x9ea4b8, 0x9ea54c), // Lucchese × Carrarese
    (23, 8, 2001, 0x9ea4cc, 0x9ea4fc), // Monza × Pisa
    (23, 8, 2001, 0x9ea4dc, 0x9ea540), // Padova × Albinoleffe
    (23, 8, 2001, 0x9ea4e4, 0x9ea530), // Reggiana × Alzano
    (23, 8, 2001, 0x9ea4c4, 0x9ea4b0), // Spezia × Varese
    (23, 8, 2001, 0x9ea504, 0x9ea50c), // Treviso × Lumezzane
    (23, 8, 2001, 0x9ea4f0, 0x9ea538), // Triestina × Arezzo
    (30, 8, 2001, 0x9ea540, 0x9ea4e4), // Albinoleffe × Reggiana
    (30, 8, 2001, 0x9ea530, 0x9ea4cc), // Alzano × Monza
    (30, 8, 2001, 0x9ea538, 0x9ea4dc), // Arezzo × Padova
    (30, 8, 2001, 0x9ea54c, 0x9ea528), // Carrarese × Livorno
    (30, 8, 2001, 0x9ea520, 0x9ea504), // Cesena × Treviso
    (30, 8, 2001, 0x9ea518, 0x9ea4f0), // Lecco × Triestina
    (30, 8, 2001, 0x9ea4fc, 0x9ea4c4), // Pisa × Spezia
    (30, 8, 2001, 0x9ea4d4, 0x9ea4b8), // Spal × Lucchese
    (30, 8, 2001, 0x9ea4b0, 0x9ea50c), // Varese × Lumezzane
    (7, 9, 2001, 0x9ea530, 0x9ea4b0), // Alzano × Varese
    (7, 9, 2001, 0x9ea4b8, 0x9ea518), // Lucchese × Lecco
    (7, 9, 2001, 0x9ea50c, 0x9ea520), // Lumezzane × Cesena
    (7, 9, 2001, 0x9ea4cc, 0x9ea538), // Monza × Arezzo
    (7, 9, 2001, 0x9ea4dc, 0x9ea4f0), // Padova × Triestina
    (7, 9, 2001, 0x9ea4fc, 0x9ea540), // Pisa × Albinoleffe
    (7, 9, 2001, 0x9ea4e4, 0x9ea528), // Reggiana × Livorno
    (7, 9, 2001, 0x9ea4c4, 0x9ea54c), // Spezia × Carrarese
    (7, 9, 2001, 0x9ea504, 0x9ea4d4), // Treviso × Spal
    (14, 9, 2001, 0x9ea540, 0x9ea4b8), // Albinoleffe × Lucchese
    (14, 9, 2001, 0x9ea538, 0x9ea4b0), // Arezzo × Varese
    (14, 9, 2001, 0x9ea54c, 0x9ea4e4), // Carrarese × Reggiana
    (14, 9, 2001, 0x9ea520, 0x9ea4c4), // Cesena × Spezia
    (14, 9, 2001, 0x9ea518, 0x9ea530), // Lecco × Alzano
    (14, 9, 2001, 0x9ea528, 0x9ea4fc), // Livorno × Pisa
    (14, 9, 2001, 0x9ea4dc, 0x9ea504), // Padova × Treviso
    (14, 9, 2001, 0x9ea4d4, 0x9ea4cc), // Spal × Monza
    (14, 9, 2001, 0x9ea4f0, 0x9ea50c), // Triestina × Lumezzane
    (21, 9, 2001, 0x9ea520, 0x9ea538), // Cesena × Arezzo
    (21, 9, 2001, 0x9ea50c, 0x9ea540), // Lumezzane × Albinoleffe
    (21, 9, 2001, 0x9ea4cc, 0x9ea4dc), // Monza × Padova
    (21, 9, 2001, 0x9ea4fc, 0x9ea530), // Pisa × Alzano
    (21, 9, 2001, 0x9ea4e4, 0x9ea4b8), // Reggiana × Lucchese
    (21, 9, 2001, 0x9ea4c4, 0x9ea518), // Spezia × Lecco
    (21, 9, 2001, 0x9ea504, 0x9ea54c), // Treviso × Carrarese
    (21, 9, 2001, 0x9ea4f0, 0x9ea528), // Triestina × Livorno
    (21, 9, 2001, 0x9ea4b0, 0x9ea4d4), // Varese × Spal
    (28, 9, 2001, 0x9ea540, 0x9ea504), // Albinoleffe × Treviso
    (28, 9, 2001, 0x9ea530, 0x9ea520), // Alzano × Cesena
    (28, 9, 2001, 0x9ea538, 0x9ea50c), // Arezzo × Lumezzane
    (28, 9, 2001, 0x9ea54c, 0x9ea4cc), // Carrarese × Monza
    (28, 9, 2001, 0x9ea518, 0x9ea4e4), // Lecco × Reggiana
    (28, 9, 2001, 0x9ea528, 0x9ea4dc), // Livorno × Padova
    (28, 9, 2001, 0x9ea4b8, 0x9ea4f0), // Lucchese × Triestina
    (28, 9, 2001, 0x9ea4d4, 0x9ea4c4), // Spal × Spezia
    (28, 9, 2001, 0x9ea4b0, 0x9ea4fc), // Varese × Pisa
    (4, 10, 2001, 0x9ea520, 0x9ea4b0), // Cesena × Varese
    (4, 10, 2001, 0x9ea528, 0x9ea4b8), // Livorno × Lucchese
    (4, 10, 2001, 0x9ea50c, 0x9ea4fc), // Lumezzane × Pisa
    (4, 10, 2001, 0x9ea4cc, 0x9ea540), // Monza × Albinoleffe
    (4, 10, 2001, 0x9ea4dc, 0x9ea518), // Padova × Lecco
    (4, 10, 2001, 0x9ea4e4, 0x9ea4d4), // Reggiana × Spal
    (4, 10, 2001, 0x9ea4c4, 0x9ea538), // Spezia × Arezzo
    (4, 10, 2001, 0x9ea504, 0x9ea530), // Treviso × Alzano
    (4, 10, 2001, 0x9ea4f0, 0x9ea54c), // Triestina × Carrarese
    (11, 10, 2001, 0x9ea540, 0x9ea528), // Albinoleffe × Livorno
    (11, 10, 2001, 0x9ea530, 0x9ea50c), // Alzano × Lumezzane
    (11, 10, 2001, 0x9ea538, 0x9ea504), // Arezzo × Treviso
    (11, 10, 2001, 0x9ea518, 0x9ea54c), // Lecco × Carrarese
    (11, 10, 2001, 0x9ea4b8, 0x9ea4dc), // Lucchese × Padova
    (11, 10, 2001, 0x9ea4fc, 0x9ea520), // Pisa × Cesena
    (11, 10, 2001, 0x9ea4d4, 0x9ea4f0), // Spal × Triestina
    (11, 10, 2001, 0x9ea4c4, 0x9ea4e4), // Spezia × Reggiana
    (11, 10, 2001, 0x9ea4b0, 0x9ea4cc), // Varese × Monza
    (18, 10, 2001, 0x9ea540, 0x9ea4d4), // Albinoleffe × Spal
    (18, 10, 2001, 0x9ea54c, 0x9ea4fc), // Carrarese × Pisa
    (18, 10, 2001, 0x9ea528, 0x9ea518), // Livorno × Lecco
    (18, 10, 2001, 0x9ea4b8, 0x9ea538), // Lucchese × Arezzo
    (18, 10, 2001, 0x9ea50c, 0x9ea4c4), // Lumezzane × Spezia
    (18, 10, 2001, 0x9ea4dc, 0x9ea530), // Padova × Alzano
    (18, 10, 2001, 0x9ea4e4, 0x9ea4b0), // Reggiana × Varese
    (18, 10, 2001, 0x9ea504, 0x9ea4cc), // Treviso × Monza
    (18, 10, 2001, 0x9ea4f0, 0x9ea520), // Triestina × Cesena
    (25, 10, 2001, 0x9ea530, 0x9ea540), // Alzano × Albinoleffe
    (25, 10, 2001, 0x9ea538, 0x9ea4e4), // Arezzo × Reggiana
    (25, 10, 2001, 0x9ea520, 0x9ea4dc), // Cesena × Padova
    (25, 10, 2001, 0x9ea50c, 0x9ea54c), // Lumezzane × Carrarese
    (25, 10, 2001, 0x9ea4cc, 0x9ea4b8), // Monza × Lucchese
    (25, 10, 2001, 0x9ea4fc, 0x9ea4d4), // Pisa × Spal
    (25, 10, 2001, 0x9ea4c4, 0x9ea528), // Spezia × Livorno
    (25, 10, 2001, 0x9ea504, 0x9ea4f0), // Treviso × Triestina
    (25, 10, 2001, 0x9ea4b0, 0x9ea518), // Varese × Lecco
    (2, 11, 2001, 0x9ea540, 0x9ea4b0), // Albinoleffe × Varese
    (2, 11, 2001, 0x9ea54c, 0x9ea530), // Carrarese × Alzano
    (2, 11, 2001, 0x9ea518, 0x9ea538), // Lecco × Arezzo
    (2, 11, 2001, 0x9ea528, 0x9ea4cc), // Livorno × Monza
    (2, 11, 2001, 0x9ea4b8, 0x9ea504), // Lucchese × Treviso
    (2, 11, 2001, 0x9ea4dc, 0x9ea4fc), // Padova × Pisa
    (2, 11, 2001, 0x9ea4e4, 0x9ea50c), // Reggiana × Lumezzane
    (2, 11, 2001, 0x9ea4d4, 0x9ea520), // Spal × Cesena
    (2, 11, 2001, 0x9ea4f0, 0x9ea4c4), // Triestina × Spezia
    (9, 11, 2001, 0x9ea530, 0x9ea4d4), // Alzano × Spal
    (9, 11, 2001, 0x9ea538, 0x9ea540), // Arezzo × Albinoleffe
    (9, 11, 2001, 0x9ea520, 0x9ea54c), // Cesena × Carrarese
    (9, 11, 2001, 0x9ea50c, 0x9ea528), // Lumezzane × Livorno
    (9, 11, 2001, 0x9ea4cc, 0x9ea518), // Monza × Lecco
    (9, 11, 2001, 0x9ea4fc, 0x9ea4f0), // Pisa × Triestina
    (9, 11, 2001, 0x9ea4c4, 0x9ea4b8), // Spezia × Lucchese
    (9, 11, 2001, 0x9ea504, 0x9ea4e4), // Treviso × Reggiana
    (9, 11, 2001, 0x9ea4b0, 0x9ea4dc), // Varese × Padova
    (16, 11, 2001, 0x9ea540, 0x9ea520), // Albinoleffe × Cesena
    (16, 11, 2001, 0x9ea54c, 0x9ea538), // Carrarese × Arezzo
    (16, 11, 2001, 0x9ea518, 0x9ea4fc), // Lecco × Pisa
    (16, 11, 2001, 0x9ea528, 0x9ea504), // Livorno × Treviso
    (16, 11, 2001, 0x9ea4b8, 0x9ea530), // Lucchese × Alzano
    (16, 11, 2001, 0x9ea4dc, 0x9ea4c4), // Padova × Spezia
    (16, 11, 2001, 0x9ea4e4, 0x9ea4cc), // Reggiana × Monza
    (16, 11, 2001, 0x9ea4d4, 0x9ea50c), // Spal × Lumezzane
    (16, 11, 2001, 0x9ea4f0, 0x9ea4b0), // Triestina × Varese
    (23, 11, 2001, 0x9ea530, 0x9ea528), // Alzano × Livorno
    (23, 11, 2001, 0x9ea538, 0x9ea4d4), // Arezzo × Spal
    (23, 11, 2001, 0x9ea520, 0x9ea4b8), // Cesena × Lucchese
    (23, 11, 2001, 0x9ea50c, 0x9ea4dc), // Lumezzane × Padova
    (23, 11, 2001, 0x9ea4cc, 0x9ea4f0), // Monza × Triestina
    (23, 11, 2001, 0x9ea4fc, 0x9ea4e4), // Pisa × Reggiana
    (23, 11, 2001, 0x9ea4c4, 0x9ea540), // Spezia × Albinoleffe
    (23, 11, 2001, 0x9ea504, 0x9ea518), // Treviso × Lecco
    (23, 11, 2001, 0x9ea4b0, 0x9ea54c), // Varese × Carrarese
    (6, 0, 2002, 0x9ea54c, 0x9ea540), // Carrarese × Albinoleffe
    (6, 0, 2002, 0x9ea538, 0x9ea530), // Arezzo × Alzano
    (6, 0, 2002, 0x9ea528, 0x9ea520), // Livorno × Cesena
    (6, 0, 2002, 0x9ea518, 0x9ea50c), // Lecco × Lumezzane
    (6, 0, 2002, 0x9ea504, 0x9ea4fc), // Treviso × Pisa
    (6, 0, 2002, 0x9ea4f0, 0x9ea4e4), // Triestina × Reggiana
    (6, 0, 2002, 0x9ea4dc, 0x9ea4d4), // Padova × Spal
    (6, 0, 2002, 0x9ea4cc, 0x9ea4c4), // Monza × Spezia
    (6, 0, 2002, 0x9ea4b8, 0x9ea4b0), // Lucchese × Varese
    (13, 0, 2002, 0x9ea4fc, 0x9ea538), // Pisa × Arezzo
    (13, 0, 2002, 0x9ea4d4, 0x9ea54c), // Spal × Carrarese
    (13, 0, 2002, 0x9ea540, 0x9ea518), // Albinoleffe × Lecco
    (13, 0, 2002, 0x9ea4b0, 0x9ea528), // Varese × Livorno
    (13, 0, 2002, 0x9ea50c, 0x9ea4b8), // Lumezzane × Lucchese
    (13, 0, 2002, 0x9ea520, 0x9ea4cc), // Cesena × Monza
    (13, 0, 2002, 0x9ea4e4, 0x9ea4dc), // Reggiana × Padova
    (13, 0, 2002, 0x9ea4c4, 0x9ea504), // Spezia × Treviso
    (13, 0, 2002, 0x9ea530, 0x9ea4f0), // Alzano × Triestina
    (20, 0, 2002, 0x9ea4f0, 0x9ea540), // Triestina × Albinoleffe
    (20, 0, 2002, 0x9ea4c4, 0x9ea530), // Spezia × Alzano
    (20, 0, 2002, 0x9ea528, 0x9ea538), // Livorno × Arezzo
    (20, 0, 2002, 0x9ea4dc, 0x9ea54c), // Padova × Carrarese
    (20, 0, 2002, 0x9ea4e4, 0x9ea520), // Reggiana × Cesena
    (20, 0, 2002, 0x9ea4cc, 0x9ea50c), // Monza × Lumezzane
    (20, 0, 2002, 0x9ea4b8, 0x9ea4fc), // Lucchese × Pisa
    (20, 0, 2002, 0x9ea518, 0x9ea4d4), // Lecco × Spal
    (20, 0, 2002, 0x9ea504, 0x9ea4b0), // Treviso × Varese
    (27, 0, 2002, 0x9ea520, 0x9ea518), // Cesena × Lecco
    (27, 0, 2002, 0x9ea4d4, 0x9ea528), // Spal × Livorno
    (27, 0, 2002, 0x9ea54c, 0x9ea4b8), // Carrarese × Lucchese
    (27, 0, 2002, 0x9ea4fc, 0x9ea4cc), // Pisa × Monza
    (27, 0, 2002, 0x9ea540, 0x9ea4dc), // Albinoleffe × Padova
    (27, 0, 2002, 0x9ea530, 0x9ea4e4), // Alzano × Reggiana
    (27, 0, 2002, 0x9ea4b0, 0x9ea4c4), // Varese × Spezia
    (27, 0, 2002, 0x9ea50c, 0x9ea504), // Lumezzane × Treviso
    (27, 0, 2002, 0x9ea538, 0x9ea4f0), // Arezzo × Triestina
    (3, 1, 2002, 0x9ea4e4, 0x9ea540), // Reggiana × Albinoleffe
    (3, 1, 2002, 0x9ea4cc, 0x9ea530), // Monza × Alzano
    (3, 1, 2002, 0x9ea4dc, 0x9ea538), // Padova × Arezzo
    (3, 1, 2002, 0x9ea528, 0x9ea54c), // Livorno × Carrarese
    (3, 1, 2002, 0x9ea504, 0x9ea520), // Treviso × Cesena
    (3, 1, 2002, 0x9ea4f0, 0x9ea518), // Triestina × Lecco
    (3, 1, 2002, 0x9ea4c4, 0x9ea4fc), // Spezia × Pisa
    (3, 1, 2002, 0x9ea4b8, 0x9ea4d4), // Lucchese × Spal
    (3, 1, 2002, 0x9ea50c, 0x9ea4b0), // Lumezzane × Varese
    (10, 1, 2002, 0x9ea4b0, 0x9ea530), // Varese × Alzano
    (10, 1, 2002, 0x9ea518, 0x9ea4b8), // Lecco × Lucchese
    (10, 1, 2002, 0x9ea520, 0x9ea50c), // Cesena × Lumezzane
    (10, 1, 2002, 0x9ea538, 0x9ea4cc), // Arezzo × Monza
    (10, 1, 2002, 0x9ea4f0, 0x9ea4dc), // Triestina × Padova
    (10, 1, 2002, 0x9ea540, 0x9ea4fc), // Albinoleffe × Pisa
    (10, 1, 2002, 0x9ea528, 0x9ea4e4), // Livorno × Reggiana
    (10, 1, 2002, 0x9ea54c, 0x9ea4c4), // Carrarese × Spezia
    (10, 1, 2002, 0x9ea4d4, 0x9ea504), // Spal × Treviso
    (17, 1, 2002, 0x9ea4b8, 0x9ea540), // Lucchese × Albinoleffe
    (17, 1, 2002, 0x9ea4b0, 0x9ea538), // Varese × Arezzo
    (17, 1, 2002, 0x9ea4e4, 0x9ea54c), // Reggiana × Carrarese
    (17, 1, 2002, 0x9ea4c4, 0x9ea520), // Spezia × Cesena
    (17, 1, 2002, 0x9ea530, 0x9ea518), // Alzano × Lecco
    (17, 1, 2002, 0x9ea4fc, 0x9ea528), // Pisa × Livorno
    (17, 1, 2002, 0x9ea504, 0x9ea4dc), // Treviso × Padova
    (17, 1, 2002, 0x9ea4cc, 0x9ea4d4), // Monza × Spal
    (17, 1, 2002, 0x9ea50c, 0x9ea4f0), // Lumezzane × Triestina
    (3, 2, 2002, 0x9ea538, 0x9ea520), // Arezzo × Cesena
    (3, 2, 2002, 0x9ea540, 0x9ea50c), // Albinoleffe × Lumezzane
    (3, 2, 2002, 0x9ea4dc, 0x9ea4cc), // Padova × Monza
    (3, 2, 2002, 0x9ea530, 0x9ea4fc), // Alzano × Pisa
    (3, 2, 2002, 0x9ea4b8, 0x9ea4e4), // Lucchese × Reggiana
    (3, 2, 2002, 0x9ea518, 0x9ea4c4), // Lecco × Spezia
    (3, 2, 2002, 0x9ea54c, 0x9ea504), // Carrarese × Treviso
    (3, 2, 2002, 0x9ea528, 0x9ea4f0), // Livorno × Triestina
    (3, 2, 2002, 0x9ea4d4, 0x9ea4b0), // Spal × Varese
    (10, 2, 2002, 0x9ea504, 0x9ea540), // Treviso × Albinoleffe
    (10, 2, 2002, 0x9ea520, 0x9ea530), // Cesena × Alzano
    (10, 2, 2002, 0x9ea50c, 0x9ea538), // Lumezzane × Arezzo
    (10, 2, 2002, 0x9ea4cc, 0x9ea54c), // Monza × Carrarese
    (10, 2, 2002, 0x9ea4e4, 0x9ea518), // Reggiana × Lecco
    (10, 2, 2002, 0x9ea4dc, 0x9ea528), // Padova × Livorno
    (10, 2, 2002, 0x9ea4f0, 0x9ea4b8), // Triestina × Lucchese
    (10, 2, 2002, 0x9ea4c4, 0x9ea4d4), // Spezia × Spal
    (10, 2, 2002, 0x9ea4fc, 0x9ea4b0), // Pisa × Varese
    (17, 2, 2002, 0x9ea4b0, 0x9ea520), // Varese × Cesena
    (17, 2, 2002, 0x9ea4b8, 0x9ea528), // Lucchese × Livorno
    (17, 2, 2002, 0x9ea4fc, 0x9ea50c), // Pisa × Lumezzane
    (17, 2, 2002, 0x9ea540, 0x9ea4cc), // Albinoleffe × Monza
    (17, 2, 2002, 0x9ea518, 0x9ea4dc), // Lecco × Padova
    (17, 2, 2002, 0x9ea4d4, 0x9ea4e4), // Spal × Reggiana
    (17, 2, 2002, 0x9ea538, 0x9ea4c4), // Arezzo × Spezia
    (17, 2, 2002, 0x9ea530, 0x9ea504), // Alzano × Treviso
    (17, 2, 2002, 0x9ea54c, 0x9ea4f0), // Carrarese × Triestina
    (24, 2, 2002, 0x9ea528, 0x9ea540), // Livorno × Albinoleffe
    (24, 2, 2002, 0x9ea50c, 0x9ea530), // Lumezzane × Alzano
    (24, 2, 2002, 0x9ea504, 0x9ea538), // Treviso × Arezzo
    (24, 2, 2002, 0x9ea54c, 0x9ea518), // Carrarese × Lecco
    (24, 2, 2002, 0x9ea4dc, 0x9ea4b8), // Padova × Lucchese
    (24, 2, 2002, 0x9ea520, 0x9ea4fc), // Cesena × Pisa
    (24, 2, 2002, 0x9ea4f0, 0x9ea4d4), // Triestina × Spal
    (24, 2, 2002, 0x9ea4e4, 0x9ea4c4), // Reggiana × Spezia
    (24, 2, 2002, 0x9ea4cc, 0x9ea4b0), // Monza × Varese
    (30, 2, 2002, 0x9ea4d4, 0x9ea540), // Spal × Albinoleffe
    (30, 2, 2002, 0x9ea4fc, 0x9ea54c), // Pisa × Carrarese
    (30, 2, 2002, 0x9ea518, 0x9ea528), // Lecco × Livorno
    (30, 2, 2002, 0x9ea538, 0x9ea4b8), // Arezzo × Lucchese
    (30, 2, 2002, 0x9ea4c4, 0x9ea50c), // Spezia × Lumezzane
    (30, 2, 2002, 0x9ea530, 0x9ea4dc), // Alzano × Padova
    (30, 2, 2002, 0x9ea4b0, 0x9ea4e4), // Varese × Reggiana
    (30, 2, 2002, 0x9ea4cc, 0x9ea504), // Monza × Treviso
    (30, 2, 2002, 0x9ea520, 0x9ea4f0), // Cesena × Triestina
    (7, 3, 2002, 0x9ea540, 0x9ea530), // Albinoleffe × Alzano
    (7, 3, 2002, 0x9ea4e4, 0x9ea538), // Reggiana × Arezzo
    (7, 3, 2002, 0x9ea4dc, 0x9ea520), // Padova × Cesena
    (7, 3, 2002, 0x9ea54c, 0x9ea50c), // Carrarese × Lumezzane
    (7, 3, 2002, 0x9ea4b8, 0x9ea4cc), // Lucchese × Monza
    (7, 3, 2002, 0x9ea4d4, 0x9ea4fc), // Spal × Pisa
    (7, 3, 2002, 0x9ea528, 0x9ea4c4), // Livorno × Spezia
    (7, 3, 2002, 0x9ea4f0, 0x9ea504), // Triestina × Treviso
    (7, 3, 2002, 0x9ea518, 0x9ea4b0), // Lecco × Varese
    (14, 3, 2002, 0x9ea4b0, 0x9ea540), // Varese × Albinoleffe
    (14, 3, 2002, 0x9ea530, 0x9ea54c), // Alzano × Carrarese
    (14, 3, 2002, 0x9ea538, 0x9ea518), // Arezzo × Lecco
    (14, 3, 2002, 0x9ea4cc, 0x9ea528), // Monza × Livorno
    (14, 3, 2002, 0x9ea504, 0x9ea4b8), // Treviso × Lucchese
    (14, 3, 2002, 0x9ea4fc, 0x9ea4dc), // Pisa × Padova
    (14, 3, 2002, 0x9ea50c, 0x9ea4e4), // Lumezzane × Reggiana
    (14, 3, 2002, 0x9ea520, 0x9ea4d4), // Cesena × Spal
    (14, 3, 2002, 0x9ea4c4, 0x9ea4f0), // Spezia × Triestina
    (21, 3, 2002, 0x9ea4d4, 0x9ea530), // Spal × Alzano
    (21, 3, 2002, 0x9ea540, 0x9ea538), // Albinoleffe × Arezzo
    (21, 3, 2002, 0x9ea54c, 0x9ea520), // Carrarese × Cesena
    (21, 3, 2002, 0x9ea528, 0x9ea50c), // Livorno × Lumezzane
    (21, 3, 2002, 0x9ea518, 0x9ea4cc), // Lecco × Monza
    (21, 3, 2002, 0x9ea4f0, 0x9ea4fc), // Triestina × Pisa
    (21, 3, 2002, 0x9ea4b8, 0x9ea4c4), // Lucchese × Spezia
    (21, 3, 2002, 0x9ea4e4, 0x9ea504), // Reggiana × Treviso
    (21, 3, 2002, 0x9ea4dc, 0x9ea4b0), // Padova × Varese
    (28, 3, 2002, 0x9ea520, 0x9ea540), // Cesena × Albinoleffe
    (28, 3, 2002, 0x9ea538, 0x9ea54c), // Arezzo × Carrarese
    (28, 3, 2002, 0x9ea4fc, 0x9ea518), // Pisa × Lecco
    (28, 3, 2002, 0x9ea504, 0x9ea528), // Treviso × Livorno
    (28, 3, 2002, 0x9ea530, 0x9ea4b8), // Alzano × Lucchese
    (28, 3, 2002, 0x9ea4c4, 0x9ea4dc), // Spezia × Padova
    (28, 3, 2002, 0x9ea4cc, 0x9ea4e4), // Monza × Reggiana
    (28, 3, 2002, 0x9ea50c, 0x9ea4d4), // Lumezzane × Spal
    (28, 3, 2002, 0x9ea4b0, 0x9ea4f0), // Varese × Triestina
    (5, 4, 2002, 0x9ea528, 0x9ea530), // Livorno × Alzano
    (5, 4, 2002, 0x9ea4d4, 0x9ea538), // Spal × Arezzo
    (5, 4, 2002, 0x9ea4b8, 0x9ea520), // Lucchese × Cesena
    (5, 4, 2002, 0x9ea4dc, 0x9ea50c), // Padova × Lumezzane
    (5, 4, 2002, 0x9ea4f0, 0x9ea4cc), // Triestina × Monza
    (5, 4, 2002, 0x9ea4e4, 0x9ea4fc), // Reggiana × Pisa
    (5, 4, 2002, 0x9ea540, 0x9ea4c4), // Albinoleffe × Spezia
    (5, 4, 2002, 0x9ea518, 0x9ea504), // Lecco × Treviso
    (5, 4, 2002, 0x9ea54c, 0x9ea4b0), // Carrarese × Varese
];

pub(crate) const CQN6_C1B: [(u8, u8, u16, u32, u32); 306] = [
    (2, 8, 2001, 0x9ea638, 0x9ea640), // Ascoli × Castel di Sangro
    (2, 8, 2001, 0x9ea620, 0x9ea62c), // Avellino × L'Aquila
    (2, 8, 2001, 0x9ea60c, 0x9ea614), // Catania × Benevento
    (2, 8, 2001, 0x9ea5f8, 0x9ea604), // Giulianova × Chieti
    (2, 8, 2001, 0x9ea5e0, 0x9ea5ec), // Lodigiani × Lanciano
    (2, 8, 2001, 0x9ea5d0, 0x9ea5d8), // Pescara × Fermana
    (2, 8, 2001, 0x9ea5bc, 0x9ea5c4), // Sora × Nocerina
    (2, 8, 2001, 0x9ea5a8, 0x9ea5b0), // Taranto × Vis Pesaro
    (2, 8, 2001, 0x9ea594, 0x9ea5a0), // Viterbese × Torres
    (9, 8, 2001, 0x9ea614, 0x9ea594), // Benevento × Viterbese
    (9, 8, 2001, 0x9ea640, 0x9ea5f8), // Castel di Sangro × Giulianova
    (9, 8, 2001, 0x9ea604, 0x9ea5d0), // Chieti × Pescara
    (9, 8, 2001, 0x9ea5d8, 0x9ea5e0), // Fermana × Lodigiani
    (9, 8, 2001, 0x9ea62c, 0x9ea5bc), // L'Aquila × Sora
    (9, 8, 2001, 0x9ea5ec, 0x9ea5a8), // Lanciano × Taranto
    (9, 8, 2001, 0x9ea5c4, 0x9ea620), // Nocerina × Avellino
    (9, 8, 2001, 0x9ea5a0, 0x9ea60c), // Torres × Catania
    (9, 8, 2001, 0x9ea5b0, 0x9ea638), // Vis Pesaro × Ascoli
    (16, 8, 2001, 0x9ea638, 0x9ea604), // Ascoli × Chieti
    (16, 8, 2001, 0x9ea620, 0x9ea5a0), // Avellino × Torres
    (16, 8, 2001, 0x9ea60c, 0x9ea5ec), // Catania × Lanciano
    (16, 8, 2001, 0x9ea5f8, 0x9ea594), // Giulianova × Viterbese
    (16, 8, 2001, 0x9ea5e0, 0x9ea614), // Lodigiani × Benevento
    (16, 8, 2001, 0x9ea5d0, 0x9ea62c), // Pescara × L'Aquila
    (16, 8, 2001, 0x9ea5bc, 0x9ea5d8), // Sora × Fermana
    (16, 8, 2001, 0x9ea5a8, 0x9ea640), // Taranto × Castel di Sangro
    (16, 8, 2001, 0x9ea5b0, 0x9ea5c4), // Vis Pesaro × Nocerina
    (23, 8, 2001, 0x9ea614, 0x9ea5d0), // Benevento × Pescara
    (23, 8, 2001, 0x9ea640, 0x9ea620), // Castel di Sangro × Avellino
    (23, 8, 2001, 0x9ea604, 0x9ea5a8), // Chieti × Taranto
    (23, 8, 2001, 0x9ea5d8, 0x9ea60c), // Fermana × Catania
    (23, 8, 2001, 0x9ea62c, 0x9ea5f8), // L'Aquila × Giulianova
    (23, 8, 2001, 0x9ea5ec, 0x9ea638), // Lanciano × Ascoli
    (23, 8, 2001, 0x9ea5c4, 0x9ea5e0), // Nocerina × Lodigiani
    (23, 8, 2001, 0x9ea5a0, 0x9ea5b0), // Torres × Vis Pesaro
    (23, 8, 2001, 0x9ea594, 0x9ea5bc), // Viterbese × Sora
    (30, 8, 2001, 0x9ea638, 0x9ea5a0), // Ascoli × Torres
    (30, 8, 2001, 0x9ea620, 0x9ea5ec), // Avellino × Lanciano
    (30, 8, 2001, 0x9ea640, 0x9ea604), // Castel di Sangro × Chieti
    (30, 8, 2001, 0x9ea5f8, 0x9ea5c4), // Giulianova × Nocerina
    (30, 8, 2001, 0x9ea5e0, 0x9ea594), // Lodigiani × Viterbese
    (30, 8, 2001, 0x9ea5d0, 0x9ea60c), // Pescara × Catania
    (30, 8, 2001, 0x9ea5bc, 0x9ea614), // Sora × Benevento
    (30, 8, 2001, 0x9ea5a8, 0x9ea5d8), // Taranto × Fermana
    (30, 8, 2001, 0x9ea5b0, 0x9ea62c), // Vis Pesaro × L'Aquila
    (7, 9, 2001, 0x9ea620, 0x9ea5bc), // Avellino × Sora
    (7, 9, 2001, 0x9ea614, 0x9ea5f8), // Benevento × Giulianova
    (7, 9, 2001, 0x9ea60c, 0x9ea5e0), // Catania × Lodigiani
    (7, 9, 2001, 0x9ea604, 0x9ea5c4), // Chieti × Nocerina
    (7, 9, 2001, 0x9ea5d8, 0x9ea5b0), // Fermana × Vis Pesaro
    (7, 9, 2001, 0x9ea62c, 0x9ea638), // L'Aquila × Ascoli
    (7, 9, 2001, 0x9ea5ec, 0x9ea640), // Lanciano × Castel di Sangro
    (7, 9, 2001, 0x9ea5a0, 0x9ea5a8), // Torres × Taranto
    (7, 9, 2001, 0x9ea594, 0x9ea5d0), // Viterbese × Pescara
    (14, 9, 2001, 0x9ea638, 0x9ea614), // Ascoli × Benevento
    (14, 9, 2001, 0x9ea640, 0x9ea5a0), // Castel di Sangro × Torres
    (14, 9, 2001, 0x9ea5f8, 0x9ea5d8), // Giulianova × Fermana
    (14, 9, 2001, 0x9ea5ec, 0x9ea604), // Lanciano × Chieti
    (14, 9, 2001, 0x9ea5e0, 0x9ea5d0), // Lodigiani × Pescara
    (14, 9, 2001, 0x9ea5c4, 0x9ea594), // Nocerina × Viterbese
    (14, 9, 2001, 0x9ea5bc, 0x9ea60c), // Sora × Catania
    (14, 9, 2001, 0x9ea5a8, 0x9ea62c), // Taranto × L'Aquila
    (14, 9, 2001, 0x9ea5b0, 0x9ea620), // Vis Pesaro × Avellino
    (21, 9, 2001, 0x9ea620, 0x9ea604), // Avellino × Chieti
    (21, 9, 2001, 0x9ea614, 0x9ea5c4), // Benevento × Nocerina
    (21, 9, 2001, 0x9ea60c, 0x9ea5b0), // Catania × Vis Pesaro
    (21, 9, 2001, 0x9ea5d8, 0x9ea640), // Fermana × Castel di Sangro
    (21, 9, 2001, 0x9ea62c, 0x9ea5e0), // L'Aquila × Lodigiani
    (21, 9, 2001, 0x9ea5d0, 0x9ea5f8), // Pescara × Giulianova
    (21, 9, 2001, 0x9ea5a0, 0x9ea5ec), // Torres × Lanciano
    (21, 9, 2001, 0x9ea5bc, 0x9ea638), // Sora × Ascoli
    (21, 9, 2001, 0x9ea594, 0x9ea5a8), // Viterbese × Taranto
    (28, 9, 2001, 0x9ea638, 0x9ea620), // Ascoli × Avellino
    (28, 9, 2001, 0x9ea640, 0x9ea5e0), // Castel di Sangro × Lodigiani
    (28, 9, 2001, 0x9ea604, 0x9ea5b0), // Chieti × Vis Pesaro
    (28, 9, 2001, 0x9ea5f8, 0x9ea5bc), // Giulianova × Sora
    (28, 9, 2001, 0x9ea5ec, 0x9ea62c), // Lanciano × L'Aquila
    (28, 9, 2001, 0x9ea5c4, 0x9ea5d0), // Nocerina × Pescara
    (28, 9, 2001, 0x9ea5a0, 0x9ea614), // Torres × Benevento
    (28, 9, 2001, 0x9ea5a8, 0x9ea60c), // Taranto × Catania
    (28, 9, 2001, 0x9ea594, 0x9ea5d8), // Viterbese × Fermana
    (4, 10, 2001, 0x9ea620, 0x9ea5a8), // Avellino × Taranto
    (4, 10, 2001, 0x9ea614, 0x9ea640), // Benevento × Castel di Sangro
    (4, 10, 2001, 0x9ea60c, 0x9ea594), // Catania × Viterbese
    (4, 10, 2001, 0x9ea5d8, 0x9ea5c4), // Fermana × Nocerina
    (4, 10, 2001, 0x9ea62c, 0x9ea604), // L'Aquila × Chieti
    (4, 10, 2001, 0x9ea5e0, 0x9ea5f8), // Lodigiani × Giulianova
    (4, 10, 2001, 0x9ea5d0, 0x9ea638), // Pescara × Ascoli
    (4, 10, 2001, 0x9ea5bc, 0x9ea5a0), // Sora × Torres
    (4, 10, 2001, 0x9ea5b0, 0x9ea5ec), // Vis Pesaro × Lanciano
    (11, 10, 2001, 0x9ea638, 0x9ea5a8), // Ascoli × Taranto
    (11, 10, 2001, 0x9ea614, 0x9ea5d8), // Benevento × Fermana
    (11, 10, 2001, 0x9ea640, 0x9ea5b0), // Castel di Sangro × Vis Pesaro
    (11, 10, 2001, 0x9ea604, 0x9ea5e0), // Chieti × Lodigiani
    (11, 10, 2001, 0x9ea5f8, 0x9ea60c), // Giulianova × Catania
    (11, 10, 2001, 0x9ea5c4, 0x9ea5ec), // Nocerina × Lanciano
    (11, 10, 2001, 0x9ea5d0, 0x9ea5bc), // Pescara × Sora
    (11, 10, 2001, 0x9ea5a0, 0x9ea62c), // Torres × L'Aquila
    (11, 10, 2001, 0x9ea594, 0x9ea620), // Viterbese × Avellino
    (18, 10, 2001, 0x9ea620, 0x9ea614), // Avellino × Benevento
    (18, 10, 2001, 0x9ea60c, 0x9ea5c4), // Catania × Nocerina
    (18, 10, 2001, 0x9ea604, 0x9ea5a0), // Chieti × Torres
    (18, 10, 2001, 0x9ea5d8, 0x9ea638), // Fermana × Ascoli
    (18, 10, 2001, 0x9ea62c, 0x9ea640), // L'Aquila × Castel di Sangro
    (18, 10, 2001, 0x9ea5ec, 0x9ea5d0), // Lanciano × Pescara
    (18, 10, 2001, 0x9ea5e0, 0x9ea5bc), // Lodigiani × Sora
    (18, 10, 2001, 0x9ea5a8, 0x9ea5f8), // Taranto × Giulianova
    (18, 10, 2001, 0x9ea5b0, 0x9ea594), // Vis Pesaro × Viterbese
    (25, 10, 2001, 0x9ea614, 0x9ea5a8), // Benevento × Taranto
    (25, 10, 2001, 0x9ea60c, 0x9ea638), // Catania × Ascoli
    (25, 10, 2001, 0x9ea5d8, 0x9ea604), // Fermana × Chieti
    (25, 10, 2001, 0x9ea5f8, 0x9ea5a0), // Giulianova × Torres
    (25, 10, 2001, 0x9ea5e0, 0x9ea5b0), // Lodigiani × Vis Pesaro
    (25, 10, 2001, 0x9ea5c4, 0x9ea62c), // Nocerina × L'Aquila
    (25, 10, 2001, 0x9ea5d0, 0x9ea620), // Pescara × Avellino
    (25, 10, 2001, 0x9ea5bc, 0x9ea5ec), // Sora × Lanciano
    (25, 10, 2001, 0x9ea594, 0x9ea640), // Viterbese × Castel di Sangro
    (2, 11, 2001, 0x9ea638, 0x9ea5c4), // Ascoli × Nocerina
    (2, 11, 2001, 0x9ea620, 0x9ea5d8), // Avellino × Fermana
    (2, 11, 2001, 0x9ea640, 0x9ea5bc), // Castel di Sangro × Sora
    (2, 11, 2001, 0x9ea604, 0x9ea60c), // Chieti × Catania
    (2, 11, 2001, 0x9ea62c, 0x9ea594), // L'Aquila × Viterbese
    (2, 11, 2001, 0x9ea5ec, 0x9ea614), // Lanciano × Benevento
    (2, 11, 2001, 0x9ea5a0, 0x9ea5e0), // Torres × Lodigiani
    (2, 11, 2001, 0x9ea5a8, 0x9ea5d0), // Taranto × Pescara
    (2, 11, 2001, 0x9ea5b0, 0x9ea5f8), // Vis Pesaro × Giulianova
    (9, 11, 2001, 0x9ea614, 0x9ea5b0), // Benevento × Vis Pesaro
    (9, 11, 2001, 0x9ea60c, 0x9ea62c), // Catania × L'Aquila
    (9, 11, 2001, 0x9ea5d8, 0x9ea5a0), // Fermana × Torres
    (9, 11, 2001, 0x9ea5f8, 0x9ea620), // Giulianova × Avellino
    (9, 11, 2001, 0x9ea5e0, 0x9ea638), // Lodigiani × Ascoli
    (9, 11, 2001, 0x9ea5c4, 0x9ea5a8), // Nocerina × Taranto
    (9, 11, 2001, 0x9ea5d0, 0x9ea640), // Pescara × Castel di Sangro
    (9, 11, 2001, 0x9ea5bc, 0x9ea604), // Sora × Chieti
    (9, 11, 2001, 0x9ea594, 0x9ea5ec), // Viterbese × Lanciano
    (16, 11, 2001, 0x9ea638, 0x9ea5f8), // Ascoli × Giulianova
    (16, 11, 2001, 0x9ea620, 0x9ea5e0), // Avellino × Lodigiani
    (16, 11, 2001, 0x9ea640, 0x9ea60c), // Castel di Sangro × Catania
    (16, 11, 2001, 0x9ea604, 0x9ea594), // Chieti × Viterbese
    (16, 11, 2001, 0x9ea62c, 0x9ea614), // L'Aquila × Benevento
    (16, 11, 2001, 0x9ea5ec, 0x9ea5d8), // Lanciano × Fermana
    (16, 11, 2001, 0x9ea5a0, 0x9ea5c4), // Torres × Nocerina
    (16, 11, 2001, 0x9ea5a8, 0x9ea5bc), // Taranto × Sora
    (16, 11, 2001, 0x9ea5b0, 0x9ea5d0), // Vis Pesaro × Pescara
    (23, 11, 2001, 0x9ea614, 0x9ea604), // Benevento × Chieti
    (23, 11, 2001, 0x9ea60c, 0x9ea620), // Catania × Avellino
    (23, 11, 2001, 0x9ea5d8, 0x9ea62c), // Fermana × L'Aquila
    (23, 11, 2001, 0x9ea5f8, 0x9ea5ec), // Giulianova × Lanciano
    (23, 11, 2001, 0x9ea5e0, 0x9ea5a8), // Lodigiani × Taranto
    (23, 11, 2001, 0x9ea5c4, 0x9ea640), // Nocerina × Castel di Sangro
    (23, 11, 2001, 0x9ea5d0, 0x9ea5a0), // Pescara × Torres
    (23, 11, 2001, 0x9ea5bc, 0x9ea5b0), // Sora × Vis Pesaro
    (23, 11, 2001, 0x9ea594, 0x9ea638), // Viterbese × Ascoli
    (6, 0, 2002, 0x9ea640, 0x9ea638), // Castel di Sangro × Ascoli
    (6, 0, 2002, 0x9ea62c, 0x9ea620), // L'Aquila × Avellino
    (6, 0, 2002, 0x9ea614, 0x9ea60c), // Benevento × Catania
    (6, 0, 2002, 0x9ea604, 0x9ea5f8), // Chieti × Giulianova
    (6, 0, 2002, 0x9ea5ec, 0x9ea5e0), // Lanciano × Lodigiani
    (6, 0, 2002, 0x9ea5d8, 0x9ea5d0), // Fermana × Pescara
    (6, 0, 2002, 0x9ea5c4, 0x9ea5bc), // Nocerina × Sora
    (6, 0, 2002, 0x9ea5b0, 0x9ea5a8), // Vis Pesaro × Taranto
    (6, 0, 2002, 0x9ea5a0, 0x9ea594), // Torres × Viterbese
    (13, 0, 2002, 0x9ea594, 0x9ea614), // Viterbese × Benevento
    (13, 0, 2002, 0x9ea5f8, 0x9ea640), // Giulianova × Castel di Sangro
    (13, 0, 2002, 0x9ea5d0, 0x9ea604), // Pescara × Chieti
    (13, 0, 2002, 0x9ea5e0, 0x9ea5d8), // Lodigiani × Fermana
    (13, 0, 2002, 0x9ea5bc, 0x9ea62c), // Sora × L'Aquila
    (13, 0, 2002, 0x9ea5a8, 0x9ea5ec), // Taranto × Lanciano
    (13, 0, 2002, 0x9ea620, 0x9ea5c4), // Avellino × Nocerina
    (13, 0, 2002, 0x9ea60c, 0x9ea5a0), // Catania × Torres
    (13, 0, 2002, 0x9ea638, 0x9ea5b0), // Ascoli × Vis Pesaro
    (20, 0, 2002, 0x9ea604, 0x9ea638), // Chieti × Ascoli
    (20, 0, 2002, 0x9ea5a0, 0x9ea620), // Torres × Avellino
    (20, 0, 2002, 0x9ea5ec, 0x9ea60c), // Lanciano × Catania
    (20, 0, 2002, 0x9ea594, 0x9ea5f8), // Viterbese × Giulianova
    (20, 0, 2002, 0x9ea614, 0x9ea5e0), // Benevento × Lodigiani
    (20, 0, 2002, 0x9ea62c, 0x9ea5d0), // L'Aquila × Pescara
    (20, 0, 2002, 0x9ea5d8, 0x9ea5bc), // Fermana × Sora
    (20, 0, 2002, 0x9ea640, 0x9ea5a8), // Castel di Sangro × Taranto
    (20, 0, 2002, 0x9ea5c4, 0x9ea5b0), // Nocerina × Vis Pesaro
    (27, 0, 2002, 0x9ea5d0, 0x9ea614), // Pescara × Benevento
    (27, 0, 2002, 0x9ea620, 0x9ea640), // Avellino × Castel di Sangro
    (27, 0, 2002, 0x9ea5a8, 0x9ea604), // Taranto × Chieti
    (27, 0, 2002, 0x9ea60c, 0x9ea5d8), // Catania × Fermana
    (27, 0, 2002, 0x9ea5f8, 0x9ea62c), // Giulianova × L'Aquila
    (27, 0, 2002, 0x9ea638, 0x9ea5ec), // Ascoli × Lanciano
    (27, 0, 2002, 0x9ea5e0, 0x9ea5c4), // Lodigiani × Nocerina
    (27, 0, 2002, 0x9ea5b0, 0x9ea5a0), // Vis Pesaro × Torres
    (27, 0, 2002, 0x9ea5bc, 0x9ea594), // Sora × Viterbese
    (3, 1, 2002, 0x9ea5a0, 0x9ea638), // Torres × Ascoli
    (3, 1, 2002, 0x9ea5ec, 0x9ea620), // Lanciano × Avellino
    (3, 1, 2002, 0x9ea604, 0x9ea640), // Chieti × Castel di Sangro
    (3, 1, 2002, 0x9ea5c4, 0x9ea5f8), // Nocerina × Giulianova
    (3, 1, 2002, 0x9ea594, 0x9ea5e0), // Viterbese × Lodigiani
    (3, 1, 2002, 0x9ea60c, 0x9ea5d0), // Catania × Pescara
    (3, 1, 2002, 0x9ea614, 0x9ea5bc), // Benevento × Sora
    (3, 1, 2002, 0x9ea5d8, 0x9ea5a8), // Fermana × Taranto
    (3, 1, 2002, 0x9ea62c, 0x9ea5b0), // L'Aquila × Vis Pesaro
    (10, 1, 2002, 0x9ea5bc, 0x9ea620), // Sora × Avellino
    (10, 1, 2002, 0x9ea5f8, 0x9ea614), // Giulianova × Benevento
    (10, 1, 2002, 0x9ea5e0, 0x9ea60c), // Lodigiani × Catania
    (10, 1, 2002, 0x9ea5c4, 0x9ea604), // Nocerina × Chieti
    (10, 1, 2002, 0x9ea5b0, 0x9ea5d8), // Vis Pesaro × Fermana
    (10, 1, 2002, 0x9ea638, 0x9ea62c), // Ascoli × L'Aquila
    (10, 1, 2002, 0x9ea640, 0x9ea5ec), // Castel di Sangro × Lanciano
    (10, 1, 2002, 0x9ea5a8, 0x9ea5a0), // Taranto × Torres
    (10, 1, 2002, 0x9ea5d0, 0x9ea594), // Pescara × Viterbese
    (17, 1, 2002, 0x9ea614, 0x9ea638), // Benevento × Ascoli
    (17, 1, 2002, 0x9ea5a0, 0x9ea640), // Torres × Castel di Sangro
    (17, 1, 2002, 0x9ea5d8, 0x9ea5f8), // Fermana × Giulianova
    (17, 1, 2002, 0x9ea604, 0x9ea5ec), // Chieti × Lanciano
    (17, 1, 2002, 0x9ea5d0, 0x9ea5e0), // Pescara × Lodigiani
    (17, 1, 2002, 0x9ea594, 0x9ea5c4), // Viterbese × Nocerina
    (17, 1, 2002, 0x9ea60c, 0x9ea5bc), // Catania × Sora
    (17, 1, 2002, 0x9ea62c, 0x9ea5a8), // L'Aquila × Taranto
    (17, 1, 2002, 0x9ea620, 0x9ea5b0), // Avellino × Vis Pesaro
    (3, 2, 2002, 0x9ea604, 0x9ea620), // Chieti × Avellino
    (3, 2, 2002, 0x9ea5c4, 0x9ea614), // Nocerina × Benevento
    (3, 2, 2002, 0x9ea5b0, 0x9ea60c), // Vis Pesaro × Catania
    (3, 2, 2002, 0x9ea640, 0x9ea5d8), // Castel di Sangro × Fermana
    (3, 2, 2002, 0x9ea5e0, 0x9ea62c), // Lodigiani × L'Aquila
    (3, 2, 2002, 0x9ea5f8, 0x9ea5d0), // Giulianova × Pescara
    (3, 2, 2002, 0x9ea5ec, 0x9ea5a0), // Lanciano × Torres
    (3, 2, 2002, 0x9ea638, 0x9ea5bc), // Ascoli × Sora
    (3, 2, 2002, 0x9ea5a8, 0x9ea594), // Taranto × Viterbese
    (10, 2, 2002, 0x9ea620, 0x9ea638), // Avellino × Ascoli
    (10, 2, 2002, 0x9ea5e0, 0x9ea640), // Lodigiani × Castel di Sangro
    (10, 2, 2002, 0x9ea5b0, 0x9ea604), // Vis Pesaro × Chieti
    (10, 2, 2002, 0x9ea5bc, 0x9ea5f8), // Sora × Giulianova
    (10, 2, 2002, 0x9ea62c, 0x9ea5ec), // L'Aquila × Lanciano
    (10, 2, 2002, 0x9ea5d0, 0x9ea5c4), // Pescara × Nocerina
    (10, 2, 2002, 0x9ea614, 0x9ea5a0), // Benevento × Torres
    (10, 2, 2002, 0x9ea60c, 0x9ea5a8), // Catania × Taranto
    (10, 2, 2002, 0x9ea5d8, 0x9ea594), // Fermana × Viterbese
    (17, 2, 2002, 0x9ea5a8, 0x9ea620), // Taranto × Avellino
    (17, 2, 2002, 0x9ea640, 0x9ea614), // Castel di Sangro × Benevento
    (17, 2, 2002, 0x9ea594, 0x9ea60c), // Viterbese × Catania
    (17, 2, 2002, 0x9ea5c4, 0x9ea5d8), // Nocerina × Fermana
    (17, 2, 2002, 0x9ea604, 0x9ea62c), // Chieti × L'Aquila
    (17, 2, 2002, 0x9ea5f8, 0x9ea5e0), // Giulianova × Lodigiani
    (17, 2, 2002, 0x9ea638, 0x9ea5d0), // Ascoli × Pescara
    (17, 2, 2002, 0x9ea5a0, 0x9ea5bc), // Torres × Sora
    (17, 2, 2002, 0x9ea5ec, 0x9ea5b0), // Lanciano × Vis Pesaro
    (24, 2, 2002, 0x9ea5a8, 0x9ea638), // Taranto × Ascoli
    (24, 2, 2002, 0x9ea5d8, 0x9ea614), // Fermana × Benevento
    (24, 2, 2002, 0x9ea5b0, 0x9ea640), // Vis Pesaro × Castel di Sangro
    (24, 2, 2002, 0x9ea5e0, 0x9ea604), // Lodigiani × Chieti
    (24, 2, 2002, 0x9ea60c, 0x9ea5f8), // Catania × Giulianova
    (24, 2, 2002, 0x9ea5ec, 0x9ea5c4), // Lanciano × Nocerina
    (24, 2, 2002, 0x9ea5bc, 0x9ea5d0), // Sora × Pescara
    (24, 2, 2002, 0x9ea62c, 0x9ea5a0), // L'Aquila × Torres
    (24, 2, 2002, 0x9ea620, 0x9ea594), // Avellino × Viterbese
    (30, 2, 2002, 0x9ea614, 0x9ea620), // Benevento × Avellino
    (30, 2, 2002, 0x9ea5c4, 0x9ea60c), // Nocerina × Catania
    (30, 2, 2002, 0x9ea5a0, 0x9ea604), // Torres × Chieti
    (30, 2, 2002, 0x9ea638, 0x9ea5d8), // Ascoli × Fermana
    (30, 2, 2002, 0x9ea640, 0x9ea62c), // Castel di Sangro × L'Aquila
    (30, 2, 2002, 0x9ea5d0, 0x9ea5ec), // Pescara × Lanciano
    (30, 2, 2002, 0x9ea5bc, 0x9ea5e0), // Sora × Lodigiani
    (30, 2, 2002, 0x9ea5f8, 0x9ea5a8), // Giulianova × Taranto
    (30, 2, 2002, 0x9ea594, 0x9ea5b0), // Viterbese × Vis Pesaro
    (7, 3, 2002, 0x9ea5a8, 0x9ea614), // Taranto × Benevento
    (7, 3, 2002, 0x9ea638, 0x9ea60c), // Ascoli × Catania
    (7, 3, 2002, 0x9ea604, 0x9ea5d8), // Chieti × Fermana
    (7, 3, 2002, 0x9ea5a0, 0x9ea5f8), // Torres × Giulianova
    (7, 3, 2002, 0x9ea5b0, 0x9ea5e0), // Vis Pesaro × Lodigiani
    (7, 3, 2002, 0x9ea62c, 0x9ea5c4), // L'Aquila × Nocerina
    (7, 3, 2002, 0x9ea620, 0x9ea5d0), // Avellino × Pescara
    (7, 3, 2002, 0x9ea5ec, 0x9ea5bc), // Lanciano × Sora
    (7, 3, 2002, 0x9ea640, 0x9ea594), // Castel di Sangro × Viterbese
    (14, 3, 2002, 0x9ea5c4, 0x9ea638), // Nocerina × Ascoli
    (14, 3, 2002, 0x9ea5d8, 0x9ea620), // Fermana × Avellino
    (14, 3, 2002, 0x9ea5bc, 0x9ea640), // Sora × Castel di Sangro
    (14, 3, 2002, 0x9ea60c, 0x9ea604), // Catania × Chieti
    (14, 3, 2002, 0x9ea594, 0x9ea62c), // Viterbese × L'Aquila
    (14, 3, 2002, 0x9ea614, 0x9ea5ec), // Benevento × Lanciano
    (14, 3, 2002, 0x9ea5e0, 0x9ea5a0), // Lodigiani × Torres
    (14, 3, 2002, 0x9ea5d0, 0x9ea5a8), // Pescara × Taranto
    (14, 3, 2002, 0x9ea5f8, 0x9ea5b0), // Giulianova × Vis Pesaro
    (21, 3, 2002, 0x9ea5b0, 0x9ea614), // Vis Pesaro × Benevento
    (21, 3, 2002, 0x9ea62c, 0x9ea60c), // L'Aquila × Catania
    (21, 3, 2002, 0x9ea5a0, 0x9ea5d8), // Torres × Fermana
    (21, 3, 2002, 0x9ea620, 0x9ea5f8), // Avellino × Giulianova
    (21, 3, 2002, 0x9ea638, 0x9ea5e0), // Ascoli × Lodigiani
    (21, 3, 2002, 0x9ea5a8, 0x9ea5c4), // Taranto × Nocerina
    (21, 3, 2002, 0x9ea640, 0x9ea5d0), // Castel di Sangro × Pescara
    (21, 3, 2002, 0x9ea604, 0x9ea5bc), // Chieti × Sora
    (21, 3, 2002, 0x9ea5ec, 0x9ea594), // Lanciano × Viterbese
    (28, 3, 2002, 0x9ea5f8, 0x9ea638), // Giulianova × Ascoli
    (28, 3, 2002, 0x9ea5e0, 0x9ea620), // Lodigiani × Avellino
    (28, 3, 2002, 0x9ea60c, 0x9ea640), // Catania × Castel di Sangro
    (28, 3, 2002, 0x9ea594, 0x9ea604), // Viterbese × Chieti
    (28, 3, 2002, 0x9ea614, 0x9ea62c), // Benevento × L'Aquila
    (28, 3, 2002, 0x9ea5d8, 0x9ea5ec), // Fermana × Lanciano
    (28, 3, 2002, 0x9ea5c4, 0x9ea5a0), // Nocerina × Torres
    (28, 3, 2002, 0x9ea5bc, 0x9ea5a8), // Sora × Taranto
    (28, 3, 2002, 0x9ea5d0, 0x9ea5b0), // Pescara × Vis Pesaro
    (5, 4, 2002, 0x9ea604, 0x9ea614), // Chieti × Benevento
    (5, 4, 2002, 0x9ea620, 0x9ea60c), // Avellino × Catania
    (5, 4, 2002, 0x9ea62c, 0x9ea5d8), // L'Aquila × Fermana
    (5, 4, 2002, 0x9ea5ec, 0x9ea5f8), // Lanciano × Giulianova
    (5, 4, 2002, 0x9ea5a8, 0x9ea5e0), // Taranto × Lodigiani
    (5, 4, 2002, 0x9ea640, 0x9ea5c4), // Castel di Sangro × Nocerina
    (5, 4, 2002, 0x9ea5a0, 0x9ea5d0), // Torres × Pescara
    (5, 4, 2002, 0x9ea5b0, 0x9ea5bc), // Vis Pesaro × Sora
    (5, 4, 2002, 0x9ea638, 0x9ea594), // Ascoli × Viterbese
];

// ---------- 275a leva: as regras das três divisões francesas: turnos, pontos, acesso e rebaixamento pelo ano (a virada para 20 clubes em 2002), desempates e a tabela de rodadas (fra_first.cpp, fra_second.cpp, fra_third.cpp) ----------


/// No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
pub(crate) static mut CQJ5_ENSAIO: bool = false;

/// Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
pub(crate) unsafe fn cqj5_regras(this: usize, escritas: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in escritas {
        match t {
            1 => *((this + d) as *mut u8) = v as u8,
            2 => core::ptr::write_unaligned((this + d) as *mut u16, v as u16),
            _ => core::ptr::write_unaligned((this + d) as *mut u32, v),
        }
    }
    if !CQJ5_ENSAIO {
        let rodadas: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this) + 0x3c));
        let t = rodadas(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
        core::ptr::write_unaligned((this + 0xba) as *mut usize, t);
    }
    1
}

/// A temporada 2001 (+0x40, o ano da liga, = 0x7d1): a última com 18 clubes na primeira divisão.
pub(crate) unsafe fn cqj5_em_2001(this: usize) -> bool {
    core::ptr::read_unaligned((this + 0x40) as *const u16) == 0x7d1
}

/// As regras comuns às três: 1 ponto pelo empate (+0xc3), categoria 2 nas partidas (+0xc4), clubes nacionais (+0x42
/// = 1), os desempates 1, 2 e 3 (+0xc5..+0xc7), nenhum play-off (+0xbf = +0xc0 = 0), os bits de regra +0xd9 = 0x83
/// (a liga tem campeão, as vagas de cima são de acesso e a ordem das rodadas dividida ao meio) e 3 substituições
/// (+0x4a).
pub(crate) fn cqj5_comuns() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0xbf, 1, 0), (0xc0, 1, 0), (0xd9, 2, 0x83), (0x4a, 1, 3)]
}

/// the top division: 2 rounds, 3 points for a win and 1 for a draw; nobody goes up; 2 go down in the 2001 season (18
/// clubs, the league grows to 20) and 3 afterwards; tie-breaks 1, 2 and 3 [codes]; 5 substitutes and 3 substitutions;
/// round order split in halves (+0xd9 = 0x83)
/// Original: `FraFirst::set_up_rules` (0x5bc410, fra_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x5bc410 } (thiscall, ret 0, fra_first.cpp): `FraFirst::set_up_rules()`. A primeira divisão: as comuns, 2
/// turnos, 3 pontos, +0xab e +0xdb = 0, ninguém sobe (+0xbe = 0), caem 2 em 2001 e 3 depois (+0xc1), 5 reservas
/// (+0x49), nenhuma liga acima (+0x1c = −1), +0x52 = 2 [sem nome] e a segunda abaixo (+0x20 = [0x9cf5d0]).
pub(crate) unsafe extern "thiscall" fn regras_da_franca_1(this: usize) -> u32 {
    crate::comparacao::ramo("cqj_5 regras da primeira");
    cqj5_regras(this, &cqj5_primeira(this))
}

pub(crate) unsafe fn cqj5_primeira(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqj5_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3), (0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0)]);
    let caem = if cqj5_em_2001(this) { 2 } else { 3 }; // cqj5: a primeira rebaixa 2 em 2001
    v.extend_from_slice(&[(0xc1, 1, caem), (0x49, 1, 5), (0x1c, 4, 0xffff_ffff), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_FRENCH_SECOND_DIVISION) as u32)]);
    v
}


pub(crate) unsafe fn cqj5_segunda(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqj5_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqj5: a segunda dá 3 pontos pela vitória
    let (sobem, caem) = if cqj5_em_2001(this) { (4, 2) } else { (3, 3) };
    v.extend_from_slice(&[(0xbe, 1, sobem), (0xc1, 1, caem), (0x1c, 4, read_usize(KEY_CLUB_COMP_FRENCH_FIRST_DIVISION) as u32), (0x52, 1, 2), (0x20, 4, read_usize(KEY_CLUB_COMP_FRENCH_NATIONAL) as u32), (0x49, 1, 5)]);
    v
}


pub(crate) unsafe fn cqj5_terceira(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = cqj5_comuns();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqj5: o National joga 2 turnos
    let sobem = if cqj5_em_2001(this) { 4 } else { 3 };
    v.extend_from_slice(&[(0xbe, 1, sobem), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_FRENCH_SECOND_DIVISION) as u32), (0x49, 1, 3), (0x20, 4, 0xffff_ffff), (0xa7, 2, 0xffff)]);
    v
}

/// O comparador das três (ensaio, como o da cqi_4): a original roda inteira; a faixa +0x1c..+0xec volta e a cópia
/// grava as regras sem a virtual +0x3c; os campos que a cópia grava saem iguais aos da original, e a devolução 1; no
/// fim fica a faixa da original.
pub(crate) unsafe fn cqj5_comparar(nome: &str, this: usize, escritas: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copia: impl FnOnce() -> u32) -> u32 {
    let faixa = [(this + 0x1c, 0xd0)];
    let antes = crate::comparacao::Memoria::ler(&faixa);
    let r = original();
    let mo = crate::comparacao::Memoria::ler(&faixa);
    antes.escrever();
    CQJ5_ENSAIO = true;
    let rr = crate::aleatorio::pausar_rastro(copia);
    CQJ5_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&faixa);
    mo.escrever();
    let campo = |m: &crate::comparacao::Memoria, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = escritas.iter().filter(|e| campo(&mo, e.0, e.1) != campo(&mr, e.0, e.1)).map(|e| (e.0, campo(&mo, e.0, e.1), campo(&mr, e.0, e.1))).collect();
    PLACAR.registrar(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (deslocamento, original, rust)={:x?}", nome, this, r, rr, difs));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_regras_da_franca_1(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1756);
    cqj5_comparar("regras_da_franca_1", this, cqj5_primeira(this), || o(this), || regras_da_franca_1(this))
}



// ---------- 275a leva: a tabela das três divisões francesas: os clubes (18 ou 20 na primeira, pelo ano), o fundo de prêmios e o dinheiro da TV, na segunda pela divisão de onde o clube veio (fra_first.cpp, fra_second.cpp, fra_third.cpp) ----------



/// O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
pub(crate) unsafe fn cqj6_registro(g: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_usize(g) as i32 as usize).wrapping_mul(0x6b))
}

/// League::pay_tv_rights (0x683010: thiscall(liga, valor, filtro), ret 8).
pub(crate) unsafe fn cqj6_pagar(this: usize, valor: i32, filtro: usize) {
    let pagar: unsafe extern "thiscall" fn(usize, i32, usize) = core::mem::transmute(FN_LEAGUE_PAY_TV_RIGHTS);
    pagar(this, valor, filtro);
}

/// O fundo de prêmios (+0x28 = League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4) e +0xe1 = 1 (pago pela
/// tabela no fim da liga, 0x682e10).
pub(crate) unsafe fn cqj6_fundo(this: usize, cota: i32) {
    let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
    let f = premio(this, cota);
    core::ptr::write_unaligned((this + 0x28) as *mut i32, f);
    *((this + 0xe1) as *mut u8) = 1;
}

/// O corpo comum, com +0x3e já gravado: a tabela (malloc 0x944e46 de +0x3e × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e None); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Devolve se encheu.
pub(crate) unsafe fn cqj6_encher(this: usize, arquivo: usize, linha: u32, g: usize) -> Option<bool> {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let t = alocar((read_i16(this + 0x3e) as i32 as u32).wrapping_mul(0x3b));
    core::ptr::write_unaligned((this + 0xb1) as *mut usize, t);
    if t == 0 {
        aviso_em(arquivo, linha);
        return None;
    }
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut clube = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(clube + 0x57);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            let linha = k; // cqj6: a linha k recebe o k-ésimo clube da divisão
            // ponytail: a original empurra a linha no eax com os bits de cima do id da competição; a 0x687430 só lê
            // o byte.
            iniciar(this, linha as u32, clube, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == read_i16(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
        clube = clube.wrapping_add(0x245);
    }
    Some(k as i8 as i16 == read_i16(this + 0x3e))
}

/// the top division has 18 clubs in the 2001 season and 20 afterwards; prize fund of 200,000 per place; TV pays
/// 6,000,000 to each club
/// Original: `FraFirst::create_table` (0x5bc2b0, fra_first.cpp, thiscall, ret 0).
///
/// [pt] { 0x5bc2b0 } (thiscall, ret 0, fra_first.cpp): `FraFirst::create_table()`: 18 clubes na temporada 2001 (+0x40 =
/// 0x7d1) e 20 depois ([0x9cf5cc]), +0xa7 = −1; cheia, o fundo pela cota de 200.000 e a TV de 6.000.000 a cada clube;
/// 1 (0 sem encher). Fonte 0x9ccb5c, linha 0x1d3.
pub(crate) unsafe extern "thiscall" fn tabela_da_franca_1(this: usize) -> u32 {
    let n: i16 = if core::ptr::read_unaligned((this + 0x40) as *const u16) == 0x7d1 { 0x12 } else { 0x14 };
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, n);
    core::ptr::write_unaligned((this + 0xa7) as *mut u16, 0xffff);
    if cqj6_encher(this, 0x9ccb5c, 0x1d3, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION) != Some(true) {
        return 0;
    }
    crate::comparacao::ramo("cqj_6 tabela da primeira");
    cqj6_fundo(this, 0x30d40);
    cqj6_pagar(this, 0x5b8d80, 0); // cqj6: a TV da primeira
    1
}



/// O comparador das três: a original aloca a tabela e paga a TV (com notícia): só ela. O plano, feito antes: os
/// clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu, as linhas
/// com os clubes do plano na ordem e, cheia, +0xe1 e +0x28 como a rotina grava (`cota`: o fundo da cota e 1; None: 0
/// e 2).
pub(crate) unsafe fn cqj6_comparar(nome: &str, this: usize, n: i16, g: usize, cota: Option<i32>, original: impl FnOnce() -> u32) -> u32 {
    let mut plano: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plano.len() as i16) < n {
        let clube = read_usize(G_CLUB_TABLE) + (i as usize) * 0x245;
        let comp = read_usize(clube + 0x57);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            plano.push(clube);
        }
        i += 1;
    }
    let r = original();
    let t = read_usize(this + 0xb1);
    let mut igual = r == (plano.len() as i16 == n) as u32 && read_i16(this + 0x3e) == n && t != 0;
    let mut achou: Vec<usize> = Vec::new();
    if igual {
        achou = (0..plano.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        igual = achou == plano;
        if r == 1 {
            let e1 = *((this + 0xe1) as *const u8);
            let f = core::ptr::read_unaligned((this + 0x28) as *const i32);
            igual = igual
                && match cota {
                    Some(c) => {
                        let premio: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(FN_LEAGUE_CALC_PRIZE_FUND);
                        e1 == 1 && f == premio(this, c)
                    }
                    None => e1 == 2 && f == 0,
                };
        }
    }
    PLACAR.registrar(igual, || format!("{}({:#x}) original={} plano={:x?} tabela={:x?}", nome, this, r, plano, achou));
    r
}

pub(crate) unsafe extern "thiscall" fn comparar_tabela_da_franca_1(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(1759);
    let n: i16 = if core::ptr::read_unaligned((this + 0x40) as *const u16) == 0x7d1 { 0x12 } else { 0x14 };
    cqj6_comparar("tabela_da_franca_1", this, n, KEY_CLUB_COMP_FRENCH_FIRST_DIVISION, Some(0x30d40), || o(this))
}

/// the French first division schedule: in 2001/02, 34 rounds (18 clubs) from 31 July to 20 May; from 2002/03, 38 rounds
/// (20 clubs), with four extra dates in January and March (8 and 22 January, 4 and 18 March); nearly all with two
/// matches in TV slots (days 6, 4 or 1, slots 1 and 2); main stage only
/// Original: `FraFirst::create_schedule` (0x5bb690, fra_first.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x5bb690 } (thiscall, ret 0x10, fra_first.cpp, 3.090 bytes, método +0x3c): `FraFirst::create_schedule(fase,
/// &rodadas, &extra, &é_liga)`: só a fase −1 (as outras devolvem 0). Em 2001 (o ano +0x40 = 0x7d1) 34 rodadas (18
/// clubes); nos outros anos 38 (20 clubes). é_liga (se pedido) = 1, as rodadas, extra 0, a tabela de rodadas × 0x41
/// bytes (sem memória: a caixa da linha 0xd1 e 0); as rodadas de `CQO6_PRIMEIRA_FRANCESA` em sequência (as marcadas só
/// entram com 38); no fim, se a conta não bate com as rodadas, a caixa da linha 0x1b2 (e devolve a tabela assim mesmo).
/// Fonte 0x9ccb5c.
pub(crate) unsafe extern "thiscall" fn calendario_da_primeira_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16);
    let n: i16 = if ano == 0x7d1 { 0x22 } else { 0x26 }; // cqo6: 34 rodadas em 2001, 38 depois
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = n;
    *extra = 0;
    let t = alocar((n as i32 * 0x41) as u32);
    if t == 0 {
        aviso_em(0x9ccb5c, 0xd1);
        return 0;
    }
    crate::comparacao::ramo("cqo_6 calendario da primeira francesa");
    let mut i: i16 = 0;
    for (so_38, r) in CQO6_PRIMEIRA_FRANCESA.iter() {
        if *so_38 && n != 0x26 {
            continue;
        }
        cqo6_rodadas(t, ano as u32, i as i32, core::slice::from_ref(r));
        i += 1;
    }
    if i != n {
        aviso_em(0x9ccb5c, 0x1b2);
    }
    t
}

pub(crate) unsafe extern "thiscall" fn comparar_calendario_da_primeira_francesa(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1898);
    cqo6_comparar("calendario_da_primeira_francesa", fase, rodadas, extra, liga, || o(this, fase, rodadas, extra, liga), |r, e, l| calendario_da_primeira_francesa(this, fase, r, e, l))
}
