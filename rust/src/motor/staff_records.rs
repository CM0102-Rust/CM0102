use super::*;

// ---------- 267a leva: os registros do campeão da liga e da copa e o registro do técnico humano zerado (staff_records.cpp) ----------


pub(crate) unsafe fn cqd13_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}


pub(crate) unsafe fn cqd13_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// O ensaio dos comparadores: as cópias sem os históricos (arquivo), sem as notícias (0x75c390), sem acabar as
/// convocações (0x774f30) e sem a renovação (0x4cda70), anotando os contratos que a renovação mexeria.
pub(crate) static mut CQD13_ENSAIO: bool = false;
pub(crate) static mut CQD13_RENOVADOS: Vec<usize> = Vec::new();
/// Depois da última conta de dias (0x54cdb0) da cópia, o ensaio pulou uma chamada da original que também conta dias
/// e deixa a data de rascunho estática dela (0xae2470) noutro ano: a renovação (0x4cda70, pela idade), o fim das
/// convocações (0x774f30) ou a notícia do elogio (0x75c390). Essa data então fica fora da conta.
pub(crate) static mut CQD13_DATA_SOLTA: bool = false;

/// A reputação do clube (+0x80) sobe até peso × 500, no máximo 750 de uma vez (entre 1 e 10000; soma em 16 bits).
pub(crate) unsafe fn cqd13_reputacao(clube: usize, peso: i16, campeao_da_liga: bool) {
    let alvo = (peso as i32).wrapping_mul(500);
    let rep = read_i16(clube + 0x80) as i32;
    if rep < alvo {
        let falta = alvo - rep;
        let soma = if falta > 0x2ee { 0x2ee } else { falta }; // cqd13: o teto da subida
        let v = read_i16(clube + 0x80).wrapping_add(soma as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if campeao_da_liga {
            crate::comparacao::ramo("cqd_13 reputacao do campeao da liga");
        } else {
            crate::comparacao::ramo("cqd_13 reputacao do campeao da copa");
        }
        core::ptr::write_unaligned((clube + 0x80) as *mut i16, v); // cqd13: a reputação do campeão
    }
}

/// O moral do contrato (+0x44, byte com sinal) sobe `d`, preso em −100..100.
pub(crate) unsafe fn cqd13_moral(k: usize, d: i32) {
    let c = *((k + 0x44) as *const i8);
    let x = c as i32 + d;
    *((k + 0x44) as *mut i8) = if x > 100 { 100 } else if x < -100 { -100 } else { c.wrapping_add(d as i8) };
}

/// A renovação oferecida ao jogador do próprio clube (0x4cda70 com 1,0 e submeter): com o gerente de transferências
/// ([0xde1f54]), mais de 300 dias desde a data do contrato (+0x25; 0x54cdb0) e a vaga dele na lista do clube (0x849ce0
/// com 0, 0; +0xf o id). No ensaio, só anota o contrato.
pub(crate) unsafe fn cqd13_renovar(clube: usize, p: usize, k: usize) {
    if read_usize(0xde1f54) == 0 {
        return;
    }
    CQD13_DATA_SOLTA = false;
    if cqa12_dias(0xae2c90, k + 0x25) <= 300 {
        return;
    }
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    // 0x849ce0: push 0; push 0; push p; push clube; mov ecx, [[0xde1f54]+0x856] -> thiscall (ret 0x10)
    let v = vaga_de(read_usize(read_usize(0xde1f54) + 0x856), clube, p, 0, 0);
    if v == 0 || cqd13_i(v + 0xf) != cqd13_i(p) {
        return;
    }
    if CQD13_ENSAIO {
        (*core::ptr::addr_of_mut!(CQD13_RENOVADOS)).push(k);
        CQD13_DATA_SOLTA = true;
        return;
    }
    let oferecer: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, f64, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    // 0x4cda70: push 1; push 0x3ff00000; push 0; push −1; push 0; push 0; push p; mov ecx, [0xae1050] -> thiscall
    // (p, 0, 0, −1, 1.0 em dois dwords, 1)
    oferecer(read_usize(0xae1050), p, 0, 0, -1, 1.0, 1);
}

/// Os históricos da entrada `e` para o técnico (já feito por quem chama), o presidente (+0xbf) e os 20 da escalação
/// (+0x190 do lado, 0x49 bytes, −1 vazio; pessoas válidas e fora o técnico; nula: a caixa da linha 0x1010).
pub(crate) unsafe fn cqd13_escalacao(this: usize, clube: usize, escalacao: usize, e: &[u8; 0x40]) {
    for j in 0..0x14usize {
        let id = cqd13_i(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = read_usize(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && cqd13_i(p) >= cqd13_i(0xae2364) {
            continue;
        }
        if p == read_usize(clube + 0xcf) {
            continue;
        }
        if p == 0 {
            aviso_em(0xa83b78, 0x1010);
            continue;
        }
        cpq4_historico(!CQD13_ENSAIO, this, read_usize(p), e);
    }
}

/// O histórico do clube (tipo no +8, a competição no +9).
pub(crate) unsafe fn cqd13_historico_do_clube(this: usize, clube: usize, tipo: u8, comp: usize) {
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[8] = tipo;
    cpq4_por(&mut e, 9, cqd13_i(comp));
    if !CQD13_ENSAIO {
        let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
        let p = read_usize(this + 0x10).wrapping_add(read_usize(clube).wrapping_mul(8));
        f(read_usize(this + 4), p, p + 4, e.as_ptr());
    }
}

/// A notícia do elogio do título (0x75c390 em [0xb6b7c8]; fora do ensaio).
pub(crate) unsafe fn cqd13_elogio(clube: usize, comp: usize, marca: u32) {
    if CQD13_ENSAIO {
        CQD13_DATA_SOLTA = true;
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(FN_MEDIA_QUEUE_TROPHY_PRAISE_STORY);
    // 0x75c390: push marca; push comp; push clube; mov ecx, [0xb6b7c8] -> thiscall (ret 0xc)
    f(read_usize(0xb6b7c8), clube, comp, marca);
}

/// As notas k1/k2 do técnico pela fase: na competição de fase 1 ([0xadadfc] + 4 × id, +0x42 = 1) 200 na terceira e 200
/// na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1), senão 400.
pub(crate) unsafe fn cqd13_notas(t: usize, clube: usize, id_comp: usize) -> (i32, i32) {
    let c = read_usize(read_usize(0xadadfc).wrapping_add(4 * id_comp));
    if *((c + 0x42) as *const i8) != 1 {
        (400, 400)
    } else {
        let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
        // 0x53d980: 3 pushes; add esp, 0xc -> cdecl
        (if estrangeiro(t, read_usize(clube + 0x53), 1) != 0 { 200 } else { 400 }, 200)
    }
}

/// the league champion: the club reputation (up to weight × 500), the +0x7b and the +0x69 block, the manager's ratings
/// and record (weight × 5000), the histories (type 5), the squad morale by the player's average (above the threshold:
/// +20 and +100 on the contract with the renewal; else +10 and +50) and the media praise
/// Original: `Staff_Records::add_league_champions_record` (0x868c50, staff_records.cpp, thiscall).
///
/// [pt] { 0x868c50 } (thiscall, ret 8, staff_records.cpp, SEH só das datas locais): `Staff_Records::
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
pub(crate) unsafe extern "thiscall" fn campeao_da_liga_nos_registros(this: usize, clube: usize, comp: usize) {
    if clube == 0 || comp == 0 {
        return aviso_em(0xa83b78, 0x21c);
    }
    let obj = read_usize(read_usize(0xadadfc).wrapping_add(4 * read_usize(comp)));
    if obj == 0 {
        return aviso_em(0xa83b78, 0x224);
    }
    let peso = read_i16(comp + 0x69);
    cqd13_reputacao(clube, peso, true); // cqd13: a reputação do campeão da liga
    if read_usize(clube + 0x57) != 0 {
        let b = read_usize(clube + 0x69);
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
    let n = ((read_i16(obj + 0x3e) as i32) - 1).wrapping_mul(read_i16(obj + 0x3c) as i32);
    let limiar = ftol(n as f64 * f64_em(0x968f78)) as i16; // o ftol fica no word (movsx word depois)
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 5;
    cpq4_por(&mut e, 8, cqd13_i(clube));
    cpq4_por(&mut e, 0xd, cqd13_i(comp));
    let t = read_usize(clube + 0xcf);
    if t != 0 {
        let np = read_usize(t + 0x69);
        if np != 0 {
            let (k1, k2) = cqd13_notas(t, clube, read_usize(comp));
            cqa12_subir(np + 0xa, peso, 400);
            cqa12_subir(np + 8, peso, k1);
            cqa12_subir(np + 0xc, peso, k2);
        }
        cpq4_historico(!CQD13_ENSAIO, this, read_usize(t), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        // 0x86f5b0: push 1; push técnico; mov ecx, edi (o this) -> thiscall (ret 8)
        let r = registro(this, read_usize(clube + 0xcf), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + 0x8c) as *mut i16, read_i16(r + 0x8c).wrapping_add(1));
            let v = (peso as i32).wrapping_mul(5000);
            cqd13_pi(r + 0xb6, cqd13_i(r + 0xb6).wrapping_add(v));
            cqd13_pi(r + 0xca, cqd13_i(r + 0xca).wrapping_add(v));
            cqd13_pi(r + 0xba, cqd13_i(r + 0xba).wrapping_add(v));
        }
    }
    let presidente = read_usize(clube + 0xbf);
    if presidente != 0 {
        cpq4_historico(!CQD13_ENSAIO, this, read_usize(presidente), &e);
    }
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(FN_PLAYERSTATS_GET_STAT);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    // 0x7c1e30: push 6,5f; push 1; push 2; push ficha; mov ecx, [0xb74350] -> thiscall (ret 0x10, x87); 0x544710: 3
    // pushes; add esp, 0xc -> cdecl; 0x4cf5f0: push clube; push p; mov ecx, [0xae1050] -> thiscall (ret 8)
    for j in 0..50usize {
        let p = read_usize(clube + 0xd7 + 4 * j);
        if p == 0 || read_usize(p + 0x61) == 0 || cqd13_i(p) >= cqd13_i(0xae2364) || read_usize(p + 0x39) != clube
            || p == read_usize(clube + 0xcf)
        {
            continue;
        }
        let f = media(read_usize(0xb74350), read_usize(p + 0x61), 2, 1, 6.5);
        if f > limiar as f64 { // cqd13: acima do limiar
            crate::comparacao::ramo("cqd_13 jogador destacado do campeao");
            cpq4_historico(!CQD13_ENSAIO, this, read_usize(p), &e);
            moral(p, 0x14, 1); // cqd13: o moral do jogador destacado
            let k = contrato(read_usize(0xae1050), p, clube);
            if k == 0 {
                continue;
            }
            crate::comparacao::ramo("cqd_13 contrato do destacado");
            cqd13_moral(k, 100); // cqd13: o contrato do jogador destacado
            cqd13_renovar(clube, p, k);
        } else {
            moral(p, 0xa, 1);
            let k = contrato(read_usize(0xae1050), p, clube);
            if k != 0 {
                cqd13_moral(k, 50);
            }
        }
    }
    cqd13_historico_do_clube(this, clube, 5, comp);
    if read_usize(0xb6b7c8) != 0 && peso > 3 {
        cqd13_elogio(clube, comp, 1);
        if peso > 12 {
            cqd13_elogio(clube, comp, 3);
        }
    }
}

/// the cup winner: the club reputation, its side's ratings in the match, the manager's record (weight^4 × 0.5), the
/// histories (type 0), the squad morale (+15 and +100 on the contract, with the renewal) while the reputation stays up
/// to weight × 400, the end of the call-ups and the media praise
/// Original: `Staff_Records::add_cup_winners_record` (0x86a800, staff_records.cpp, thiscall).
///
/// [pt] { 0x86a800 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::add_cup_winners_record(
/// clube, jogo, partida)`, pelas finais das copas (29 chamadores; 11 chamadas até 2002-06-01). Nulos: a caixa (linha
/// 0x51e). A reputação (como na liga); com técnico (+0xcf) que tem registro de não jogador (+0x69): as três notas do lado
/// do clube na partida (+0x188/+0x18a/+0x18c, lado 0x625 × visitante; peso × 400, × k1, × k2); o histórico do técnico
/// (tipo 0: o clube e a competição de +0x14 do jogo); o registro dele (0x86f5b0 com criar): +1 em +0x8a e ftol(peso^4 ×
/// 0,5) (pow da CRT 0x945ff0) em +0xb6, +0xca e, na seleção (0x53b900), +0xbe, senão +0xba. O presidente (+0xbf) e os
/// 20 da escalação. Os 50 do elenco enquanto a reputação não passar de peso × 400: com ficha, válidos e fora o técnico,
/// moral +15 (0x544710) e +100 no contrato; com a situação do contrato (+0x4f >> 4) 1-3 ou 5, a renovação. As
/// convocações da competição guardada (0x76d940) acabam (0x774f30); o histórico do clube (tipo 0); com a mídia e peso
/// acima de 10, o elogio (0x75c390 com 1) e, com peso acima de 12 ou reputação abaixo de 2500, com 3.
pub(crate) unsafe extern "thiscall" fn campeao_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return aviso_em(0xa83b78, 0x51e);
    }
    let comp = read_usize(jogo + 0x14);
    let peso = read_i16(comp + 0x69);
    cqd13_reputacao(clube, peso, false);
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0;
    cpq4_por(&mut e, 8, cqd13_i(clube));
    cpq4_por(&mut e, 0xd, cqd13_i(comp));
    let t = read_usize(clube + 0xcf);
    if t != 0 {
        if read_usize(t + 0x69) != 0 {
            let (k1, k2) = cqd13_notas(t, clube, read_usize(jogo));
            let lado = partida + 0x188 + 0x625 * (read_usize(jogo + 0x20) == clube) as usize;
            cqa12_subir(lado, peso, 400);
            cqa12_subir(lado + 2, peso, k1);
            cqa12_subir(lado + 4, peso, k2);
        }
        cpq4_historico(!CQD13_ENSAIO, this, read_usize(read_usize(clube + 0xcf)), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = registro(this, read_usize(clube + 0xcf), 1);
        if r != 0 {
            core::ptr::write_unaligned((r + 0x8a) as *mut i16, read_i16(r + 0x8a).wrapping_add(1));
            // fild peso; fld 4,0; _CIpow (base em st1); fmul 0,5; ftol
            let v = ftol(pow_do_jogo(read_i16(comp + 0x69) as f64, f64_em(0x968938)) * f64_em(0x968968)); // cqd13: o peso do título
            cqd13_pi(r + 0xb6, cqd13_i(r + 0xb6).wrapping_add(v));
            cqd13_pi(r + 0xca, cqd13_i(r + 0xca).wrapping_add(v));
            if e_selecao(clube) != 0 {
                cqd13_pi(r + 0xbe, cqd13_i(r + 0xbe).wrapping_add(v));
            } else {
                cqd13_pi(r + 0xba, cqd13_i(r + 0xba).wrapping_add(v));
            }
        }
    }
    let presidente = read_usize(clube + 0xbf);
    if presidente != 0 {
        cpq4_historico(!CQD13_ENSAIO, this, read_usize(presidente), &e);
    }
    let escalacao = partida + 0x190 + 0x625 * (clube != read_usize(jogo + 0x1c)) as usize;
    cqd13_escalacao(this, clube, escalacao, &e);
    let moral: unsafe extern "C" fn(usize, u32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    for j in 0..50usize {
        if (read_i16(comp + 0x69) as i32).wrapping_mul(400) < read_i16(clube + 0x80) as i32 {
            break;
        }
        let p = read_usize(clube + 0xd7 + 4 * j);
        if p == 0 || read_usize(p + 0x61) == 0 || cqd13_i(p) >= cqd13_i(0xae2364) || p == read_usize(clube + 0xcf) {
            continue;
        }
        crate::comparacao::ramo("cqd_13 elenco do campeao da copa");
        moral(p, 0xf, 1); // cqd13: o moral do jogador do campeão da copa
        let k = contrato(read_usize(0xae1050), p, clube);
        if k == 0 {
            continue;
        }
        crate::comparacao::ramo("cqd_13 moral do elenco campeao da copa");
        cqd13_moral(k, 100); // cqd13: o moral do contrato do campeão da copa
        let s = *((k + 0x4f) as *const u8) >> 4;
        if s == 0 || (s > 3 && s != 5) {
            continue;
        }
        cqd13_renovar(clube, p, k);
    }
    let guardada: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    // 0x76d940 e 0x774f30: push comp; mov ecx, [0xb74318] (a 0x76d940 não lê o ecx e só compara o id; ret 4 nas duas)
    if guardada(read_usize(0xb74318), read_usize(jogo + 0x14)) != 0 {
        if CQD13_ENSAIO {
            CQD13_DATA_SOLTA = true;
        } else {
            let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_END_DUTY_FOR_COMPETITION);
            acabar(read_usize(0xb74318), read_usize(jogo + 0x14));
        }
    }
    cqd13_historico_do_clube(this, clube, 0, comp);
    if read_usize(0xb6b7c8) != 0 && peso > 10 {
        cqd13_elogio(clube, comp, 1);
        if read_i16(comp + 0x69) > 12 || read_i16(clube + 0x80) < 0x9c4 {
            cqd13_elogio(clube, comp, 3);
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
pub(crate) unsafe fn cqd13_comparar(nome: &str, this: usize, clube: usize, notas: (usize, usize), liga: bool, original: impl FnOnce(), copia: impl FnOnce()) {
    let mut fixas: Vec<(usize, usize)> = vec![(clube + 0x80, 2), (0xae2470, 8), notas];
    let t = read_usize(clube + 0xcf);
    if t != 0 {
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = registro(this, t, 1);
        if r != 0 {
            fixas.push((r, 0xd2));
        }
    }
    if liga && read_usize(clube + 0x57) != 0 && read_usize(clube + 0x69) != 0 {
        fixas.push((clube + 0x7b, 4));
        fixas.push((read_usize(clube + 0x69) + 0x44, 4));
    }
    let contrato: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    let mut contratos = Vec::new();
    let mut fichas = Vec::new(); // (moral da ficha, contrato ou 0, moral de antes), na ordem do elenco
    for j in 0..50usize {
        let p = read_usize(clube + 0xd7 + 4 * j);
        if p == 0 || read_usize(p + 0x61) == 0 || cqd13_i(p) >= cqd13_i(0xae2364) {
            continue;
        }
        let k = contrato(read_usize(0xae1050), p, clube);
        let m = read_usize(p + 0x61) + 0x45;
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
    let faixa = |&(m, _, m0): &(usize, usize, i8)| -> bool {
        let (m0, v) = (m0 as i32, *(m as *const i8) as i32);
        v >= m0.clamp(1, 20) && v <= (m0 + 20).clamp(1, 20)
    };
    let mut na_faixa = fichas.iter().skip(corte + 1).all(faixa);
    let mr = crate::comparacao::Memoria::ler(&conta);
    antes.escrever();
    original();
    na_faixa &= fichas.iter().skip(corte + 1).all(faixa);
    let mo = crate::comparacao::Memoria::ler(&conta);
    PLACAR.registrar(mo == mr && na_faixa, || format!("{}(clube {:#x}) renovados {} (o primeiro na ficha {} de {}; fora da faixa: {}) original={:x?} rust={:x?}",
        nome, clube, renovados.len(), corte, fichas.len(), !na_faixa, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_campeao_da_liga_nos_registros(this: usize, clube: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1576);
    if clube == 0 || comp == 0 || read_usize(read_usize(0xadadfc).wrapping_add(4 * read_usize(comp))) == 0 {
        return o(this, clube, comp);
    }
    let t = read_usize(clube + 0xcf);
    let notas = if t != 0 && read_usize(t + 0x69) != 0 { (read_usize(t + 0x69) + 8, 6) } else { (clube + 0x80, 2) };
    cqd13_comparar("campeao_da_liga_nos_registros", this, clube, notas, true, || o(this, clube, comp),
        || campeao_da_liga_nos_registros(this, clube, comp));
}

pub(crate) unsafe extern "thiscall" fn comparar_campeao_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1577);
    if clube == 0 || jogo == 0 || partida == 0 {
        return o(this, clube, jogo, partida);
    }
    let notas = (partida + 0x188 + 0x625 * (read_usize(jogo + 0x20) == clube) as usize, 6);
    cqd13_comparar("campeao_da_copa_nos_registros", this, clube, notas, false, || o(this, clube, jogo, partida),
        || campeao_da_copa_nos_registros(this, clube, jogo, partida));
}

/// the human manager record (one of the 16; else the person itself, as in the original) cleared, with −1 in the slots
/// Original: `Staff_Records::clear_human_manager_record` (0x86fc70, staff_records.cpp, thiscall).
///
/// [pt] { 0x86fc70 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::clear_human_manager_record(pessoa)`, pela
/// aposentadoria do técnico humano (0x6b2b40) e pelo setup (0x82b9xx). Pessoa entre os 16 humanos (id de [0xae2364] − 16
/// até [0xae2364]): o registro dele no vetor +8 (vaga id − [0xae2364] + [0xae2368] + 16, 0xd2 bytes); senão, como a
/// original, a própria pessoa. Zera os 0xd2 bytes e põe −1 em +0x86, +0xc2, +0xc6, +0x4a, +0x5e e +0x72.
pub(crate) unsafe extern "thiscall" fn limpar_registro_do_humano(this: usize, pessoa: usize) {
    let id = cqd13_i(pessoa);
    let lim = cqd13_i(0xae2364);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let vaga = id.wrapping_sub(lim).wrapping_add(cqd13_i(0xae2368)).wrapping_add(0x10);
        read_usize(this + 8).wrapping_add((vaga as usize).wrapping_mul(0xd2))
    } else {
        pessoa
    };
    core::ptr::write_bytes(r as *mut u8, 0, 0xd2);
    for o in [0x86usize, 0xc2, 0xc6, 0x4a, 0x5e, 0x72] {
        cqd13_pi(r + o, -1);
    }
}

/// Comparador: lado a lado pelo registro (os 0xd2 bytes que a rotina zera).
pub(crate) unsafe extern "thiscall" fn comparar_limpar_registro_do_humano(this: usize, pessoa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1578);
    let id = cqd13_i(pessoa);
    let lim = cqd13_i(0xae2364);
    let r = if id >= lim.wrapping_sub(0x10) && id < lim {
        let vaga = id.wrapping_sub(lim).wrapping_add(cqd13_i(0xae2368)).wrapping_add(0x10);
        read_usize(this + 8).wrapping_add((vaga as usize).wrapping_mul(0xd2))
    } else {
        pessoa
    };
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, pessoa),
        || limpar_registro_do_humano(this, pessoa));
    PLACAR.registrar(igual, || format!("limpar_registro_do_humano(pessoa {:#x}) {:x?} x {:x?}", pessoa, mo, mr));
}

// ---------- 263a leva: os registros dos técnicos: o resultado de cada jogo, a posição no fim da liga e o jogador do ano da torcida (staff_records.cpp) ----------


pub(crate) unsafe fn cqa11_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}


/// O byte com sinal em `a`.
pub(crate) unsafe fn cqa11_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

pub(crate) unsafe fn cqa11_somar(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, cqa11_i(a).wrapping_add(v));
}

pub(crate) unsafe fn cqa11_mais1(a: usize) {
    core::ptr::write_unaligned(a as *mut i16, read_i16(a).wrapping_add(1));
}

pub(crate) unsafe fn cqa11_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    f(n)
}

/// O registro de pontos do técnico do clube (+0xcf, lido sem conferir o clube; 0x86f5b0 com criar), ou 0 sem técnico.
pub(crate) unsafe fn cqa11_registro(this: usize, clube: usize) -> usize {
    let t = read_usize(clube + 0xcf);
    if t == 0 {
        return 0;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    f(this, t, 1)
}

/// Os pontos de um lado na partida `m`: ((a + b) × 5 + (c − b, em 8 bits)) × 2 + d, vezes 5 (bytes com sinal).
pub(crate) unsafe fn cqa11_pontos(m: usize, a: usize, b: usize, c: usize, d: usize) -> i32 {
    let x = cqa11_c(m + a).wrapping_add(cqa11_c(m + b));
    let y = (*((m + c) as *const u8)).wrapping_sub(*((m + b) as *const u8)) as i8 as i32;
    let x = x.wrapping_mul(5).wrapping_add(y);
    cqa11_c(m + d).wrapping_add(x.wrapping_mul(2)).wrapping_mul(5)
}

/// A nota do jogo para o técnico: a importância (0x48e190 da competição do jogo, com `lado` e a saída em `saida`; só o
/// byte, com sinal) × (reputação do adversário / 500) × o peso da competição (+0x69 de [+0x14]) × (pontos + bônus) × 50,
/// dividido por 100000 (divisões com sinal truncadas, como as multiplicações mágicas da original; produtos em 32 bits).
pub(crate) unsafe fn cqa11_nota(p: usize, lado: u32, saida: &mut u32, adversario: usize, pontos: i32) -> i32 {
    let comp = read_usize(read_usize(0xadadfc).wrapping_add(4 * read_usize(p)));
    let importancia: unsafe extern "thiscall" fn(usize, usize, u32, *mut u32) -> u32 = core::mem::transmute(FN_COMP_GET_FIXTURE_MATCH_IMPORTANCE);
    let i = importancia(comp, p, lado, saida as *mut u32) as u8 as i8 as i32;
    let x = i
        .wrapping_mul(read_i16(adversario + 0x80) as i32 / 500)
        .wrapping_mul(read_i16(read_usize(p + 0x14) + 0x69) as i32)
        .wrapping_mul(pontos)
        .wrapping_mul(50);
    x / 100000
}

/// in both managers' records: the match, the goals, the win, draw or loss and the match rating (importance × opponent
/// reputation × weight × points) in the month, the year, the club's league and the club or national season; the away
/// manager gets the home rating in the season total (original bug)
/// Original: `Staff_Records::record_match_result` (0x868830, staff_records.cpp, thiscall).
///
/// [pt] { 0x868830 } (thiscall, ret 8, staff_records.cpp): `Staff_Records::record_match_result(jogo, partida)`, pelo
/// fechamento da partida do dia (0x6b5f00), a cada jogo. Jogo não disputado (+0x3a = 1): nada. Os registros dos dois
/// técnicos (+0xcf do mandante +0x1c e do visitante +0x20; 0x86f5b0 com criar): +1 jogo (+0x8e); os gols pró (+0x96) e
/// contra (+0x9a) pelo placar (+0x47/+0x48, ou +0x43/+0x44 com +0x47 = −1); vitória (+0x90, bônus 900), empate (+0x92,
/// 300 para os dois) ou derrota (+0x94). A nota de cada um (0x48e190 com 0 / 1, a reputação do adversário, os pontos da
/// partida em +0x11f.. / +0x744..) vai para +0xb6 e +0xca; na liga do clube (+0x57 ou +0x60 = a competição +0x14) também
/// para os pontos do mês (+0xce); na seleção (0x53b900) para +0xbe, senão +0xba. Defeito da original, reproduzido: no
/// visitante, +0xbe/+0xba recebem o [esp+0x10] da rotina, que é a nota do mandante (ou o bônus dele, sem registro).
pub(crate) unsafe extern "thiscall" fn resultado_nos_registros_dos_tecnicos(this: usize, p: usize, m: usize) {
    if *((p + 0x3a) as *const u8) == 1 {
        return;
    }
    let (casa, fora) = (read_usize(p + 0x1c), read_usize(p + 0x20));
    let a = cqa11_registro(this, casa);
    let v = cqa11_registro(this, fora);
    if a != 0 {
        cqa11_mais1(a + 0x8e);
    }
    if v != 0 {
        cqa11_mais1(v + 0x8e);
    }
    let (ga, gv) = if *((p + 0x47) as *const u8) == 0xff {
        (cqa11_c(p + 0x43), cqa11_c(p + 0x44))
    } else {
        (cqa11_c(p + 0x47), cqa11_c(p + 0x48))
    };
    if a != 0 {
        cqa11_somar(a + 0x96, ga);
        cqa11_somar(a + 0x9a, gv);
    }
    if v != 0 {
        cqa11_somar(v + 0x96, gv);
        cqa11_somar(v + 0x9a, ga);
    }
    let (mut ba, mut bv) = (0i32, 0i32);
    if ga > gv {
        ba = 900; // cqa11: o bônus da vitória do mandante
        if a != 0 {
            crate::comparacao::ramo("cqa_11 vitoria do mandante");
            cqa11_mais1(a + 0x90);
        }
        if v != 0 {
            cqa11_mais1(v + 0x94);
        }
    } else if ga < gv {
        bv = 900;
        if v != 0 {
            cqa11_mais1(v + 0x90);
        }
        if a != 0 {
            cqa11_mais1(a + 0x94);
        }
    } else {
        ba = 300;
        bv = 300;
        if a != 0 {
            cqa11_mais1(a + 0x92);
        }
        if v != 0 {
            cqa11_mais1(v + 0x92);
        }
    }
    // a saída da 0x48e190 vai na vaga do argumento `jogo` da original, que começa com o próprio jogo
    let mut saida = p as u32;
    let mut guardado = ba; // o [esp+0x10] da original: o bônus do mandante, depois a nota dele
    let comp = read_usize(p + 0x14);
    if a != 0 {
        crate::comparacao::ramo("cqa_11 nota do mandante");
        let q = cqa11_nota(p, 0, &mut saida, fora, cqa11_pontos(m, 0x11f, 0x128, 0x127, 0x129).wrapping_add(ba));
        cqa11_somar(a + 0xb6, q);
        cqa11_somar(a + 0xca, q);
        guardado = q;
        if comp == read_usize(casa + 0x57) || comp == read_usize(casa + 0x60) {
            crate::comparacao::ramo("cqa_11 pontos do mes");
            cqa11_somar(a + 0xce, q); // cqa11: os pontos do mês do mandante
        }
        if e_selecao(casa) != 0 {
            cqa11_somar(a + 0xbe, q);
        } else {
            cqa11_somar(a + 0xba, q);
        }
    }
    if v != 0 {
        let q = cqa11_nota(p, 1, &mut saida, casa, cqa11_pontos(m, 0x744, 0x74d, 0x74c, 0x74e).wrapping_add(bv));
        cqa11_somar(v + 0xb6, q);
        cqa11_somar(v + 0xca, q);
        if comp == read_usize(fora + 0x57) || comp == read_usize(fora + 0x60) {
            cqa11_somar(v + 0xce, q);
        }
        if e_selecao(fora) != 0 {
            cqa11_somar(v + 0xbe, guardado);
        } else {
            cqa11_somar(v + 0xba, guardado);
        }
    }
}

/// Comparador lado a lado nos dois registros (0xd2 bytes), criados antes pela 0x86f5b0 como a original faria (a segunda
/// chamada acha o mesmo registro). Jogo não disputado: só a original (não cria nada).
pub(crate) unsafe extern "thiscall" fn comparar_resultado_nos_registros_dos_tecnicos(this: usize, p: usize, m: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1449);
    if *((p + 0x3a) as *const u8) == 1 {
        return o(this, p, m);
    }
    let a = cqa11_registro(this, read_usize(p + 0x1c));
    let v = cqa11_registro(this, read_usize(p + 0x20));
    let mut reg = Vec::new();
    for r in [a, v] {
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, p, m), || resultado_nos_registros_dos_tecnicos(this, p, m));
    PLACAR.registrar(igual, || format!("resultado_nos_registros_dos_tecnicos(jogo {:#x}) original={:x?} rust={:x?}", p, mo, mr));
}

/// at the end of the league, each club's final position in its manager's history
/// Original: `Staff_Records::add_league_finish_records` (0x86a070, staff_records.cpp, thiscall).
///
/// [pt] { 0x86a070 } (thiscall, ret 4, staff_records.cpp, SEH só da data local): `Staff_Records::add_league_finish_records(
/// liga)`, pela liga completa (0x689ad0), no fim da liga. Para cada linha da tabela (0x6826d0, até +0x3e), o histórico do
/// técnico do clube (+0xcf; tipo 0xa: o id do clube, a competição de +4, o ano +0x3a e a posição k + 1), no arquivo
/// (0x58e340).
pub(crate) unsafe extern "thiscall" fn posicoes_no_fim_da_liga(this: usize, liga: usize) {
    crate::comparacao::ramo("cqa_11 fim da liga");
    let linha: unsafe extern "thiscall" fn(usize, i32) -> usize = core::mem::transmute(FN_LEAGUE_GET_TABLE_ROW);
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0xa;
    cpq4_por(&mut e, 0x11, read_i16(liga + 0x3a) as i32);
    cpq4_por(&mut e, 0xd, cqa11_i(read_usize(liga + 4)));
    let mut k: i16 = 0;
    while k < read_i16(liga + 0x3e) {
        let clube = read_usize(linha(liga, k as i32));
        cpq4_por(&mut e, 0x15, k as i32 + 1);
        cpq4_por(&mut e, 8, cqa11_i(clube));
        let t = read_usize(clube + 0xcf);
        if t != 0 {
            cpq4_historico(true, this, read_usize(t), &e);
        }
        k += 1;
    }
}

// ponytail: só grava o histórico no arquivo (não roda duas vezes): só a original; a cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_posicoes_no_fim_da_liga(this: usize, liga: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(1450)(this, liga)
}

/// O ensaio do comparador da 0x86dc30: a cópia sem o histórico (arquivo).
pub(crate) static mut CQA11_ENSAIO: bool = false;

/// the supporters' player of the year in his history and, by a draw, the favourite link between him and the club
/// Original: `Staff_Records::add_supporters_player_of_year_record` (0x86dc30, staff_records.cpp, thiscall).
///
/// [pt] { 0x86dc30 } (thiscall, ret 0x10, staff_records.cpp, SEH só da data local): `Staff_Records::
/// add_supporters_player_of_year_record(pessoa, clube, a, b)`, pelo fim da temporada dos recordes (0x447880). O histórico
/// da pessoa (tipo 0x35: o id do clube, `a` em 16 bits sem sinal e `b`), no arquivo (0x58e340); depois, se sorteio(21 −
/// +0x59 da pessoa) < sorteio(10), a pessoa gosta do clube (0x547d90 com sorteio(2)) e o clube dela (0x547870 com
/// sorteio(2)). Pessoa nula: a original (a caixa e a leitura de +0x59 da pessoa nula).
pub(crate) unsafe extern "thiscall" fn jogador_do_ano_da_torcida_no_registro(this: usize, pessoa: usize, clube: usize, a: u32, b: u32) {
    if pessoa == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1451)(this, pessoa, clube, a, b);
    }
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x35;
    cpq4_por(&mut e, 8, cqa11_i(clube));
    cpq4_por(&mut e, 0xd, (a & 0xffff) as i32);
    cpq4_por(&mut e, 0x11, b as i32);
    cpq4_historico(!CQA11_ENSAIO, this, read_usize(pessoa), &e);
    let r1 = cqa11_sortear(0x15 - cqa11_c(pessoa + 0x59));
    if r1 < cqa11_sortear(10) {
        crate::comparacao::ramo("cqa_11 torcida favorita");
        let gosta: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_STAFF_ADD_FAVOURED_CLUB);
        let favorita: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(FN_CLUB_ADD_FAVOURED_STAFF);
        gosta(pessoa, clube, cqa11_sortear(2) as u32);
        favorita(clube, pessoa, cqa11_sortear(2) as u32);
    }
}

/// Comparador em ensaio: a cópia sem o histórico e a original a partir do mesmo estado (sorteio, as pessoas favoritas e
/// rejeitadas do clube +0x9b e os clubes favoritos e rejeitados da pessoa, preferências +0x65: +4..+0x18). Pessoa nula:
/// só a original.
pub(crate) unsafe extern "thiscall" fn comparar_jogador_do_ano_da_torcida_no_registro(this: usize, pessoa: usize, clube: usize, a: u32, b: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1451);
    if pessoa == 0 || clube == 0 {
        return o(this, pessoa, clube, a, b);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((clube + 0x9b, 0x18));
    let pref = read_usize(pessoa + 0x65);
    if pref != 0 {
        reg.push((pref + 4, 0x18));
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQA11_ENSAIO = true;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| jogador_do_ano_da_torcida_no_registro(this, pessoa, clube, a, b)));
    CQA11_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, pessoa, clube, a, b);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("jogador_do_ano_da_torcida_no_registro(pessoa {:#x}, clube {:#x}) original={:x?} rust={:x?}", pessoa, clube, mo, mr));
}

// ---------- 263a leva: os registros das copas e da troca de técnico: o vice, o terceiro e a passagem do técnico pelo clube (staff_records.cpp) ----------


pub(crate) unsafe fn cqa12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



pub(crate) unsafe fn cqa12_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

/// O ensaio dos comparadores: as cópias sem o histórico (arquivo) e sem acabar as convocações.
pub(crate) static mut CQA12_ENSAIO: bool = false;

/// A entrada `e` no fim da lista do clube `id` no arquivo do histórico dos clubes (FILE_LLIST [this+4], 0x58e340; o
/// começo e o fim em [this+0x10] + id × 8). No ensaio, nada.
pub(crate) unsafe fn cqa12_historico_do_clube(this: usize, id: usize, e: &[u8; 0x40]) {
    if CQA12_ENSAIO {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let p = read_usize(this + 0x10).wrapping_add(id.wrapping_mul(8));
    f(read_usize(this + 4), p, p + 4, e.as_ptr());
}

/// Uma nota do lado do clube na partida (palavra em `a`): soma ftol(peso × k × [0x96ff88]) (produto em 32 bits) e fica
/// no piso peso × k (16 bits) quando está abaixo dele, senão no máximo 10000 (comparações com sinal em 16 bits).
pub(crate) unsafe fn cqa12_subir(a: usize, peso: i16, k: i32) {
    let soma = ftol((peso as i32).wrapping_mul(k) as f64 * f64_em(0x96ff88));
    let x = read_i16(a).wrapping_add(soma as i16);
    write_i16(a, x);
    let piso = peso.wrapping_mul(k as i16);
    write_i16(a, if piso > x { piso } else if x > 10000 { 10000 } else { x });
}

/// O vice (tipo 1: limiar 500, teto 350, notas 300/150) e o terceiro (tipo 2: 500, 200, 200/100; o limiar 500 conferido em 0x86b774) da copa: a reputação do
/// clube (+0x80) sobe até peso da competição (+0x69 de [jogo+0x14]) × limiar, no máximo `teto` de uma vez (entre 1 e
/// 10000); com técnico (+0xcf) que tem registro de não jogador (+0x69), as três notas do lado do clube na partida
/// (+0x188/+0x18a/+0x18c, lado 0x625 × visitante): peso × k, e na competição de fase 1 ([0xadadfc] +0x42 = 1) peso ×
/// metade na terceira e na segunda se o técnico for estrangeiro (0x53d980 com a nação +0x53 e 1). Os históricos (data,
/// tipo, id do clube, competição): do técnico, do presidente (+0xbf) e dos 20 da escalação do lado (+0x190, 0x49 bytes,
/// −1 vazio; pessoas válidas e fora o técnico); no terceiro, as convocações acabam nas competições guardadas (0x76d940,
/// 0x774f90 com os dois clubes); por fim o histórico do clube (tipo no +8, competição no +9).
pub(crate) unsafe fn cqa12_colocado(this: usize, clube: usize, jogo: usize, partida: usize, tipo: u8) {
    let (limiar, teto, k, metade) = if tipo == 1 { (500, 350, 300, 150) } else { (500, 200, 200, 100) };
    let comp = read_usize(jogo + 0x14);
    let peso = read_i16(comp + 0x69);
    let alvo = (peso as i32).wrapping_mul(limiar); // cqa12: o limiar da reputação
    let rep = read_i16(clube + 0x80) as i32;
    if rep < alvo {
        let falta = alvo - rep;
        let soma = if falta > teto { teto } else { falta }; // cqa12: o teto da subida
        let v = read_i16(clube + 0x80).wrapping_add(soma as i16);
        let v = if v < 1 { 1 } else if v > 10000 { 10000 } else { v };
        if tipo == 1 {
            crate::comparacao::ramo("cqa_12 reputacao do vice");
        } else {
            crate::comparacao::ramo("cqa_12 reputacao do terceiro");
        }
        write_i16(clube + 0x80, v); // cqa12: a reputação do colocado
    }
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = tipo;
    cpq4_por(&mut e, 8, cqa12_i(clube));
    cpq4_por(&mut e, 0xd, cqa12_i(comp));
    let t = read_usize(clube + 0xcf);
    if t != 0 {
        if read_usize(t + 0x69) != 0 {
            let c = read_usize(read_usize(0xadadfc).wrapping_add(4 * read_usize(jogo)));
            let (k1, k2) = if *((c + 0x42) as *const i8) != 1 {
                (k, k)
            } else {
                let estrangeiro: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
                (if estrangeiro(t, read_usize(clube + 0x53), 1) != 0 { metade } else { k }, metade)
            };
            let lado = partida + 0x188 + 0x625 * (read_usize(jogo + 0x20) == clube) as usize;
            cqa12_subir(lado, peso, k);
            cqa12_subir(lado + 2, peso, k1);
            cqa12_subir(lado + 4, peso, k2);
        }
        cpq4_historico(!CQA12_ENSAIO, this, read_usize(t), &e);
    }
    let presidente = read_usize(clube + 0xbf);
    if presidente != 0 {
        cpq4_historico(!CQA12_ENSAIO, this, read_usize(presidente), &e);
    }
    let escalacao = partida + 0x190 + 0x625 * (clube != read_usize(jogo + 0x1c)) as usize;
    for j in 0..0x14usize {
        let id = cqa12_i(escalacao + 0x49 * j);
        if id == -1 {
            continue;
        }
        let p = read_usize(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize);
        if p != 0 && cqa12_i(p) >= cqa12_i(0xae2364) {
            continue;
        }
        if p == read_usize(clube + 0xcf) {
            continue;
        }
        if p == 0 {
            aviso_em(0xa83b78, 0x1010);
            continue;
        }
        cpq4_historico(!CQA12_ENSAIO, this, read_usize(p), &e);
    }
    if tipo == 2 && !CQA12_ENSAIO {
        let guardada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
        if guardada(read_usize(jogo + 0x14)) != 0 {
            let acabar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_END_DUTY_FOR_NATION_IN_COMPETITION);
            acabar(read_usize(0xb74318), read_usize(jogo + 0x1c), read_usize(jogo + 0x14));
            acabar(read_usize(0xb74318), read_usize(jogo + 0x20), read_usize(jogo + 0x14));
        }
    }
    let mut e2 = cpq4_entrada();
    cpq4_datar(e2.as_mut_ptr());
    e2[8] = tipo;
    cpq4_por(&mut e2, 9, cqa12_i(comp));
    cqa12_historico_do_clube(this, read_usize(clube), &e2);
}

/// the cup runner-up: the club reputation rises up to weight × 500 (at most 350 at once), its side's three match
/// ratings by the weight, and the manager, chairman, line-up and club histories
/// Original: `Staff_Records::add_cup_runners_up_record` (0x86b0b0, staff_records.cpp, thiscall).
///
/// [pt] { 0x86b0b0 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_runners_up_record(clube, jogo, partida)`, pela final das copas (Cup 0x519890, Liga dos Campeões 0x584ae0 e
/// as copas de cada nação): `cqa12_colocado` com o tipo 1. Clube, jogo ou partida nulos: a caixa (linha 0x5f6).
pub(crate) unsafe extern "thiscall" fn vice_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return aviso_em(0xa83b78, 0x5f6);
    }
    cqa12_colocado(this, clube, jogo, partida, 1);
}

/// the cup third place: the same with weight × 200 (at most 200) and smaller ratings; ends the call-ups in the stored
/// competitions
/// Original: `Staff_Records::add_cup_third_place_record` (0x86b710, staff_records.cpp, thiscall).
///
/// [pt] { 0x86b710 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// add_cup_third_place_record(clube, jogo, partida)`, pelas copas com disputa de terceiro: `cqa12_colocado` com o tipo
/// 2. Nulos: a caixa (linha 0x67b).
pub(crate) unsafe extern "thiscall" fn terceiro_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return aviso_em(0xa83b78, 0x67b);
    }
    cqa12_colocado(this, clube, jogo, partida, 2);
}

/// Comparador em ensaio dos colocados: a cópia sem os históricos e sem acabar as convocações, e a original a partir do
/// mesmo estado (a reputação do clube e as três notas do lado dele na partida). Nulos: só a original.
pub(crate) unsafe fn cqa12_comparar_colocado(nome: &str, clube: usize, jogo: usize, partida: usize, original: impl FnOnce(), copia: impl FnOnce()) {
    if clube == 0 || jogo == 0 || partida == 0 {
        return original();
    }
    let reg = [(clube + 0x80, 2), (partida + 0x188 + 0x625 * (read_usize(jogo + 0x20) == clube) as usize, 6)];
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQA12_ENSAIO = true;
    copia();
    CQA12_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    original();
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("{}(clube {:#x}, jogo {:#x}) original={:x?} rust={:x?}", nome, clube, jogo, mo, mr));
}

pub(crate) unsafe extern "thiscall" fn comparar_vice_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1452);
    cqa12_comparar_colocado("vice_da_copa_nos_registros", clube, jogo, partida, || o(this, clube, jogo, partida),
        || vice_da_copa_nos_registros(this, clube, jogo, partida))
}

pub(crate) unsafe extern "thiscall" fn comparar_terceiro_da_copa_nos_registros(this: usize, clube: usize, jogo: usize, partida: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(1453);
    cqa12_comparar_colocado("terceiro_da_copa_nos_registros", clube, jogo, partida, || o(this, clube, jogo, partida),
        || terceiro_da_copa_nos_registros(this, clube, jogo, partida))
}

/// A data em `src` copiada para `dst` (0x417c10; a cópia de retorno vai para uma data local).
pub(crate) unsafe fn cqa12_copiar(dst: usize, src: usize) {
    let copia: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copia(dst, t.as_mut_ptr(), src);
}

/// Os dias da data `a` menos a `b` (0x54cdb0, `b` por valor; grava a data estática 0xae2470).
pub(crate) unsafe fn cqa12_dias(a: usize, b: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    f(a, read_usize(b) as u32, read_usize(b + 4) as u32)
}

/// closes the manager's current spell (shortest and longest), opens the new one when hired and counts the spells; the
/// manager, chairman and club histories
/// Original: `Staff_Records::record_manager_job_change` (0x869aa0, staff_records.cpp, thiscall).
///
/// [pt] { 0x869aa0 } (thiscall, ret 0xc, staff_records.cpp, SEH só das datas locais): `Staff_Records::
/// record_manager_job_change(técnico, clube, contratado)`, pela saída do técnico (0x69b790) e pelo técnico novo
/// (0x694c80). Fora da seleção (0x53b900), no registro do técnico (0x86f5b0 com criar), quando o começo do jogo
/// ([0xdd6ec8]) não é depois de hoje: a passagem atual (+0x4a clube, +0x4e começo) fecha; a mais curta (+0x72, +0x76
/// começo, +0x7e fim) é trocada pela atual se não houver ou se a atual for mais curta (0x54cdb0), ou só tem o fim
/// renovado se for a mesma; a mais longa (+0x5e, +0x62, +0x6a) é trocada se a atual for mais longa (o fim vira o +0x56)
/// ou se não houver (o fim vira hoje). Depois +0x4a = −1 e, contratado = 1, a passagem nova (o id do clube, hoje,
/// +0x56 = 13/11/1975 da 0x549f70) e +1 em +0xb4. Na seleção, contratado = 1: +1 em +0xb5. Os históricos: do técnico
/// (tipo 0x96: clube, contratado), do presidente do clube (+0xbf; tipo 0x97 com o técnico e as três chaves) e do clube
/// (tipo 0x96 no +8, o técnico, as chaves e contratado). Técnico ou clube nulos: a caixa (linha 0x3b1).
pub(crate) unsafe extern "thiscall" fn troca_de_tecnico_nos_registros(this: usize, tecnico: usize, clube: usize, contratado: u32) {
    if clube == 0 || tecnico == 0 {
        return aviso_em(0xa83b78, 0x3b1);
    }
    let contratado = contratado as u8 as i8;
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    if e_selecao(clube) == 0 {
        let r = registro(this, tecnico, 1);
        if r != 0 {
            let hoje = 0xae2c90usize;
            let (ai, ah) = (*((0xdd6ec8 + 2) as *const u16), *((hoje + 2) as *const u16));
            let depois = ai > ah || (ai == ah && read_i16(0xdd6ec8) > read_i16(hoje));
            if !depois {
                crate::comparacao::ramo("cqa_12 passagem fechada");
                if cqa12_i(r + 0x72) != -1 && cqa12_i(r + 0x4a) != -1 {
                    if cqa12_i(r + 0x72) == cqa12_i(r + 0x4a) && read_i16(r + 0x50) == read_i16(r + 0x78) && read_i16(r + 0x4e) == read_i16(r + 0x76) {
                        cqa12_copiar(r + 0x7e, hoje);
                    } else {
                        let atual = cqa12_dias(hoje, r + 0x4e);
                        if atual < cqa12_dias(r + 0x7e, r + 0x76) {
                            cqa12_pi(r + 0x72, cqa12_i(r + 0x4a));
                            cqa12_copiar(r + 0x76, r + 0x4e);
                            cqa12_copiar(r + 0x7e, hoje);
                        }
                    }
                } else {
                    cqa12_pi(r + 0x72, cqa12_i(r + 0x4a));
                    cqa12_copiar(r + 0x76, r + 0x4e);
                    cqa12_copiar(r + 0x7e, hoje);
                }
                if cqa12_i(r + 0x5e) != -1 && cqa12_i(r + 0x4a) != -1 {
                    let atual = cqa12_dias(hoje, r + 0x4e);
                    if atual > cqa12_dias(r + 0x6a, r + 0x62) {
                        cqa12_pi(r + 0x5e, cqa12_i(r + 0x4a));
                        cqa12_copiar(r + 0x62, r + 0x4e);
                        cqa12_copiar(r + 0x6a, r + 0x56);
                    }
                } else {
                    cqa12_pi(r + 0x5e, cqa12_i(r + 0x4a));
                    cqa12_copiar(r + 0x62, r + 0x4e);
                    cqa12_copiar(r + 0x6a, hoje);
                }
            }
            cqa12_pi(r + 0x4a, -1);
            if contratado == 1 {
                crate::comparacao::ramo("cqa_12 passagem nova");
                cqa12_pi(r + 0x4a, cqa12_i(clube));
                cqa12_copiar(r + 0x4e, hoje);
                let data: unsafe extern "thiscall" fn(*mut u8, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_DATE_549F70);
                let mut t = [0u8; 8];
                let d = data(t.as_mut_ptr(), 0xd, 0xb, 0x7b7, u32::MAX);
                cqa12_copiar(r + 0x56, d);
                *((r + 0xb4) as *mut u8) = (*((r + 0xb4) as *const u8)).wrapping_add(1); // cqa12: as passagens
            }
        }
    } else if contratado == 1 {
        let r = registro(this, tecnico, 1);
        if r != 0 {
            *((r + 0xb5) as *mut u8) = (*((r + 0xb5) as *const u8)).wrapping_add(1);
        }
    }
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x96;
    cpq4_por(&mut e, 8, cqa12_i(clube));
    cpq4_por(&mut e, 0xd, contratado as i32);
    cpq4_historico(!CQA12_ENSAIO, this, read_usize(tecnico), &e);
    if read_usize(clube + 0xbf) != 0 {
        let mut e2 = cpq4_entrada();
        cpq4_datar(e2.as_mut_ptr());
        e2[0xc] = 0x97;
        cpq4_por(&mut e2, 8, cqa12_i(clube));
        cpq4_por(&mut e2, 0xd, contratado as i32);
        cpq4_quem(&mut e2, 0x11, tecnico);
        cpq4_historico(!CQA12_ENSAIO, this, read_usize(read_usize(clube + 0xbf)), &e2);
    }
    let mut e3 = cpq4_entrada();
    cpq4_datar(e3.as_mut_ptr());
    e3[8] = 0x96;
    cpq4_quem(&mut e3, 9, tecnico);
    cpq4_por(&mut e3, 0x19, contratado as i32);
    cqa12_historico_do_clube(this, read_usize(clube), &e3);
}

/// Comparador em ensaio: o registro do técnico criado antes (0x86f5b0, como a original faria nos dois caminhos que o
/// pedem), a cópia sem os históricos e a original a partir do mesmo estado (o registro, 0xd2 bytes, e a data estática da
/// 0x54cdb0). Nulos: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_troca_de_tecnico_nos_registros(this: usize, tecnico: usize, clube: usize, contratado: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1454);
    if clube == 0 || tecnico == 0 {
        return o(this, tecnico, clube, contratado);
    }
    let mut reg = vec![(0xae2470usize, 8usize)];
    if e_selecao(clube) == 0 || contratado as u8 == 1 {
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = registro(this, tecnico, 1);
        if r != 0 {
            reg.push((r, 0xd2));
        }
    }
    let antes = crate::comparacao::Memoria::ler(&reg);
    CQA12_ENSAIO = true;
    troca_de_tecnico_nos_registros(this, tecnico, clube, contratado);
    CQA12_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    o(this, tecnico, clube, contratado);
    let mo = crate::comparacao::Memoria::ler(&reg);
    PLACAR.registrar(mo == mr, || format!("troca_de_tecnico_nos_registros(tecnico {:#x}, clube {:#x}, {}) original={:x?} rust={:x?}", tecnico, clube, contratado as u8, mo, mr));
}

// ---------- 255a leva: os gols do jogador pelo clube, o registro da seleção do ano e a ordem dos clubes pelos pontos do técnico (player_stats.cpp, staff_records.cpp) ----------


pub(crate) unsafe fn cps10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// sums the goals of the player's seasons for the club in the career table (0x7c3e40), in 16 bits
/// Original: `player_stats_get_goals_for_club` (0x7c3bb0, player_stats.cpp, stdcall, ret 8).
///
/// [pt] { 0x7c3bb0 } (stdcall, ret 8, player_stats.cpp; a original recebe o PlayerStats no ecx e não o lê):
/// `player_stats_get_goals_for_club(pessoa, clube)` (pela MATCH_PLAYER::setup 0x6eaea0, que grava em +0x3d do jogador
/// da partida): pessoa nula, a caixa "Error" (linha 0xb04) e 0; clube nulo, a da linha 0xb0c e 0; id a partir de
/// [0xae2364], 0; sem a ficha de jogador (+0x61), a da linha 0xb1e e 0. Senão a tabela da carreira (0x7c3e40 do
/// [0xb74350]: entradas de 0x2b bytes, seis por temporada e clube) e a soma, em 16 bits, do byte +0xdf dos grupos cujo
/// clube (+0xd9, com sinal, não negativo) é o id do `clube`; a tabela é liberada (0x9452ca, também quando nula).
/// Quente (cada jogador de cada partida): sem contador.
pub(crate) unsafe extern "stdcall" fn gols_pelo_clube(pessoa: usize, clube: usize) -> u16 {
    if pessoa == 0 {
        aviso_em(0xa78310, 0xb04);
        return 0;
    }
    if clube == 0 {
        aviso_em(0xa78310, 0xb0c);
        return 0;
    }
    if cps10_i(pessoa) >= cps10_i(0xae2364) {
        return 0;
    }
    if read_u32_as_usize(pessoa + 0x61) == 0 {
        aviso_em(0xa78310, 0xb1e);
        return 0;
    }
    let montar: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(FN_PLAYERSTATS_BUILD_CAREER_HISTORY);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut n = 0i32;
    let t = montar(read_u32_as_usize(0xb74350), pessoa, &mut n);
    let mut s = 0u16;
    if t != 0 {
        let mut i = 0i32;
        while i < n {
            let g = t + 0xd9 + (i / 6) as usize * 0x102;
            let v = cps10_i(g);
            if v >= 0 && v == cps10_i(clube) { // cps10: só os grupos do clube
                s = s.wrapping_add(*((g + 6) as *const u8) as u16);
            }
            i += 6;
        }
    }
    liberar(t);
    s // cps10: os gols pelo clube
}

/// Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
pub(crate) unsafe extern "stdcall" fn comparar_gols_pelo_clube(pessoa: usize, clube: usize) -> u16 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize) -> u16>(1143);
    if pessoa == 0 || clube == 0 || (cps10_i(pessoa) < cps10_i(0xae2364) && read_u32_as_usize(pessoa + 0x61) == 0) {
        return o(pessoa, clube);
    }
    let r = o(pessoa, clube);
    let n = gols_pelo_clube(pessoa, clube);
    PLACAR.registrar(r == n, || format!("gols_pelo_clube(pessoa {:x}, clube {:x}) original={} rust={}", pessoa, clube, r, n));
    r
}

/// the team of the year history of the person (type 0x32: the club, the award and two values)
/// Original: `Staff_Records::add_team_of_year_record` (0x86c730, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86c730 } (thiscall, ret 0x14, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_team_of_year_record(pessoa, clube, prêmio, a, b)` (pela TEAM_AWARD::record_winners 0x8affb0):
/// pessoa ou prêmio nulos, a caixa "Error" (linha 0x7ee). Senão o histórico da pessoa (tipo 0x32: o id do clube ou −1,
/// o id do prêmio, `a` em 16 bits sem sinal e `b` como byte com sinal), no arquivo (0x58e340).
pub(crate) unsafe extern "thiscall" fn registrar_a_selecao_do_ano(this: usize, pessoa: usize, clube: usize, premio: usize, a: u32, b: u32) {
    crate::comparacao::ramo("cps_10 selecao do ano");
    if pessoa == 0 || premio == 0 {
        aviso_em(0xa83b78, 0x7ee);
        return;
    }
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x32;
    cpq4_por(&mut e, 8, cpq4_id_ou_menos_1(clube));
    cpq4_por(&mut e, 0xd, cps10_i(premio));
    cpq4_por(&mut e, 0x11, (a & 0xffff) as i32);
    cpq4_por(&mut e, 0x15, b as u8 as i8 as i32);
    cpq4_historico(true, this, read_u32_as_usize(pessoa), &e);
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_registrar_a_selecao_do_ano(this: usize, pessoa: usize, clube: usize, premio: usize, a: u32, b: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32)>(1144)(this, pessoa, clube, premio, a, b)
}

/// O registro de pontos do técnico do clube (+0xcf; 0x86f5b0 do [0xdd7f00], criando), ou 0 sem técnico.
pub(crate) unsafe fn cps10_registro(clube: usize) -> usize {
    let tecnico = read_u32_as_usize(clube + 0xcf);
    if tecnico == 0 {
        return 0;
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    registro(read_u32_as_usize(0xdd7f00), tecnico, 1)
}

/// orders two clubs by their manager's league season points (highest first; no record last)
/// Original: `staff_records_compare_clubs_by_manager_points` (0x870220, staff_records.cpp, cdecl).
///
/// [pt] { 0x870220 } (cdecl, staff_records.cpp): `staff_records_compare_clubs_by_manager_points(a, b)`, a ordem do qsort dos
/// candidatos a técnico do ano (0x86fab0): os registros dos técnicos dos dois clubes (cps10_registro); sem os dois, 0;
/// só sem o de `a`, 1; só sem o de `b`, −1; com os dois, os pontos da temporada na liga (+0xba) de `b` menos os de
/// `a` (mais pontos primeiro).
pub(crate) unsafe extern "C" fn ordem_pelos_pontos_do_tecnico(a: usize, b: usize) -> i32 {
    let (ca, cb) = (read_u32_as_usize(a), read_u32_as_usize(b));
    let ra = cps10_registro(ca);
    let rb = cps10_registro(cb);
    if ra == 0 {
        return if rb != 0 { 1 } else { 0 };
    }
    if rb == 0 {
        return -1;
    }
    crate::comparacao::ramo("cps_10 pontos dos dois");
    cps10_i(rb + 0xba).wrapping_sub(cps10_i(ra + 0xba)) // cps10: mais pontos primeiro
}

// ponytail: a 0x86f5b0 cria o registro que falta; a original roda antes e a cópia só acha os que ela criou.
pub(crate) unsafe extern "C" fn comparar_ordem_pelos_pontos_do_tecnico(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(1145)(a, b);
    let n = ordem_pelos_pontos_do_tecnico(a, b);
    PLACAR.registrar(r == n, || format!("ordem_pelos_pontos_do_tecnico(clubes {:x}, {:x}) original={} rust={}", read_u32_as_usize(a), read_u32_as_usize(b), r, n));
    r
}

// ---------- 253a leva: os registros de carreira: o acesso, o rebaixamento e a classificação para a fase seguinte (staff_records.cpp) ----------


pub(crate) unsafe fn cpq3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}




pub(crate) unsafe fn cpq3_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}

pub(crate) unsafe fn cpq3_somar(a: usize, v: i32) {
    cpq3_pi(a, cpq3_i(a).wrapping_add(v));
}

/// A constante double do exe em `a`.
pub(crate) unsafe fn cpq3_k(a: usize) -> f64 {
    core::ptr::read_unaligned(a as *const f64)
}

/// Uma entrada de histórico na pilha: a data (0x54a110) com a data do jogo copiada (0x417c10).
/// ponytail: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
pub(crate) unsafe fn cpq3_entrada() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let nova: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    nova(e.as_mut_ptr());
    copia(e.as_mut_ptr(), t.as_mut_ptr(), 0xae2c90);
    e
}

pub(crate) unsafe fn cpq3_por(e: &mut [u8; 0x40], o: usize, v: u32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// A entrada `e` no fim da lista do arquivo `arq` (FILE_LLIST, 0x58e340) da pessoa ou do clube `id` (o começo e o fim
/// em `base` + id × 8). Sem `gravar` (a cópia pelo comparador, depois da original), nada.
pub(crate) unsafe fn cpq3_historico(gravar: bool, arq: usize, base: usize, id: usize, e: &[u8; 0x40]) {
    if !gravar {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let p = base.wrapping_add(id.wrapping_mul(8));
    f(arq, p, p + 4, e.as_ptr());
}

/// O histórico da pessoa (o arquivo [this], as listas em [this+0xc]).
pub(crate) unsafe fn cpq3_da_pessoa(gravar: bool, this: usize, pessoa: usize, e: &[u8; 0x40]) {
    cpq3_historico(gravar, read_usize(this), read_usize(this + 0xc), read_usize(pessoa), e);
}

/// O histórico do clube (o arquivo [this+4], as listas em [this+0x10]) com o tipo e a competição.
pub(crate) unsafe fn cpq3_do_clube(gravar: bool, this: usize, clube: usize, tipo: u8, comp: usize) {
    let mut e = cpq3_entrada();
    e[8] = tipo;
    cpq3_por(&mut e, 9, read_usize(comp) as u32);
    cpq3_historico(gravar, read_usize(this + 4), read_usize(this + 0x10), read_usize(clube), &e);
}

/// A entrada da pessoa: o clube (+8), o tipo (+0xc) e a competição (+0xd).
pub(crate) unsafe fn cpq3_entrada_do_clube(clube: usize, tipo: u8, comp: u32) -> [u8; 0x40] {
    let mut e = cpq3_entrada();
    cpq3_por(&mut e, 8, read_usize(clube) as u32);
    e[0xc] = tipo;
    cpq3_por(&mut e, 0xd, comp);
    e
}

/// Os jogadores do elenco (50 vagas em +0xd7) que contam nos registros: com registro de jogador (+0x61), id abaixo de
/// [0xae2364] e que não são o técnico.
pub(crate) unsafe fn cpq3_elenco(clube: usize) -> impl Iterator<Item = usize> {
    (0..50usize).filter_map(move |k| {
        let p = read_usize(clube + 0xd7 + 4 * k);
        if p == 0 || read_usize(p + 0x61) == 0 || cpq3_i(p) >= cpq3_i(0xae2364) || p == read_usize(clube + 0xcf) {
            None
        } else {
            Some(p)
        }
    })
}

pub(crate) unsafe fn cpq3_contrato(p: usize, clube: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT_FOR_CLUB);
    f(read_usize(0xae1050), p, clube)
}

/// O contrato `c` do jogador `p` vai a renovação (a 0x4cda70 roda): com o gerente de transferências, o contrato com
/// mais de 300 dias (desde +0x25) e o jogador na lista do clube (0x849ce0, item +0xf = id dele). Só leitura.
pub(crate) unsafe fn cpq3_vai_renovar(clube: usize, p: usize, c: usize) -> bool {
    let gt = read_usize(0xde1f54);
    if gt == 0 {
        return false;
    }
    let dias: unsafe extern "thiscall" fn(usize, usize, usize) -> i32 = core::mem::transmute(FN_DATE_DAYS_SINCE);
    if dias(0xae2c90, read_usize(c + 0x25), read_usize(c + 0x29)) <= 0x12c {
        return false;
    }
    let entrada: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(FN_SHORTLIST_MANAGER_FIND_CLUB_SHORTLIST_ENTRY);
    let e = entrada(read_usize(gt + 0x856), clube, p, 0, 0);
    e != 0 && read_usize(e + 0xf) == read_usize(p)
}

/// A oferta de renovação do contrato (0x4cda70 do gerente de contratos: o jogador, sem clube nem termos, vaga −1,
/// fator 1,0, modo 1).
pub(crate) unsafe fn cpq3_renovar(p: usize) {
    let f: unsafe extern "thiscall" fn(usize, usize, u32, u32, i32, u32, u32, u32) -> u32 = core::mem::transmute(FN_CONTRACT_MANAGER_OFFER_CONTRACT);
    f(read_usize(0xae1050), p, 0, 0, -1, 0, 0x3ff0_0000, 1);
}

pub(crate) unsafe fn cpq3_moral(p: usize, delta: i32) {
    let f: unsafe extern "C" fn(usize, i32, u32) -> u32 = core::mem::transmute(FN_STAFF_ADJUST_MORALE);
    f(p, delta, 1);
}

/// A nota do não jogador em `a` sobe ftol(x × peso × 10/61) e fica no máximo entre x × peso (16 bits) e ela mesma,
/// presa a 10000.
pub(crate) unsafe fn cpq3_nota_do_acesso(a: usize, x: i32, peso: i32) {
    write_i16(a, read_i16(a).wrapping_add(ftol(x.wrapping_mul(peso) as f64 * cpq3_k(0x96ff88)) as i16));
    let minimo = (x as i16).wrapping_mul(peso as i16);
    let v = read_i16(a);
    write_i16(a, if minimo > v { minimo } else if v > 0x2710 { 0x2710 } else { v });
}

/// O acesso; `gravar` falso só na cópia pelo comparador (sem os históricos em arquivo).
pub(crate) unsafe fn cpq3_acesso(this: usize, clube: usize, comp: usize, peso: u32, gravar: bool) {
    if clube == 0 || comp == 0 {
        aviso_em(0xa83b78, 0x301);
        return;
    }
    let e = cpq3_entrada_do_clube(clube, 8, read_usize(comp) as u32);
    write_i16(clube + 0x80, read_i16(clube + 0x80).wrapping_add(500));
    if read_i16(clube + 0x80) > 0x2710 {
        write_i16(clube + 0x80, 0x2710);
    }
    let v = cpq3_i(clube + 0x7b);
    if v < 0x249f0 && v < cpq3_i(clube + 0x73).wrapping_add(500) {
        cpq3_pi(clube + 0x7b, v.wrapping_add(read_i16(comp + 0x69) as i32 * 10));
    }
    if read_usize(clube + 0xcf) != 0 {
        let tec = read_usize(clube + 0xcf);
        if read_usize(tec + 0x69) != 0 {
            let objeto = read_usize(read_usize(0xadadfc).wrapping_add(read_usize(comp).wrapping_mul(4)));
            let mut pela = comp;
            if objeto != 0 && cpq3_i(objeto + 0x1c) > -1 {
                let cc = read_usize(0xae23d0).wrapping_add((cpq3_i(objeto + 0x1c) as usize).wrapping_mul(0x6b));
                pela = if cc != 0 { cc } else { comp };
            }
            let (p1, p2) = if read_u8(objeto + 0x42) as i8 != 1 {
                (400, 400)
            } else {
                let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
                (200, if estrangeira(tec, read_usize(clube + 0x53), 1) != 0 { 200 } else { 400 })
            };
            let x = read_i16(pela + 0x69) as i32;
            let nj = || read_usize(read_usize(clube + 0xcf) + 0x69);
            crate::comparacao::ramo("cpq_3 acesso do tecnico");
            cpq3_nota_do_acesso(nj() + 0xa, x, 400);
            cpq3_nota_do_acesso(nj() + 8, x, p2);
            cpq3_nota_do_acesso(nj() + 0xc, x, p1);
        }
        cpq3_da_pessoa(gravar, this, read_usize(clube + 0xcf), &e);
        let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
        let r = registro(this, read_usize(clube + 0xcf), 1);
        if r != 0 {
            let pontos = (read_i16(comp + 0x69) as i32).wrapping_mul(peso as u8 as i8 as i32).wrapping_mul(2500) / 100;
            cpq3_somar(r + 0xb6, pontos);
            cpq3_somar(r + 0xba, pontos);
            cpq3_somar(r + 0xca, pontos);
        }
    }
    if read_usize(clube + 0xbf) != 0 {
        cpq3_da_pessoa(gravar, this, read_usize(clube + 0xbf), &e);
    }
    for p in cpq3_elenco(clube) {
        cpq3_moral(p, 0xf);
        let c = cpq3_contrato(p, clube);
        if c == 0 {
            continue;
        }
        let v = read_u8(c + 0x44) as i8;
        let s = v as i32 + 0x4b;
        *((c + 0x44) as *mut u8) = if s > 0x64 { 0x64 } else if s < -0x64 { 0x9c } else { v.wrapping_add(0x4b) as u8 };
        let tipo = read_u8(c + 0x4f) >> 4;
        if (tipo == 0 || (tipo > 3 && tipo != 5)) || !cpq3_vai_renovar(clube, p, c) {
            continue;
        }
        crate::comparacao::ramo("cpq_3 acesso renova");
        cpq3_renovar(p);
    }
    cpq3_do_clube(gravar, this, clube, 8, comp);
}

/// club reputation +500 (up to 10000) and +0x7b; the manager ratings by the competition importance (× 10/61, with a
/// minimum); the manager, chairman and club histories (type 8); the manager points; squad morale +15 and bond +75 and
/// renewal of the expiring contracts
/// Original: `Staff_Records::add_promotion_record` (0x869480, staff_records.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x869480 } (thiscall, ret 0xc, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_promotion_record(clube,
/// comp, peso)`: a reputação do clube (+0x80) sobe 500 (até 10000) e +0x7b sobe 10 × a importância da competição
/// (+0x69) se está abaixo de 150000 e de +0x73 + 500. O técnico (+0xcf) com não jogador: as notas +0xa, +8 e +0xc sobem
/// pela importância da competição (a do registro de competição de clube +0x1c dela, se há) × 400, × 200/400 (estrangeiro
/// na liga com +0x42 = 1, 0x53d980) e × 200/400 (liga com +0x42 = 1) × 10/61, com o mínimo de importância × peso. O
/// histórico do técnico e do presidente (tipo 8), os pontos do técnico (0x86f5b0: +0xb6, +0xba, +0xca somam importância
/// × peso × 25), o moral +15 de cada jogador do elenco (0x544710), o vínculo do contrato (+0x44) +75 (−100..100) e a
/// renovação (0x4cda70) dos contratos dos tipos 1, 2, 3 e 5 que vencem; por fim o histórico do clube (tipo 8). Clube ou
/// competição nulos: a caixa "Error" (linha 0x301).
pub(crate) unsafe extern "thiscall" fn registrar_o_acesso(this: usize, clube: usize, comp: usize, peso: u32) {
    crate::comparacao::ramo("cpq_3 acesso");
    cpq3_acesso(this, clube, comp, peso, true)
}

/// As regiões que o acesso ou o rebaixamento mexem (sem os históricos): o estado do sorteador (o moral sorteia), o clube,
/// o não jogador do técnico, o registro de pontos dele (criado antes, como a original faria), o moral (+0x45) e as
/// reputações (+9..+0xe) de cada jogador do elenco e o vínculo (+0x44) do contrato dele. `None` se algum contrato vai a
/// renovação (a 0x4cda70 faz a oferta: não roda duas vezes; no acesso os tipos 1, 2, 3 e 5, no rebaixamento 1 e 2).
pub(crate) unsafe fn cpq3_regioes(this: usize, clube: usize, acesso: bool) -> Option<Vec<(usize, usize)>> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((clube, 0x245));
    let tec = read_usize(clube + 0xcf);
    if tec != 0 {
        if read_usize(tec + 0x69) != 0 {
            reg.push((read_usize(tec + 0x69), 0x10));
        }
        if acesso {
            let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
            let r = registro(this, tec, 1);
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    for p in cpq3_elenco(clube) {
        let j = read_usize(p + 0x61);
        reg.extend([(j + 9, 6), (j + 0x45, 1)]);
        let c = cpq3_contrato(p, clube);
        if c != 0 {
            let tipo = read_u8(c + 0x4f) >> 4;
            let renova = if acesso { tipo != 0 && (tipo <= 3 || tipo == 5) } else { tipo == 1 || tipo == 2 };
            if renova && cpq3_vai_renovar(clube, p, c) {
                return None;
            }
            reg.push((c + 0x44, 1));
        }
    }
    Some(reg)
}

/// Comparador: lado a lado pelas regiões (a cópia sem os históricos em arquivo, que a original já gravou, e sem contar
/// os sorteios); com renovação de contrato ou clube/competição nulos, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_registrar_o_acesso(this: usize, clube: usize, comp: usize, peso: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(1051);
    let reg = if clube != 0 && comp != 0 { cpq3_regioes(this, clube, true) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, clube, comp, peso),
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, comp, peso),
        || crate::aleatorio::sem_contar(|| cpq3_acesso(this, clube, comp, peso, false)));
    PLACAR.registrar(igual, || format!("registrar_o_acesso(clube {:x}, peso {}) {:x?} x {:x?}", clube, peso as u8 as i8, mo, mr));
}

/// A nota do não jogador em `a` soma `v` (16 bits) e fica presa a 1..10000.
pub(crate) unsafe fn cpq3_nota_do_rebaixamento(a: usize, v: i16) {
    write_i16(a, read_i16(a).wrapping_add(v));
    let n = read_i16(a);
    write_i16(a, if n < 1 { 1 } else if n > 0x2710 { 0x2710 } else { n });
}

/// As reputações (+0xb atual, +9 na nação, +0xd mundial) do registro de jogador `j` × `k`.
pub(crate) unsafe fn cpq3_reputacoes(j: usize, k: f64) {
    for o in [0xb, 9, 0xd] {
        write_i16(j + o, ftol(read_i16(j + o) as i32 as f64 * k) as i16);
    }
}

pub(crate) unsafe fn cpq3_rebaixamento(this: usize, clube: usize, comp: usize, gravar: bool) {
    if clube == 0 || comp == 0 {
        aviso_em(0xa83b78, 0x476);
        return;
    }
    let e = cpq3_entrada_do_clube(clube, 9, read_usize(comp) as u32);
    write_i16(clube + 0x80, read_i16(clube + 0x80).wrapping_sub(500));
    if read_i16(clube + 0x80) < 0x2ee {
        write_i16(clube + 0x80, 0x2ee);
    }
    if read_usize(clube + 0xcf) != 0 {
        let tec = read_usize(clube + 0xcf);
        if read_usize(tec + 0x69) != 0 {
            let estrangeira: unsafe extern "C" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_STAFF_FOREIGN);
            let fator: i16 = if estrangeira(tec, read_usize(clube + 0x53), 1) != 0 { -8 } else { -16 };
            let nj = || read_usize(read_usize(clube + 0xcf) + 0x69);
            let x = || read_i16(comp + 0x69);
            crate::comparacao::ramo("cpq_3 rebaixamento do tecnico");
            cpq3_nota_do_rebaixamento(nj() + 0xa, x().wrapping_mul(-16));
            cpq3_nota_do_rebaixamento(nj() + 8, x().wrapping_mul(fator));
            cpq3_nota_do_rebaixamento(nj() + 0xc, x().wrapping_mul(-8));
        }
        cpq3_da_pessoa(gravar, this, read_usize(clube + 0xcf), &e);
    }
    if read_usize(clube + 0xbf) != 0 {
        cpq3_da_pessoa(gravar, this, read_usize(clube + 0xbf), &e);
    }
    for p in cpq3_elenco(clube) {
        cpq3_moral(p, -0xf);
        let c = cpq3_contrato(p, clube);
        if c == 0 {
            continue;
        }
        let j = read_usize(p + 0x61);
        match read_u8(c + 0x4f) >> 4 {
            1 | 2 => {
                if j != 0 {
                    cpq3_reputacoes(j, cpq3_k(0x9689d8)); // 0,9
                }
                if cpq3_vai_renovar(clube, p, c) {
                    crate::comparacao::ramo("cpq_3 rebaixamento renova");
                    cpq3_renovar(p);
                }
            }
            3..=5 => {
                if j != 0 {
                    cpq3_reputacoes(j, cpq3_k(0x9689d0)); // 0,95
                }
            }
            _ => {}
        }
    }
    cpq3_do_clube(gravar, this, clube, 9, comp);
}

/// club reputation −500 (minimum 750); the manager ratings drop by the competition importance (1..10000); the manager,
/// chairman and club histories (type 9); squad morale −15, reputations × 0.9 (contracts 1 and 2, renewing the expiring
/// ones) or × 0.95 (3 to 5)
/// Original: `Staff_Records::add_relegation_record` (0x86a1c0, staff_records.cpp, thiscall, ret 8).
///
/// [pt] { 0x86a1c0 } (thiscall, ret 8, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_relegation_record(clube,
/// comp)`: a reputação do clube (+0x80) cai 500 (mínimo 750). O técnico com não jogador: as notas +0xa (−16), +8 (−8
/// estrangeiro, 0x53d980, ou −16) e +0xc (−8) × a importância da competição (+0x69), presas a 1..10000; o histórico do
/// técnico e do presidente (tipo 9). Cada jogador do elenco: moral −15 (0x544710) e, pelo tipo do contrato (+0x4f >> 4):
/// 1 e 2, reputações × 0,9 e a renovação (0x4cda70) se vence; 3 a 5, reputações × 0,95. Por fim o histórico do clube
/// (tipo 9). Clube ou competição nulos: a caixa "Error" (linha 0x476).
pub(crate) unsafe extern "thiscall" fn registrar_o_rebaixamento(this: usize, clube: usize, comp: usize) {
    crate::comparacao::ramo("cpq_3 rebaixamento");
    cpq3_rebaixamento(this, clube, comp, true)
}

/// Comparador: como o do acesso (sem o registro de pontos, que o rebaixamento não usa).
pub(crate) unsafe extern "thiscall" fn comparar_registrar_o_rebaixamento(this: usize, clube: usize, comp: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize)>(1052);
    let reg = if clube != 0 && comp != 0 { cpq3_regioes(this, clube, false) } else { None };
    let reg = match reg {
        Some(r) => r,
        None => return o(this, clube, comp),
    };
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, comp),
        || crate::aleatorio::sem_contar(|| cpq3_rebaixamento(this, clube, comp, false)));
    PLACAR.registrar(igual, || format!("registrar_o_rebaixamento(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

pub(crate) unsafe fn cpq3_classificacao(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32, gravar: bool) {
    if clube == 0 {
        aviso_em(0xa83b78, 0x703);
        return;
    }
    let mut e = cpq3_entrada_do_clube(clube, 3, if comp != 0 { read_usize(comp) as u32 } else { u32::MAX });
    cpq3_por(&mut e, 0x11, fase as u16 as i16 as i32 as u32);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_GET_BASE_ROUND_ID);
    cpq3_por(&mut e, 0x15, base(rodada) as u16 as i16 as i32 as u32);
    if read_usize(clube + 0xcf) == 0 {
        return;
    }
    cpq3_da_pessoa(gravar, this, read_usize(clube + 0xcf), &e);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let r = registro(this, read_usize(clube + 0xcf), 1);
    if r == 0 {
        return;
    }
    // ponytail: a original faz pow(x, 2,0) (0x945ff0): x × x, exato para a importância (16 bits)
    let x = read_i16(comp + 0x69) as f64;
    let pontos = ftol(x * x * (peso as u8 as i8 as i32) as f64);
    cpq3_somar(r + 0xb6, pontos);
    cpq3_somar(r + 0xca, pontos);
    let selecao: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(FN_DATABASE_IS_NATIONAL_TEAM);
    if selecao(clube) != 0 {
        crate::comparacao::ramo("cpq_3 classificado selecao");
        cpq3_somar(r + 0xbe, pontos);
    } else {
        crate::comparacao::ramo("cpq_3 classificado clube");
        cpq3_somar(r + 0xba, pontos);
    }
}

/// the manager history (type 3, with the round and the base round) and his points: importance² × weight in the year,
/// the month and the cup (national team) or the league (club)
/// Original: `Staff_Records::add_qualified_for_round_record` (0x86bdd0, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86bdd0 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_qualified_for_round_record(
/// clube, comp, fase, rodada, peso)`: com técnico (+0xcf), o histórico dele (tipo 3: clube, competição ou −1, a fase com
/// sinal e a rodada base 0x4b0870) e os pontos (0x86f5b0): importância² × peso somados em +0xb6, +0xca e, na seleção
/// (0x53b900), +0xbe, ou no clube +0xba. Clube nulo: a caixa "Error" (linha 0x703).
pub(crate) unsafe extern "thiscall" fn registrar_a_classificacao_na_fase(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    cpq3_classificacao(this, clube, comp, fase, rodada, peso, true)
}

/// Comparador: lado a lado pelo registro de pontos do técnico (criado antes, como a original faria), a cópia sem o
/// histórico em arquivo; sem clube ou sem técnico, só a original.
pub(crate) unsafe extern "thiscall" fn comparar_registrar_a_classificacao_na_fase(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, peso: u32) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(1053);
    if clube == 0 || read_usize(clube + 0xcf) == 0 {
        return o(this, clube, comp, fase, rodada, peso);
    }
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let r = registro(this, read_usize(clube + 0xcf), 1);
    if r == 0 {
        return o(this, clube, comp, fase, rodada, peso);
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&[(r, 0xd2)], || o(this, clube, comp, fase, rodada, peso),
        || cpq3_classificacao(this, clube, comp, fase, rodada, peso, false));
    PLACAR.registrar(igual, || format!("registrar_a_classificacao_na_fase(clube {:x}, peso {}) {:x?} x {:x?}", clube, peso as u8 as i8, mo, mr));
}

// ---------- 253a leva: os registros de carreira: a transferência, a eliminação e a não classificação (staff_records.cpp) ----------


pub(crate) unsafe fn cpq4_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}



pub(crate) unsafe fn cpq4_pi(a: usize, v: i32) {
    core::ptr::write_unaligned(a as *mut i32, v);
}


/// O inteiro de 64 bits em `a` somado de `v` (estendido com sinal: cdq, add, adc).
pub(crate) unsafe fn cpq4_somar64(a: usize, v: i32) {
    let x = core::ptr::read_unaligned(a as *const i64);
    core::ptr::write_unaligned(a as *mut i64, x.wrapping_add(v as i64));
}

pub(crate) unsafe fn cpq4_por(e: &mut [u8; 0x40], o: usize, v: i32) {
    e[o..o + 4].copy_from_slice(&v.to_le_bytes());
}

/// A pessoa do id (0x6e bytes em [0xae23bc]).
pub(crate) unsafe fn cpq4_pessoa(id: usize) -> usize {
    read_usize(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// A data do jogo copiada (0x417c10) em `a` (a cópia também vai para uma data local).
pub(crate) unsafe fn cpq4_datar(a: *mut u8) {
    let copia: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(FN_DATE_OPERATOR_3D);
    let mut t = [0u8; 8];
    copia(a, t.as_mut_ptr(), 0xae2c90);
}

/// Uma entrada de histórico na pilha com a data construída (0x54a110).
/// ponytail: os bytes que a original não grava (lixo da pilha dela) saem zerados aqui; só vão para o arquivo do
/// histórico (FILE_LLIST), que a carreira não lê de volta.
pub(crate) unsafe fn cpq4_entrada() -> [u8; 0x40] {
    let mut e = [0u8; 0x40];
    let nova: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(FN_DATE_DATE_54A110);
    nova(e.as_mut_ptr());
    e
}

/// A entrada `e` no fim da lista da pessoa `id` no arquivo do histórico das pessoas (FILE_LLIST [this], 0x58e340; o
/// começo e o fim em [this+0xc] + id × 8). Sem `gravar` (a cópia pelo comparador), nada.
pub(crate) unsafe fn cpq4_historico(gravar: bool, this: usize, id: usize, e: &[u8; 0x40]) {
    if !gravar {
        return;
    }
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const u8) -> usize = core::mem::transmute(FN_FILE_LLIST_ADD_NODE);
    let p = read_usize(this + 0xc).wrapping_add(id.wrapping_mul(8));
    f(read_usize(this), p, p + 4, e.as_ptr());
}

/// O id do clube contratante da pessoa (+0x39), ou −1.
pub(crate) unsafe fn cpq4_clube_de(pessoa: usize) -> i32 {
    let c = read_usize(pessoa + 0x39);
    if c == 0 {
        -1
    } else {
        cpq4_i(c)
    }
}

pub(crate) unsafe fn cpq4_id_ou_menos_1(p: usize) -> i32 {
    if p == 0 {
        -1
    } else {
        cpq4_i(p)
    }
}

/// O id da pessoa e os de +4, +8 e +0xc dela (+0x33 de cada) em `e` a partir de `o`.
pub(crate) unsafe fn cpq4_quem(e: &mut [u8; 0x40], o: usize, p: usize) {
    cpq4_por(e, o, cpq4_i(p));
    for k in 0..3usize {
        cpq4_por(e, o + 4 + 4 * k, cpq4_i(read_usize(p + 4 + 4 * k) + 0x33));
    }
}

/// Os itens da lista da troca (LLIST em +4 da transferência, relida a cada passo; o cursor reposto antes), na ordem.
pub(crate) unsafe fn cpq4_troca(t: usize, mut f: impl FnMut(usize)) {
    let l = read_usize(t + 4);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut u32, read_usize(l + 2) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let mut e = proximo(read_usize(t + 4));
    while e != 0 {
        f(e);
        e = proximo(read_usize(t + 4));
    }
}

/// O tipo do cargo é de jogador (0x53b620: 11, 12, 13 ou 15)?
pub(crate) unsafe fn cpq4_jogador(cargo: u8) -> bool {
    let f: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_DATABASE_IS_PLAYER_JOB);
    f(cargo as u32) != 0 // ponytail: a original empurra o registro com o byte no al; a 0x53b620 lê só o byte
}

pub(crate) unsafe fn cpq4_registro(this: usize, tec: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    f(this, tec, 1)
}

/// Os lados da transferência: (pessoa, contrato da pessoa 0x4cf5a0, vendedor ou 0, comprador).
pub(crate) unsafe fn cpq4_lados(t: usize) -> (usize, usize, usize, usize) {
    let pessoa = cpq4_pessoa(read_usize(t + 0xc));
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_CONTRACT_MANAGER_GET_CONTRACT);
    let c = contrato(read_usize(0xae1050), pessoa);
    let vendedor = if c != 0 && read_u8(c + 0x35) & 0x3f != 5 {
        read_usize(0xae23b4).wrapping_add(read_usize(c + 4).wrapping_mul(0x245))
    } else {
        0
    };
    (pessoa, c, vendedor, read_usize(0xae23b4).wrapping_add(read_usize(t + 8).wrapping_mul(0x245)))
}

pub(crate) unsafe fn cpq4_transferencia(this: usize, t: usize, gravar: bool) {
    let (pessoa, c, vendedor, comprador) = cpq4_lados(t);
    *(0xae1080 as *mut usize) = c;
    let mut e = cpq4_entrada();
    let total: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> i32 = core::mem::transmute(FN_TRANSFER_OFFER_GET_TOTAL_VALUE);
    let valor = total(t, 1, 0, 0, 1);
    let com_troca = || (read_usize(t + 4) != 0) as i32;
    let tipo = read_u8(t + 0x11) as i8 as i32;
    if comprador != 0 && read_usize(comprador + 0xcf) != 0 {
        let tec = read_usize(comprador + 0xcf);
        let r = if cpq4_jogador(read_u8(t + 0x11)) { cpq4_registro(this, tec) } else { 0 };
        if r != 0 {
            crate::comparacao::ramo("cpq_4 compra");
            if valor > cpq4_i(r + 0x18) {
                crate::comparacao::ramo("cpq_4 maior compra");
                for k in 0..4usize {
                    cpq4_pi(r + 4 * k, if k == 0 { cpq4_i(pessoa) } else { cpq4_i(read_usize(pessoa + 4 * k) + 0x33) });
                }
                cpq4_datar((r + 0x10) as *mut u8);
                cpq4_pi(r + 0x18, valor);
                *((r + 0x1c) as *mut u8) = com_troca() as u8;
                cpq4_pi(r + 0x1d, cpq4_id_ou_menos_1(vendedor));
                cpq4_pi(r + 0x21, cpq4_i(comprador));
            }
            write_i16(r + 0xa0, read_i16(r + 0xa0).wrapping_add(1));
            cpq4_somar64(r + 0xa4, valor);
            if read_usize(t + 4) != 0 {
                write_i16(r + 0xa2, read_i16(r + 0xa2).wrapping_add(read_i16(read_usize(t + 4))));
                cpq4_troca(t, |x| cpq4_somar64(r + 0xac, cpq4_i(cpq4_pessoa(read_usize(x)) + 0x52)));
            }
        }
        // o histórico do técnico comprador: a compra e cada jogador dado na troca
        cpq4_datar(e.as_mut_ptr());
        e[0xc] = 0x64;
        cpq4_por(&mut e, 8, cpq4_clube_de(tec));
        cpq4_quem(&mut e, 0xd, pessoa);
        cpq4_por(&mut e, 0x1d, valor);
        cpq4_por(&mut e, 0x21, com_troca());
        cpq4_por(&mut e, 0x25, cpq4_id_ou_menos_1(vendedor));
        cpq4_por(&mut e, 0x29, tipo);
        cpq4_historico(gravar, this, read_usize(tec), &e);
        cpq4_troca(t, |x| {
            e[0xc] = 0x65;
            cpq4_quem(&mut e, 0xd, cpq4_pessoa(read_usize(x)));
            cpq4_por(&mut e, 0x1d, 0);
            cpq4_por(&mut e, 0x21, 1);
            cpq4_por(&mut e, 0x25, cpq4_i(vendedor));
            cpq4_por(&mut e, 0x29, tipo);
            cpq4_historico(gravar, this, read_usize(tec), &e);
        });
    }
    if vendedor != 0 && read_usize(vendedor + 0xcf) != 0 {
        let tec = read_usize(vendedor + 0xcf);
        let r = if cpq4_jogador(read_u8(pessoa + 0x3d)) { cpq4_registro(this, tec) } else { 0 };
        if r != 0 {
            crate::comparacao::ramo("cpq_4 venda");
            if valor > cpq4_i(r + 0x3d) {
                for k in 0..4usize {
                    cpq4_pi(r + 0x25 + 4 * k, if k == 0 { cpq4_i(pessoa) } else { cpq4_i(read_usize(pessoa + 4 * k) + 0x33) });
                }
                cpq4_datar((r + 0x35) as *mut u8);
                cpq4_pi(r + 0x3d, valor);
                *((r + 0x41) as *mut u8) = com_troca() as u8;
                cpq4_pi(r + 0x42, cpq4_i(vendedor));
                cpq4_pi(r + 0x46, cpq4_id_ou_menos_1(comprador));
            }
            write_i16(r + 0xa2, read_i16(r + 0xa2).wrapping_add(1));
            cpq4_somar64(r + 0xac, valor);
            if read_usize(t + 4) != 0 {
                write_i16(r + 0xa0, read_i16(r + 0xa0).wrapping_add(read_i16(read_usize(t + 4))));
                cpq4_troca(t, |x| cpq4_somar64(r + 0xa4, cpq4_i(cpq4_pessoa(read_usize(x)) + 0x52)));
            }
        }
        // o histórico do técnico vendedor: a venda e cada jogador recebido na troca
        cpq4_datar(e.as_mut_ptr());
        e[0xc] = 0x65;
        cpq4_por(&mut e, 8, cpq4_clube_de(tec));
        cpq4_quem(&mut e, 0xd, pessoa);
        cpq4_por(&mut e, 0x1d, valor);
        cpq4_por(&mut e, 0x21, com_troca());
        cpq4_por(&mut e, 0x25, cpq4_id_ou_menos_1(comprador));
        cpq4_por(&mut e, 0x29, tipo);
        cpq4_historico(gravar, this, read_usize(tec), &e);
        cpq4_troca(t, |x| {
            e[0xc] = 0x64;
            cpq4_quem(&mut e, 0xd, cpq4_pessoa(read_usize(x)));
            cpq4_por(&mut e, 0x1d, 0);
            cpq4_por(&mut e, 0x21, 1);
            cpq4_por(&mut e, 0x25, cpq4_i(comprador));
            cpq4_por(&mut e, 0x29, 0xb);
            cpq4_historico(gravar, this, read_usize(tec), &e);
        });
    }
    // o histórico do jogador transferido e o de cada jogador da troca
    cpq4_datar(e.as_mut_ptr());
    e[0xc] = 0x66;
    cpq4_por(&mut e, 8, cpq4_clube_de(pessoa));
    cpq4_por(&mut e, 0xd, cpq4_id_ou_menos_1(comprador));
    cpq4_por(&mut e, 0x11, valor);
    cpq4_por(&mut e, 0x19, tipo);
    cpq4_por(&mut e, 0x15, com_troca());
    if pessoa != 0 {
        cpq4_historico(gravar, this, read_usize(pessoa), &e);
    } else {
        aviso_em(0xa83b78, 0x1010);
    }
    cpq4_troca(t, |x| {
        let q = cpq4_pessoa(read_usize(x));
        cpq4_por(&mut e, 8, cpq4_i(read_usize(q + 0x39)));
        cpq4_por(&mut e, 0x11, 0);
        cpq4_por(&mut e, 0xd, cpq4_i(vendedor));
        cpq4_por(&mut e, 0x15, 1);
        cpq4_por(&mut e, 0x19, read_u8(x + 6) as i8 as i32);
        if q != 0 {
            cpq4_historico(gravar, this, read_usize(q), &e);
        } else {
            aviso_em(0xa83b78, 0x1010);
        }
    });
}

/// in the buying and selling managers' records: the biggest signing or sale, the counts, the 64-bit totals and the
/// exchange players; the buy (0x64), sale (0x65) and transfer (0x66) histories of the managers, the person and the
/// exchange players
/// Original: `Staff_Records::add_transfer_records` (0x86cd70, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86cd70 } (thiscall, ret 4, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_transfer_records(transf)`:
/// o contrato da pessoa (0x4cf5a0) vai para [0xae1080] e dá o vendedor (fora do estado 5); o valor total da oferta
/// (0x8e7a70 com 1, 0, 0, 1). O técnico do comprador, sendo de jogador o tipo da oferta (+0x11, 0x53b620): no registro
/// (0x86f5b0) a maior compra (+0..+0x24 se o valor passa +0x18), as compras (+0xa0), o gasto (+0xa4, 64 bits), os
/// jogadores dados (+0xa2) e o valor deles (+0x52 de cada, em +0xac). O técnico do vendedor, sendo de jogador o cargo
/// da pessoa (+0x3d): a maior venda (+0x25..+0x49 se passa +0x3d), as vendas (+0xa2), a receita (+0xac), os recebidos
/// (+0xa0) e o valor deles (+0xa4). Os históricos (tipos 0x64 compra, 0x65 venda, 0x66 transferência) do técnico do
/// comprador, do vendedor, da pessoa e dos jogadores da troca, no arquivo (0x58e340).
pub(crate) unsafe extern "thiscall" fn registrar_a_transferencia(this: usize, t: usize) {
    cpq4_transferencia(this, t, true)
}

/// Comparador: lado a lado pela global [0xae1080], o estado do sorteador (o valor da oferta pode pedir a avaliação), a
/// lista da troca (o cursor) e os registros dos dois técnicos (criados antes só quando a original os pede); a cópia
/// sem os históricos em arquivo, que a original já gravou, e sem contar os sorteios.
pub(crate) unsafe extern "thiscall" fn comparar_registrar_a_transferencia(this: usize, t: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1054);
    let (pessoa, _, vendedor, comprador) = cpq4_lados(t);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.push((0xae1080, 4));
    if read_usize(t + 4) != 0 {
        reg.push((read_usize(t + 4), 0xe));
    }
    for (clube, cargo) in [(comprador, read_u8(t + 0x11)), (vendedor, read_u8(pessoa + 0x3d))] {
        if clube != 0 && read_usize(clube + 0xcf) != 0 && cpq4_jogador(cargo) {
            let r = cpq4_registro(this, read_usize(clube + 0xcf));
            if r != 0 {
                reg.push((r, 0xd2));
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, t),
        || crate::aleatorio::sem_contar(|| cpq4_transferencia(this, t, false)));
    PLACAR.registrar(igual, || format!("registrar_a_transferencia(pessoa {:x}) {:x?} x {:x?}", pessoa, mo, mr));
}

/// A entrada da pessoa: o clube (+8), o tipo (+0xc), a competição ou −1 (+0xd) e a fase com sinal (+0x11).
pub(crate) unsafe fn cpq4_entrada_da_fase(clube: usize, tipo: u8, comp: usize, fase: u32) -> [u8; 0x40] {
    let mut e = cpq4_entrada();
    cpq4_datar(e.as_mut_ptr());
    cpq4_por(&mut e, 8, cpq4_i(clube));
    e[0xc] = tipo;
    cpq4_por(&mut e, 0xd, cpq4_id_ou_menos_1(comp));
    cpq4_por(&mut e, 0x11, fase as u16 as i16 as i32);
    e
}

/// Na competição de seleção guardada (0x76d940 de [0xb74318]), as convocações da seleção para ela acabam (0x774f90).
pub(crate) unsafe fn cpq4_fim_das_convocacoes(clube: usize, comp: usize) {
    let especial: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_NATIONAL_TEAMS_IS_FINALS_COMP);
    if especial(read_usize(0xb74318), comp) != 0 {
        crate::comparacao::ramo("cpq_4 fim das convocacoes");
        let encerrar: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_END_DUTY_FOR_NATION_IN_COMPETITION);
        encerrar(read_usize(0xb74318), clube, comp);
    }
}

/// the manager history (type 4, with the round and the base round) and the end of the national team duty in the stored
/// competition
/// Original: `Staff_Records::add_knocked_out_record` (0x86c000, staff_records.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x86c000 } (thiscall, ret 0x14, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_knocked_out_record(clube,
/// comp, fase, rodada, _)`: o histórico do técnico (+0xcf; tipo 4: clube, competição ou −1, a fase com sinal e a rodada
/// base 0x4b0870) e o fim das convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x73b).
pub(crate) unsafe extern "thiscall" fn registrar_a_eliminacao(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, _x: u32) {
    if clube == 0 {
        return aviso_em(0xa83b78, 0x73b);
    }
    crate::comparacao::ramo("cpq_4 eliminado");
    let mut e = cpq4_entrada_da_fase(clube, 4, comp, fase);
    let base: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(FN_COMP_UTIL_GET_BASE_ROUND_ID);
    cpq4_por(&mut e, 0x15, base(rodada) as u16 as i16 as i32);
    if read_usize(clube + 0xcf) != 0 {
        cpq4_historico(true, this, read_usize(read_usize(clube + 0xcf)), &e);
    }
    cpq4_fim_das_convocacoes(clube, comp);
}

// ponytail: o histórico vai para o arquivo e as convocações acabam (não rodam duas vezes) e a cópia sem eles não mexe
// em nada: só a original; a cópia leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_registrar_a_eliminacao(this: usize, clube: usize, comp: usize, fase: u32, rodada: u32, x: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32)>(1055)(this, clube, comp, fase, rodada, x)
}

/// the manager history (type 7, with the round) and the end of the national team duty in the stored competition
/// Original: `Staff_Records::add_failed_to_qualify_record` (0x86c1d0, staff_records.cpp, thiscall, ret 0x10).
///
/// [pt] { 0x86c1d0 } (thiscall, ret 0x10, staff_records.cpp, SEH sem destrutor): `Staff_Records::add_failed_to_qualify_record(
/// clube, comp, fase, _)`: o histórico do técnico (tipo 7: clube, competição ou −1, a fase com sinal) e o fim das
/// convocações da seleção na competição guardada. Clube nulo: a caixa "Error" (linha 0x768).
pub(crate) unsafe extern "thiscall" fn registrar_a_nao_classificacao(this: usize, clube: usize, comp: usize, fase: u32, _x: u32) {
    if clube == 0 {
        return aviso_em(0xa83b78, 0x768);
    }
    crate::comparacao::ramo("cpq_4 nao classificado");
    let e = cpq4_entrada_da_fase(clube, 7, comp, fase);
    if read_usize(clube + 0xcf) != 0 {
        cpq4_historico(true, this, read_usize(read_usize(clube + 0xcf)), &e);
    }
    cpq4_fim_das_convocacoes(clube, comp);
}

// ponytail: como a eliminação: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_registrar_a_nao_classificacao(this: usize, clube: usize, comp: usize, fase: u32, x: u32) {
    orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32)>(1056)(this, clube, comp, fase, x)
}

// ---------- 251a leva: os pontos dos técnicos: a virada do mês e do ano, o fim da temporada da competição e o técnico do mês (staff_records.cpp) ----------


pub(crate) unsafe fn cpo3_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}


pub(crate) unsafe fn cpo3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// O registro de pontos de cada técnico (Staff_Records +8): 0xd2 bytes por pessoa que não é jogador ([0xae2368]) e
/// pelos 16 humanos. +0x86 id da pessoa (−1 vazio), +0xb6 pontos do ano, +0xba/+0xbe pontos na liga e na copa (+0xc2 e
/// +0xc6 as competições deles, −1 sem), +0xca/+0xce os do mês.
pub(crate) const CPO3_REGISTRO: usize = 0xd2;

pub(crate) unsafe fn cpo3_quantos() -> i32 {
    cpo3_i(0xae2368).wrapping_add(0x10)
}

pub(crate) unsafe fn cpo3_registro(this: usize, k: i32) -> usize {
    read_usize(this + 8).wrapping_add((k as usize).wrapping_mul(CPO3_REGISTRO))
}

/// A pessoa pelo id (registro de 0x6e bytes em [0xae23bc]; −1 dá o registro antes da tabela, como na original).
pub(crate) unsafe fn cpo3_pessoa(id: usize) -> usize {
    read_usize(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O mês da data (0x54cfb0, o al com sinal).
pub(crate) unsafe fn cpo3_mes(data: usize) -> i16 {
    let mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_DATE_GET_MONTH);
    mes(data) as u8 as i8 as i16
}

/// at the turn of the month or year clears every manager's monthly points and stores the month and year; on the first
/// day of the year passes the yearly points to the hall of fame and clears them
/// Original: `Staff_Records::daily_update` (0x86dda0, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86dda0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::daily_update(data)`: se o mês (0x54cfb0) ou o ano
/// (+2) da data não são os guardados (+0x14 mês, +0x16 ano), zera os pontos do mês (+0xca, +0xce) de todos os
/// registros e guarda o mês e o ano novos. No primeiro dia do ano (dia +0 = 0) passa os pontos do ano (+0xb6) de cada
/// registro com pessoa (+0x86) ao hall da fama (0x5ef270 em [0xb5ded8]) e os zera.
pub(crate) unsafe extern "thiscall" fn pontos_dos_tecnicos_no_dia(this: usize, data: usize) {
    if cpo3_mes(data) as i32 != read_i16(this + 0x14) as u16 as i32 || read_i16(data + 2) != read_i16(this + 0x16) {
        crate::comparacao::ramo("cpo_3 virada do mes");
        let base = read_usize(this + 8);
        let n = cpo3_quantos();
        for k in 0..n.max(0) as usize {
            let e = base.wrapping_add(k * CPO3_REGISTRO);
            cpo3_por(e + 0xca, 0);
            cpo3_por(e + 0xce, 0); // os pontos do mês
        }
        let m = cpo3_mes(data);
        core::ptr::write_unaligned((this + 0x14) as *mut i16, m);
        core::ptr::write_unaligned((this + 0x16) as *mut i16, read_i16(data + 2));
    }
    if read_i16(data) == 0 {
        let base = read_usize(this + 8);
        let hall: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_HALLOFFAME_ADD_CALENDAR_YEAR_ENTRY);
        let mut k = 0i32;
        while k < cpo3_quantos() {
            let e = base.wrapping_add((k as usize).wrapping_mul(CPO3_REGISTRO));
            let id = read_usize(e + 0x86);
            if id as i32 != -1 {
                hall(read_usize(0xb5ded8), cpo3_pessoa(id), read_usize(e + 0xb6));
            }
            cpo3_por(e + 0xb6, 0);
            k += 1;
        }
    }
}

/// at the end of the competition's season passes its managers' league points (with the cup points of the same
/// competition) and cup points to the hall of fame and clears them
/// Original: `Staff_Records::end_season_for_nation` (0x86d6e0, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86d6e0 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::end_season_for_nation(entrada)`: com a competição
/// da entrada (+0, id em +0 dela; sem competição −1; sem competição e sem +4 igual a [0x9cfa10], nada), para cada
/// registro: liga (+0xc2) da competição: soma a copa (+0xbe) se ela também é dessa competição (+0xc6 = −1, +0xbe = 0),
/// passa os pontos (+0xba) ao hall da fama (0x5ef330) se a competição guarda (+0x11c), zera e +0xc2 = −1; só a copa da
/// competição: passa +0xbe, zera e +0xc6 = −1.
pub(crate) unsafe extern "thiscall" fn pontos_dos_tecnicos_no_fim_da_competicao(this: usize, entrada: usize) {
    let p = read_usize(entrada + 4);
    let pela_global = p != 0 && read_usize(p) == read_usize(KEY_CONTINENT_EUROPE);
    if !pela_global && read_usize(entrada) == 0 {
        return;
    }
    let comp = read_usize(entrada);
    let id = if comp != 0 { cpo3_i(comp) } else { -1 };
    let hall: unsafe extern "thiscall" fn(usize, usize, i32, usize) = core::mem::transmute(FN_HALLOFFAME_ADD_LEAGUE_ENTRY);
    let guarda = |c: usize| c != 0 && *((c + 0x11c) as *const u8) != 0;
    let base = read_usize(this + 8);
    let mut k = 0i32;
    while k < cpo3_quantos() {
        let s = base.wrapping_add((k as usize).wrapping_mul(CPO3_REGISTRO)) + 0xba;
        let copa = cpo3_i(s + 0xc);
        if cpo3_i(s + 8) == id {
            crate::comparacao::ramo("cpo_3 fim da liga");
            if copa == id {
                cpo3_por(s, read_usize(s).wrapping_add(read_usize(s + 4)));
                cpo3_por(s + 0xc, usize::MAX);
                cpo3_por(s + 4, 0);
            }
            if guarda(read_usize(entrada)) {
                hall(read_usize(0xb5ded8), cpo3_pessoa(read_usize(s - 0x34)), id, read_usize(s));
            }
            cpo3_por(s, 0);
            cpo3_por(s + 8, usize::MAX);
        } else if copa == id {
            if guarda(read_usize(entrada)) {
                hall(read_usize(0xb5ded8), cpo3_pessoa(read_usize(s - 0x34)), id, read_usize(s + 4));
            }
            cpo3_por(s + 4, 0);
            cpo3_por(s + 0xc, usize::MAX);
        }
        k += 1;
    }
}

/// the league manager with the most points this month (above 0) among those two weeks in the job; a league without a
/// table: the "Error" box and 0
/// Original: `Staff_Records::find_manager_of_the_month` (0x86f950, staff_records.cpp, thiscall, ret 4).
///
/// [pt] { 0x86f950 } (thiscall, ret 4, staff_records.cpp): `Staff_Records::find_manager_of_the_month(indice)`: na liga
/// ([0xadadfc] pelo índice; +0x43 tem de ser 2, senão a caixa "Error" e 0), entre os clubes da tabela (+0xb1, 0x3b
/// bytes, conta +0x3e) com técnico (+0xcf) no clube (+0x39) desde duas semanas atrás (0x54cab0 da data do jogo; ano
/// +0x40 sem sinal, dia +0x3e com sinal), o de mais pontos no mês (+0xce do registro 0x86f5b0, criado se falta), só
/// acima de 0. Devolve o técnico ou 0.
pub(crate) unsafe extern "thiscall" fn tecnico_do_mes_na_liga(this: usize, indice: i32) -> usize {
    let liga = read_usize(read_usize(0xadadfc).wrapping_add((indice as usize).wrapping_mul(4)));
    if *((liga + 0x43) as *const u8) != 2 {
        aviso_em(0xa83b78, 0x104e);
        return 0;
    }
    let antes: unsafe extern "thiscall" fn(usize, *mut u32, i32) -> usize = core::mem::transmute(FN_DATE_SUB_WEEKS);
    let registro: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_STAFF_RECORDS_GET_MANAGER_RECORD);
    let mut melhor_pontos = 0i32;
    let mut melhor = 0usize;
    let mut k = 0i32;
    while k < read_i16(liga + 0x3e) as i32 {
        let clube = read_usize(read_usize(liga + 0xb1).wrapping_add((k as usize).wrapping_mul(0x3b)));
        let tecnico = read_usize(clube + 0xcf);
        if tecnico != 0 && read_usize(tecnico + 0x39) != 0 {
            let mut d = [0u32; 2];
            let q = antes(0xae2c90, d.as_mut_ptr(), 2); // duas semanas atrás
            let lo = read_usize(q) as u32;
            let ano = (lo >> 16) as u16;
            let ano_tecnico = read_i16(tecnico + 0x40) as u16;
            let recente = ano < ano_tecnico || (ano == ano_tecnico && (lo as u16 as i16) < read_i16(tecnico + 0x3e));
            if recente {
                crate::comparacao::ramo("cpo_3 tecnico recente");
            } else {
                let r = registro(this, tecnico, 1);
                if r != 0 {
                    let pontos = cpo3_i(r + 0xce);
                    if pontos > melhor_pontos { // o melhor do mês
                        crate::comparacao::ramo("cpo_3 melhor do mes");
                        melhor_pontos = pontos;
                        melhor = read_usize(clube + 0xcf);
                    }
                }
            }
        }
        k += 1;
    }
    melhor
}

/// Comparador: os pontos vão ao hall da fama (0x5ef270, soma): roda só a original e confere o plano: na virada, os pontos
/// do mês zerados e o mês e o ano guardados; no primeiro dia do ano, os pontos do ano zerados.
pub(crate) unsafe extern "thiscall" fn comparar_pontos_dos_tecnicos_no_dia(this: usize, data: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(973);
    let virada = cpo3_mes(data) as i32 != read_i16(this + 0x14) as u16 as i32 || read_i16(data + 2) != read_i16(this + 0x16);
    let ano = read_i16(data) == 0;
    let (m, a) = (cpo3_mes(data), read_i16(data + 2));
    o(this, data);
    if !virada && !ano {
        return;
    }
    let mut ok = !virada || (read_i16(this + 0x14) == m && read_i16(this + 0x16) == a);
    let mut primeiro = -1i32;
    for k in 0..cpo3_quantos().max(0) {
        let e = cpo3_registro(this, k);
        let certo = (!virada || (read_usize(e + 0xca) == 0 && read_usize(e + 0xce) == 0)) && (!ano || read_usize(e + 0xb6) == 0);
        if !certo && primeiro < 0 {
            primeiro = k;
        }
        ok &= certo;
    }
    PLACAR.registrar(ok, || format!("pontos_dos_tecnicos_no_dia(mês {}, ano {}) virada {} ano novo {}: registro {} não zerado", m, a, virada, ano, primeiro));
}

/// Comparador: os pontos vão ao hall da fama (0x5ef330, soma): roda só a original e confere o plano feito antes: os
/// quatro campos (+0xba, +0xbe, +0xc2, +0xc6) de cada registro.
pub(crate) unsafe extern "thiscall" fn comparar_pontos_dos_tecnicos_no_fim_da_competicao(this: usize, entrada: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(974);
    let p = read_usize(entrada + 4);
    if !(p != 0 && read_usize(p) == read_usize(KEY_CONTINENT_EUROPE)) && read_usize(entrada) == 0 {
        return o(this, entrada);
    }
    let comp = read_usize(entrada);
    let id = if comp != 0 { cpo3_i(comp) } else { -1 };
    let n = cpo3_quantos().max(0);
    let mut plano = Vec::with_capacity(n as usize);
    for k in 0..n {
        let s = cpo3_registro(this, k) + 0xba;
        let (mut liga, mut copa, mut cl, mut cc) = (read_usize(s), read_usize(s + 4), cpo3_i(s + 8), cpo3_i(s + 0xc));
        if cl == id {
            if cc == id {
                cc = -1; // a copa somada à liga, que vai ao hall da fama e zera
                copa = 0;
            }
            liga = 0;
            cl = -1;
        } else if cc == id {
            copa = 0;
            cc = -1;
        }
        plano.push((liga, copa, cl, cc));
    }
    o(this, entrada);
    let errado = (0..n).find(|&k| {
        let s = cpo3_registro(this, k) + 0xba;
        plano[k as usize] != (read_usize(s), read_usize(s + 4), cpo3_i(s + 8), cpo3_i(s + 0xc))
    });
    PLACAR.registrar(errado.is_none(), || format!("pontos_dos_tecnicos_no_fim_da_competicao(competição {}) registro {:?} difere do plano", id, errado));
}

/// Comparador: pelo valor (o registro 0x86f5b0 criado pela original já existe para a cópia).
pub(crate) unsafe extern "thiscall" fn comparar_tecnico_do_mes_na_liga(this: usize, indice: i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32) -> usize>(975);
    let liga = read_usize(read_usize(0xadadfc).wrapping_add((indice as usize).wrapping_mul(4)));
    if *((liga + 0x43) as *const u8) != 2 {
        return o(this, indice);
    }
    let r = o(this, indice);
    let n = tecnico_do_mes_na_liga(this, indice);
    PLACAR.registrar(r == n, || format!("tecnico_do_mes_na_liga({}) original={:x} rust={:x}", indice, r, n));
    r
}

