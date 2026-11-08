// ---------- %%LEVA%%a leva: as datas da transferência: a data de conclusão da oferta, o primeiro dia que as regras permitem e a janela fixa de transferências (transfer_manager.cpp) ----------

unsafe fn cpq5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpq5_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O dia de hoje: Data de 8 bytes em 0xae2c90 (+0 dia do ano i16, +2 ano u16, +4 bissexto).
const CPQ5_HOJE: usize = 0xae2c90;

/// As estáticas das operações de data (0x54c5b0, 0x54c8e0, 0x54c770, 0x54cdb0: valor e guarda), para os comparadores.
const CPQ5_ESTATICAS: [(usize, usize); 8] = [(0xae2420, 8), (0xae2469, 1), (0xae24c0, 8), (0xae249e, 1), (0xae2410, 8), (0xae2430, 1), (0xae2470, 8), (0xae2418, 1)];

/// `a` antes de `b` na ordem dos originais: o ano (+2, 16 bits sem sinal) e, no mesmo ano, o dia (+0, 16 bits com sinal).
unsafe fn date_pair_before(a: *const [u32; 2], b: *const [u32; 2]) -> bool {
    let ler = |p: *const [u32; 2]| (core::ptr::read_unaligned(p as *const i16), core::ptr::read_unaligned((p as usize + 2) as *const u16));
    let ((da, aa), (db, ab)) = (ler(a), ler(b));
    aa < ab || (aa == ab && da < db)
}

/// O clube de id `id` (registros de 0x245 bytes em [0xae23b4]).
unsafe fn cpq5_clube(id: usize) -> usize {
    cpq5_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` (registros de 0x6e bytes em [0xae23bc]).
unsafe fn cpq5_pessoa(id: usize) -> usize {
    cpq5_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O clube do contrato da pessoa da oferta (+0xc): 0x4cf5a0 com o gerente de contratos [0xae1050], guardado em
/// [0xae1080]; contrato de tipo 5 (+0x35 & 0x3f) não conta.
unsafe fn offer_seller_club(oferta: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = contract_of(cpq5_d(0xae1050), cpq5_pessoa(cpq5_d(oferta + 0xc)));
    *(0xae1080 as *mut usize) = c;
    if c != 0 && cpq5_b(c + 0x35) & 0x3f != 5 {
        cpq5_clube(cpq5_d(c + 4))
    } else {
        0
    }
}

/// O método +0xc das regras (`TRANSFER_RULES::can_make_offer(oferta, clube, data, motivo)`): 1 permite; 0 e o motivo.
unsafe fn rules_can_make_offer(regra: usize, oferta: usize, clube: usize, data: *const [u32; 2], motivo: *mut u32) -> u32 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, *const [u32; 2], *mut u32) -> u32 = core::mem::transmute(cpq5_d(cpq5_d(regra) + 0xc));
    f(regra, oferta, clube, data, motivo)
}

/// Os motivos que não seguram a data (o dia vale assim mesmo): 0x14, 0x15, 0x1f e 0x11.
fn reason_ignores_date(m: u32) -> bool {
    matches!(m as u16, 0x14 | 0x15 | 0x1f | 0x11)
}

/// { 0x8cee70 } (thiscall, ret 8, transfer_manager.cpp): `transfer_manager_get_offer_completion_date(oferta, saida)`:
/// a data em que a oferta pode se concretizar, pelo tipo de prazo (+0x30 & 0xf): 0 hoje; 5 o início da janela fixa
/// (0x8d0bb0); 1 e 4 o primeiro dia que as regras permitem (0x8cf0e0, que dá o motivo); 2 o fim da temporada da nação
/// do clube vendedor mais 7 dias (0x669bd0; fora da temporada, hoje; sem vendedor, a caixa "Error" 0x31da); 3 o dia
/// seguinte ao fim do contrato (+0x2d), nunca antes de hoje (sem contrato, hoje). Outros tipos não gravam. Devolve o
/// motivo (só o ax: 0 fora dos tipos 1 e 4). O vendedor (0x4cf5a0, guardado em [0xae1080]) é calculado sempre.
unsafe extern "thiscall" fn transfer_manager_get_offer_completion_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let mais: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(0x417c10usize);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    let (mut t, mut t2) = ([0u32; 2], [0u32; 2]);
    let vendedor = offer_seller_club(oferta);
    match cpq5_b(oferta + 0x30) & 0xf {
        0 => {
            crate::comparacao::ramo("cpq_5 conclusao imediata");
            copia(saida, &mut t, hoje); // tipo 0: hoje
        }
        5 => {
            let janela: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(0x8d0bb0usize);
            janela(saida, 0);
        }
        1 | 4 => {
            let primeiro: unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16 = core::mem::transmute(0x8cf0e0usize);
            return primeiro(this, oferta, saida);
        }
        2 => {
            if vendedor == 0 {
                show_error_at(0xa8d514, 0x31da);
            } else {
                let temporada: unsafe extern "C" fn(usize, usize, *mut [u32; 2]) -> u32 = core::mem::transmute(0x669bd0usize);
                if temporada(vendedor, 0, saida) != 0 {
                    let r = mais(saida, &mut t, 7);
                    copia(saida, &mut t2, r);
                } else {
                    copia(saida, &mut t2, hoje);
                }
            }
        }
        3 => {
            let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
            let c = contract_of(cpq5_d(0xae1050), cpq5_pessoa(cpq5_d(oferta + 0xc)));
            if c != 0 {
                let r = mais((c + 0x2d) as *const [u32; 2], &mut t2, 1);
                copia(saida, &mut t, r);
                if date_pair_before(saida, hoje) {
                    copia(saida, &mut t2, hoje);
                }
            } else {
                copia(saida, &mut t2, hoje);
            }
        }
        _ => {}
    }
    0
}

/// { 0x8cf0e0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_next_allowed_transfer_date(oferta,
/// saida)`: grava hoje na saída e começa no maior entre hoje e, no tipo de prazo 4 (+0x30 & 0xf), o fim do contrato
/// (+0x2d). Tipo 5: o início da janela fixa (0x8d0bb0) e o método +0xc da regra da nação do comprador (+8) dá o motivo
/// (9 não conta). Pessoa sem ficha de jogador (+0x61): hoje. Senão anda dia a dia (até hoje + 365, sem chegar) pelas
/// regras da nação do comprador e, com o +4 da oferta e um vendedor (0x4cf5a0, guardado em [0xae1080]), da do vendedor:
/// o primeiro dia que as duas permitem (ou cujo motivo libera: 0x14, 0x15, 0x1f, 0x11) vai para a saída e devolve 0;
/// sem dia, devolve o primeiro motivo (9 não conta) e a saída fica hoje. Só o ax é devolvido.
unsafe extern "thiscall" fn find_next_allowed_transfer_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let padrao: unsafe extern "thiscall" fn(*mut [u32; 2]) -> usize = core::mem::transmute(0x54a110usize);
    let mais: unsafe extern "thiscall" fn(*const [u32; 2], *mut [u32; 2], u32) -> *const [u32; 2] = core::mem::transmute(0x54c5b0usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(0x417c10usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    let (mut dia, mut limit, mut inicio, mut t, mut t2) = ([0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2]);
    padrao(&mut dia);
    mais(hoje, &mut limit, 0x16d);
    padrao(&mut inicio);
    let mut motivo = 0u32;
    // ponytail: o motivo da regra é uma variável só, sem valor antes da primeira chamada (a original não a zera; a
    // regra o grava sempre que devolve 0)
    let mut m = 0u32;
    copia(saida, &mut t, hoje);
    let pessoa = cpq5_pessoa(cpq5_d(oferta + 0xc));
    if cpq5_b(oferta + 0x30) & 0xf == 4 {
        let c = contract_of(cpq5_d(0xae1050), pessoa);
        if c != 0 {
            copia(&mut inicio, &mut t, (c + 0x2d) as *const [u32; 2]);
            t = *hoje;
            if date_pair_before(&inicio, hoje) {
                copia(&mut inicio, &mut t, hoje);
            }
        } else {
            copia(&mut inicio, &mut t, hoje);
        }
    } else {
        copia(&mut inicio, &mut t, hoje);
    }
    let comprador = cpq5_clube(cpq5_d(oferta + 8));
    let vendedor = offer_seller_club(oferta);
    if cpq5_b(oferta + 0x30) & 0xf == 5 {
        let janela: unsafe extern "stdcall" fn(*mut [u32; 2], usize) = core::mem::transmute(0x8d0bb0usize);
        janela(saida, 0);
        let regra = nation_transfer_rules_or_zero(this, cpq5_d(comprador + 0x53));
        if regra != 0 && rules_can_make_offer(regra, oferta, comprador, saida, &mut m) == 0 && m as u16 != 9 {
            motivo = m;
        }
        return motivo as u16;
    }
    if cpq5_d(pessoa + 0x61) == 0 {
        copia(saida, &mut t, hoje);
        return 0;
    }
    let regra_c = nation_transfer_rules_or_zero(this, cpq5_d(comprador + 0x53));
    let regra_v = if vendedor != 0 && cpq5_d(oferta + 4) != 0 { nation_transfer_rules_or_zero(this, cpq5_d(vendedor + 0x53)) } else { 0 };
    crate::comparacao::ramo("cpq_5 dia permitido");
    copia(&mut dia, &mut t, &inicio); // o primeiro dia testado é o início
    while date_pair_before(&dia, &limit) {
        let mut ve_vendedor = true;
        if regra_c != 0 && rules_can_make_offer(regra_c, oferta, comprador, &dia, &mut m) == 0 {
            if m as u16 != 9 && motivo as u16 == 0 {
                motivo = m;
            }
            ve_vendedor = reason_ignores_date(motivo);
        }
        if ve_vendedor {
            if regra_v == 0 || rules_can_make_offer(regra_v, oferta, vendedor, &dia, &mut m) != 0 {
                copia(saida, &mut t2, &dia);
                return 0;
            }
            if m as u16 != 9 && motivo as u16 == 0 {
                motivo = m;
            }
            if reason_ignores_date(motivo) {
                copia(saida, &mut t2, &dia);
                return 0;
            }
        }
        let r = mais(&dia, &mut t, 1);
        copia(&mut dia, &mut t2, r);
    }
    motivo as u16
}

/// { 0x8d0bb0 } (stdcall, ret 8, transfer_manager.cpp): `transfer_manager_get_next_window_dates(inicio, fim)`: as
/// janelas fixas do ano de hoje são 1º a 31 de julho e 15 de dezembro a 15 de janeiro (0x549f70; comparadas só pelo
/// dia do ano, com sinal). Dentro de uma janela, o início é hoje e o fim o da janela (15 de janeiro do ano seguinte, se
/// for a de dezembro); antes de julho, 1º a 31 de julho; entre agosto e 14 de dezembro, 15 de dezembro a 15 de janeiro
/// do ano seguinte. O fim só com `fim` não nulo. Não usa o ecx (os chamadores passam o gerente).
unsafe extern "stdcall" fn transfer_manager_get_next_window_dates(inicio: *mut [u32; 2], fim: *mut [u32; 2]) {
    let dma: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> *mut [u32; 2] = core::mem::transmute(0x549f70usize);
    let copia: unsafe extern "thiscall" fn(*mut [u32; 2], *mut [u32; 2], *const [u32; 2]) -> usize = core::mem::transmute(0x417c10usize);
    let hoje = CPQ5_HOJE as *const [u32; 2];
    // ponytail: a original empurra o ano num registrador com lixo acima (e o ano + 1 sobre a palavra do argumento); a
    // 0x549f70 só usa os 16 bits de baixo
    let ano = || *((CPQ5_HOJE + 2) as *const u16) as u32;
    let (mut a, mut b, mut c, mut d, mut e, mut t) = ([0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2], [0u32; 2]);
    dma(&mut a, 1, 6, ano(), 0xffff_ffff); // 1º de julho
    dma(&mut b, 0x1f, 6, ano(), 0xffff_ffff); // 31 de julho
    dma(&mut c, 0xf, 0xb, ano(), 0xffff_ffff); // 15 de dezembro
    dma(&mut d, 0xf, 0, ano(), 0xffff_ffff); // 15 de janeiro
    let dia = |x: &[u32; 2]| x[0] as u16 as i16;
    let h = *(CPQ5_HOJE as *const i16);
    if h >= dia(&a) && h <= dia(&b) {
        copia(inicio, &mut t, hoje);
        if !fim.is_null() {
            copia(fim, &mut t, &b);
        }
    } else if h >= dia(&c) {
        copia(inicio, &mut t, hoje);
        if !fim.is_null() {
            dma(&mut e, 0xf, 0, (ano() as u16).wrapping_add(1) as u32, 0xffff_ffff);
            copia(fim, &mut t, &e);
        }
    } else if h <= dia(&d) {
        copia(inicio, &mut t, hoje);
        if !fim.is_null() {
            copia(fim, &mut t, &d);
        }
    } else if h < dia(&a) {
        crate::comparacao::ramo("cpq_5 janela de julho");
        copia(inicio, &mut t, &a);
        if !fim.is_null() {
            copia(fim, &mut t, &b);
        }
    } else {
        copia(inicio, &mut t, &c);
        if !fim.is_null() {
            dma(&mut e, 0xf, 0, (ano() as u16).wrapping_add(1) as u32, 0xffff_ffff);
            copia(fim, &mut t, &e);
        }
    }
}

/// As regiões dos comparadores de datas: a saída, o contrato guardado [0xae1080] e as estáticas das datas.
fn date_comparer_regions(saidas: &[usize]) -> Vec<(usize, usize)> {
    let mut r: Vec<(usize, usize)> = saidas.iter().filter(|&&s| s != 0).map(|&s| (s, 8)).collect();
    r.push((0xae1080, 4));
    r.extend_from_slice(&CPQ5_ESTATICAS);
    r
}

/// Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo (as regras, a temporada e a
/// janela só calculam).
unsafe extern "thiscall" fn comparar_transfer_manager_get_offer_completion_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(%%I0%%);
    let reg = date_comparer_regions(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || transfer_manager_get_offer_completion_date(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("transfer_manager_get_offer_completion_date(tipo {}) motivo {} x {}, memória igual {}", cpq5_b(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparador: lado a lado pela saída, [0xae1080] e as estáticas das datas, e pelo motivo.
unsafe extern "thiscall" fn comparar_find_next_allowed_transfer_date(this: usize, oferta: usize, saida: *mut [u32; 2]) -> u16 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, *mut [u32; 2]) -> u16>(%%I1%%);
    let reg = date_comparer_regions(&[saida as usize]);
    let (ro, rr, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this, oferta, saida), || find_next_allowed_transfer_date(this, oferta, saida));
    PLACAR.registrar(igual && ro == rr, || format!("find_next_allowed_transfer_date(tipo {}) motivo {} x {}, memória igual {}", cpq5_b(oferta + 0x30) & 0xf, ro, rr, igual));
    ro
}

/// Comparador: lado a lado pelo início e o fim (só datas, sem estáticas).
unsafe extern "stdcall" fn comparar_transfer_manager_get_next_window_dates(inicio: *mut [u32; 2], fim: *mut [u32; 2]) {
    let o = original_fn::<unsafe extern "stdcall" fn(*mut [u32; 2], *mut [u32; 2])>(%%I2%%);
    let reg = date_comparer_regions(&[inicio as usize, fim as usize]);
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(inicio, fim), || transfer_manager_get_next_window_dates(inicio, fim));
    PLACAR.registrar(igual, || format!("transfer_manager_get_next_window_dates(hoje {:x}, fim {}) diferente", *(CPQ5_HOJE as *const u32), !fim.is_null()));
}
