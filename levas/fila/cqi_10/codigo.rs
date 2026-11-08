// ---------- %%LEVA%%a leva: a Copa Intertoto e a Copa da UEFA: os três vencedores do Intertoto na Copa da UEFA e o destino de cada clube nas duas (intertoto_cup.cpp, uefa_cup.cpp) ----------

unsafe fn cqi10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqi10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqi10_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O objeto da Copa da UEFA ([[0xadadfc] + 4 × [0x9cf6f0]]), ou 0.
unsafe fn uefa_cup_object() -> usize {
    cqi10_d(cqi10_d(0xadadfc).wrapping_add(cqi10_d(0x9cf6f0).wrapping_mul(4)))
}

/// Os vencedores do Intertoto (+5 = 1 no registro de 6 bytes de +0xa7) que cabem nas três últimas vagas da UEFA, na
/// ordem em que a 0x633d40 os põe: as vagas vão de +0x3e da UEFA − 27 até + 3 (palavras com sinal).
unsafe fn intertoto_winners_for_uefa(this: usize, uefa: usize) -> ([usize; 3], usize) {
    let mut v = [0usize; 3];
    let mut k = 0usize;
    let mut vaga = cqi10_w(uefa + 0x3e).wrapping_sub(0x1b);
    let limit = vaga.wrapping_add(3);
    let mut i = 0i16;
    if cqi10_w(this + 0x3e) > 0 {
        loop {
            if vaga >= limit {
                break;
            }
            let e = cqi10_d(this + 0xa7).wrapping_add((i as isize * 6) as usize);
            if cqi10_b(e + 5) == 1 {
                v[k] = cqi10_d(e);
                k += 1;
                vaga = vaga.wrapping_add(1);
            }
            i = i.wrapping_add(1);
            if i >= cqi10_w(this + 0x3e) {
                break;
            }
        }
    }
    (v, k)
}

/// { 0x633d40 } (thiscall, ret 8, intertoto_cup.cpp; a virtual +0x14 do Intertoto): `IntertotoCup::update_for_date(hoje,
/// fase)`. Uma vez (+0xb2 = 0), quando a copa acabou (virtual +0x10 com 0, 1: push 1; push 0; mov ecx, this ->
/// thiscall, ret 8; o byte de baixo): sem a Copa da UEFA, a caixa da linha 0x29e; com ela, cada time do Intertoto com
/// +5 = 1 (os vencedores das finais) vai, em ordem, para as vagas de +0x3e − 27 em diante da tabela +0xa7 da UEFA (até
/// três: o clube, +4 e +5 zerados) e para o livro de inscritos dela (+0x14, 6 bytes: o clube e a situação 4 em +4;
/// +0x36 sobe); depois as posições previstas da UEFA (virtual +0x5c: mov ecx, uefa -> thiscall, ret 0), +0xb2 = 1 e de
/// novo as posições previstas (a original relê a UEFA). Sempre, no fim, a atualização da copa (0x51f890: push fase;
/// push hoje; mov ecx, this -> thiscall, ret 8), cuja devolução sai.
unsafe extern "thiscall" fn intertoto_cup_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqi10_d(this + 0xb2) == 0 {
        let acabou: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(cqi10_d(cqi10_d(this) + 0x10));
        if acabou(this, 0, 1) as u8 != 0 {
            let uefa = uefa_cup_object();
            if uefa == 0 {
                show_error_at(0x9e9e14, 0x29e);
            } else {
                let mut vaga = cqi10_w(uefa + 0x3e).wrapping_sub(0x1b);
                let limit = vaga.wrapping_add(3);
                let mut i = 0i16;
                if cqi10_w(this + 0x3e) > 0 {
                    loop {
                        if vaga >= limit {
                            break;
                        }
                        let e = cqi10_d(this + 0xa7).wrapping_add((i as isize * 6) as usize);
                        if cqi10_b(e + 5) == 1 {
                            crate::comparacao::ramo("cqi_10 vencedor na uefa");
                            let s = cqi10_d(uefa + 0xa7).wrapping_add((vaga as isize * 6) as usize);
                            core::ptr::write_unaligned(s as *mut u32, cqi10_d(e) as u32);
                            *((s + 4) as *mut u8) = 0;
                            *((s + 5) as *mut u8) = 0;
                            let l = cqi10_d(uefa + 0x14).wrapping_add((cqi10_w(uefa + 0x36) as isize * 6) as usize);
                            core::ptr::write_unaligned(l as *mut u32, cqi10_d(s) as u32);
                            *((l + 4) as *mut u8) = 4; // cqi10: a situação do vencedor do Intertoto no livro da UEFA
                            core::ptr::write_unaligned((uefa + 0x36) as *mut i16, cqi10_w(uefa + 0x36).wrapping_add(1));
                            vaga = vaga.wrapping_add(1);
                        }
                        i = i.wrapping_add(1);
                        if i >= cqi10_w(this + 0x3e) {
                            break;
                        }
                    }
                }
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqi10_d(cqi10_d(uefa) + 0x5c));
                prever(uefa);
                core::ptr::write_unaligned((this + 0xb2) as *mut u32, 1);
                let uefa = uefa_cup_object();
                let prever: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqi10_d(cqi10_d(uefa) + 0x5c));
                prever(uefa);
                let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
                return base(this, hoje, fase);
            }
            core::ptr::write_unaligned((this + 0xb2) as *mut u32, 1);
        }
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
    base(this, hoje, fase)
}

// ponytail: a atualização da copa (0x51f890) joga e sorteia: não roda duas vezes. Só a original roda; o plano, feito
// antes: se +0xb2 passa de 0 a 1 com a Copa da UEFA, o livro dela (+0x14) ganha os vencedores (intertoto_winners_for_uefa), em
// ordem e com a situação 4, e +0x36 sobe tanto quanto; senão o livro não cresce.
unsafe extern "thiscall" fn comparar_intertoto_cup_update_for_date(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let uefa = uefa_cup_object();
    if uefa == 0 || cqi10_d(this + 0xb2) != 0 {
        return o(this, hoje, fase);
    }
    let (v, k) = intertoto_winners_for_uefa(this, uefa);
    let n36 = cqi10_w(uefa + 0x36);
    let r = o(this, hoje, fase);
    let passou = cqi10_d(this + 0xb2) == 1;
    let previsto = if passou { n36.wrapping_add(k as i16) } else { n36 };
    let mut igual = cqi10_w(uefa + 0x36) == previsto;
    if passou && igual {
        for (j, &c) in v.iter().enumerate().take(k) {
            let l = cqi10_d(uefa + 0x14).wrapping_add(((n36 as isize) + j as isize) as usize * 6);
            igual &= cqi10_d(l) == c && cqi10_b(l + 4) == 4;
        }
    }
    PLACAR.registrar(igual, || format!("intertoto_cup_update_for_date({:#x}) passou={} livro {} -> {} previsto {} vencedores {:x?}", this, passou, n36, cqi10_w(uefa + 0x36), previsto, &v[..k]));
    r
}

/// { 0x633c10 } (thiscall, ret 0x18, intertoto_cup.cpp; a virtual +0x44 do Intertoto): `IntertotoCup::
/// update_staff_records_for_status(clube, desfecho, _, _, jogo, _)`; devolve 0. Peso = 0x4bf850(baixo, alto, rodada do
/// jogo +0x34, rodadas +0x3c) (cdecl, 4 pushes; add esp, 0x10).
/// - 1 (venceu a final): a classificação (0x86bdd0) para a Copa da UEFA (a competição [0xae23d0] + 0x6b × [0x9cf6f0]),
///   fase 0, rodada 0xa, peso(0x3c, 0x64), e o clube passa à Copa da UEFA (+0x1db = [0x9cf6f0]).
/// - 0 (passou de rodada): se a rodada seguinte existe (+0x34 + 1 < +0x3c), a classificação no Intertoto (+4) com a
///   fase +0x32 do jogo, o +7 da rodada seguinte (+0xa3, 0x68 bytes) e o peso rodada × 99 / (rodadas − 1) + 1 (só o
///   byte de baixo sobe: inc al).
/// - 0xff ou 2 (caiu): a eliminação (0x86c000) no Intertoto, fase 0, rodada +0x30 do jogo, peso(0x14, 0x50), e o clube
///   sai da competição europeia (+0x1db = −1).
unsafe extern "thiscall" fn intertoto_cup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    match desfecho as u8 {
        1 => {
            let peso = scale_by_round(0x3c, 0x64, cqi10_w(jogo + 0x34), cqi10_w(this + 0x3c) as u16 as u32);
            let uefa = cqi10_d(0xae23d0).wrapping_add(cqi10_d(0x9cf6f0).wrapping_mul(0x6b));
            european_cup_add_qualified_record(clube, uefa, 0, 0xa, peso);
            crate::comparacao::ramo("cqi_10 vencedor do intertoto");
            core::ptr::write_unaligned((clube + 0x1db) as *mut u32, cqi10_d(0x9cf6f0) as u32); // cqi10: o vencedor passa à UEFA
        }
        0 => {
            let r = cqi10_w(jogo + 0x34);
            let n = cqi10_w(this + 0x3c);
            let seguinte = r.wrapping_add(1);
            if seguinte >= n {
                return 0;
            }
            let div = n as i32 - 1;
            if div == 0 {
                return original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%)(this, clube, desfecho, a3, a4, jogo, a6); // ponytail: a original cai na divisão
            }
            let q = (r as i32 * 99) / div;
            let peso = (q as u32 & !0xff) | (q as u8).wrapping_add(1) as u32; // inc al
            let rodada = core::ptr::read_unaligned((cqi10_d(this + 0xa3).wrapping_add((seguinte as isize * 0x68) as usize) + 7) as *const u16) as u32;
            // ponytail: a fase vai num registrador cujos 16 bits altos são os do movsx da rodada seguinte (zero); limpa
            european_cup_add_qualified_record(clube, cqi10_d(this + 4), core::ptr::read_unaligned((jogo + 0x32) as *const u16) as u32, rodada, peso);
        }
        0xff | 2 => {
            let peso = scale_by_round(0x14, 0x50, cqi10_w(jogo + 0x34), cqi10_w(this + 0x3c) as u16 as u32);
            // ponytail: a rodada +0x30 vai num registrador com lixo nos 16 bits altos; a 0x86c000 só usa a palavra
            european_cup_add_knocked_out_record(clube, cqi10_d(this + 4), 0, core::ptr::read_unaligned((jogo + 0x30) as *const u16) as u32, peso);
            crate::comparacao::ramo("cqi_10 caiu no intertoto");
            leave_euro_competition(clube); // cqi10: quem cai no Intertoto sai da competição europeia
        }
        _ => {}
    }
    0
}

// ponytail: os registros vão para o histórico das pessoas (reputação, moral): não rodam duas vezes. Só a original roda;
// o plano confere o +0x1db do clube: [0x9cf6f0] para o vencedor, −1 para quem cai, intacto nos outros.
unsafe extern "thiscall" fn comparar_intertoto_cup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I1%%);
    if clube == 0 {
        return o(this, clube, desfecho, a3, a4, jogo, a6);
    }
    let antes = cqi10_d(clube + 0x1db);
    let previsto = match desfecho as u8 {
        1 => cqi10_d(0x9cf6f0),
        0xff | 2 => 0xffff_ffff,
        _ => antes,
    };
    let r = o(this, clube, desfecho, a3, a4, jogo, a6);
    let depois = cqi10_d(clube + 0x1db);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("intertoto_cup_update_staff_records_for_status({:#x}, desfecho {}) +0x1db {:x} -> {:x}, previsto {:x}, devolveu {}", clube, desfecho as u8, antes, depois, previsto, r));
    r
}

/// { 0x9030e0 } (thiscall, ret 0x18, uefa_cup.cpp; a virtual +0x44 da Copa da UEFA): `UefaCup::
/// update_staff_records_for_status(clube, desfecho, a3, a4, jogo, a6)`: com o desfecho (byte) 1, 2 ou 0xff (campeão,
/// vice ou eliminado) o clube sai da competição europeia (+0x1db = −1); depois os registros da base (0x519890: os seis
/// argumentos como chegaram, o desfecho como dword; mov ecx, this -> thiscall, ret 0x18), cuja devolução sai.
unsafe extern "thiscall" fn uefa_cup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let d = desfecho as u8;
    if d == 1 || d == 2 || d == 0xff {
        if d == 0xff {
            crate::comparacao::ramo("cqi_10 caiu na uefa");
        }
        core::ptr::write_unaligned((clube + 0x1db) as *mut i32, -1); // cqi10: quem termina na UEFA sai da competição europeia
    }
    let base: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32 = core::mem::transmute(0x519890usize);
    base(this, clube, desfecho, a3, a4, jogo, a6)
}

// ponytail: a base grava os registros (histórico, reputação, moral) e pode dar a vaga da UEFA ao campeão: não roda duas
// vezes. Só a original roda; o plano confere o +0x1db do clube (−1 nos desfechos 1, 2 e 0xff, intacto nos outros).
unsafe extern "thiscall" fn comparar_uefa_cup_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, a3: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(%%I2%%);
    if clube == 0 {
        return o(this, clube, desfecho, a3, a4, jogo, a6);
    }
    let d = desfecho as u8;
    let antes = cqi10_d(clube + 0x1db);
    let previsto = if d == 1 || d == 2 || d == 0xff { 0xffff_ffff } else { antes };
    let r = o(this, clube, desfecho, a3, a4, jogo, a6);
    let depois = cqi10_d(clube + 0x1db);
    PLACAR.registrar(r == 0 && depois == previsto, || format!("uefa_cup_update_staff_records_for_status({:#x}, desfecho {}) +0x1db {:x} -> {:x}, previsto {:x}, devolveu {}", clube, d, antes, depois, previsto, r));
    r
}
