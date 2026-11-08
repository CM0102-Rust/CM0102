// ---------- %%LEVA%%a leva: as suspensões na troca de clube: acrescentar a suspensão, soltar as vagas que não valem no clube novo e devolver as suspensões guardadas no fim do empréstimo (discipline.cpp) ----------

unsafe fn cpo11_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cpo11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpo11_por(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v);
}

unsafe fn cpo11_por_w(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// O órgão disciplinar da regra `regra` (byte com sinal): [[this] + 4 × regra].
unsafe fn ruling_body_of_rule(this: usize, regra: u8) -> usize {
    cpo11_d(cpo11_d(this).wrapping_add((regra as i8 as isize as usize).wrapping_mul(4)))
}

/// A entrada (14 bytes; +0 livre = −1, +6 o índice da suspensão, −1 sem) da pessoa no órgão: 0x7e82e0 sem criar
/// (só lê: acha pela vaga da pessoa no registro de 29 bytes, 0x55cae0).
unsafe fn cpo11_entrada(orgao: usize, id: u32) -> usize {
    let achar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x7e82e0usize);
    achar(orgao, id, 0)
}

/// O registro de 29 bytes da pessoa `id` em [this+0x14] (4 vagas de 5 bytes: a regra e o índice da entrada; +0x14/+0x18
/// o histórico; +0x1c quantas suspensões guardadas).
unsafe fn discipline_record(this: usize, id: u32) -> usize {
    cpo11_d(this + 0x14).wrapping_add((id as usize).wrapping_mul(29))
}

/// O órgão vale para o clube? Com nação (+0x17 ≠ −1): a nação do clube (+0x53) é ela; sem nação, com continente
/// (+0x1b ≠ −1): o continente da nação do clube (+0x71) é ele; sem os dois, vale sempre. Clube nulo ou sem nação: não.
unsafe fn ruling_body_covers_club(orgao: usize, clube: usize) -> bool {
    let nacao = cpo11_d(orgao + 0x17) as u32;
    if nacao == 0xffff_ffff {
        let continente = cpo11_d(orgao + 0x1b) as u32;
        if continente == 0xffff_ffff {
            return true;
        }
        if clube == 0 || cpo11_d(clube + 0x53) == 0 {
            return false;
        }
        let c = cpo11_d(cpo11_d(clube + 0x53) + 0x71);
        return c != 0 && cpo11_d(c) as u32 == continente;
    }
    clube != 0 && cpo11_d(clube + 0x53) != 0 && cpo11_d(cpo11_d(clube + 0x53)) as u32 == nacao
}

/// Solta a suspensão da entrada (se tem: +6 > −1) e libera a vaga `k` do registro da pessoa (regra 0xff, índice −1) e a
/// entrada (+0 = −1). O índice vai no ax; o alto do eax é o do ponteiro da entrada (a 0x556d00 só lê a palavra).
unsafe fn release_entry_ban(this: usize, id: u32, e: usize) {
    let i = cpo11_w(e + 6);
    if i > -1 {
        let soltar: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x556d00usize);
        soltar(this, id, (e as u32 & 0xffff_0000) | i as u16 as u32);
    }
}

unsafe fn clear_discipline_slot(this: usize, id: u32, k: usize, e: usize) {
    let v = discipline_record(this, id).wrapping_add(k * 5);
    *(v as *mut u8) = 0xff;
    cpo11_por(v + 1, 0xffff_ffff);
    cpo11_por(e, 0xffff_ffff);
}

/// { 0x556940 } (thiscall, ret 8, discipline.cpp): `Discipline::add_ban(id, suspensao)`: se alguma das 4 regras da
/// suspensão nova (+0xc..+0xf) já tem suspensão da pessoa (a entrada do órgão com +6 > −1), estende a velha (jogos +0
/// somados, ou, com algum dos dois por dias (0xff), os dias +2) e marca as duas (+0x13 bit 0); senão (o bit 0 da nova
/// limpo a cada regra sem suspensão) copia a nova (0x556c70) na primeira vaga livre do vetor (+0 = 0xff e +2 = −1), ou
/// no fim (conta +0xa, capacidade +8; cresce 100, até 0x7d00, malloc/realloc, as vagas novas livres), liga cada regra
/// ao índice (0x7e8510) e, com a quarta regra, põe {id, nação} no índice das nações (+0xc, capacidade +0x10, conta
/// +0x12; cresce 20; qsort 0x55e5f0). Caixas "Error" (capacidade no máximo, sem memória) pela show_error_at.
unsafe extern "thiscall" fn add_ban(this: usize, id: u32, ban: usize) {
    for k in 0..4usize {
        let regra = *((ban + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpo11_entrada(ruling_body_of_rule(this, regra), id);
        if e != 0 && cpo11_w(e + 6) > -1 {
            crate::comparison::branch("cpo_11 suspensao estendida");
            let s = cpo11_d(this + 4).wrapping_add((cpo11_w(e + 6) as i32 as usize).wrapping_mul(20));
            let jogos = *(s as *const u8);
            let novos = *(ban as *const u8);
            if jogos != 0xff && novos != 0xff {
                *(s as *mut u8) = novos.wrapping_add(jogos);
            } else {
                cpo11_por_w(s + 2, (cpo11_w(s + 2) as u16).wrapping_add(cpo11_w(ban + 2) as u16));
            }
            *((ban + 0x13) as *mut u8) |= 1;
            *((s + 0x13) as *mut u8) |= 1;
            return;
        }
        *((ban + 0x13) as *mut u8) &= 0xfe;
    }
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x556c70usize);
    let n = cpo11_w(this + 0xa);
    let mut vaga: i16 = -1;
    for i in 0..n.max(0) {
        let s = cpo11_d(this + 4).wrapping_add(i as usize * 20);
        if *(s as *const u8) == 0xff && cpo11_w(s + 2) == -1 {
            vaga = i;
            copiar(s, ban);
            break;
        }
    }
    if vaga == -1 {
        let conta = cpo11_w(this + 0xa);
        let cap = cpo11_w(this + 8);
        if conta < cap {
            cpo11_por_w(this + 0xa, conta.wrapping_add(1) as u16);
            copiar(cpo11_d(this + 4).wrapping_add((conta as i32 as usize).wrapping_mul(20)), ban);
            vaga = cpo11_w(this + 0xa).wrapping_sub(1);
        } else {
            if cap >= 0x7d00 {
                show_error_at(0x9b8f18, 0x401);
                return;
            }
            let nova = cap.wrapping_add(100);
            cpo11_por_w(this + 8, nova as u16);
            let tamanho = (nova as i32 as u32).wrapping_mul(20);
            let p = if cpo11_d(this + 4) == 0 {
                let malloc: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
                malloc(tamanho)
            } else {
                let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
                realloc(cpo11_d(this + 4), tamanho)
            };
            cpo11_por(this + 4, p as u32);
            if p == 0 {
                show_error_at(0x9b8f18, 0x410);
                return;
            }
            vaga = cpo11_w(this + 0xa);
            cpo11_por_w(this + 0xa, vaga.wrapping_add(1) as u16);
            copiar(p.wrapping_add((vaga as i32 as usize).wrapping_mul(20)), ban);
            let mut i = cpo11_w(this + 0xa);
            while i < cpo11_w(this + 8) {
                let s = cpo11_d(this + 4).wrapping_add((i as i32 as usize).wrapping_mul(20));
                *(s as *mut u8) = 0xff;
                cpo11_por_w(s + 2, 0xffff);
                i = i.wrapping_add(1);
            }
        }
    }
    crate::comparison::branch("cpo_11 suspensao nova");
    let ligar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x7e8510usize);
    for k in 0..4usize {
        let regra = *((ban + 0xc + k) as *const u8);
        if regra != 0xff {
            ligar(ruling_body_of_rule(this, regra), id, vaga as u16 as u32); // liga a regra à suspensão
        }
    }
    if *((ban + 0xf) as *const u8) == 0xff {
        return;
    }
    let cap = cpo11_w(this + 0x10);
    if cap == cpo11_w(this + 0x12) {
        let nova = cap.wrapping_add(0x14);
        cpo11_por_w(this + 0x10, nova as u16);
        let realloc: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
        let p = realloc(cpo11_d(this + 0xc), (nova as i32 as u32).wrapping_mul(8));
        cpo11_por(this + 0xc, p as u32);
        if p == 0 {
            show_error_at(0x9b8f18, 0x43e);
            return;
        }
    }
    let n = cpo11_w(this + 0x12);
    let item = cpo11_d(this + 0xc).wrapping_add((n as i32 as usize).wrapping_mul(8));
    cpo11_por(item, id);
    let pessoa = cpo11_d(0xae23bc).wrapping_add((id as usize).wrapping_mul(0x6e));
    cpo11_por(item + 4, cpo11_d(cpo11_d(pessoa + 0x1a)) as u32); // a nação
    cpo11_por_w(this + 0x12, n.wrapping_add(1) as u16);
    let qsort: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(0x945333usize);
    qsort(cpo11_d(this + 0xc), cpo11_w(this + 0x12) as i32, 8, 0x55e5f0);
}

/// { 0x556270 } (thiscall, ret 0xc, discipline.cpp): `Discipline::clear_bans_invalid_for_club(pessoa, _, clube)`: cada
/// vaga ocupada das 4 do registro da pessoa cujo órgão não vale para o clube novo (`ruling_body_covers_club`) é liberada: a
/// suspensão solta (0x556d00), a vaga livre e a entrada do órgão livre. Com suspensões guardadas (+0x1c ≠ 0), a caixa
/// "Error" antes (e segue). Da troca de clube (0x8c9720).
unsafe extern "thiscall" fn clear_bans_invalid_for_club(this: usize, pessoa: usize, _nada: u32, clube: usize) {
    let id = cpo11_d(pessoa) as u32;
    let r = discipline_record(this, id);
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if regra == 0xff {
            continue;
        }
        let orgao = ruling_body_of_rule(this, regra);
        if ruling_body_covers_club(orgao, clube) {
            continue;
        }
        let e = cpo11_entrada(orgao, id);
        if e == 0 {
            continue;
        }
        crate::comparison::branch("cpo_11 transferido solta a vaga");
        release_entry_ban(this, id, e);
        if *((r + 0x1c) as *const u8) != 0 {
            show_error_at(0x9b8f18, 0x239);
        }
        clear_discipline_slot(this, id, k, e);
    }
}

/// O que a 0x556570 vai fazer, lido antes (a lista das guardadas perde e libera os itens): o contrato e o clube, os
/// itens guardados da pessoa (a vaga e os 5 bytes) e, depois deles, as vagas 0..2 não devolvidas que o clube não cobre.
struct ContractReturnPlan {
    contrato: usize,
    clube: usize,
    guardadas: Vec<(usize, [u8; 5])>,
    resto: u8,
    soltas: Vec<(usize, usize)>,
}

/// O plano da 0x556570 (o cursor da lista é zerado pela original na entrada; a busca sem criar só lê).
unsafe fn return_bans_plan(this: usize, pessoa: usize) -> ContractReturnPlan {
    let id = cpo11_d(pessoa) as u32;
    let r = discipline_record(this, id);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contrato = contract_of(cpo11_d(0xae1050), pessoa);
    let clube = if contrato == 0 { 0 } else { cpo11_d(0xae23b4).wrapping_add(cpo11_d(contrato + 4).wrapping_mul(0x245)) };
    let mut p = ContractReturnPlan { contrato, clube, guardadas: Vec::new(), resto: *((r + 0x1c) as *const u8), soltas: Vec::new() };
    let lista = this + 0x1c;
    cpo11_por(lista + 0xa, cpo11_d(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let mut vagas = [0u8; 20];
    core::ptr::copy_nonoverlapping(r as *const u8, vagas.as_mut_ptr(), 20);
    let mut marcas = [false; 4];
    let mut item = proximo(lista);
    while item != 0 && (p.resto as i8) > 0 {
        if cpo11_d(item) as u32 == id {
            let k = (*((item + 4) as *const u8) as i8 as isize as usize) & 3;
            let mut b = [0u8; 5];
            core::ptr::copy_nonoverlapping((item + 5) as *const u8, b.as_mut_ptr(), 5);
            vagas[k * 5..k * 5 + 5].copy_from_slice(&b);
            marcas[k] = true;
            p.guardadas.push((k, b));
            p.resto = p.resto.wrapping_sub(1);
        }
        item = proximo(lista);
    }
    for k in 0..3usize {
        if marcas[k] || vagas[k * 5] == 0xff {
            continue;
        }
        let orgao = ruling_body_of_rule(this, vagas[k * 5]);
        if !ruling_body_covers_club(orgao, clube) {
            let e = cpo11_entrada(orgao, id);
            if e != 0 {
                p.soltas.push((k, e));
            }
        }
    }
    p
}

/// { 0x556570 } (thiscall, ret 4, discipline.cpp): `Discipline::restore_held_bans(pessoa)`: no fim do empréstimo
/// (0x4d07e0): o contrato (0x4cf5a0, guardado em [0xae1080]) dá o clube; cada suspensão guardada da pessoa na lista
/// [this+0x1c] (id, vaga +4, os 5 bytes +5), enquanto houver guardadas (+0x1c do registro, com sinal): a vaga ocupada é
/// liberada antes (a suspensão solta, 0x556d00), a guardada volta para a vaga, sai da lista (0x68d1f0, que a libera) e a
/// conta cai. Depois, as vagas 0..2 não devolvidas (a quarta nunca: `k != 3` da original) cujo órgão não vale para o
/// clube são liberadas.
unsafe extern "thiscall" fn restore_held_bans(this: usize, pessoa: usize) {
    let id = cpo11_d(pessoa) as u32;
    let r = discipline_record(this, id);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let contrato = contract_of(cpo11_d(0xae1050), pessoa);
    cpo11_por(0xae1080, contrato as u32);
    let clube = if contrato == 0 { 0 } else { cpo11_d(0xae23b4).wrapping_add(cpo11_d(contrato + 4).wrapping_mul(0x245)) };
    let lista = this + 0x1c;
    cpo11_por(lista + 0xa, cpo11_d(this + 0x1e) as u32);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let remover: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let mut marcas = [0u32; 4];
    let mut item = proximo(lista);
    while item != 0 && *((r + 0x1c) as *const i8) > 0 {
        if cpo11_d(item) as u32 == id {
            let k = *((item + 4) as *const u8) as i8 as isize as usize;
            let regra = *(r.wrapping_add(k.wrapping_mul(5)) as *const u8);
            if regra != 0xff {
                let e = cpo11_entrada(ruling_body_of_rule(this, regra), id);
                if e != 0 {
                    release_entry_ban(this, id, e);
                    clear_discipline_slot(this, id, k, e);
                }
            }
            crate::comparison::branch("cpo_11 guardada devolvida");
            marcas[k & 3] = 1;
            let v = r.wrapping_add(k.wrapping_mul(5));
            cpo11_por(v, cpo11_d(item + 5) as u32);
            *((v + 4) as *mut u8) = *((item + 9) as *const u8);
            remover(lista, item); // libera o item
            *((r + 0x1c) as *mut u8) = (*((r + 0x1c) as *const u8)).wrapping_sub(1);
        }
        item = proximo(lista);
    }
    for k in 0..4usize {
        let regra = *((r + k * 5) as *const u8);
        if marcas[k] != 0 || k == 3 || regra == 0xff {
            continue;
        }
        let orgao = ruling_body_of_rule(this, regra);
        if ruling_body_covers_club(orgao, clube) {
            continue;
        }
        let e = cpo11_entrada(orgao, id);
        if e != 0 {
            crate::comparison::branch("cpo_11 fim do emprestimo solta a vaga");
            release_entry_ban(this, id, e);
            clear_discipline_slot(this, id, k, e);
        }
    }
}

/// O caminho da 0x556940, lido antes: a suspensão velha estendida (e os valores novos dela), ou a vaga da nova e a
/// conta do vetor depois; `None` nas caixas (capacidade no máximo).
enum SuspensionPlan {
    Estender { s: usize, jogos: u8, dias: i16 },
    Nova { vaga: i16, conta: i16 },
    Caixa,
}

unsafe fn add_ban_plan(this: usize, id: u32, ban: usize) -> SuspensionPlan {
    for k in 0..4usize {
        let regra = *((ban + 0xc + k) as *const u8);
        if regra == 0xff {
            continue;
        }
        let e = cpo11_entrada(ruling_body_of_rule(this, regra), id);
        if e != 0 && cpo11_w(e + 6) > -1 {
            let s = cpo11_d(this + 4).wrapping_add((cpo11_w(e + 6) as i32 as usize).wrapping_mul(20));
            let (jogos, novos) = (*(s as *const u8), *(ban as *const u8));
            return if jogos != 0xff && novos != 0xff {
                SuspensionPlan::Estender { s, jogos: novos.wrapping_add(jogos), dias: cpo11_w(s + 2) }
            } else {
                SuspensionPlan::Estender { s, jogos, dias: cpo11_w(s + 2).wrapping_add(cpo11_w(ban + 2)) }
            };
        }
    }
    let n = cpo11_w(this + 0xa);
    for i in 0..n.max(0) {
        let s = cpo11_d(this + 4).wrapping_add(i as usize * 20);
        if *(s as *const u8) == 0xff && cpo11_w(s + 2) == -1 {
            return SuspensionPlan::Nova { vaga: i, conta: n };
        }
    }
    if n < cpo11_w(this + 8) || cpo11_w(this + 8) < 0x7d00 {
        return SuspensionPlan::Nova { vaga: n, conta: n.wrapping_add(1) };
    }
    SuspensionPlan::Caixa
}

/// Comparador: a 0x556940 realoca os vetores e liga os órgãos (não pode rodar duas vezes): o plano antes, a original
/// roda; confere a velha estendida (jogos, dias, bit 0 das duas), ou a nova na vaga (jogos, dias, as 4 regras), a
/// conta, as entradas das regras apontando para a vaga e, com a quarta regra, o índice das nações com um a mais.
unsafe extern "thiscall" fn comparar_add_ban(this: usize, id: u32, ban: usize) {
    let plano = add_ban_plan(this, id, ban);
    let indice = cpo11_w(this + 0x12);
    let mut regras = [0u8; 4];
    core::ptr::copy_nonoverlapping((ban + 0xc) as *const u8, regras.as_mut_ptr(), 4);
    original_fn::<unsafe extern "thiscall" fn(usize, u32, usize)>(%%I0%%)(this, id, ban);
    let (ok, detalhe) = match plano {
        SuspensionPlan::Caixa => return,
        SuspensionPlan::Estender { s, jogos, dias } => {
            let ok = *(s as *const u8) == jogos && cpo11_w(s + 2) == dias && *((s + 0x13) as *const u8) & 1 == 1
                && *((ban + 0x13) as *const u8) & 1 == 1;
            (ok, format!("estender: jogos {} (esperava {}), dias {} (esperava {})", *(s as *const u8), jogos, cpo11_w(s + 2), dias))
        }
        SuspensionPlan::Nova { vaga, conta } => {
            let s = cpo11_d(this + 4).wrapping_add((vaga as i32 as usize).wrapping_mul(20));
            let mut ok = cpo11_w(this + 0xa) == conta && *(s as *const u8) == *(ban as *const u8) && cpo11_w(s + 2) == cpo11_w(ban + 2);
            for k in 0..4usize {
                ok &= *((s + 0xc + k) as *const u8) == regras[k];
                if regras[k] != 0xff {
                    let e = cpo11_entrada(ruling_body_of_rule(this, regras[k]), id);
                    ok &= e != 0 && cpo11_w(e + 6) == vaga;
                }
            }
            if regras[3] != 0xff {
                ok &= cpo11_w(this + 0x12) == indice.wrapping_add(1);
            }
            (ok, format!("nova: vaga {} conta {} (esperava {}), índice {} (antes {})", vaga, cpo11_w(this + 0xa), conta, cpo11_w(this + 0x12), indice))
        }
    };
    SCOREBOARD.record(ok, || format!("add_ban({}) {}", id, detalhe));
}

/// Comparador: a 0x556270 solta suspensões (índice das nações, órgãos): o plano antes (as vagas que o clube não cobre
/// e as entradas), a original roda; confere as vagas liberadas (0xff, −1, a entrada −1) e as outras intactas. Uma
/// entrada repetida em duas vagas não entra na conferência da segunda.
unsafe extern "thiscall" fn comparar_clear_bans_invalid_for_club(this: usize, pessoa: usize, nada: u32, clube: usize) {
    let id = cpo11_d(pessoa) as u32;
    let r = discipline_record(this, id);
    let mut antes = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, antes.as_mut_ptr(), 29);
    let mut plano: [Option<usize>; 4] = [None; 4];
    let mut duvida = [false; 4];
    for k in 0..4usize {
        let regra = antes[k * 5];
        if regra == 0xff {
            continue;
        }
        let orgao = ruling_body_of_rule(this, regra);
        if !ruling_body_covers_club(orgao, clube) {
            let e = cpo11_entrada(orgao, id);
            if e != 0 {
                duvida[k] = plano[..k].iter().any(|x| *x == Some(e));
                plano[k] = Some(e);
            }
        }
    }
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize)>(%%I1%%)(this, pessoa, nada, clube);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let mut ok = depois[20..] == antes[20..];
    for k in 0..4usize {
        if duvida[k] {
            continue;
        }
        let v = &depois[k * 5..k * 5 + 5];
        ok &= match plano[k] {
            Some(e) => v == [0xff; 5] && cpo11_d(e) as u32 == 0xffff_ffff,
            None => *v == antes[k * 5..k * 5 + 5],
        };
    }
    SCOREBOARD.record(ok, || format!("clear_bans_invalid_for_club({}) plano {:x?} antes {:02x?} depois {:02x?}", id, plano, antes, depois));
}

/// Comparador: a 0x556570 tira e libera itens da lista (lição 1: o esperado é montado ANTES; nenhum item é lido depois)
/// e solta suspensões: o plano antes, a original roda; confere [0xae1080], as vagas devolvidas (os 5 bytes guardados), as
/// soltas (0xff, −1, a entrada −1), a conta de guardadas e as outras vagas intactas.
unsafe extern "thiscall" fn comparar_restore_held_bans(this: usize, pessoa: usize) {
    let id = cpo11_d(pessoa) as u32;
    let r = discipline_record(this, id);
    let p = return_bans_plan(this, pessoa);
    let mut antes = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, antes.as_mut_ptr(), 29);
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I2%%)(this, pessoa);
    let mut depois = [0u8; 29];
    core::ptr::copy_nonoverlapping(r as *const u8, depois.as_mut_ptr(), 29);
    let mut esperado = antes;
    for (k, b) in &p.guardadas {
        esperado[k * 5..k * 5 + 5].copy_from_slice(b);
    }
    let mut ok = cpo11_d(0xae1080) == p.contrato;
    for (k, e) in &p.soltas {
        esperado[k * 5..k * 5 + 5].copy_from_slice(&[0xff; 5]);
        ok &= cpo11_d(*e) as u32 == 0xffff_ffff;
    }
    esperado[0x1c] = p.resto;
    ok &= depois == esperado;
    SCOREBOARD.record(ok, || format!("restore_held_bans({}) clube {:x} esperado {:02x?} depois {:02x?}", id, p.clube, esperado, depois));
}
