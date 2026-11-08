// ---------- %%LEVA%%a leva: a troca com compensação, a ordem das vagas da lista de compras e o jogador sob oferta (shortlist_manager.cpp) ----------

unsafe fn cqd12_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqd12_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// Contador de ramo para a rotina quente (~27 milhões de chamadas até 2002-06-01): o `ramo` só a cada 4096 passagens
/// (a primeira inclusive), para não pesar na carreira.
fn count_hot_branch(n: &core::sync::atomic::AtomicU32, nome: &'static str) {
    // ponytail: rotina quente; o contador do ramo conta de 4096 em 4096
    if n.fetch_add(1, core::sync::atomic::Ordering::Relaxed) & 0xfff == 0 {
        crate::comparacao::ramo(nome);
    }
}

static BRANCH_BOSMAN: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static HOT_BRANCH_BAND: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);
static FAR_BAND_HITS: core::sync::atomic::AtomicU32 = core::sync::atomic::AtomicU32::new(0);

/// { 0x84b6f0 } (thiscall, ret 0x10, shortlist_manager.cpp; o this não é lido): `shortlist_manager_is_compensation_deal(
/// vaga, contrato, clube, &taxa)`, pela vaga da lista de compras (0x842e10, 0x840000), pelo relatório do olheiro
/// (0x7b62a0), pelo lance (0x8e5d60) e pela tela. Zera a taxa; sem contrato, sem vaga ou com o byte +0x49 do contrato:
/// 1. A pessoa do contrato ([0xae23bc] + 110 × [+0]). Fora do período protegido da FIFA (0x868210 zero): a taxa de
/// compensação (0x8d0780 com 0) e 1 se ela é menor que o valor (+0x52) da pessoa. Senão: sem lei Bosman no clube do
/// jogador (0x8d0a70), 0; a taxa (0x8d0780 com 1) acima do valor, 0; pela faixa do vencimento (0x867d30): 0 → 0; 1-2,
/// 3-4, 5-6, 7-8 → 1 se o +3 da vaga × 0,5 / 0,65 / 0,8 / 0,9 passa do preço (+0xb); 9-14 → 1; acima, 0.
unsafe extern "thiscall" fn shortlist_manager_is_compensation_deal(this: usize, vaga: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let _ = this;
    *taxa = 0;
    if contrato == 0 || vaga == 0 || *((contrato + 0x49) as *const u8) != 0 {
        return 1;
    }
    let pessoa = |c: usize| cqd12_d(0xae23bc).wrapping_add(cqd12_d(c).wrapping_mul(0x6e));
    let protegido: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> u32 = core::mem::transmute(0x868210usize);
    let compensacao: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> i32 = core::mem::transmute(0x8d0780usize);
    let bosman: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x8d0a70usize);
    let band: unsafe extern "thiscall" fn(usize) -> u8 = core::mem::transmute(0x867d30usize);
    // 0x868210: push 0; push 0; push clube; mov ecx, contrato -> thiscall; 0x8d0780 e 0x8d0a70: mov ecx, [0xde1f54]
    // antes dos pushes (a cópia passa o ecx como a original; as duas só leem a pilha, ret 0xc e ret 4); 0x867d30: mov
    // ecx, contrato -> thiscall
    let gerente = cqd12_d(0xde1f54);
    if protegido(contrato, clube, 0, 0) == 0 {
        let t = compensacao(gerente, pessoa(contrato), clube, 0);
        *taxa = t;
        return (t < cqd12_i(pessoa(contrato) + 0x52)) as u32;
    }
    if bosman(gerente, pessoa(contrato)) == 0 { // cqd12: sem a lei Bosman
        return 0;
    }
    count_hot_branch(&BRANCH_BOSMAN, "cqd_12 com lei Bosman");
    let t = compensacao(gerente, pessoa(contrato), clube, 1);
    *taxa = t;
    if t > cqd12_i(pessoa(contrato) + 0x52) {
        return 0;
    }
    let k = band(contrato) as i8 as i32;
    if k as u32 > 0xe {
        return 0;
    }
    let fator = match k {
        0 => return 0,
        1 | 2 => 0.5, // cqd12: a faixa 1-2
        3 | 4 => 0.65,
        5 | 6 => 0.8,
        7 | 8 => 0.9,
        _ => {
            count_hot_branch(&FAR_BAND_HITS, "cqd_12 band longe");
            return 1; // cqd12: as faixas 9-14
        }
    };
    count_hot_branch(&HOT_BRANCH_BAND, "cqd_12 band pelo fator");
    // fild [vaga+0xb]; fild [vaga+3]; fmul fator; fcompp: 1 se o +3 × fator passa do +0xb (precisão dupla)
    (cqd12_i(vaga + 3) as f64 * fator > cqd12_i(vaga + 0xb) as f64) as u32
}

/// Comparador: lado a lado (rotina pura: só a taxa e a devolução), pelas cópias leves.
unsafe extern "thiscall" fn comparar_shortlist_manager_is_compensation_deal(this: usize, vaga: usize, contrato: usize, clube: usize, taxa: *mut i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i32) -> u32>(%%I0%%);
    let reg = [(taxa as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, vaga, contrato, clube, taxa),
        || shortlist_manager_is_compensation_deal(this, vaga, contrato, clube, taxa));
    PLACAR.registrar(igual && r == n, || format!("shortlist_manager_is_compensation_deal(vaga {:#x}, contrato {:#x}, clube {:#x}) original={} rust={} {:x?} x {:x?}",
        vaga, contrato, clube, r, n, mo, mr));
    r
}

/// { 0x83b6c0 } (cdecl, shortlist_manager.cpp): `shortlist_compare_slots_by_index(a, b)`, o critério do qsort das 40
/// vagas (0x25 bytes) da lista de compras na avaliação do elenco (0x83cb40): a diferença dos índices (+0x23, byte com
/// sinal); iguais, 1 (nunca empata).
unsafe extern "C" fn shortlist_compare_slots_by_index(a: usize, b: usize) -> i32 {
    let d = (*((a + 0x23) as *const i8) as i32) - (*((b + 0x23) as *const i8) as i32);
    if d != 0 { d } else { 1 }
}

/// Comparador: a devolução (pura).
unsafe extern "C" fn comparar_shortlist_compare_slots_by_index(a: usize, b: usize) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%);
    let r = o(a, b);
    let n = shortlist_compare_slots_by_index(a, b);
    PLACAR.registrar(r == n, || format!("shortlist_compare_slots_by_index({:#x}, {:#x}) original={} rust={}", a, b, r, n));
    r
}

/// Os clubes interessados no jogador, como a 0x84b580 os pede (0x842180 com modo 1 numa lista local; os itens de 8
/// bytes: id do jogador, id do clube).
unsafe fn interested_clubs(this: usize, jogador: usize) -> Vec<usize> {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x842180usize);
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    juntar(this, jogador, lista, 0, 1, 0);
    let mut v = Vec::new();
    let mut no = cqd12_d(lista + 2);
    while no != 0 && v.len() < 0x10000 {
        v.push(cqd12_d(cqd12_d(no) + 4));
        no = cqd12_d(no + 6);
    }
    soltar(lista);
    v
}

/// { 0x84b580 } (thiscall, ret 0xc, shortlist_manager.cpp, quadro SEH da lista local 0x68d020/0x68d040):
/// `Shortlist_Manager::mark_player_under_offer(jogador, comprador, troca)`, pela negociação acrescentada (0x8be9d0) e
/// pelo lance aceito (0x8c0080). Os clubes interessados no jogador (0x842180 com modo 1, numa lista local), menos os de
/// humano que decide (0x600a50 com 0, 1): com a lista `troca` (cursor na cabeça), um clube de alguma oferta dela
/// (0x8c61e0 pelo id do item; o clube +8 da oferta) fica de fora; senão a vaga do jogador (0x849ce0, só de tipo 11 se o
/// byte +0x11 da última oferta achada é 11) ganha o bit 0x20 (+0x24); sem a lista, fora o comprador, a vaga de tipo 11
/// ou, sem ela, a de outro tipo. Clube sem vaga encerra.
unsafe extern "thiscall" fn mark_player_under_offer(this: usize, jogador: usize, comprador: usize, troca: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let juntar: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x842180usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    let oferta: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x8c61e0usize);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    // 0x68d020/0x68d040/0x68d1b0: lea ecx, lista -> thiscall; 0x842180: 5 pushes; mov ecx, ebx (ret 0x14); 0x600a50:
    // 5 pushes; add esp, 0x14 -> cdecl; 0x8c61e0: push id; mov ecx, [0xde1f54] (ret 4); 0x849ce0: 4 pushes; mov ecx,
    // ebx (ret 0x10)
    let mut l = [0u8; 0x10];
    let lista = l.as_mut_ptr() as usize;
    nova(lista);
    juntar(this, jogador, lista, 0, 1, 0);
    core::ptr::write_unaligned((lista + 0xa) as *mut usize, cqd12_d(lista + 2));
    let mut ultima = jogador; // o ebp da original: o jogador, depois a última oferta achada (vale entre os clubes)
    let mut item = proximo(lista);
    'clubes: while item != 0 {
        let k = cqd12_d(item + 4);
        let clube = cqd12_d(0xae23b4).wrapping_add(k.wrapping_mul(0x245));
        if humano(clube, 0, 1, 0, 0) == 0 {
            let v = if troca != 0 {
                core::ptr::write_unaligned((troca + 0xa) as *mut usize, cqd12_d(troca + 2));
                let mut q = proximo(troca);
                while q != 0 {
                    ultima = oferta(cqd12_d(0xde1f54), cqd12_d(q));
                    if ultima != 0 && cqd12_d(cqd12_d(ultima) + 8) == k {
                        item = proximo(lista);
                        continue 'clubes;
                    }
                    q = proximo(troca);
                }
                let tipo = *((cqd12_d(ultima) + 0x11) as *const u8);
                vaga_de(this, clube, jogador, (tipo != 0xb) as u32, 0)
            } else {
                if clube == comprador {
                    item = proximo(lista);
                    continue 'clubes;
                }
                let v = vaga_de(this, clube, jogador, 0, 0);
                if v != 0 { v } else { vaga_de(this, clube, jogador, 1, 0) }
            };
            if v == 0 {
                break;
            }
            *((v + 0x24) as *mut u8) |= 0x20;
        }
        item = proximo(lista);
    }
    soltar(lista);
}

/// Comparador: lado a lado pelas listas de 40 vagas dos clubes interessados (a cópia e a original montam e soltam a
/// mesma lista local) e pelo cursor da lista `troca`.
unsafe extern "thiscall" fn comparar_mark_player_under_offer(this: usize, jogador: usize, comprador: usize, troca: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I2%%);
    let mut reg: Vec<(usize, usize)> = interested_clubs(this, jogador).into_iter()
        .map(|k| cqd12_d(cqd12_d(this + 0xdc).wrapping_add(k.wrapping_mul(9))))
        .filter(|&t| t != 0)
        .map(|t| (t, 40 * 0x25))
        .collect();
    if troca != 0 {
        reg.push((troca + 0xa, 4));
    }
    let ((), (), igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, jogador, comprador, troca),
        || mark_player_under_offer(this, jogador, comprador, troca));
    PLACAR.registrar(igual, || format!("mark_player_under_offer(jogador {:#x}, comprador {:#x}, troca {:#x}) {:x?} x {:x?}",
        jogador, comprador, troca, mo, mr));
}
