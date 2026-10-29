// ---------- 245a leva: os recordes do clube: a construção dos recordes de valor de transferência pago e recebido e do recorde de sequência ----------

/// { 0x4485d0 } (thiscall, club_records.cpp): `transfer_fee_record_construct`: constrói o recorde de maior valor de
/// transferência pago: as datas em +0x14 e +0x20 (0x54a110). O quadro SEH do original só cobre a 2ª data (a 0x54a110
/// não falha). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_valor_pago(this: usize) -> usize {
    cpf2_data_padrao(this + 0x14); // a data do valor pago
    cpf2_data_padrao(this + 0x20);
    this
}

/// { 0x448620 } (thiscall, club_records.cpp): `transfer_fee_record_construct_2`: constrói o recorde de maior valor de
/// transferência recebido: as datas em +0x14 e +0x20 (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_valor_recebido(this: usize) -> usize {
    cpf2_data_padrao(this + 0x14);
    cpf2_data_padrao(this + 0x20); // a 2ª data do valor recebido
    this
}

/// { 0x448670 } (thiscall, club_records.cpp): `run_record_construct`: constrói o recorde de sequência: as datas do
/// começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_sequencia(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo da sequência
    cpf2_data_padrao(this + 0xc);
    this
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_valor_pago(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(846);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || construir_recorde_de_valor_pago(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_valor_pago {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_valor_recebido(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(847);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || construir_recorde_de_valor_recebido(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_valor_recebido {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_sequencia(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(848);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_sequencia(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_sequencia {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 245a leva: os recordes do clube: a construção dos recordes de vitórias, derrotas e invencibilidade seguidas ----------

/// { 0x450ad0 } (thiscall, club_records.cpp): `won_run_record_construct`: constrói o recorde de vitórias seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_vitorias_seguidas(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo das vitórias
    cpf2_data_padrao(this + 0xc);
    this
}

/// { 0x450b20 } (thiscall, club_records.cpp): `lost_run_record_construct`: constrói o recorde de derrotas seguidas: as
/// datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_derrotas_seguidas(this: usize) -> usize {
    cpf2_data_padrao(this + 4);
    cpf2_data_padrao(this + 0xc); // o fim das derrotas
    this
}

/// { 0x450b70 } (thiscall, club_records.cpp): `unbeaten_run_record_construct`: constrói o recorde de jogos sem perder:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_invencibilidade(this: usize) -> usize {
    cpf2_data_padrao(this + 4); // o começo da invencibilidade
    cpf2_data_padrao(this + 0xc);
    this
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_vitorias_seguidas(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(849);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_vitorias_seguidas(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_vitorias_seguidas {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_derrotas_seguidas(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(850);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_derrotas_seguidas(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_derrotas_seguidas {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_invencibilidade(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(851);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_de_invencibilidade(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_invencibilidade {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 245a leva: os recordes do clube: a construção dos recordes de jogos sem vencer, de gols numa partida e do jogador mais novo ----------

/// { 0x450bc0 } (thiscall, club_records.cpp): `winless_run_record_construct`: constrói o recorde de jogos sem vencer:
/// as datas do começo (+4) e do fim (+0xc) (0x54a110). O quadro SEH do original não é preciso (a 0x54a110 não falha).
/// Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_sem_vencer(this: usize) -> usize {
    cpf2_data_padrao(this + 4);
    cpf2_data_padrao(this + 0xc); // o fim dos jogos sem vencer
    this
}

/// { 0x450c10 } (thiscall, club_records.cpp): `goals_in_match_record_construct`: constrói o recorde de gols numa
/// partida: a data do recordista (+0x11) e a da partida (+0x19) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_de_gols_na_partida(this: usize) -> usize {
    cpf2_data_padrao(this + 0x11);
    cpf2_data_padrao(this + 0x19); // a data da partida
    this
}

/// { 0x450c70 } (thiscall, club_records.cpp): `youngest_player_record_construct`: constrói o recorde do jogador mais
/// novo: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_do_mais_novo(this: usize) -> usize {
    cpf2_data_padrao(this); // a idade do mais novo
    cpf2_data_padrao(this + 0x18);
    cpf2_data_padrao(this + 0x20);
    this
}

unsafe extern "thiscall" fn comparar_construir_recorde_sem_vencer(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(852);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x14)], || o(this), || construir_recorde_sem_vencer(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_sem_vencer {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_de_gols_na_partida(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(853);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x21)], || o(this), || construir_recorde_de_gols_na_partida(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_de_gols_na_partida {:x?} x {:x?}", mo, mr));
    r
}

unsafe extern "thiscall" fn comparar_construir_recorde_do_mais_novo(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(854);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || construir_recorde_do_mais_novo(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_do_mais_novo {:x?} x {:x?}", mo, mr));
    r
}

// ---------- 245a leva: as regras de transferência da nação: o texto da notícia do fim do prazo e o fim do empréstimo; o pow do game.cpp ----------

unsafe fn cpi4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// As regras de transferência (+0x8ac, 4 bytes cada) do índice `i` no gerenciador de transferências `this`.
unsafe fn cpi4_regras(this: usize, i: usize) -> usize {
    cpi4_d(this + 0x8ac + i.wrapping_mul(4))
}

/// { 0x8e05a0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::write_deadline_news_text(noticia,
/// completo, texto)`: o texto da notícia do fim do prazo de transferências fica com as regras da nação da vaga 0 da
/// notícia (0x785830): o método +0x30 delas, com os mesmos argumentos. Devolve o que ele devolve.
unsafe extern "thiscall" fn texto_do_fim_do_prazo(this: usize, noticia: usize, completo: u32, texto: usize) -> u32 {
    let vaga: unsafe extern "thiscall" fn(usize, i8) -> u32 = core::mem::transmute(0x785830usize);
    let regras = cpi4_regras(this, vaga(noticia, 0) as usize); // as regras da vaga 0
    let metodo: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(cpi4_d(cpi4_d(regras) + 0x30));
    metodo(regras, noticia, completo, texto)
}

/// As regras (das 28 em +0x8ac) da nação do `clube` (+0x53), se o clube dono do `jogador` (+0x39) é da mesma nação;
/// senão nenhuma.
unsafe fn cpi4_regras_do_emprestimo(this: usize, clube: usize, jogador: usize) -> Option<usize> {
    let dono = cpi4_d(jogador + 0x39);
    if dono == 0 || cpi4_d(dono + 0x53) != cpi4_d(clube + 0x53) {
        return None;
    }
    let nacao = cpi4_d(clube + 0x53);
    (0..0x1c).map(|i| cpi4_regras(this, i)).find(|&r| cpi4_d(r + 0xa) == nacao) // a nação das regras
}

/// { 0x8d0530 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::can_end_loan(clube, jogador, a, b,
/// c)`: com o clube dono do jogador (+0x39) na mesma nação (+0x53) do `clube`, quem responde são as regras dessa nação
/// (a primeira das 28 em +0x8ac com +0xa igual): o método +0x24 delas com `clube`, `jogador`, `a`, `b`, `c`. Senão 1.
unsafe extern "thiscall" fn pode_encerrar_emprestimo(this: usize, clube: usize, jogador: usize, a: u32, b: u32, c: u32) -> u32 {
    match cpi4_regras_do_emprestimo(this, clube, jogador) {
        Some(r) => {
            let metodo: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(cpi4_d(cpi4_d(r) + 0x24));
            metodo(r, clube, jogador, a, b, c)
        }
        None => 1,
    }
}

/// { 0x5d5a20 } (cdecl, game.cpp): `pow_double(base, expoente)`: o pow da CRT do jogo (0x945ff0), resultado em st0.
unsafe extern "C" fn pow_do_game(base: f64, expoente: f64) -> f64 {
    pow_do_jogo(base, expoente)
}

/// Comparador: o método das regras pode escrever o texto e sortear: só a original; a escolha das regras da cópia é a
/// mesma conta (a prova é a carreira e o modo Rust).
unsafe extern "thiscall" fn comparar_texto_do_fim_do_prazo(this: usize, noticia: usize, completo: u32, texto: usize) -> u32 {
    crate::comparacao::ramo("texto_do_fim_do_prazo");
    orig::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(855)(this, noticia, completo, texto)
}

/// Comparador: quando as regras da nação respondem, só a original (o método é delas); senão os dois devolvem 1.
unsafe extern "thiscall" fn comparar_pode_encerrar_emprestimo(this: usize, clube: usize, jogador: usize, a: u32, b: u32, c: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32>(856)(this, clube, jogador, a, b, c);
    if cpi4_regras_do_emprestimo(this, clube, jogador).is_some() {
        crate::comparacao::ramo("pode_encerrar_emprestimo pelas regras");
        return r;
    }
    PLACAR.registrar(r == 1, || format!("pode_encerrar_emprestimo({:x}, {:x}) original={} rust=1", clube, jogador, r));
    r
}

unsafe extern "C" fn comparar_pow_do_game(base: f64, expoente: f64) -> f64 {
    let r = orig::<unsafe extern "C" fn(f64, f64) -> f64>(857)(base, expoente);
    let n = pow_do_game(base, expoente);
    PLACAR.registrar(r.to_bits() == n.to_bits(), || format!("pow_do_game({}, {}) original={} rust={}", base, expoente, r, n));
    r
}

// ---------- 245a leva: tirar as ofertas guardadas da pessoa, guardar as suspensões do emprestado e preencher a vaga da comissão ----------

unsafe fn cpi5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x4cf560 } (thiscall, ret 4, contract_manager.cpp): `CONTRACT_MANAGER::remove_held_offers(pessoa)`: da lista das
/// ofertas guardadas (+0x38; o cursor +0xa volta ao começo +2), tira (0x68d1f0) cada item cujo +0 é o id `pessoa`
/// (0x68d1b0 percorre). Sem lista: nada.
unsafe extern "thiscall" fn tirar_ofertas_guardadas(this: usize, pessoa: u32) {
    let l = cpi5_d(this + 0x38);
    if l == 0 {
        return;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpi5_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let tirar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x68d1f0usize);
    let mut e = proximo(l);
    while e != 0 {
        if cpi5_d(e) as u32 == pessoa { // a oferta da pessoa
            tirar(l, e);
        }
        e = proximo(l);
    }
}

/// A suspensão do tipo `t` (vetor de tipos [this], 4 bytes cada) vale no clube do empréstimo? Pela competição do tipo
/// (+0x17, −1 = nenhuma) igual à nação do clube (+0x53, id em +0); sem competição, pelo +0x1b (−1: vale) igual ao
/// [+0x71] da nação. Clube sem nação: não vale.
unsafe fn cpi5_vale_no_clube(this: usize, t: i8, clube: usize) -> bool {
    let tipo = cpi5_d(cpi5_d(this).wrapping_add((t as isize as usize).wrapping_mul(4)));
    let nacao = cpi5_d(clube + 0x53);
    let comp = cpi5_d(tipo + 0x17) as i32;
    if comp != -1 {
        return nacao != 0 && cpi5_d(nacao) as i32 == comp;
    }
    let x = cpi5_d(tipo + 0x1b) as i32;
    if x == -1 {
        return true;
    }
    if nacao == 0 {
        return false;
    }
    let m = cpi5_d(nacao + 0x71);
    m != 0 && cpi5_d(m) as i32 == x // a do +0x71 da nação
}

/// O clube do empréstimo (+4 da ficha de contrato, clubes de 0x245 bytes em [0xae23b4]) e o registro de suspensões
/// do jogador (29 bytes em [this+0x14]); as vagas (5 bytes: tipo e o resto) que não valem lá, menos a 4ª (índice 3,
/// que o original pula).
unsafe fn cpi5_vagas_a_guardar(this: usize, ficha: usize, jogador: usize) -> (usize, Vec<u8>) {
    let clube = cpi5_d(0xae23b4).wrapping_add(cpi5_d(ficha + 4).wrapping_mul(0x245));
    let reg = cpi5_d(this + 0x14).wrapping_add(cpi5_d(jogador).wrapping_mul(29));
    let mut v = Vec::new();
    for k in 0..4u8 {
        let t = *((reg + 5 * k as usize) as *const i8);
        if k == 3 || t == -1 {
            continue;
        }
        if !cpi5_vale_no_clube(this, t, clube) {
            v.push(k);
        }
    }
    (reg, v)
}

/// { 0x5563f0 } (thiscall, ret 4, discipline.cpp): `Discipline::hold_bans_for_loan(jogador)`: com a ficha de contrato
/// do jogador (0x4cf6d0 sobre [0xae1050]), cada suspensão do registro dele (vagas 0 a 2) que não vale no clube do
/// empréstimo vai para a lista das guardadas (+0x1c, 0x68d0d0 com 10 bytes: o id, a vaga e os 5 bytes dela), a vaga
/// fica −1 e o contador +0x1c do registro sobe. Sem ficha: a original (caixa "Error").
unsafe extern "thiscall" fn guardar_suspensoes_do_emprestado(this: usize, jogador: usize) {
    let ficha_de: unsafe extern "thiscall" fn(usize, *const i32) -> usize = core::mem::transmute(0x4cf6d0usize);
    let ficha = ficha_de(cpi5_d(0xae1050), jogador as *const i32);
    if ficha == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(859)(this, jogador); // caixa: só a original
    }
    let (reg, vagas) = cpi5_vagas_a_guardar(this, ficha, jogador);
    let acrescentar: unsafe extern "thiscall" fn(usize, *const u8, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
    for k in vagas {
        let vaga = reg + 5 * k as usize;
        let mut r = [0u8; 10];
        r[..4].copy_from_slice(&(cpi5_d(jogador) as u32).to_le_bytes());
        r[4] = k; // a vaga
        r[5..].copy_from_slice(core::slice::from_raw_parts(vaga as *const u8, 5));
        acrescentar(this + 0x1c, r.as_ptr(), 10);
        core::ptr::write_unaligned(vaga as *mut u32, u32::MAX);
        *((vaga + 4) as *mut u8) = 0xff;
        *((reg + 0x1c) as *mut u8) = (*((reg + 0x1c) as *const u8)).wrapping_add(1);
    }
}

/// O cargo que a vaga `cargo` (byte com sinal) procura: 6, 8, 9 e 10 o mesmo; 13 procura 6 e 15 procura 8 (o byte do
/// argumento é trocado); os outros, nenhum.
fn cpi5_cargo_procurado(cargo: u32) -> Option<u32> {
    match cargo as u8 as i8 {
        6 | 8 | 9 | 10 => Some(cargo),
        13 => Some((cargo & !0xff) | 6),
        15 => Some((cargo & !0xff) | 8), // o 15 procura 8
        _ => None,
    }
}

/// A primeira pessoa (de [0xae23bc], 0x6e bytes, [0xae2364] delas) sem clube (+0x39), no cargo (+0x3d), com registro
/// (+0x69) e da nação (+0x1a) do clube (+0x53).
unsafe fn cpi5_desempregado(clube: usize, cargo: u8) -> Option<usize> {
    let n = cpi5_d(0xae2364) as i32;
    let base = cpi5_d(0xae23bc);
    (0..n.max(0) as usize).map(|i| base + i * 0x6e).find(|&p| {
        cpi5_d(p + 0x39) == 0 && *((p + 0x3d) as *const u8) == cargo && cpi5_d(p + 0x69) != 0 && cpi5_d(p + 0x1a) == cpi5_d(clube + 0x53)
    })
}

/// { 0x8cb6d0 } (stdcall, ret 8, transfer_manager.cpp): `fill_vacated_staff_job(clube, cargo)`: a vaga da comissão
/// que ficou livre é preenchida pela primeira pessoa desempregada da nação do clube no cargo procurado (6, 8, 9 e 10;
/// 13 procura 6, 15 procura 8): 0x53bd10 com o cargo (o dword do argumento), a data de hoje [0xae2c90], gravar e sem
/// reduzir. Clube nulo: a original (caixa "Error").
unsafe extern "stdcall" fn preencher_vaga_da_comissao(clube: usize, cargo: u32) {
    if clube == 0 {
        return orig::<unsafe extern "stdcall" fn(usize, u32)>(860)(clube, cargo); // caixa: só a original
    }
    let Some(c) = cpi5_cargo_procurado(cargo) else { return };
    if let Some(p) = cpi5_desempregado(clube, c as u8) {
        let incluir: unsafe extern "C" fn(usize, usize, u32, usize, u32, u32) -> u32 = core::mem::transmute(0x53bd10usize);
        incluir(clube, p, c, 0xae2c90, 1, 0);
    }
}

/// Comparador: o plano antes (a 0x68d1f0 libera o item tirado, que não pode mais ser lido); a original tira; confere
/// que ficaram os itens de antes menos os da pessoa, na mesma ordem.
unsafe extern "thiscall" fn comparar_tirar_ofertas_guardadas(this: usize, pessoa: u32) {
    let l = cpi5_d(this + 0x38);
    let antes: Vec<usize> = if l != 0 { cpd1_itens(l) } else { Vec::new() };
    let esperado: Vec<usize> = antes.iter().copied().filter(|&e| cpi5_d(e) as u32 != pessoa).collect();
    orig::<unsafe extern "thiscall" fn(usize, u32)>(858)(this, pessoa);
    let depois: Vec<usize> = if l != 0 { cpd1_itens(l) } else { Vec::new() };
    PLACAR.registrar(depois == esperado, || format!("tirar_ofertas_guardadas({:x}) {} itens, esperava {}, ficaram {}", pessoa, antes.len(), esperado.len(), depois.len()));
}

/// Comparador: o plano da cópia (as vagas que não valem no clube do empréstimo) antes; a original guarda; confere o
/// registro do jogador: as vagas do plano em −1, as outras iguais e o contador somado.
unsafe extern "thiscall" fn comparar_guardar_suspensoes_do_emprestado(this: usize, jogador: usize) {
    let ficha_de: unsafe extern "thiscall" fn(usize, *const i32) -> usize = core::mem::transmute(0x4cf6d0usize);
    let ficha = ficha_de(cpi5_d(0xae1050), jogador as *const i32);
    if ficha == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize)>(859)(this, jogador);
    }
    let (reg, vagas) = cpi5_vagas_a_guardar(this, ficha, jogador);
    let mut esperado = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    for &k in &vagas {
        esperado[5 * k as usize..5 * k as usize + 5].fill(0xff);
    }
    esperado[0x1c] = esperado[0x1c].wrapping_add(vagas.len() as u8);
    orig::<unsafe extern "thiscall" fn(usize, usize)>(859)(this, jogador);
    let depois = core::slice::from_raw_parts(reg as *const u8, 29).to_vec();
    PLACAR.registrar(depois == esperado, || format!("guardar_suspensoes_do_emprestado({:x}) vagas {:?} {:x?} x {:x?}", jogador, vagas, depois, esperado));
}

/// Comparador: a escolha da cópia antes; a original preenche; a pessoa escolhida tem de estar agora no clube (+0x39).
unsafe extern "stdcall" fn comparar_preencher_vaga_da_comissao(clube: usize, cargo: u32) {
    let escolhida = if clube != 0 { cpi5_cargo_procurado(cargo).and_then(|c| cpi5_desempregado(clube, c as u8)) } else { None };
    orig::<unsafe extern "stdcall" fn(usize, u32)>(860)(clube, cargo);
    if let Some(p) = escolhida {
        PLACAR.registrar(cpi5_d(p + 0x39) == clube, || format!("preencher_vaga_da_comissao({:x}, {}) a pessoa {:x} ficou em {:x}", clube, cargo as u8, p, cpi5_d(p + 0x39)));
    } else {
        crate::comparacao::ramo("preencher_vaga_da_comissao sem ninguem");
    }
}

// ---------- 245a leva: a notícia para os técnicos humanos de uma nação, a missão do olheiro da busca e se a copa terminou ----------

unsafe fn cpi6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x786520 } (thiscall, ret 8, news.cpp): `News::send_to_nation_club_managers(noticia, nacao)`: a notícia fica com
/// o alcance da nação (+0xde = 2) e vai (0x785d40 com 0) para cada um dos 16 técnicos humanos (os últimos 16 de
/// [0xae23bc], 0x6e bytes) ativos (+0x5f = 1) com clube (+0x39) da `nacao` (+0x53); a primeira entrega recusada (0)
/// encerra. Devolve 0. Notícia ou nação nulas: a original (caixa "Error").
unsafe extern "thiscall" fn noticia_para_os_tecnicos_da_nacao(this: usize, noticia: usize, nacao: usize) -> u32 {
    if noticia == 0 || nacao == 0 {
        return orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(861)(this, noticia, nacao); // caixa: só a original
    }
    *((noticia + 0xde) as *mut u8) = 2; // o alcance da nação
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u32 = core::mem::transmute(0x785d40usize);
    for i in 0..0x10usize {
        let p = cpi6_d(0xae23bc).wrapping_add(cpi6_d(0xae2364).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
        if *((p + 0x5f) as *const u8) != 1 {
            continue;
        }
        let clube = cpi6_d(p + 0x39);
        if clube == 0 || cpi6_d(clube + 0x53) != nacao {
            continue;
        }
        if entregar(this, noticia, p, 0) == 0 {
            break; // a entrega recusada encerra
        }
    }
    0
}

/// O olheiro da busca confere (o tipo +0xa é 4, o índice +0x180 > −1 e a pessoa ainda confere com os nomes e a data
/// guardados pela 0x53e990)? Devolve a pessoa, ou nenhuma (a original mostra a caixa).
unsafe fn cpi6_olheiro(this: usize) -> Option<usize> {
    let i = cpi6_d(this + 0x180) as i32;
    if *((this + 0xa) as *const u8) != 4 || i <= -1 {
        return None;
    }
    let p = cpi6_d(0xae23bc).wrapping_add((i as usize).wrapping_mul(0x6e));
    let confere: unsafe extern "C" fn(usize, usize, usize, usize, usize) -> u32 = core::mem::transmute(0x53e990usize);
    if confere(p, cpi6_d(this + 0x174), cpi6_d(this + 0x178), cpi6_d(this + 0x17c), this + 0x184) == 0 || p == 0 {
        return None;
    }
    Some(p)
}

/// { 0x7aefd0 } (thiscall, ret 0x18, player_search.cpp): `PLAYER_SEARCH::get_scout_assignment(clube, nacao, regiao,
/// comp, base, adversario)`: com o olheiro conferido, se alguma das missões está marcada (clube +0x190, nação +0x18c,
/// região +0x194 byte, competição +0x195 acima de −1; base +0x199, adversário +0x19d não nulos), copia as seis para
/// as saídas e devolve 1; nenhuma: 0. Olheiro que não confere: a original (caixa "Error" e 0).
unsafe extern "thiscall" fn missao_do_olheiro(this: usize, clube: *mut u32, nacao: *mut u32, regiao: *mut u8, comp: *mut u32, base: *mut u32, adversario: *mut u32) -> u32 {
    if cpi6_olheiro(this).is_none() {
        return orig::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(862)(this, clube, nacao, regiao, comp, base, adversario); // caixa: só a original
    }
    let marcada = cpi6_d(this + 0x190) as i32 > -1
        || cpi6_d(this + 0x18c) as i32 > -1
        || *((this + 0x194) as *const i8) > -1
        || cpi6_d(this + 0x195) as i32 > -1 // a competição
        || cpi6_d(this + 0x199) != 0
        || cpi6_d(this + 0x19d) != 0;
    if !marcada {
        return 0;
    }
    clube.write_unaligned(cpi6_d(this + 0x190) as u32);
    nacao.write_unaligned(cpi6_d(this + 0x18c) as u32);
    *regiao = *((this + 0x194) as *const u8);
    comp.write_unaligned(cpi6_d(this + 0x195) as u32);
    base.write_unaligned(cpi6_d(this + 0x199) as u32);
    adversario.write_unaligned(cpi6_d(this + 0x19d) as u32);
    1
}

/// { 0x521e00 } (thiscall, ret 8, Cup.cpp): `Cup::check_complete(fases, _)`: 0 se a copa não começou (+0xb1). Com
/// `fases` = 1 e sem fase marcada (+0x44 = 0xff): cada fase de 0 até a atual (+0x30, vetor +0xc) tem de estar completa
/// (método +0x10 com 0, 1) e a atual tem de ser a última (+0x2c − 1); senão 0. Fora isso, 1.
unsafe extern "thiscall" fn copa_completa(this: usize, fases: u32, _x: u32) -> u8 {
    if *((this + 0xb1) as *const u8) == 0 {
        return 0;
    }
    if fases != 1 || *((this + 0x44) as *const u8) != 0xff {
        return 1;
    }
    let mut i = 0i32;
    while i <= cpi6_d(this + 0x30) as i32 {
        let fase = cpi6_d(cpi6_d(this + 0xc).wrapping_add(i as usize * 4));
        let completa: unsafe extern "thiscall" fn(usize, u32, u32) -> u8 = core::mem::transmute(cpi6_d(cpi6_d(fase) + 0x10));
        if completa(fase, 0, 1) == 0 {
            return 0; // uma fase incompleta
        }
        i += 1;
    }
    (cpi6_d(this + 0x30) as i32 >= (cpi6_d(this + 0x2c) as i32).wrapping_sub(1)) as u8
}

/// Comparador: a entrega (0x785d40) põe a notícia nas caixas dos técnicos: só a original (a prova é a carreira e o
/// modo Rust).
unsafe extern "thiscall" fn comparar_noticia_para_os_tecnicos_da_nacao(this: usize, noticia: usize, nacao: usize) -> u32 {
    crate::comparacao::ramo("noticia_para_os_tecnicos_da_nacao");
    orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(861)(this, noticia, nacao)
}

/// Comparador: lado a lado pelas seis saídas (a 0x53e990 só lê); com a caixa, só a original.
unsafe extern "thiscall" fn comparar_missao_do_olheiro(this: usize, clube: *mut u32, nacao: *mut u32, regiao: *mut u8, comp: *mut u32, base: *mut u32, adversario: *mut u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, *mut u32, *mut u32, *mut u8, *mut u32, *mut u32, *mut u32) -> u32>(862);
    if cpi6_olheiro(this).is_none() {
        return o(this, clube, nacao, regiao, comp, base, adversario);
    }
    let reg = [(clube as usize, 4), (nacao as usize, 4), (regiao as usize, 1), (comp as usize, 4), (base as usize, 4), (adversario as usize, 4)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this, clube, nacao, regiao, comp, base, adversario),
        || missao_do_olheiro(this, clube, nacao, regiao, comp, base, adversario));
    PLACAR.registrar(igual && r == n, || format!("missao_do_olheiro original={} rust={} {:x?} x {:x?}", r, n, mo, mr));
    r
}

/// Comparador: pelo valor (o método das fases só lê).
unsafe extern "thiscall" fn comparar_copa_completa(this: usize, fases: u32, x: u32) -> u8 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u8>(863)(this, fases, x);
    let n = copa_completa(this, fases, x);
    PLACAR.registrar(r == n, || format!("copa_completa({:x}, {}) original={} rust={}", this, fases, r, n));
    r
}

// ---------- 245a leva: o recorde do jogador mais velho e as permissões de trabalho: achar a do jogador e se ele passa no limite de estrangeiros ----------

unsafe fn cpj1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// { 0x450cd0 } (thiscall, club_records.cpp): `oldest_player_record_construct`: constrói o recorde do jogador mais
/// velho: a idade (+0), a data do recordista (+0x18) e a da partida (+0x20) (0x54a110). O quadro SEH do original não
/// é preciso (a 0x54a110 não falha). Devolve `this`.
unsafe extern "thiscall" fn construir_recorde_do_mais_velho(this: usize) -> usize {
    cpf2_data_padrao(this);
    cpf2_data_padrao(this + 0x18); // a data do mais velho
    cpf2_data_padrao(this + 0x20);
    this
}

/// { 0x8c7260 } (thiscall, ret 4, transfer_manager.cpp): `TRANSFER_MANAGER::find_work_permit(pessoa)`: a permissão
/// de trabalho da pessoa: id fora dos [0xae2364] (com sinal), 0; sem a marca 0x20 no byte do id (8 bytes cada, em
/// [+0x86e]), 0; senão a busca binária (bsearch 0x946464, registros de 0xe bytes em [+0x93a], [+0x93e] deles,
/// comparação 0x8e55c0) pelo id. Pessoa nula: lê o endereço 0 como o original.
unsafe extern "thiscall" fn permissao_de_trabalho_de(this: usize, pessoa: *const i32) -> usize {
    let id = core::ptr::read_volatile(pessoa);
    if !pessoa.is_null() && id >= cpj1_d(0xae2364) as i32 {
        return 0;
    }
    if *((cpj1_d(this + 0x86e).wrapping_add((id as usize).wrapping_mul(8))) as *const u8) & 0x20 == 0 {
        return 0; // sem a marca da permissão
    }
    let chave = id;
    let bsearch: unsafe extern "C" fn(*const i32, usize, usize, usize, usize) -> usize = core::mem::transmute(0x946464usize);
    bsearch(&chave, cpj1_d(this + 0x93a), cpj1_d(this + 0x93e), 0xe, 0x8e55c0)
}

/// { 0x84b180 } (stdcall, ret 0xc, shortlist_manager.cpp): `passes_work_permit_check(jogador, _, comprador)`: o
/// jogador pode ir para o clube comprador? Sem nação (+0x1a): não. Nação do comprador (+0x53) sem a regra 0xe (+0x76),
/// a mesma nação, a 2ª nação (+0x1e) ou comunitário (0x53d740): sim. Sem seleção (+0x24) nem jogos por ela (+0x22):
/// não. Senão a permissão de trabalho (0x8c7950 sobre [0xde1f54], sem oferta, registro nem sorteio). O 2º argumento
/// não é usado.
unsafe extern "stdcall" fn passa_na_permissao(jogador: usize, _atual: usize, comprador: usize) -> u8 {
    let nacao = cpj1_d(jogador + 0x1a);
    if nacao == 0 {
        return 0;
    }
    let destino = cpj1_d(comprador + 0x53);
    if *((destino + 0x76) as *const u8) != 0xe || nacao == destino || cpj1_d(jogador + 0x1e) == destino {
        return 1;
    }
    let comunitaria: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x53d740usize);
    if comunitaria(jogador, destino) != 0 {
        return 1; // comunitário
    }
    if cpj1_d(jogador + 0x24) == 0 && *((jogador + 0x22) as *const u8) == 0 {
        return 0;
    }
    let permissao: unsafe extern "thiscall" fn(usize, usize, usize, usize, usize, u32) -> u8 = core::mem::transmute(0x8c7950usize);
    permissao(cpj1_d(0xde1f54), jogador, destino, 0, 0, 0)
}

unsafe extern "thiscall" fn comparar_construir_recorde_do_mais_velho(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(864);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x28)], || o(this), || construir_recorde_do_mais_velho(this));
    PLACAR.registrar(igual && r == n, || format!("construir_recorde_do_mais_velho {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: pelo valor (a busca só lê).
unsafe extern "thiscall" fn comparar_permissao_de_trabalho_de(this: usize, pessoa: *const i32) -> usize {
    let r = orig::<unsafe extern "thiscall" fn(usize, *const i32) -> usize>(865)(this, pessoa);
    let n = permissao_de_trabalho_de(this, pessoa);
    PLACAR.registrar(r == n, || format!("permissao_de_trabalho_de({:x}) original={:#x} rust={:#x}", pessoa as usize, r, n));
    r
}

/// Comparador: a 0x8c7950 (sem sorteio pedido) pode mexer no sorteio: lado a lado pelo estado dele, a cópia em
/// sem_contar.
unsafe extern "stdcall" fn comparar_passa_na_permissao(jogador: usize, atual: usize, comprador: usize) -> u8 {
    let o = orig::<unsafe extern "stdcall" fn(usize, usize, usize) -> u8>(866);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(jogador, atual, comprador),
        || crate::aleatorio::sem_contar(|| passa_na_permissao(jogador, atual, comprador)));
    PLACAR.registrar(igual && r == n, || format!("passa_na_permissao({:x}, {:x}) original={} rust={} {:x?} x {:x?}", jogador, comprador, r, n, mo, mr));
    r
}
