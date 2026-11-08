// ---------- %%LEVA%%a leva: a oferta de empréstimo pelo jogador da lista, a notícia do contrato acabando e a da saída pela lei Bosman (shortlist_manager.cpp, contract_manager.cpp) ----------

unsafe fn cpw3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw3_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpw3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de clube de id `id` ([0xae23b4], 0x245 bytes).
unsafe fn cpw3_clube(id: usize) -> usize {
    cpw3_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// A pessoa de id `id` ([0xae23bc], 0x6e bytes).
unsafe fn cpw3_pessoa(id: usize) -> usize {
    cpw3_d(0xae23bc).wrapping_add(id.wrapping_mul(0x6e))
}

/// O contrato principal da pessoa (0x4cf5a0).
unsafe fn main_contract_via_manager(p: usize) -> usize {
    let contract_of: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    contract_of(cpw3_d(0xae1050), p)
}

/// O interesse da pessoa em ir para o clube (Shortlist_Manager::staff_interest_in_joining_club 0x846920: thiscall(lista,
/// pessoa, clube, salário, modo, situação, motivos), ret 0x18, float no st0).
unsafe fn staff_interest_in_club(lista: usize, pessoa: usize, clube: usize, salario: u32, modo: u32, situacao: u32) -> f64 {
    let f: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32, usize) -> f64 = core::mem::transmute(0x846920usize);
    f(lista, pessoa, clube, salario, modo, situacao, 0)
}

/// O ensaio do comparador: a cópia roda sem a oferta de empréstimo (0x8bed60) nem a atualização da lista (0x843e10),
/// que alocam e mexem nas listas; devolve −2 onde faria a oferta e −3 onde atualizaria a lista.
static mut LOAN_OFFER_DRY_RUN: bool = false;

/// A data (dia i16, ano u16) de 8 bytes em `a`, como a original a compara (o ano sem sinal, o dia com sinal).
unsafe fn contract_manager_date_is_before(a: usize, b: usize) -> bool {
    let (aa, ab) = (cpw3_w(a + 2) as u16, cpw3_w(b + 2) as u16);
    aa < ab || (aa == ab && cpw3_w(a) < cpw3_w(b))
}

/// { 0x842740 } (thiscall, ret 8, shortlist_manager.cpp, quadro SEH sem destrutor): `Shortlist_Manager::
/// try_loan_offer_for_player(jogador, clube)`, pela avaliação das listas dos clubes (0x84a020): a vaga da oferta de
/// empréstimo, ou −1. Sem dono (contrato 0x4cf5a0, guardado em [0xae1080]): a vaga da lista (0x849ce0) volta ao estado 1
/// (0x843e10) e −1. O interesse (0x846920, com a situação da vaga) abaixo de 3,5: −1. O período do empréstimo (0x8d0590):
/// os meses até o fim e até o início (no mínimo 1); início depois do fim: −1; clube da nação 0x9cf408 fora de março a
/// setembro: −1. O interesse sem salário de 2,75 (fora das nações 0x9cf488, 0x9cf434, 0x9cf360) conta do início; senão do
/// fim. A data de volta (dia de hoje, no máximo 28; 0x549f70); sem poder encerrar o empréstimo (0x8d0530), depois do fim
/// do contrato, antes de 4 semanas do começo da janela da nação-chave (0x669360, [0xb63d8a]) ou a menos de 3 semanas de
/// hoje: −1. A porcentagem do salário (100 com reputação 2500 acima do dono, 0 com 1500 abaixo, senão 50) e a oferta
/// (0x8bed60).
unsafe extern "thiscall" fn try_loan_offer_for_player(this: usize, jogador: usize, clube: usize) -> i32 {
    let shortlist_entry: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> usize = core::mem::transmute(0x849ce0usize);
    let entrada = shortlist_entry(this, clube, jogador, 0, 0);
    let c = main_contract_via_manager(jogador);
    *(0xae1080 as *mut usize) = c;
    let dono = if c != 0 { cpw3_clube(cpw3_d(c + 4)) } else { 0 };
    if dono == 0 {
        if entrada != 0 {
            if LOAN_OFFER_DRY_RUN {
                return -3;
            }
            let estado: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32) -> u32 = core::mem::transmute(0x843e10usize);
            estado(this, jogador, clube, 1, 0);
        }
        return -1;
    }
    // ponytail: a original empurra a situação da vaga num registrador cujo resto é o endereço do clube dono (base +
    // id); a cópia reproduz o dword.
    let situacao = (cpw3_d(0xae23b4).wrapping_add(cpw3_d(c + 4)) as u32 & !0xff) | ((cpw3_b(entrada + 0x24) >> 1) & 0xf) as u32;
    let f1 = staff_interest_in_club(cpw3_d(cpw3_d(0xde1f54) + 0x856), jogador, clube, 0, 1, situacao);
    crate::comparison::branch("cpw_3 interesse no emprestimo");
    if !(f1 >= read_f64_at(0x968e38)) { // cpw3: o interesse mínimo
        return -1;
    }
    let mes_de: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cfb0usize);
    let m = mes_de(0xae2c90) as u8;
    let c = main_contract_via_manager(jogador);
    *(0xae1080 as *mut usize) = c;
    let a2 = if c != 0 { cpw3_clube(cpw3_d(c + 4)) } else { 0 };
    let periodo: unsafe extern "thiscall" fn(usize, usize, usize, usize, *mut i8, *mut u16, *mut i8, *mut u16) = core::mem::transmute(0x8d0590usize);
    // ponytail: o mês do fim vai para a vaga do argumento do clube (o byte baixo), o resto fica; os outros são locais
    let (mut ini_mes, mut ini_ano, mut mes, mut ano) = (0i8, 0u16, clube as u8 as i8, 0u16);
    periodo(cpw3_d(0xde1f54), clube, a2, jogador, &mut ini_mes, &mut ini_ano, &mut mes, &mut ano);
    let hoje_ano = cpw3_w(0xae2c92) as u16;
    let m16 = m as i8 as i16;
    let ate_fim = (ano.wrapping_sub(hoje_ano) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(mes as i16);
    let ate_ini = (ini_ano.wrapping_sub(hoje_ano) as i16).wrapping_mul(12).wrapping_sub(m16).wrapping_add(ini_mes as i16);
    let inicio = if ate_ini >= 1 { ate_ini } else { 1 };
    if inicio > ate_fim {
        return -1;
    }
    let nacao = cpw3_d(clube + 0x53);
    if nacao != 0 && cpw3_d(nacao) == cpw3_d(0x9cf408) && (m as i8 >= 0xa || m as i8 <= 2) {
        return -1;
    }
    let f2 = staff_interest_in_club(this, jogador, clube, 0, 0, 1);
    let mut meses = ate_fim as u8;
    crate::comparison::branch("cpw_3 conta do inicio");
    if f2 >= read_f64_at(0x9690a8) { // cpw3: o interesse para contar do início
        let n = cpw3_d(cpw3_d(clube + 0x53));
        if n != cpw3_d(0x9cf488) && n != cpw3_d(0x9cf434) && n != cpw3_d(0x9cf360) {
            meses = inicio as u8;
        }
    }
    let mut mes_volta = meses.wrapping_add(m);
    // ponytail: o ano vai na vaga do argumento do jogador (a palavra baixa; o resto é o endereço do jogador), e a
    // porcentagem depois no byte baixo dela; a cópia reproduz o dword.
    let mut vaga = (jogador as u32 & 0xffff_0000) | hoje_ano as u32;
    if mes_volta as i8 > 0xb {
        let q = (mes_volta as i8 as i32 as u32) / 12;
        mes_volta = mes_volta.wrapping_add((q as u8).wrapping_mul(0xf4));
        vaga = vaga.wrapping_add(q);
    }
    let dia_do_mes: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x54cf70usize);
    let dia = dia_do_mes(0xae2c90) as i16;
    // a original relê o dia (segunda chamada) quando ele não passa de 28
    let dia = if dia > 0x1c { 0x1c } else { dia_do_mes(0xae2c90) as i16 as i32 };
    let data_dma: unsafe extern "thiscall" fn(*mut [u16; 4], i32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut volta = [0u16; 4];
    data_dma(&mut volta, dia, mes_volta as u32, vaga, 0xffff_ffff);
    let v = volta.as_ptr() as usize;
    let encerrar: unsafe extern "thiscall" fn(usize, usize, usize, u32, usize, u32) -> u32 = core::mem::transmute(0x8d0530usize);
    if encerrar(cpw3_d(0xde1f54), clube, jogador, 0, v, 1) == 0 {
        return -1;
    }
    let c = main_contract_via_manager(jogador);
    if c == 0 || contract_manager_date_is_before(c + 0x2d, v) {
        return -1;
    }
    let chave: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x669360usize);
    let k = chave(clube) as u8 as i8 as i32;
    let janela = (0xb63d8a + k * 72) as usize;
    let menos_semanas: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(0x54cab0usize);
    let mais_semanas: unsafe extern "thiscall" fn(usize, *mut [u16; 4], u32) -> usize = core::mem::transmute(0x54c770usize);
    let mut t = [0u16; 4];
    let r = menos_semanas(v, &mut t, 4);
    if contract_manager_date_is_before(r, janela) {
        return -1;
    }
    crate::comparison::branch("cpw_3 tres semanas");
    let r = mais_semanas(0xae2c90, &mut t, 3); // cpw3: as 3 semanas a partir de hoje
    if contract_manager_date_is_before(v, r) {
        return -1;
    }
    let dif = cpw3_w(clube + 0x80) as i32 - cpw3_w(dono + 0x80) as i32;
    let pct = if dif > 0x9c4 { 100 } else if dif <= -0x5dc { 0 } else { 50 };
    let vaga = (vaga & !0xff) | pct;
    if LOAN_OFFER_DRY_RUN {
        return -2;
    }
    let oferta: unsafe extern "thiscall" fn(usize, usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x8bed60usize);
    oferta(cpw3_d(0xde1f54), jogador, clube, v, vaga, 1, 1) as i32
}

/// O comparador: a cópia em ensaio e a original a partir do mesmo estado ([0xae1080], o sorteio e as estáticas das
/// operações de data). A cópia recusou (−1): a original também, com a mesma memória. A cópia atualizaria a lista (−3):
/// a original devolve −1. A cópia faria a oferta (−2): a original não devolve −1.
unsafe extern "thiscall" fn compare_try_loan_offer_for_player(this: usize, jogador: usize, clube: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> i32>(%%I0%%);
    let mut regioes = vec![(0xae1080usize, 4usize)];
    regioes.extend(crate::rng::STATE_REGIONS);
    regioes.extend(TRANSFER_MANAGER_DATE_STATICS);
    let antes = crate::comparison::MemorySnapshot::read(&regioes);
    LOAN_OFFER_DRY_RUN = true;
    let n = crate::rng::pause_trace(|| crate::rng::without_counting(|| try_loan_offer_for_player(this, jogador, clube)));
    LOAN_OFFER_DRY_RUN = false;
    let mr = crate::comparison::MemorySnapshot::read(&regioes);
    antes.write_back();
    let r = o(this, jogador, clube);
    let mo = crate::comparison::MemorySnapshot::read(&regioes);
    let igual = match n {
        -1 => r == -1 && mo == mr,
        -3 => r == -1,
        _ => r != -1,
    };
    SCOREBOARD.record(igual, || format!("try_loan_offer_for_player(jogador {:#x}, clube {:#x}) original={} {:x?} rust={} {:x?}", cpw3_d(jogador), cpw3_d(clube), r, mo, n, mr));
    r
}

/// Os dados da pessoa numa notícia: o id na vaga 2 e as três chaves (+0x33 de +4, +8 e +0xc) nas vagas 3 a 5.
unsafe fn cpw3_dados_da_pessoa(n: usize, p: usize) {
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    gravar(n, 2, cpw3_d(p) as u32);
    gravar(n, 3, cpw3_d(cpw3_d(p + 4) + 0x33) as u32);
    gravar(n, 4, cpw3_d(cpw3_d(p + 8) + 0x33) as u32);
    gravar(n, 5, cpw3_d(cpw3_d(p + 0xc) + 0x33) as u32);
}

/// A nação está ativa (0x544670, cdecl)?
unsafe fn contract_manager_nation_allows_free_transfers(nacao: usize) -> bool {
    let f: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x544670usize);
    f(nacao) != 0
}

/// { 0x4d6790 } (thiscall, ret 0x10, contract_manager.cpp, quadro SEH da notícia e da lista locais): `CONTRACT_MANAGER::
/// post_contract_expiry_news(contrato, período, bosman, avisar_interessados)`, pelo passe dos contratos. Sem contrato: a
/// caixa "Error" (linha 0x24f0). Senão a notícia 0xbbd ao técnico do clube (0x7861e0): o clube, a pessoa, o período (com
/// sinal), `bosman` e, na vaga 8, se a pessoa tem ficha (+0x61). Com `avisar_interessados` (anulado se a nação do clube,
/// +0x53, não estiver ativa, 0x544670): os clubes interessados no jogador (0x842180 numa LLIST local, com 1, 1, 0) de
/// outra nação ativa também a recebem.
unsafe extern "thiscall" fn post_contract_expiry_news(this: usize, contrato: usize, periodo: u32, bosman: u32, avisar: u32) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x24f0);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbd, 0);
    let clube = cpw3_clube(cpw3_d(contrato + 4));
    let p = cpw3_pessoa(cpw3_d(contrato));
    gravar(n, 0, cpw3_d(clube) as u32);
    cpw3_dados_da_pessoa(n, p);
    gravar(n, 6, periodo as u8 as i8 as i32 as u32);
    gravar(n, 7, bosman);
    gravar(n, 8, (cpw3_d(p + 0x61) != 0) as u32);
    let nacao = cpw3_d(clube + 0x53);
    let avisar = if nacao != 0 && !contract_manager_nation_allows_free_transfers(nacao) { 0 } else { avisar };
    entregar(cpw3_d(0xb7431c), n, clube);
    if avisar == 0 {
        return;
    }
    crate::comparison::branch("cpw_3 avisar interessados");
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d020usize);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let interessados: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) = core::mem::transmute(0x842180usize);
    let mut lista = [0u8; 0xe];
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    interessados(cpw3_d(cpw3_d(0xde1f54) + 0x856), p, l, 1, 1, 0);
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpw3_d(l + 2));
    let mut x = proximo(l);
    while x != 0 {
        let id = cpw3_d(x + 4);
        if id != cpw3_d(clube) {
            let nac = cpw3_d(cpw3_clube(id) + 0x53);
            if nac != 0 && contract_manager_nation_allows_free_transfers(nac) {
                entregar(cpw3_d(0xb7431c), n, cpw3_clube(cpw3_d(x + 4)));
            }
        }
        x = proximo(l);
    }
    destruir(l);
}

// ponytail: a notícia vai às caixas dos técnicos (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_contract_expiry_news(this: usize, contrato: usize, periodo: u32, bosman: u32, avisar: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32)>(%%I1%%)(this, contrato, periodo, bosman, avisar)
}

/// { 0x4d6d70 } (thiscall, ret 8, contract_manager.cpp, quadro SEH da notícia local): `CONTRACT_MANAGER::
/// post_bosman_departure_news(contrato, destino)`. Sem contrato: a caixa "Error" (linha 0x2611). Senão a notícia 0xbbe ao
/// técnico do clube (0x7861e0): o clube, a pessoa e o clube de destino (−1 sem ele).
unsafe extern "thiscall" fn post_bosman_departure_news(this: usize, contrato: usize, destino: usize) {
    let _ = this;
    if contrato == 0 {
        return show_error_at(0x9a8190, 0x2611);
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    crate::comparison::branch("cpw_3 saida pela lei bosman");
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    construir(n, 0xbbe, 0);
    let clube = cpw3_clube(cpw3_d(contrato + 4));
    let p = cpw3_pessoa(cpw3_d(contrato));
    gravar(n, 0, cpw3_d(clube) as u32);
    cpw3_dados_da_pessoa(n, p);
    gravar(n, 6, if destino != 0 { cpw3_d(destino) as u32 } else { 0xffff_ffff });
    entregar(cpw3_d(0xb7431c), n, clube);
}

// ponytail: a notícia vai à caixa do técnico (aloca): só a original; a cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_bosman_departure_news(this: usize, contrato: usize, destino: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize)>(%%I2%%)(this, contrato, destino)
}
