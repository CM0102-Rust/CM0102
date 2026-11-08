// ---------- %%LEVA%%a leva: a fila da transferência livre, a próxima janela aberta e o empréstimo negociado (transfer_manager.cpp) ----------

unsafe fn cps3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cps3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cps3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cps3_por(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut usize, v);
}

/// { 0x8c18c0 } (thiscall, ret 0, transfer_manager.cpp, quadro SEH das duas LLIST locais 0x68d020/0x68d040):
/// `TRANSFER_MANAGER::process_free_transfer_queue()`: pelo dia a dia das transferências (0x8c5880). Percorre uma cópia
/// do cabeçalho da fila (+0x92c, 14 bytes, com o cursor no começo); para cada item (+0 id da pessoa, +4 motivo, +5
/// compensação) com contrato (0x4cf5a0, guardado em [0xae1080]) e clube: no clube humano (0x7875e0) sem motivo, a
/// diretoria pode barrar (0x696940; barrando, 0x6a0990 e o próximo); senão o clube entra nos rejeitados (0x547e70) e o
/// técnico (+0xcf) nos desafetos (0x547af0), e a diretoria registra (0x697430). Sem motivo, um sorteio pelo +0x5b contra
/// outro de 20 (ou gostar do clube, 0x547d60) põe o técnico nos desafetos. Depois os registros da liberação (0x86d800),
/// as ofertas canceladas (0x8df9b0) e a liberação (0x8c1af0 com motivo e compensação). No fim a cópia vira a lista
/// vazia, a fila é limpa (0x68d080) e as duas locais são destruídas.
unsafe extern "thiscall" fn process_free_transfer_queue(this: usize) {
    let nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let soltar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let limpar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let barra: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x696940usize);
    let barrado: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x6a0990usize);
    let rejeitado: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547e70usize);
    let desafeto: unsafe extern "C" fn(usize, usize, u32) = core::mem::transmute(0x547af0usize);
    let diretoria: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x697430usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let gosta: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x547d60usize);
    let registros: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x86d800usize);
    let cancelar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x8df9b0usize);
    let liberar: unsafe extern "thiscall" fn(usize, usize, usize, i32, usize, u32) = core::mem::transmute(0x8c1af0usize);
    let (mut l1, mut l2) = ([0u8; 14], [0u8; 14]);
    let (a, b) = (l1.as_mut_ptr() as usize, l2.as_mut_ptr() as usize);
    nova(a);
    nova(b);
    let fila = this + 0x92c;
    // a cópia do cabeçalho: os 10 primeiros bytes e o cursor (+0xa) no começo (+2)
    core::ptr::copy_nonoverlapping(fila as *const u8, a as *mut u8, 0xa);
    cps3_por(a + 0xa, cps3_d(a + 2));
    let mut e = proximo(a);
    while e != 0 {
        let p = cps3_d(0xae23bc).wrapping_add(cps3_d(e).wrapping_mul(0x6e));
        let c = contract_of(cps3_d(0xae1050), p);
        cps3_por(0xae1080, c);
        let clube = if c != 0 { cps3_d(0xae23b4).wrapping_add(cps3_d(c + 4).wrapping_mul(0x245)) } else { 0 };
        if clube != 0 {
            let mut seguir = true;
            if humano(cps3_d(0xb7431c), clube) != 0 && cps3_b(e + 4) == 0 {
                let r = barra(cps3_d(0xb67a44), p, clube);
                if r as u16 != 0 {
                    barrado(cps3_d(0xb67a44), p, clube, r);
                    seguir = false;
                } else {
                    if cps3_b(e + 4) == 0 {
                        rejeitado(p, clube, 0);
                        if cps3_d(clube + 0xcf) != 0 {
                            desafeto(p, cps3_d(clube + 0xcf), 1);
                        }
                    }
                    diretoria(cps3_d(0xb67a44), p, clube);
                }
            }
            if seguir {
                if cps3_b(e + 4) == 0 {
                    let r = sorteio(cps3_b(p + 0x5b) as i8 as i32);
                    if r < sorteio(0x14) || gosta(p, clube) != 0 { // cps3: o sorteio do desafeto
                        crate::comparison::branch("cps_3 manager grudge");
                        desafeto(p, cps3_d(clube + 0xcf), 1);
                    }
                }
                crate::comparison::branch("cps_3 released");
                registros(cps3_d(0xdd7f00), p); // cps3: os registros da liberação
                cancelar(this, p);
                let comp = cps3_d(e + 5); // cps3: a compensação ao clube
                if comp != 0 {
                    crate::comparison::branch("cps_3 compensation");
                }
                liberar(this, p, clube, cps3_b(e + 4) as i8 as i32, comp, 0);
            }
        }
        e = proximo(a);
    }
    // a cópia vira a lista vazia (os 14 bytes), a fila é limpa e as duas locais saem
    core::ptr::copy_nonoverlapping(b as *const u8, a as *mut u8, 14);
    limpar(fila);
    soltar(b);
    soltar(a);
}

/// { 0x8cf470 } (thiscall, ret 8, transfer_manager.cpp, quadro SEH só das datas locais): `TRANSFER_MANAGER::
/// find_next_window_open_date(clube, saida)`: a regra da nação do clube (+0x53) entre as 28 de +0x8ac (+0xa); de hoje
/// ([0xae2c90]) até um ano depois (0x54c5b0 com 365), o primeiro dia com a janela aberta (0x8e8e50; sem regra, hoje) vai
/// para `saida`; sem achar, a caixa "Error" 0x32c6. A contratação desejada (0x85fc50) regrava a data com hoje logo depois.
unsafe extern "thiscall" fn find_next_window_open_date(this: usize, clube: usize, saida: usize) {
    let data_padrao: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x54a110usize);
    let mais: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    let aberta: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x8e8e50usize);
    let (mut d, mut lim, mut t1, mut t2) = ([0u8; 8], [0u8; 8], [0u8; 8], [0u8; 8]);
    let (dp, lp, p1, p2) = (d.as_mut_ptr() as usize, lim.as_mut_ptr() as usize, t1.as_mut_ptr() as usize, t2.as_mut_ptr() as usize);
    data_padrao(dp);
    mais(0xae2c90, lp, 0x16d);
    let nacao = cps3_d(clube + 0x53);
    let mut regra = 0usize;
    for k in 0..28 {
        let r = cps3_d(this + 0x8ac + k * 4);
        if cps3_d(r + 0xa) == nacao {
            regra = r;
            break;
        }
    }
    copiar(dp, p1, 0xae2c90);
    loop {
        let (la, da) = (cps3_w(lp + 2), cps3_w(dp + 2));
        if !(la > da || (la == da && (cps3_w(lp) as i16) > (cps3_w(dp) as i16))) {
            return show_error_at(0xa8d514, 0x32c6);
        }
        if regra == 0 || aberta(regra, dp, 0) != 0 {
            copiar(saida, p2, dp);
            return;
        }
        let r = mais(dp, p1, 1);
        copiar(dp, p2, r);
    }
}

/// { 0x8c72d0 } (thiscall, ret 8, transfer_manager.cpp): `TRANSFER_MANAGER::find_negotiated_loan_contract(pessoa,
/// clube)`: na lista [this+0x946] (o cursor +0xa volta ao começo), o item com a pessoa (+0) e o clube (+4) pelos ids;
/// sem lista ou sem achar, 0.
unsafe extern "thiscall" fn find_negotiated_loan_contract(this: usize, pessoa: usize, clube: usize) -> usize {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let l = cps3_d(this + 0x946);
    if l == 0 {
        return 0;
    }
    cps3_por(l + 0xa, cps3_d(l + 2));
    let mut e = proximo(cps3_d(this + 0x946));
    while e != 0 {
        if cps3_d(pessoa) == cps3_d(e) && cps3_d(clube) == cps3_d(e + 4) {
            return e;
        }
        e = proximo(cps3_d(this + 0x946));
    }
    0
}

// ponytail: sorteia (0x90cfc0) e libera os jogadores (0x8c1af0, 0x8df9b0), limpando a fila: só a original; a cópia
// leva os ramos.
unsafe extern "thiscall" fn compare_process_free_transfer_queue(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%)(this)
}

/// Comparador: lado a lado pela data de saída (a data estática da 0x54c5b0 é rascunho). Sem janela num ano, a caixa
/// sairia duas vezes (não acontece com as regras do jogo).
unsafe extern "thiscall" fn compare_find_next_window_open_date(this: usize, clube: usize, saida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I1%%);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&[(saida, 8)], || o(this, clube, saida), || find_next_window_open_date(this, clube, saida));
    SCOREBOARD.record(igual, || format!("find_next_window_open_date(clube {:x}) {:x?} x {:x?}", clube, mo, mr));
}

/// Comparador: pelo valor (as duas voltas deixam o cursor da lista no mesmo lugar).
unsafe extern "thiscall" fn compare_find_negotiated_loan_contract(this: usize, pessoa: usize, clube: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I2%%)(this, pessoa, clube);
    let n = find_negotiated_loan_contract(this, pessoa, clube);
    SCOREBOARD.record(r == n, || format!("find_negotiated_loan_contract(pessoa {}, clube {}) original={:x} rust={:x}", cps3_d(pessoa), cps3_d(clube), r, n));
    r
}
