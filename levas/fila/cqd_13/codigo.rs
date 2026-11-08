// ---------- %%LEVA%%a leva: os registros do campeão da liga e da copa e o registro do técnico humano zerado (staff_records.cpp) ----------

unsafe fn cqd13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqd13_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqd13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqd13_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// O ensaio dos comparadores: as cópias sem os históricos (arquivo), sem as notícias (0x75c390), sem acabar as
/// convocações (0x774f30) e sem a renovação (0x4cda70), anotando os contratos que a renovação mexeria.
static mut CQD13_ENSAIO: bool = false;
static mut CQD13_RENOVADOS: Vec<usize> = Vec::new();
/// Depois da última conta de dias (0x54cdb0) da cópia, o ensaio pulou uma chamada da original que também conta dias
/// e deixa a data de rascunho estática dela (0xae2470) noutro ano: a renovação (0x4cda70, pela idade), o fim das
/// convocações (0x774f30) ou a notícia do elogio (0x75c390). Essa data então fica fora da conta.
static mut CQD13_DATA_SOLTA: bool = false;

/// A reputação do clube (+0x80) sobe até peso × 500, no máximo 750 de uma vez (entre 1 e 10000; soma em 16 bits).
unsafe fn raise_champion_reputation(clube: usize, peso: i16, campeao_da_liga: bool) {
    let alvo = (peso as i32).wrapping_mul(500);
    let rep = cqd13_w(clube + 0x80) as i32;
    if rep < alvo {
        let falta = alvo - rep;
        let soma = if falta > 0x2ee { 0x2ee } else { falta }; // cqd13: o teto da subida
        let v = cqd13_w(clube + 0x80).wrapping_add(soma as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if campeao_da_liga {
            crate::comparacao::ramo("cqd_13 injury_word_0c do campeao da liga");
        } else {
            crate::comparacao::ramo("cqd_13 injury_word_0c do campeao da copa");
        }
        core::ptr::write_unaligned((clube + 0x80) as *mut i16, v); // cqd13: a reputação do campeão
    }
}

/// O moral do contrato (+0x44, byte com sinal) sobe `d`, preso em −100..100.
unsafe fn add_contract_morale(k: usize, d: i32) {
    let c = *((k + 0x44) as *const i8);
    let x = c as i32 + d;
    *((k + 0x44) as *mut i8) = if x > 100 { 100 } else if x < -100 { -100 } else { c.wrapping_add(d as i8) };
}

/// A renovação oferecida ao jogador do próprio clube (0x4cda70 com 1,0 e submeter): com o gerente de transferências
/// ([0xde1f54]), mais de 300 dias desde a data do contrato (+0x25; 0x54cdb0) e a vaga dele na lista do clube (0x849ce0
/// com 0, 0; +0xf o id). No ensaio, só anota o contrato.
unsafe fn offer_renewal_if_due(clube: usize, p: usize, k: usize) {
    if cqd13_d(0xde1f54) == 0 {
        return;
    }
    CQD13_DATA_SOLTA = false;
    if days_between(0xae2c90, k + 0x25) <= 300 {
        return;
    }
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    // 0x849ce0: push 0; push 0; push p; push clube; mov ecx, [[0xde1f54]+0x856] -> thiscall (ret 0x10)
    let v = vaga_de(cqd13_d(cqd13_d(0xde1f54) + 0x856), clube, p, 0, 0);
    if v == 0 || cqd13_i(v + 0xf) != cqd13_i(p) {
        return;
    }
    if CQD13_ENSAIO {
        (*core::ptr::addr_of_mut!(CQD13_RENOVADOS)).push(k);
        CQD13_DATA_SOLTA = true;
        return;
    }
    let oferecer: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, f64, u32) -> u32 = core::mem::transmute(0x4cda70usize);
    // 0x4cda70: push 1; push 0x3ff00000; push 0; push −1; push 0; push 0; push p; mov ecx, [0xae1050] -> thiscall
    // (p, 0, 0, −1, 1.0 em dois dwords, 1)
    oferecer(cqd13_d(0xae1050), p, 0, 0, -1, 1.0, 1);
}

/// Os históricos da entrada `e` para o técnico (já feito por quem chama), o presidente (+0xbf) e os 20 da escalação
/// (+0x190 do lado, 0x49 bytes, −1 vazio; pessoas válidas e fora o técnico; nula: a caixa da linha 0x1010).
unsafe fn add_lineup_histories(this: usize, clube: usize, escalacao: usize, e: &[u8; 0x40]) {
    for j in 0..0x14usize {
        let id = cqd13_i(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = cqd13_d(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && cqd13_i(p) >= cqd13_i(0xae2364) {
            continue;
        }
        if p == cqd13_d(clube + 0xcf) {
            continue;
        }
        if p == 0 {
            show_error_at(0xa83b78, 0x1010);
            continue;
        }
        append_person_history_by_id(!CQD13_ENSAIO, this, cqd13_d(p), e);
    }
}

/// O histórico do clube (tipo no +8, a competição no +9).
unsafe fn add_club_history(this: usize, clube: usize, tipo: u8, comp: usize) {
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[8] = tipo;
    put_i32(&mut e, 9, cqd13_i(comp));
    if !CQD13_ENSAIO {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(0x58e340usize);
        let p = cqd13_d(this + 0x10).wrapping_add(cqd13_d(clube).wrapping_mul(8));
        f(cqd13_d(this + 4), p, p + 4, e.as_ptr());
    }
}

/// A notícia do elogio do título (0x75c390 em [0xb6b7c8]; fora do ensaio).
unsafe fn post_trophy_praise(clube: usize, comp: usize, marca: u32) {
    if CQD13_ENSAIO {
        CQD13_DATA_SOLTA = true;
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x75c390usize);
    // 0x75c390: push marca; push comp; push clube; mov ecx, [0xb6b7c8] -> thiscall (ret 0xc)
    f(cqd13_d(0xb6b7c8), clube, comp, marca);
}

/// As notas k1/k2 do técnico pela fase: na competição de fase 1 ([0xadadfc] + 4 × id, +0x42 = 1) 200 na terceira e 200
/// na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1), senão 400.
unsafe fn manager_rating_gains(t: usize, clube: usize, id_comp: usize) -> (i32, i32) {
    let c = cqd13_d(cqd13_d(0xadadfc).wrapping_add(4 * id_comp));
    if *((c + 0x42) as *const i8) != 1 {
        (400, 400)
    } else {
        let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x53d980usize);
        // 0x53d980: 3 pushes; add esp, 0xc -> cdecl
        (if estrangeiro(t, cqd13_d(clube + 0x53), 1) != 0 { 200 } else { 400 }, 200)
    }
}

/// { 0x868c50 } (thiscall, ret 8, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_league_champions_record(clube, comp)`, pelos campeões das ligas no fim delas (25 chamadores, um por liga; 4
/// chamadas até 2002-06-01). Clube ou competição nulos: a caixa (linha 0x21c); a competição sem objeto em [0xadadfc]
/// (+ 4 × id): a caixa (0x224). A reputação (até peso × 500, +750 no máximo); com +0x57 e o bloco +0x69 do clube: o
/// +0x7b sobe peso × 20 (abaixo de 150000 e de +0x73 + 500) e o +0x44 do bloco 25 × (peso + 20) (abaixo de 120000). O
/// limiar dos jogadores: ftol((+0x3e − 1) × +0x3c do objeto × 0,33), em 16 bits. Com técnico (+0xcf): as três notas
/// dele (+0xa peso × 400, +8 peso × k1, +0xc peso × k2 no registro +0x69, se houver) e o histórico (tipo 5: o clube e a
/// competição); o registro dele (0x86f5b0 com criar): +1 em +0x8c e peso × 5000 em +0xb6, +0xca e +0xba. O presidente
/// (+0xbf). Os 50 do elenco (+0xd7) com ficha, válidos, do clube (+0x39) e fora o técnico: a média 2/1 (0x7c1e30 com
/// 6,5) acima do limiar dá o histórico, moral +20 (0x544710) e +100 no contrato (0x4cf5f0) com a renovação; senão moral
/// +10 e +50 no contrato. O histórico do clube (tipo 5); com a mídia ([0xb6b7c8]) e peso acima de 3, o elogio (0x75c390
/// com 1) e, acima de 12, com 3.
unsafe extern "thiscall" fn add_league_champions_record(this: usize, clube: usize, comp: usize) {
    if clube == 0 || comp == 0 {
        return show_error_at(0xa83b78, 0x21c);
    }
    let obj = cqd13_d(cqd13_d(0xadadfc).wrapping_add(4 * cqd13_d(comp)));
    if obj == 0 {
        return show_error_at(0xa83b78, 0x224);
    }
    let peso = cqd13_w(comp + 0x69);
    raise_champion_reputation(clube, peso, true); // cqd13: a reputação do campeão da liga
    if cqd13_d(clube + 0x57) != 0 {
        let b = cqd13_d(clube + 0x69);
        if b != 0 {
            let x = cqd13_i(clube + 0x7b);
            if x < 150000 && x < cqd13_i(clube + 0x73).wrapping_add(500) {
                cqd13_pi(clube + 0x7b, x.wrapping_add((peso as i32).wrapping_mul(20)));
            }
            let y = cqd13_i(b + 0x44);
            if y < 120000 {
                cqd13_pi(b + 0x44, y.wrapping_add(((peso as i32).wrapping_add(20)).wrapping_mul(25)));
            }
        }
    }
    let n = ((cqd13_w(obj + 0x3e) as i32) - 1).wrapping_mul(cqd13_w(obj + 0x3c) as i32);
    let limiar = match_pl_ftol(n as f64 * read_f64_at(0x968f78)) as i16; // o ftol fica no word (movsx word depois)
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 5;
    put_i32(&mut e, 8, cqd13_i(clube));
    put_i32(&mut e, 0xd, cqd13_i(comp));
    let t = cqd13_d(clube + 0xcf);
    if t != 0 {
        let np = cqd13_d(t + 0x69);
        if np != 0 {
            let (k1, k2) = manager_rating_gains(t, clube, cqd13_d(comp));
            raise_side_rating(np + 0xa, peso, 400);
            raise_side_rating(np + 8, peso, k1);
            raise_side_rating(np + 0xc, peso, k2);
        }
        append_person_history_by_id(!CQD13_ENSAIO, this, cqd13_d(t), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        // 0x86f5b0: push 1; push técnico; mov ecx, edi (o this) -> thiscall (ret 8)
        let r = registro(this, cqd13_d(clube + 0xcf), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + 0x8c) as *mut i16, cqd13_w(r + 0x8c).wrapping_add(1));
            let v = (peso as i32).wrapping_mul(5000);
            cqd13_pi(r + 0xb6, cqd13_i(r + 0xb6).wrapping_add(v));
            cqd13_pi(r + 0xca, cqd13_i(r + 0xca).wrapping_add(v));
            cqd13_pi(r + 0xba, cqd13_i(r + 0xba).wrapping_add(v));
        }
    }
    let presidente = cqd13_d(clube + 0xbf);
    if presidente != 0 {
        append_person_history_by_id(!CQD13_ENSAIO, this, cqd13_d(presidente), &e);
    }
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x544710usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    // 0x7c1e30: push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall (ret 0x10, x87); 0x544710: 3
    // pushes; add esp, 0xc -> cdecl; 0x4cf5f0: push clube; push p; mov ecx, [0xae1050] -> thiscall (ret 8)
    for j in 0..50usize {
        let p = cqd13_d(clube + 0xd7 + 4 * j);
        if p == 0 || cqd13_d(p + 0x61) == 0 || cqd13_i(p) >= cqd13_i(0xae2364) || cqd13_d(p + 0x39) != clube
            || p == cqd13_d(clube + 0xcf)
        {
            continue;
        }
        let f = media(cqd13_d(0xb74350), cqd13_d(p + 0x61), 2, 1, 6.5);
        if f > limiar as f64 { // cqd13: acima do limiar
            crate::comparacao::ramo("cqd_13 jogador destacado do campeao");
            append_person_history_by_id(!CQD13_ENSAIO, this, cqd13_d(p), &e);
            moral(p, 0x14, 1); // cqd13: o moral do jogador destacado
            let k = contrato(cqd13_d(0xae1050), p, clube);
            if k == 0 {
                continue;
            }
            crate::comparacao::ramo("cqd_13 contrato do destacado");
            add_contract_morale(k, 100); // cqd13: o contrato do jogador destacado
            offer_renewal_if_due(clube, p, k);
        } else {
            moral(p, 0xa, 1);
            let k = contrato(cqd13_d(0xae1050), p, clube);
            if k != 0 {
                add_contract_morale(k, 50);
            }
        }
    }
    add_club_history(this, clube, 5, comp);
    if cqd13_d(0xb6b7c8) != 0 && peso > 3 {
        post_trophy_praise(clube, comp, 1);
        if peso > 12 {
            post_trophy_praise(clube, comp, 3);
        }
    }
}

/// { 0x86a800 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::add_cup_winners_record(
/// clube, jogo, partida)`, pelas finais das copas (29 chamadores; 11 chamadas até 2002-06-01). Nulos: a caixa (linha
/// 0x51e). A reputação (como na liga); com técnico (+0xcf) que tem registro de não jogador (+0x69): as três notas do lado
/// do clube na partida (+0x188/+0x18a/+0x18c, lado 0x625 × visitante; peso × 400, × k1, × k2); o histórico do técnico
/// (tipo 0: o clube e a competição de +0x14 do jogo); o registro dele (0x86f5b0 com criar): +1 em +0x8a e ftol(peso^4 ×
/// 0,5) (pow da CRT 0x945ff0) em +0xb6, +0xca e, na seleção (0x53b900), +0xbe, senão +0xba. O presidente (+0xbf) e os
/// 20 da escalação. Os 50 do elenco enquanto a reputação não passar de peso × 400: com ficha, válidos e fora o técnico,
/// moral +15 (0x544710) e +100 no contrato; com a situação do contrato (+0x4f >> 4) 1-3 ou 5, a renovação. As
/// convocações da competição guardada (0x76d940) acabam (0x774f30); o histórico do clube (tipo 0); com a mídia e peso
/// acima de 10, o elogio (0x75c390 com 1) e, com peso acima de 12 ou reputação abaixo de 2500, com 3.
unsafe extern "thiscall" fn add_cup_winners_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return show_error_at(0xa83b78, 0x51e);
    }
    let comp = cqd13_d(jogo + 0x14);
    let peso = cqd13_w(comp + 0x69);
    raise_champion_reputation(clube, peso, false);
    let mut e = history_entry();
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0;
    put_i32(&mut e, 8, cqd13_i(clube));
    put_i32(&mut e, 0xd, cqd13_i(comp));
    let t = cqd13_d(clube + 0xcf);
    if t != 0 {
        if cqd13_d(t + 0x69) != 0 {
            let (k1, k2) = manager_rating_gains(t, clube, cqd13_d(jogo));
            let lado = partida + 0x188 + 0x625 * (cqd13_d(jogo + 0x20) == clube) as usize;
            raise_side_rating(lado, peso, 400);
            raise_side_rating(lado + 2, peso, k1);
            raise_side_rating(lado + 4, peso, k2);
        }
        append_person_history_by_id(!CQD13_ENSAIO, this, cqd13_d(cqd13_d(clube + 0xcf)), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        let r = registro(this, cqd13_d(clube + 0xcf), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + 0x8a) as *mut i16, cqd13_w(r + 0x8a).wrapping_add(1));
            // fild peso; fld 4,0; _CIpow (base em st1); fmul 0,5; ftol
            let v = match_pl_ftol(game_pow(cqd13_w(comp + 0x69) as f64, read_f64_at(0x968938)) * read_f64_at(0x968968)); // cqd13: o peso do título
            cqd13_pi(r + 0xb6, cqd13_i(r + 0xb6).wrapping_add(v));
            cqd13_pi(r + 0xca, cqd13_i(r + 0xca).wrapping_add(v));
            if is_national_team(clube) != 0 {
                cqd13_pi(r + 0xbe, cqd13_i(r + 0xbe).wrapping_add(v));
            } else {
                cqd13_pi(r + 0xba, cqd13_i(r + 0xba).wrapping_add(v));
            }
        }
    }
    let presidente = cqd13_d(clube + 0xbf);
    if presidente != 0 {
        append_person_history_by_id(!CQD13_ENSAIO, this, cqd13_d(presidente), &e);
    }
    let escalacao = partida + 0x190 + 0x625 * (clube != cqd13_d(jogo + 0x1c)) as usize;
    add_lineup_histories(this, clube, escalacao, &e);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x544710usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    for j in 0..50usize {
        if (cqd13_w(comp + 0x69) as i32).wrapping_mul(400) < cqd13_w(clube + 0x80) as i32 {
            break;
        }
        let p = cqd13_d(clube + 0xd7 + 4 * j);
        if p == 0 || cqd13_d(p + 0x61) == 0 || cqd13_i(p) >= cqd13_i(0xae2364) || p == cqd13_d(clube + 0xcf) {
            continue;
        }
        crate::comparacao::ramo("cqd_13 elenco do campeao da copa");
        moral(p, 0xf, 1); // cqd13: o moral do jogador do campeão da copa
        let k = contrato(cqd13_d(0xae1050), p, clube);
        if k == 0 {
            continue;
        }
        crate::comparacao::ramo("cqd_13 moral do elenco campeao da copa");
        add_contract_morale(k, 100); // cqd13: o moral do contrato do campeão da copa
        let s = *((k + 0x4f) as *const u8) >> 4;
        if s == 0 || (s > 3 && s != 5) {
            continue;
        }
        offer_renewal_if_due(clube, p, k);
    }
    let guardada: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x76d940usize);
    // 0x76d940 e 0x774f30: push comp; mov ecx, [0xb74318] (a 0x76d940 não lê o ecx e só compara o id; ret 4 nas duas)
    if guardada(cqd13_d(0xb74318), cqd13_d(jogo + 0x14)) != 0 {
        if CQD13_ENSAIO {
            CQD13_DATA_SOLTA = true;
        } else {
            let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x774f30usize);
            acabar(cqd13_d(0xb74318), cqd13_d(jogo + 0x14));
        }
    }
    add_club_history(this, clube, 0, comp);
    if cqd13_d(0xb6b7c8) != 0 && peso > 10 {
        post_trophy_praise(clube, comp, 1);
        if cqd13_w(comp + 0x69) > 12 || cqd13_w(clube + 0x80) < 0x9c4 {
            post_trophy_praise(clube, comp, 3);
        }
    }
}

/// O comparador em ensaio dos dois campeões: o registro do técnico criado antes (0x86f5b0 com criar, como a original
/// faz), a cópia em ensaio (sem históricos, notícias, convocações e renovação; os sorteios do moral 0x544710 sem contar)
/// e a original a partir do mesmo estado. Compara: a reputação do clube, o registro do técnico (0xd2 bytes), as notas
/// (as três do lado na partida, ou +8..+0xd do registro de não jogador do técnico), o +0x7b do clube e o +0x44 do bloco
/// +0x69 (na liga), o moral (+0x45 da ficha) e o do contrato (+0x44) dos 50 do elenco e a data estática da 0x54cdb0. O
/// moral dos contratos que a renovação mexeria (a original renova) e o estado do sorteador (as notícias da original
/// sorteiam) não entram na conta; o sorteador volta ao de antes para a original. A renovação da original também sorteia
/// e conta dias (0x54cdb0): o moral das fichas depois do primeiro renovado fica fora da igualdade e é conferido pela
/// faixa da 0x544710; a data estática fica fora quando o ensaio pulou, depois da última conta de dias da cópia, uma
/// chamada da original que conta dias (CQD13_DATA_SOLTA). Visto na ENG em maio de 2002 e na auditoria das 8 ligas.
unsafe fn compare_champion_update(nome: &str, this: usize, clube: usize, notas: (usize, usize), liga: bool, original: impl FnOnce(), copia: impl FnOnce()) {
    let mut fixas: Vec<(usize, usize)> = vec![(clube + 0x80, 2), (0xae2470, 8), notas];
    let t = cqd13_d(clube + 0xcf);
    if t != 0 {
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x86f5b0usize);
        let r = registro(this, t, 1);
        if r != 0 {
            fixas.push((r, 0xd2));
        }
    }
    if liga && cqd13_d(clube + 0x57) != 0 && cqd13_d(clube + 0x69) != 0 {
        fixas.push((clube + 0x7b, 4));
        fixas.push((cqd13_d(clube + 0x69) + 0x44, 4));
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x4cf5f0usize);
    let mut contratos = Vec::new();
    let mut fichas = Vec::new(); // (moral da ficha, contrato ou 0, moral de antes), na ordem do elenco
    for j in 0..50usize {
        let p = cqd13_d(clube + 0xd7 + 4 * j);
        if p == 0 || cqd13_d(p + 0x61) == 0 || cqd13_i(p) >= cqd13_i(0xae2364) {
            continue;
        }
        let k = contrato(cqd13_d(0xae1050), p, clube);
        let m = cqd13_d(p + 0x61) + 0x45;
        fichas.push((m, k, *(m as *const i8)));
        if k != 0 {
            contratos.push(k);
        }
    }
    let mut todas = fixas.clone();
    todas.extend(fichas.iter().map(|&(m, _, _)| (m, 1)));
    todas.extend(contratos.iter().map(|&k| (k + 0x44, 1)));
    todas.extend(crate::aleatorio::REGIOES_DO_ESTADO.iter().copied());
    let antes = crate::comparacao::Memoria::ler(&todas);
    (*core::ptr::addr_of_mut!(CQD13_RENOVADOS)).clear();
    CQD13_DATA_SOLTA = false;
    CQD13_ENSAIO = true;
    crate::aleatorio::sem_contar(copia);
    CQD13_ENSAIO = false;
    let renovados = core::mem::take(&mut *core::ptr::addr_of_mut!(CQD13_RENOVADOS));
    let mut conta = fixas;
    if CQD13_DATA_SOLTA {
        conta.retain(|&(a, _)| a != 0xae2470);
    }
    // A renovação da original sorteia (0x4cda70): o moral dos jogadores depois do primeiro renovado (0x544710 sorteia)
    // sai de outro ponto do sorteador; o dele e os de antes entram na conta.
    let corte = renovados.first().and_then(|r| fichas.iter().position(|&(_, k, _)| k == *r)).unwrap_or(fichas.len());
    conta.extend(fichas.iter().take(corte + 1).map(|&(m, _, _)| (m, 1)));
    conta.extend(contratos.iter().filter(|k| !renovados.contains(k)).map(|&k| (k + 0x44, 1)));
    // Os de depois, pela faixa da 0x544710: o moral m sobe de 0 a 20 (+20 ou +10 sorteados), preso em 1..20.
    let band = |&(m, _, m0): &(usize, usize, i8)| -> bool {
        let (m0, v) = (m0 as i32, *(m as *const i8) as i32);
        v >= m0.clamp(1, 20) && v <= (m0 + 20).clamp(1, 20)
    };
    let mut na_faixa = fichas.iter().skip(corte + 1).all(band);
    let mr = crate::comparacao::Memoria::ler(&conta);
    antes.escrever();
    original();
    na_faixa &= fichas.iter().skip(corte + 1).all(band);
    let mo = crate::comparacao::Memoria::ler(&conta);
    PLACAR.registrar(mo == mr && na_faixa, || format!("{}(clube {:#x}) renovados {} (o primeiro na ficha {} de {}; fora da band: {}) original={:x?} rust={:x?}",
        nome, clube, renovados.len(), corte, fichas.len(), !na_faixa, mo, mr));
}

unsafe extern "thiscall" fn comparar_add_league_champions_record(this: usize, clube: usize, comp: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I0%%);
    if clube == 0 || comp == 0 || cqd13_d(cqd13_d(0xadadfc).wrapping_add(4 * cqd13_d(comp))) == 0 {
        return o(this, clube, comp);
    }
    let t = cqd13_d(clube + 0xcf);
    let notas = if t != 0 && cqd13_d(t + 0x69) != 0 { (cqd13_d(t + 0x69) + 8, 6) } else { (clube + 0x80, 2) };
    compare_champion_update("add_league_champions_record", this, clube, notas, true, || o(this, clube, comp),
        || add_league_champions_record(this, clube, comp));
}

unsafe extern "thiscall" fn comparar_add_cup_winners_record(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I1%%);
    if clube == 0 || jogo == 0 || partida == 0 {
        return o(this, clube, jogo, partida);
    }
    let notas = (partida + 0x188 + 0x625 * (cqd13_d(jogo + 0x20) == clube) as usize, 6);
    compare_champion_update("add_cup_winners_record", this, clube, notas, false, || o(this, clube, jogo, partida),
        || add_cup_winners_record(this, clube, jogo, partida));
}

/// { 0x86fc70 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::clear_human_manager_record(pessoa)`, pela
/// aposentadoria do técnico humano (0x6b2b40) e pelo setup (0x82b9xx). Pessoa entre os 16 humanos (id de [0xae2364] − 16
/// até [0xae2364]): o registro dele no vetor +8 (vaga id − [0xae2364] + [0xae2368] + 16, 0xd2 bytes); senão, como a
/// original, a própria pessoa. Zera os 0xd2 bytes e põe −1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e e +0x72.
unsafe extern "thiscall" fn clear_human_manager_record(this: usize, pessoa: usize) {
    let id = cqd13_i(pessoa);
    let lim = cqd13_i(0xae2364);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let vaga = id.wrapping_sub(lim).wrapping_add(cqd13_i(0xae2368)).wrapping_add(0x10);
        cqd13_d(this + 8).wrapping_add((vaga as usize).wrapping_mul(0xd2))
    } else {
        pessoa
    };
    core::ptr::write_bytes(r as *mut u8, 0, 0xd2);
    for o in [0x86usize, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        cqd13_pi(r + o, -1);
    }
}

/// Comparador: lado a lado pelo registro (os 0xd2 bytes que a rotina zera).
unsafe extern "thiscall" fn comparar_clear_human_manager_record(this: usize, pessoa: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%);
    let id = cqd13_i(pessoa);
    let lim = cqd13_i(0xae2364);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let vaga = id.wrapping_sub(lim).wrapping_add(cqd13_i(0xae2368)).wrapping_add(0x10);
        cqd13_d(this + 8).wrapping_add((vaga as usize).wrapping_mul(0xd2))
    } else {
        pessoa
    };
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, pessoa),
        || clear_human_manager_record(this, pessoa));
    PLACAR.registrar(igual, || format!("clear_human_manager_record(pessoa {:#x}) {:x?} x {:x?}", pessoa, mo, mr));
}
