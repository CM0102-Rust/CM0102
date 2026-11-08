// ---------- %%LEVA%%a leva: o calendário das copas intercontinentais e da Intertoto: a Intertoto, a Copa Intercontinental, a Copa Interamericana e a Copa dos Campeões da CONCACAF (intertoto_cup.cpp, world_club_cup.cpp, inter_amer_cup.cpp, con_champ.cpp) ----------

/// A Intertoto: cinco rodadas em ida e volta de junho a agosto (a 1ª com 40 times; 12 novos na 2ª, 8 na 3ª).
const CQP13_INTERTOTO: [Cql10R; 5] = [
    cql10r((0x19, 5, 0, 4), (0x1a, 5, 0, 5, 1, 0), 0xa, (0, 4, 7), 7, (0x28, 0x14, 0x28, 0), (0, 2, 4), (0, 0, 0)), // cqp13: a 1ª rodada da Intertoto
    cql10r((1, 6, 0, 3), (3, 6, 0, 5, 1, 0), 0x14, (0, 0, 3), 7, (0x20, 0x10, 0xc, 0x28), (0, 2, 7), (0, 0, 0)),
    cql10r((0xb, 6, 0, 6), (0x11, 6, 0, 5, 1, 0), 0x1e, (0, 0, 3), 7, (0x18, 0xc, 8, 0x34), (0, 2, 7), (0, 0, 0)),
    cql10r((0x19, 6, 0, 6), (0x1c, 6, 0, 2, 2, 0), 0x82, (0, 0, 3), 8, (0xc, 6, 0, 0), (0, 2, 7), (0, 0, 0)),
    cql10r((5, 7, 0, 3), (0xa, 7, 0, 1, 2, 0), 0x96, (0, 0, 3), 8, (6, 3, 0, 0), (0, 2, 0xe), (0, 0, 0)),
];

/// { 0x633300 } (thiscall, ret 0x10, intertoto_cup.cpp; a virtual +0x3c): `IntertotoCup::create_round_table(fase,
/// &rodadas, &extra, &saida)`: na fase −1, as 5 rodadas; a caixa da linha 0xe4 sem memória.
unsafe extern "thiscall" fn intertoto_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP13_INTERTOTO, 0x9e9e14, 0xe4, "cqp_13 tabela da intertoto")
}

unsafe extern "thiscall" fn comparar_intertoto_cup_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I0%%);
    cql10_comparar("intertoto_cup_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| intertoto_cup_create_round_table(this, fase, r, e, s))
}

/// A Copa Intercontinental: jogo único em 1º de dezembro (sorteio em 10 de outubro), mando 4 (Tóquio [inferência]).
const CQP13_INTERCONTINENTAL: [Cql10R; 1] = [
    cql10r((0xa, 9, 0, 5), (1, 0xb, 0, 1, 2, 4), 0, (0, 3, 0), 6, (2, 1, 2, 0), (0, 1, 0), (0, 0, 0)), // cqp13: a Copa Intercontinental
];

/// { 0x92b770 } (thiscall, ret 0x10, world_club_cup.cpp; a virtual +0x3c): `WorldClubCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
unsafe extern "thiscall" fn intertoto_cup_create_round_table_92b770(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP13_INTERCONTINENTAL, 0xad5848, 0x9a, "cqp_13 tabela da intercontinental")
}

unsafe extern "thiscall" fn comparar_intertoto_cup_create_round_table_92b770(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I1%%);
    cql10_comparar("intertoto_cup_create_round_table_92b770", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| intertoto_cup_create_round_table_92b770(this, fase, r, e, s))
}

/// A Copa Interamericana: ida e volta a partir de 26 de janeiro do ano seguinte (sorteio em 10 de outubro), 7 dias.
const CQP13_INTERAMERICANA: [Cql10R; 1] = [
    cql10r((0xa, 9, 0, 5), (0x1a, 0, 1, 2, 2, 0), 0, (0, 0, 3), 8, (2, 1, 2, 0), (0, 2, 7), (0, 0, 0)),
];

/// { 0x632340 } (thiscall, ret 0x10, inter_amer_cup.cpp; a virtual +0x3c): `InterAmerCup::create_round_table(...)`:
/// na fase −1, a rodada única; a caixa da linha 0x9a sem memória.
unsafe extern "thiscall" fn intertoto_cup_create_round_table_632340(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    cql10_tabela(this, fase, rodadas, extra, saida, &CQP13_INTERAMERICANA, 0x9e9d94, 0x9a, "cqp_13 tabela da interamericana")
}

unsafe extern "thiscall" fn comparar_intertoto_cup_create_round_table_632340(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I2%%);
    cql10_comparar("intertoto_cup_create_round_table_632340", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| intertoto_cup_create_round_table_632340(this, fase, r, e, s))
}

/// A Copa dos Campeões da CONCACAF: quartas (28/8), semifinal (1/9) e final (4/9) em jogo único, 8 times.
const CQP13_CONCACAF: [Cql10R; 3] = [
    cql10r((1, 7, 0, 5), (0x1c, 8, 0, 3, 2, 0), 0xa, (0, 3, 0), 8, (8, 4, 8, 0), (0, 0, 0), (0, 0xffff_ffff, 0)), // cqp13: as quartas da CONCACAF
    cql10r((0x1d, 8, 0, 4), (1, 9, 0, 6, 2, 0), 0x82, (0, 3, 0), 8, (4, 2, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
    cql10r((2, 9, 0, 0), (4, 9, 0, 2, 2, 0), 0x96, (0, 3, 0), 8, (2, 1, 0, 0), (0, 0, 0), (0, 0xffff_ffff, 0)),
];

/// { 0x4c0f00 } (thiscall, ret 0x10, con_champ.cpp; a virtual +0x3c): `ConChamp::create_round_table(fase, &rodadas,
/// &extra, &saida)`: na fase −1, saida (se pedida) = 0, rodadas = 3, extra = 0; a tabela (0x944e46 de 0x138; sem
/// memória, a caixa da linha 0x97 e 0) com as 3 rodadas e, nas duas primeiras, o dia 0 do confronto vazio (0x521f10
/// com −1, −1, −1 e mando 0; a original o chama entre o jogo e os campos, que não se sobrepõem). Fora da −1, 0.
unsafe extern "thiscall" fn con_champ_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    if fase as u8 != 0xff {
        return 0;
    }
    if !saida.is_null() {
        *saida = 0;
    }
    *rodadas = 3;
    *extra = 0;
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let t = alocar(0x138);
    if t == 0 {
        aviso_em(0x9a7d30, 0x97);
        return 0;
    }
    crate::comparacao::ramo("cqp_13 tabela da concacaf");
    let elo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) -> u32 = core::mem::transmute(0x521f10usize);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const i16) as i32 as u32;
    for (i, r) in CQP13_CONCACAF.iter().enumerate() {
        cql10_rodada(t, i as u32, r, ano);
        if i < 2 {
            elo(t, i as i32, 0, -1, -1, -1, 0);
        }
    }
    t
}

unsafe extern "thiscall" fn comparar_con_champ_create_round_table(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, saida: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(%%I3%%);
    let a = cql10_comparar("con_champ_create_round_table", fase, rodadas, extra, saida, || o(this, fase, rodadas, extra, saida),
        |r, e, s| con_champ_create_round_table(this, fase, r, e, s));
    // ponytail: a cql10_comparar não olha os dias dos confrontos (+0x23..); o dia 0 das duas primeiras rodadas é
    // conferido aqui contra o valor fixo que a cópia grava
    if a != 0 {
        let ok = (0..2usize).all(|r| {
            let e = a + r * 0x68 + 0x23;
            core::ptr::read_unaligned(e as *const u32) == 0 && core::slice::from_raw_parts((e + 4) as *const u8, 3) == [0xff, 0xff, 0xff]
        });
        PLACAR.registrar(ok, || format!("con_champ_create_round_table: o dia 0 dos confrontos da original difere"));
    }
    a
}
