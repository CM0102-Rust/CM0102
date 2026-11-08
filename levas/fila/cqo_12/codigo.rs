// ---------- %%LEVA%%a leva: a Copa do Brasil: as rodadas, a tabela dos times e as regras dos jogos (bra_cup.cpp) ----------

/// A Copa do Brasil: sete rodadas em ida e volta de fevereiro a julho do ano da copa (o jogo de volta 14 dias depois do
/// de ida até as oitavas, 21 nas quartas e 7 na semifinal; 4 dias nas três últimas, ao meio da semana [inferência]).
const COPA_DO_BRASIL_ROUNDS: [EngCcCupCupRound; 7] = [
    cql10r((0x12, 0, 0, 6), (0x17, 1, 0, 2, 2, 0), 0xa, (0, 0, 1), 4, (0x36, 0x1b, 0x36, 0), (0, 2, 0xe), (0, 0xffff_ffff, 0)), // cqo12: a 1ª rodada da Copa do Brasil
    cql10r((9, 2, 0, 3), (0x16, 2, 0, 2, 2, 0), 0x14, (0, 0, 1), 4, (0x22, 0x11, 7, 0x36), (0, 2, 0xe), (0, 0xffff_ffff, 0)),
    cql10r((6, 3, 0, 3), (0x1a, 3, 0, 2, 2, 0), 0x1e, (0, 0, 1), 6, (0x16, 0xb, 5, 0x3d), (0, 2, 0x15), (0, 0xffff_ffff, 0)),
    cql10r((0x12, 4, 0, 3), (7, 5, 0, 2, 2, 0), 0x28, (0, 0, 1), 6, (0x10, 8, 5, 0x42), (0, 2, 7), (0, 0xffff_ffff, 0)),
    cql10r((0xf, 5, 0, 3), (0x11, 5, 0, 5, 1, 0), 0x78, (0, 0, 1), 6, (8, 4, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),
    cql10r((0x16, 5, 0, 3), (0x18, 5, 0, 5, 1, 0), 0x82, (0, 0, 1), 6, (4, 2, 0, 0), (0, 2, 4), (0, 0xffff_ffff, 0)),
    cql10r((0x1d, 5, 0, 3), (1, 6, 0, 5, 1, 0), 0x96, (0, 0, 1), 6, (2, 1, 0, 0), (0, 2, 4), (0, 0xf4240, 0xc3500)),
];

/// { 0x427700 } (thiscall, ret 0x10, bra_cup.cpp; a virtual +0x3c da Copa do Brasil): `BraCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 7 rodadas; a caixa da linha 0xa4 sem memória.
unsafe extern "thiscall" fn bra_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    eng_cc_cup_create_single_stage_round_table(this, fase, rodadas, extra, saida, &COPA_DO_BRASIL_ROUNDS, 0x98a3bc, 0xa4, "cqo_12 tabela da copa do brasil")
}

unsafe extern "thiscall" fn comparar_bra_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    eng_cc_cup_compare_round_table("bra_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| bra_cup_create_round_table(this, fase, r, e, s))
}

unsafe fn cqo12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqo12_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// { 0x427c20 } (thiscall, ret 0, bra_cup.cpp): `BraCup::build_team_table()`: +0x3e = 71 e a tabela dos times (0x944e46
/// de 0x1aa em +0xa7; sem memória, a caixa da linha 0x143 e −1); cada vaga i (6 bytes) recebe o clube da inscrição i da
/// lista +0x14 (6 bytes), a situação +5 = 0xff e a classe +4 = 0. Devolve 1.
unsafe fn build_brazil_cup_teams(this: usize) -> i32 {
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x47);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let l = alocar(0x1aa);
    core::ptr::write_unaligned((this + 0xa7) as *mut u32, l as u32);
    if l == 0 {
        show_error_at(0x98a3bc, 0x143);
        return -1;
    }
    crate::comparison::branch("cqo_12 times da copa do brasil");
    let mut i = 0i16;
    while i < cqo12_w(this + 0x3e) {
        let o = i as usize * 6;
        let fonte = o; // cqo12: a inscrição de cada vaga
        let c = cqo12_d(cqo12_d(this + 0x14) + fonte);
        let e = cqo12_d(this + 0xa7) + o;
        core::ptr::write_unaligned(e as *mut u32, c as u32);
        *((e + 5) as *mut u8) = 0xff;
        *((e + 4) as *mut u8) = 0;
        i = i.wrapping_add(1);
    }
    1
}

unsafe extern "thiscall" fn bra_cup_build_team_table(this: usize) -> i32 {
    build_brazil_cup_teams(this)
}

/// Lado a lado: o +0x3e e o [0xb67a34] a partir do mesmo estado; a original deixa a tabela dela em +0xa7, a cópia faz a
/// sua, comparada nas 71 vagas (com a devolução 1) e liberada.
unsafe extern "thiscall" fn comparar_bra_cup_build_team_table(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I1%%);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let reg = [(this + 0x3e, 2usize), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        lo.set(cqo12_d(this + 0xa7));
        r
    }, || {
        let r = build_brazil_cup_teams(this);
        lr.set(cqo12_d(this + 0xa7));
        core::ptr::write_unaligned((this + 0xa7) as *mut u32, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let k = if a == 1 { cqo12_w(this + 0x3e).max(0) as usize * 6 } else { 0 };
    let lista = if lo != 0 && lr != 0 {
        core::slice::from_raw_parts(lo as *const u8, k) == core::slice::from_raw_parts(lr as *const u8, k)
    } else {
        lo == 0 && lr == 0
    };
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    SCOREBOARD.record(igual && a == b && lista, || format!("bra_cup_build_team_table({:#x}) devolveu {} x {} memória {} lista {}", this, a, b, igual, lista));
    a
}

/// { 0x427ec0 } (thiscall, ret 0, bra_cup.cpp; uma virtual da copa): `BraCup::set_up_rounds()`: 7 reservas (+0x49), 3
/// substituições (+0x4a), o fundo de prêmios de 500 mil (+0x28) e a tabela das rodadas (a virtual +0x3c com a fase −1,
/// as rodadas em +0x3c e o extra em +0x3a, sem saída) em +0xa3. Devolve 1.
unsafe fn set_up_brazil_cup_rounds(this: usize) -> i32 {
    crate::comparison::branch("cqo_12 regras da copa do brasil");
    *((this + 0x49) as *mut u8) = 7;
    *((this + 0x4a) as *mut u8) = 3; // cqo12: as substituições da Copa do Brasil
    core::ptr::write_unaligned((this + 0x28) as *mut u32, 0x7a120);
    let tabela: unsafe extern "thiscall" fn(usize, u32, usize, usize, usize) -> usize = core::mem::transmute(cqo12_d(cqo12_d(this) + 0x3c));
    let t = tabela(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    core::ptr::write_unaligned((this + 0xa3) as *mut u32, t as u32);
    1
}

unsafe extern "thiscall" fn bra_cup_set_up_rounds(this: usize) -> i32 {
    set_up_brazil_cup_rounds(this)
}

/// Lado a lado: +0x28, +0x3a..+0x3d, +0x49/+0x4a e o [0xb67a34] a partir do mesmo estado; as tabelas das rodadas das
/// duas (+0xa3) comparadas nos bytes gravados (eng_cc_cup_written_round_bytes) e a da cópia liberada.
unsafe extern "thiscall" fn comparar_bra_cup_set_up_rounds(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    let to = core::cell::Cell::new(0usize);
    let tr = core::cell::Cell::new(0usize);
    let antes = cqo12_d(this + 0xa3);
    let reg = [(this + 0x28, 4usize), (this + 0x3a, 4), (this + 0x49, 2), (0xb67a34, 4)];
    let (a, b, igual, _, _) = crate::comparison::side_by_side(&reg, || {
        let r = o(this);
        to.set(cqo12_d(this + 0xa3));
        core::ptr::write_unaligned((this + 0xa3) as *mut u32, antes as u32);
        r
    }, || {
        let r = set_up_brazil_cup_rounds(this);
        tr.set(cqo12_d(this + 0xa3));
        core::ptr::write_unaligned((this + 0xa3) as *mut u32, to.get() as u32);
        r
    });
    let (to, tr) = (to.get(), tr.get());
    let n = cqo12_w(this + 0x3c);
    let tabela = if to != 0 && tr != 0 { eng_cc_cup_written_round_bytes(to, n) == eng_cc_cup_written_round_bytes(tr, n) } else { to == 0 && tr == 0 };
    if tr != 0 && tr != to {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(tr);
    }
    SCOREBOARD.record(igual && a == b && tabela, || format!("bra_cup_set_up_rounds({:#x}) devolveu {} x {} memória {} tabela {}", this, a, b, igual, tabela));
    a
}
