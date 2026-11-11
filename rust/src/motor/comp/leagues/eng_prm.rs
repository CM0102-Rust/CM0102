use super::*;

/// What changes from one English division to another in the constructor.
///
/// [pt] O que muda de uma divisão inglesa para outra no construtor.
pub(crate) struct EnglishDivision {
    /// The vtable of the derived class.
    ///
    /// [pt] a vtable da classe derivada
    pub(crate) vtable: usize,
    /// The minimum stadium capacity (`+0xe2`) and the minimum seated places (`+0xe4`), when the division writes them.
    ///
    /// [pt] a capacidade mínima do estádio (+0xe2) e os lugares sentados mínimos (+0xe4), quando a divisão os grava
    pub(crate) stadium: Option<(u16, u16)>,
    /// How many stages besides the league (+0x2c): 1 = the promotion play-off (the [+0xc] vector of 1 stage, malloc of 4).
    ///
    /// [pt] quantas fases além da liga (+0x2c): 1 = o play-off de acesso (o vetor [+0xc] de 1 fase, malloc de 4)
    pub(crate) extra_stages: u32,
    /// The division’s set_up_rules and create_table (thiscall, ret 0; they return 0 on failure).
    ///
    /// [pt] set_up_rules e create_table da divisão (mov ecx, this -> thiscall, ret 0; devolvem 0 na falha)
    pub(crate) set_up_rules: usize,
    pub(crate) create_table: usize,
    /// The source file and the lines of the five error boxes (rules, table, fixtures, order, statistics).
    ///
    /// [pt] o fonte e as linhas das cinco caixas (regras, tabela, jogos, ordem, estatísticas)
    pub(crate) source_file: usize,
    pub(crate) box_lines: [u32; 5],
}

/// The body of the four English division constructors (thiscall(this, year, record), ret 8, SEH frame only for the
/// operator new).
///
/// The base (League::League `exe::new_league`: mov ecx -> thiscall, ret 0), the year (+0x40, word), the competition record (+4),
/// the vtable, discipline rule 9 (+0x50) and the minimum stadium; tries to load the state from the temp file
/// (League::load_from_temp_file `exe::league_load_from_temp_file`: push 1; mov ecx -> thiscall, ret 4). Loaded: done.
///
/// Otherwise, the new league: stage 0xff (+0x44), current stage −1 (+0x30), +0x2c stages (and the vector [+0xc]), the
/// rules, the table, the fixtures (League::create_fixtures `create_fixtures`: mov ecx -> thiscall, ret 0), the table order
/// (League::sort_table `sort_table`: push 0; mov ecx -> thiscall, ret 4), the stats (operator new `exe::crt_operator_new` of 0x5ce: push;
/// add esp, 4 -> cdecl; Comp_Stats::Comp_Stats `exe::new_comp_stats`: push league; mov ecx, new -> thiscall, ret 4) in +8 and the
/// predicted positions (League::set_predicted_positions `league_set_predicted_positions`: mov ecx -> thiscall, ret 0).
///
/// Each step that fails shows its line's box and stops. Returns this.
///
/// [pt] O corpo dos quatro construtores (thiscall(this, ano, registro), ret 8, quadro SEH só para o operator new): a base
/// (League::League 0x682200: mov ecx -> thiscall, ret 0), o ano (+0x40, palavra), o registro da competição (+4), a vtable,
/// a regra de disciplina 9 (+0x50) e o estádio mínimo; tenta carregar o estado do arquivo temporário
/// (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4). Carregou: pronto. Senão, a liga nova:
/// fase 0xff (+0x44), fase atual −1 (+0x30), +0x2c fases (e o vetor [+0xc]), as regras, a tabela, os jogos
/// (League::create_fixtures 0x6835c0: mov ecx -> thiscall, ret 0), a ordem da tabela (League::sort_table 0x6827d0:
/// push 0; mov ecx -> thiscall, ret 4), as estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl;
/// Comp_Stats::Comp_Stats 0x49ee70: push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas
/// (League::set_predicted_positions 0x68a850: mov ecx -> thiscall, ret 0). Cada passo que falha mostra a caixa da
/// sua linha e encerra. Devolve this.
pub(crate) unsafe fn construct_english_division(this: League, year: u32, registro: usize, p: &EnglishDivision) -> usize {
    let set_up_rules: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.set_up_rules);
    let create_table: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(p.create_table);
    exe::new_league(this);
    write_usize(this.addr() + league::COMPETITION, registro);
    write_usize(this.addr(), p.vtable);
    this.set_year((year as u16) as i16);
    this.set_discipline_rule(discipline_rule::ENGLAND); // cql6: the discipline rule of the English leagues
    if let Some((capacidade, sentados)) = p.stadium {
        this.set_required_capacity(capacidade);
        this.set_required_seats(sentados);
    }
    if exe::league_load_from_temp_file(this, 1) != 0 {
        return this.addr();
    }
    crate::comparison::branch("cql_6 new english division");
    this.set_stage_index(stage_index::MAIN);
    this.set_current_stage(0xffff_ffff);
    this.set_stage_count(p.extra_stages as usize);
    if p.extra_stages != 0 {
        let v = exe::crt_malloc(4);
        this.set_stages(v);
    } else {
        this.set_stages(0);
    }
    if set_up_rules(this.addr()) == 0 {
        show_error_at(p.source_file, p.box_lines[0]);
        return this.addr();
    }
    if create_table(this.addr()) == 0 {
        show_error_at(p.source_file, p.box_lines[1]);
        return this.addr();
    }
    if exe::league_create_fixtures(this) == 0 {
        show_error_at(p.source_file, p.box_lines[2]);
        return this.addr();
    }
    if exe::league_sort_table(this, (0) as u32 as usize) == 0 {
        show_error_at(p.source_file, p.box_lines[3]);
        return this.addr();
    }
    let e = exe::crt_operator_new(0x5ce);
    let s = if e != 0 { exe::new_comp_stats(e, this) } else { 0 };
    this.set_comp_stats(s);
    if s == 0 {
        show_error_at(p.source_file, p.box_lines[4]);
        return this.addr();
    }
    exe::league_set_predicted_positions(this);
    this.addr()
}

pub(crate) const PREMIER_DIVISION: EnglishDivision = EnglishDivision {
    vtable: 0x969d1c,
    stadium: Some((0x2710, 0x2710)),
    extra_stages: 0,
    set_up_rules: 0x574b70,
    create_table: 0x574a20,
    source_file: 0x9c43b0,
    box_lines: [0x4d, 0x54, 0x5b, 0x64, 0x6b],
};
pub(crate) const FIRST_DIVISION: EnglishDivision = EnglishDivision {
    vtable: 0x969c68,
    stadium: None,
    extra_stages: 1, // cql6: the First Division play-off
    set_up_rules: 0x572d90,
    create_table: 0x572bf0,
    source_file: 0x9c4374,
    box_lines: [0x49, 0x50, 0x57, 0x60, 0x68],
};
pub(crate) const ENG_SECOND_DIVISION: EnglishDivision = EnglishDivision {
    vtable: 0x969dd0,
    stadium: None,
    extra_stages: 1, // cql6: the Second Division play-off
    set_up_rules: 0x576780,
    create_table: 0x576630,
    source_file: 0x9c43e8,
    box_lines: [0x4a, 0x51, 0x58, 0x61, 0x68],
};
pub(crate) const THIRD_DIVISION: EnglishDivision = EnglishDivision {
    vtable: 0x969e84,
    stadium: Some((0x1770, 0x3e8)),
    extra_stages: 1,
    set_up_rules: 0x5780c0,
    create_table: 0x577f70,
    source_file: 0x9c4424,
    box_lines: [0x4e, 0x55, 0x5c, 0x65, 0x6d],
};

/// the Premiership: discipline rule 9 (the English leagues one), minimum ground of 10,000 all seated, only the league
/// (no play-off); with no saved state, the rules, the table, the fixtures, the order, the stats and the predicted
/// positions
/// Original: `EngPrm::EngPrm` (0x573380, eng_prm.cpp, thiscall, ret 8).
///
/// [pt] { 0x573380 } (thiscall, ret 8, eng_prm.cpp): `EngPrm::EngPrm(ano, registro)`. A Premier: sem fases além da liga
/// (+0x2c = 0, +0xc = 0) e o estádio mínimo de 10.000 lugares, todos sentados.
pub(crate) unsafe extern "thiscall" fn construct_eng_prm(this: usize, year: u32, record: usize) -> usize {
    construct_english_division(League(this), year, record, &PREMIER_DIVISION)
}

/// Comparer of the four constructors: runs only the original and checks return value, vtable, year, record, discipline,
/// stadium and, for a new league, stages and stats.
///
/// [pt] O comparador dos quatro: alocam (as estatísticas, a tabela, os jogos): só a original, conferindo o que ela grava:
/// a devolução, a vtable, o ano, o registro e a regra de disciplina; na liga nova (+0x44 = 0xff), +0x2c, o vetor das
/// fases, a fase atual −1 e as estatísticas em +8.
pub(crate) unsafe fn compare_english_constructor(name: &str, this: League, year: u32, record: usize, p: &EnglishDivision, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let mut ok = r == this.addr() && read_usize(this.addr()) == p.vtable && (this.year() as u16) == year as u16
        && this.competition().addr() == record && this.discipline_rule() == discipline_rule::ENGLAND;
    if let Some((c, s)) = p.stadium {
        ok = ok && this.required_capacity() == c && this.required_seats() == s;
    }
    if this.stage_index() == stage_index::MAIN && this.comp_stats() != 0 {
        ok = ok && this.stage_count() == p.extra_stages as usize && (this.stages() != 0) == (p.extra_stages != 0);
    }
    SCOREBOARD.record(ok, || format!("{}({:#x}): returned {:#x}, vtable {:#x}, +0x2c = {}, +0xc = {:#x}, +8 = {:#x}", name, this.addr(), r, read_usize(this.addr()), this.stage_count(), this.stages(), this.comp_stats()));
    r
}

/// In the rehearsal (the comparer) the copy writes the rules and does not call virtual +0x3c (the round table, which
/// allocates).
///
/// [pt] No ensaio (o comparador) a cópia grava as regras e não chama a virtual +0x3c (a tabela de rodadas, que aloca).
pub(crate) static ENG_PRM_RULES_DRY_RUN: AtomicBool = AtomicBool::new(false);

/// Writes the rule fields (offset, size, value) into the league and builds the main-stage round table via virtual +0x3c
/// into +0xba (skipped in dry run); returns 1.
///
/// [pt] Grava as regras na liga (deslocamento, tamanho 1/2/4, valor), na ordem da lista, e monta a tabela de rodadas da
/// fase principal: a virtual +0x3c (create_schedule: thiscall(liga, −1, &+0xa9, &+0x3a, 0), ret 0x10) com o
/// resultado em +0xba. Devolve 1.
pub(crate) unsafe fn eng_prm_apply_league_rules(this: League, written: &[(usize, u8, u32)]) -> u32 {
    for &(d, t, v) in written {
        match t {
            1 => *((this.addr() + d) as *mut u8) = v as u8,
            2 => write_u16(this.addr() + d, v as u16),
            _ => write_u32(this.addr() + d, v),
        }
    }
    if !ENG_PRM_RULES_DRY_RUN.load(Relaxed) {
        let rounds: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(read_usize(read_usize(this.addr()) + 0x3c));
        let t = rounds(this.addr(), 0xffff_ffff, this.addr() + league::ROUND_COUNT, this.addr() + league::STAGE_ID, 0);
        this.set_round_dates(t);
    }
    1
}

/// The rules common to the four English divisions (the halves `+0x3c` and the win points `+0xc2` are set in each one).
///
/// 1 point for a draw (`+0xc3`), category 2 for the matches (`+0xc4`, league), national clubs (`+0x42` = 1),
/// tie-breakers 1, 2 and 3 (`+0xc5`..`+0xc7`: goal difference, goals scored and position [inference from the English
/// codes]) and `+0x52` = 2 [unnamed].
///
/// [pt] As regras comuns às quatro divisões (os turnos +0x3c e os pontos da vitória +0xc2 vão em cada uma): 1 ponto pelo empate (+0xc3),
/// categoria 2 nas partidas (+0xc4, liga), clubes nacionais (+0x42 = 1), os desempates 1, 2 e 3 (+0xc5..+0xc7: saldo,
/// gols pró e a posição [inferência pelos códigos da Inglaterra]) e +0x52 = 2 [sem nome].
pub(crate) fn english_common_rules() -> Vec<(usize, u8, u32)> {
    vec![(0xc3, 1, 1), (0xc4, 1, 2), (0x42, 1, 1), (0xc5, 1, 1), (0xc6, 1, 2), (0xc7, 1, 3), (0x52, 1, 2)]
}

/// the Premier (20 clubs): 2 rounds, 3 points for a win and 1 for a draw; nobody goes up, the bottom 3 go straight down
/// to the First; tie-breaks goal difference, goals for and position [inference from codes 1, 2, 3]; builds the round
/// schedule
/// Original: `EngPrm::set_up_rules` (0x574b70, eng_prm.cpp, thiscall, ret 0).
///
/// [pt] { 0x574b70 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::set_up_rules()`. A Premier: as comuns, +0xab e +0xdb = 0
/// (as palavras copiadas em cada partida), ninguém sobe (+0xbe = 0, +0xbf = 0, +0xc0 = 0), 3 caem direto (+0xc1),
/// nenhuma liga acima (+0x1c = −1) e a First abaixo (+0x20 = [0x9cf5c0]).
pub(crate) unsafe extern "thiscall" fn eng_prm_set_up_rules(this: usize) -> u32 {
    crate::comparison::branch("cqi_4 premier rules");
    eng_prm_apply_league_rules(League(this), &premier_rules())
}

/// Rule writes for the Premier Division (2 rounds, 3 points, 3 relegated, lower division First).
pub(crate) unsafe fn premier_rules() -> Vec<(usize, u8, u32)> {
    let mut v = english_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Premier, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xab, 2, 0), (0xdb, 2, 0), (0xbe, 1, 0), (0xbf, 1, 0), (0xc0, 1, 0)]);
    v.push((0xc1, 1, 3)); // cqi4: a Premier rebaixa 3
    v.extend_from_slice(&[(0x1c, 4, 0xffff_ffff), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) as u32)]);
    v
}

/// Rule writes for the First Division.
pub(crate) unsafe fn eng_prm_first_division_rules() -> Vec<(usize, u8, u32)> {
    let mut v = english_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a First joga 2 turnos
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 3), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION) as u32), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) as u32), (0x49, 1, 5)]);
    v
}

/// Rule writes for the Second Division.
pub(crate) unsafe fn eng_prm_second_division_rules() -> Vec<(usize, u8, u32)> {
    let mut v = english_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: the Second gives 3 points per win
    v.extend_from_slice(&[(0xbe, 1, 2), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 4), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_FIRST_DIVISION) as u32), (0x4a, 1, 3), (0x20, 4, read_usize(KEY_CLUB_COMP_ENGLISH_THIRD_DIVISION) as u32), (0x49, 1, 5)]);
    v
}

/// Rule writes for the Third Division (lower division Conference only when selected).
pub(crate) unsafe fn third_division_rules(this: usize) -> Vec<(usize, u8, u32)> {
    let mut v = english_common_rules();
    v.extend_from_slice(&[(0x3c, 2, 2), (0xc2, 1, 3)]); // cqi4: a Third, 2 turnos e 3 pontos
    v.extend_from_slice(&[(0xbe, 1, 3), (0xbf, 1, 4), (0xc0, 1, 0), (0xc1, 1, 1), (0x1c, 4, read_usize(KEY_CLUB_COMP_ENGLISH_SECOND_DIVISION) as u32)]);
    let nation = Nation(read_usize(read_usize(this + 4) + competition::NATION));
    let below = if nation.league_selected() & 4 != 0 { read_usize(KEY_CLUB_COMP_ENGLISH_CONFERENCE) as u32 } else { 0xffff_ffff };
    v.extend_from_slice(&[(0x20, 4, below), (0x4a, 1, 3), (0x49, 1, 5)]);
    v
}

/// The comparer of the four rule routines, as a dry run: the original builds the round table (it allocates; it does not
/// run twice).
///
/// The original runs in full; the range +0x1c..+0xec is restored to what it was and the copy writes the rules without
/// virtual +0x3c; the fields the copy writes must come out equal to the original's, and the return value must be 1; at
/// the end the original's range is kept (with its round table at +0xba, +0xa9 and +0x3a).
///
/// [pt] O comparador das quatro: a original monta a tabela de rodadas (aloca; não roda duas vezes), então é ensaio. A
/// original roda inteira; a faixa +0x1c..+0xec volta como estava e a cópia grava as regras sem a virtual +0x3c; os
/// campos que a cópia grava têm de sair iguais aos da original, e a devolução 1; no fim fica a faixa da original
/// (com a tabela de rodadas dela em +0xba, +0xa9 e +0x3a).
pub(crate) unsafe fn eng_prm_compare_league_rules(name: &str, this: League, written: Vec<(usize, u8, u32)>, original: impl FnOnce() -> u32, copy: impl FnOnce() -> u32) -> u32 {
    let band = [(this.addr() + league::UPPER_DIVISION_ID, 0xd0)];
    let before = crate::comparison::MemorySnapshot::read(&band);
    let r = original();
    let mo = crate::comparison::MemorySnapshot::read(&band);
    before.write_back();
    ENG_PRM_RULES_DRY_RUN.store(true, Relaxed);
    let rr = crate::rng::pause_trace(copy);
    ENG_PRM_RULES_DRY_RUN.store(false, Relaxed);
    let mr = crate::comparison::MemorySnapshot::read(&band);
    mo.write_back();
    let field = |m: &crate::comparison::MemorySnapshot, d: usize, t: u8| -> u32 {
        let b = &m.0[0].1[d - 0x1c..];
        match t {
            1 => b[0] as u32,
            2 => u16::from_le_bytes([b[0], b[1]]) as u32,
            _ => u32::from_le_bytes([b[0], b[1], b[2], b[3]]),
        }
    };
    let difs: Vec<(usize, u32, u32)> = written.iter().filter(|e| field(&mo, e.0, e.1) != field(&mr, e.0, e.1)).map(|e| (e.0, field(&mo, e.0, e.1), field(&mr, e.0, e.1))).collect();
    SCOREBOARD.record(difs.is_empty() && r == rr, || format!("{}({:#x}) original={} rust={} (offset, original, rust)={:x?}", name, this.addr(), r, rr, difs));
    r
}

/// Club competition record (0x6b bytes in G_CLUB_COMP_TABLE) of the id stored in global g.
///
/// [pt] O registro de competição (0x6b bytes em [0xae23d0]) do id guardado no global `g`.
pub(crate) unsafe fn comp_record_of(g: usize) -> usize {
    read_usize(G_CLUB_COMP_TABLE).wrapping_add((read_usize(g) as i32 as usize).wrapping_mul(0x6b))
}

/// True if the current game year is after 1999 (larger TV money).
///
/// [pt] O ano do jogo ([0xae2c92]) passou de 1999 (o dinheiro da TV maior).
pub(crate) unsafe fn is_after_1999() -> bool {
    read_u16(G_CURRENT_DATE_YEAR) > 0x7cf
}

/// League pay_tv_rights (`pay_tv_rights`): pays the value to every table club, or only those that came from competition
/// `filtro`.
///
/// [pt] League::pay_tv_rights (0x683010: thiscall(liga, valor, filtro), ret 8): o valor a cada clube da tabela, ou só aos
/// que vieram da competição `filtro` (+0x5b do clube).
pub(crate) unsafe fn eng_prm_pay_tv_rights(this: usize, value: i32, filter: usize) {
    exe::league_pay_tv_rights(this, value, filter);
}

/// The common table body: allocates the table and fills it with the division's clubs; when full, sets the prize fund,
/// `+0xe1` and pays the TV money; returns 1 if full.
///
/// `+0x3e` = `n` clubs; the table (malloc `0x944e46` of n × 0x3b; out of memory: the box of source file `arquivo` at
/// line `linha` and returns 0); the clubs of [`0xae23b4`] (0x245 bytes, [`0xae235c`] of them) whose competition
/// (`+0x57`, the id at `+0`) is that of global `g`, in database order, each in the next row (League::init_table_row
/// `0x687430`: thiscall(league, row, club, 0, 0), ret 0x10), until it is full.
///
/// Not full: 0. Full: the prize fund `+0x28` (League::calc_prize_fund `0x682f70`: thiscall(league, share), ret 4),
/// `+0xe1` = 1, the TV money (`pagar`) and 1.
///
/// [pt] O corpo comum: +0x3e = `n` clubes; a tabela (malloc 0x944e46 de n × 0x3b; sem memória: a caixa do fonte
/// `arquivo` na linha `linha` e devolve 0); os clubes de [0xae23b4] (0x245 bytes, [0xae235c] deles) cuja competição
/// (+0x57, o id no +0) é a do global `g`, na ordem do banco, cada um na linha seguinte (League::init_table_row
/// 0x687430: thiscall(liga, linha, clube, 0, 0), ret 0x10), até encher. Sem encher: 0. Cheia: o fundo de prêmios
/// +0x28 (League::calc_prize_fund 0x682f70: thiscall(liga, cota), ret 4), +0xe1 = 1, o dinheiro da TV (`pagar`) e 1.
pub(crate) unsafe fn eng_prm_fill_league_table(this: League, n: i16, file: usize, row: u32, g: usize, quota: i32, pay: impl FnOnce()) -> u32 {
    this.set_club_count(n);
    let t = exe::crt_malloc((n as i32 as u32).wrapping_mul(0x3b));
    this.set_table(t);
    if t == 0 {
        show_error_at(file, row);
        return 0;
    }
    let mut k: u8 = 0;
    let mut i: i16 = 0;
    let mut club = read_usize(G_CLUB_TABLE);
    while (i as i32) < read_usize(G_CLUB_COUNT) as i32 {
        let comp = read_usize(club + club::DIVISION);
        if comp != 0 && read_usize(comp) == read_usize(g) {
            let row = k; // cqi5: row k receives the k-th club of the division
            // simplification: the original pushes the row in eax with the high bits of the competition id; 0x687430 only reads
            // o byte.
            exe::league_init_table_row(this, row as u32, club, 0, 0);
            k = k.wrapping_add(1);
            if k as i8 as i16 == this.club_count() {
                break;
            }
        }
        i = i.wrapping_add(1);
        club = club.wrapping_add(club::SIZE);
    }
    if k as i8 as i16 != this.club_count() {
        return 0;
    }
    crate::comparison::branch("cqi_5 full table");
    let f = exe::league_calc_prize_fund(this, quota);
    this.set_prize_fund(f);
    this.set_prize_mode(1);
    pay();
    1
}

/// the Premier has 20 clubs; the prize fund starts at 250,000 per place; TV pays 12,000,000 to each club (5,000,000 up
/// to 1999)
/// Original: `EngPrm::create_table` (0x574a20, eng_prm.cpp, thiscall, ret 0).
///
/// [pt] { 0x574a20 } (thiscall, ret 0, eng_prm.cpp): `EngPrm::create_table()`: 20 clubes ([0x9cf5bc]), cota 250.000 do
/// fundo de prêmios e a TV: 12.000.000 a cada clube depois de 1999 (5.000.000 antes). Fonte 0x9c43b0, linha 0x1ee.
pub(crate) unsafe extern "thiscall" fn eng_prm_create_table(this: usize) -> u32 {
    eng_prm_fill_league_table(League(this), 0x14, 0x9c43b0, 0x1ee, KEY_CLUB_COMP_ENGLISH_PREMIER_DIVISION, 0x3d090, || {
        let value = if is_after_1999() { 0xb71b00 } else { 0x4c4b40 }; // cqi5: the Premier TV money
        eng_prm_pay_tv_rights(this, value, 0)
    })
}

/// Comparer of the four: plans the division's clubs beforehand, runs only the original routine and checks the count,
/// return value, rows and prize fund.
///
/// The original allocates the table and pays the clubs (it cannot run twice), so only it runs. The plan, made
/// beforehand: the division's clubs in database order (up to `n`).
///
/// Afterwards: +0x3e = `n`, the return value = 1 if the plan filled up (otherwise 0) and the table rows hold the
/// planned clubs, in order; when full, +0xe1 = 1 and +0x28 = the share fund.
///
/// [pt] O comparador das quatro: a original aloca a tabela e paga os clubes (não roda duas vezes): só ela. O plano, feito
/// antes: os clubes da divisão na ordem do banco (até `n`). Depois: +0x3e = `n`, a devolução = 1 se o plano encheu
/// (senão 0) e as linhas da tabela com os clubes do plano, na ordem; cheia, +0xe1 = 1 e +0x28 = o fundo da cota.
pub(crate) unsafe fn eng_prm_compare_league_table(name: &str, this: League, n: i16, g: usize, quota: i32, original: impl FnOnce() -> u32) -> u32 {
    let mut plan: Vec<usize> = Vec::new();
    let mut i: i32 = 0;
    while i < read_usize(G_CLUB_COUNT) as i32 && (plan.len() as i16) < n {
        let club = Club(read_usize(G_CLUB_TABLE) + (i as usize) * club::SIZE);
        let comp = club.division().addr();
        if comp != 0 && read_usize(comp) == read_usize(g) {
            plan.push(club.addr());
        }
        i += 1;
    }
    let r = original();
    let t = this.table();
    let expected = (plan.len() as i16 == n) as u32;
    let mut same = r == expected && this.club_count() == n && t != 0;
    let mut found: Vec<usize> = Vec::new();
    if same {
        found = (0..plan.len()).map(|k| read_usize(t + k * 0x3b)).collect();
        same = found == plan;
        if r == 1 {
            same = same && this.prize_mode() == 1 && this.prize_fund() == exe::league_calc_prize_fund(this, quota);
        }
    }
    SCOREBOARD.record(same, || format!("{}({:#x}) original={} plan={:x?} table={:x?}", name, this.addr(), r, plan, found));
    r
}

/// the Premiership schedule: 38 rounds (20 clubs), from 19 August to 20 May; Christmas (four fixed-date rounds) by the
/// day the 16 December round falls on; no play-offs (main stage only)
/// Original: `EngPrm::create_schedule` (0x573660, eng_prm.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x573660 } (thiscall, ret 0x10, eng_prm.cpp, 5.019 bytes, método +0x3c): `EngPrm::create_schedule(fase, &rodadas,
/// &extra, &é_liga)`: só a fase −1, 38 rodadas (`PREMIERSHIP_CHRISTMAS`); as outras devolvem 0. Fonte 0x9c43b0.
pub(crate) unsafe extern "thiscall" fn eng_prm_create_schedule(this: usize, stage: u32, rounds: *mut i16, extra: *mut i16, league: *mut i32) -> usize {
    let this = League(this);
    match stage as u8 {
        0xff => {
            if !league.is_null() {
                *league = 1;
            }
            *rounds = 38;
            *extra = 0;
            let t = exe::crt_malloc(0x9a6);
            if t == 0 {
                show_error_at(0x9c43b0, 0xa7);
                return 0;
            }
            crate::comparison::branch("cqp_5 premiership calendar");
            let year = (this.year() as u16) as u32;
            eng_first_set_christmas_rounds(t, year, &PREMIERSHIP_CHRISTMAS, 0x9c43b0);
            t
        }
        _ => 0,
    }
}

