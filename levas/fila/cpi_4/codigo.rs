// ---------- %%LEVA%%a leva: as regras de transferência da nação: o texto da notícia do fim do prazo e o fim do empréstimo; o pow do game.cpp ----------

unsafe fn cpi4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// As regras de transferência (+0x8ac, 4 bytes cada) do índice `i` no gerenciador de transferências `this`.
unsafe fn transfer_rules_at(this: usize, i: usize) -> usize {
    cpi4_d(this + 0x8ac + i.wrapping_mul(4))
}

/// { 0x8e05a0 } (thiscall, ret 0xc, transfer_manager.cpp): `TRANSFER_MANAGER::write_deadline_news_text(noticia,
/// completo, texto)`: o texto da notícia do fim do prazo de transferências fica com as regras da nação da vaga 0 da
/// notícia (0x785830): o método +0x30 delas, com os mesmos argumentos. Devolve o que ele devolve.
unsafe extern "thiscall" fn write_deadline_news_text(this: usize, noticia: usize, completo: u32, texto: usize) -> u32 {
    let vaga: unsafe extern "thiscall" fn(usize, i8) -> u32 = core::mem::transmute(0x785830usize);
    let regras = transfer_rules_at(this, vaga(noticia, 0) as usize); // as regras da vaga 0
    let metodo: unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32 = core::mem::transmute(cpi4_d(cpi4_d(regras) + 0x30));
    metodo(regras, noticia, completo, texto)
}

/// As regras (das 28 em +0x8ac) da nação do `clube` (+0x53), se o clube dono do `jogador` (+0x39) é da mesma nação;
/// senão nenhuma.
unsafe fn loan_transfer_rules(this: usize, clube: usize, jogador: usize) -> Option<usize> {
    let dono = cpi4_d(jogador + 0x39);
    if dono == 0 || cpi4_d(dono + 0x53) != cpi4_d(clube + 0x53) {
        return None;
    }
    let nacao = cpi4_d(clube + 0x53);
    (0..0x1c).map(|i| transfer_rules_at(this, i)).find(|&r| cpi4_d(r + 0xa) == nacao) // a nação das regras
}

/// { 0x8d0530 } (thiscall, ret 0x14, transfer_manager.cpp): `TRANSFER_MANAGER::can_end_loan(clube, jogador, a, b,
/// c)`: com o clube dono do jogador (+0x39) na mesma nação (+0x53) do `clube`, quem responde são as regras dessa nação
/// (a primeira das 28 em +0x8ac com +0xa igual): o método +0x24 delas com `clube`, `jogador`, `a`, `b`, `c`. Senão 1.
unsafe extern "thiscall" fn can_end_loan(this: usize, clube: usize, jogador: usize, a: u32, b: u32, c: u32) -> u32 {
    match loan_transfer_rules(this, clube, jogador) {
        Some(r) => {
            let metodo: unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(cpi4_d(cpi4_d(r) + 0x24));
            metodo(r, clube, jogador, a, b, c)
        }
        None => 1,
    }
}

/// { 0x5d5a20 } (cdecl, game.cpp): `pow_double(base, expoente)`: o pow da CRT do jogo (0x945ff0), resultado em st0.
unsafe extern "C" fn pow_double(base: f64, expoente: f64) -> f64 {
    game_pow(base, expoente)
}

/// Comparador: o método das regras pode escrever o texto e sortear: só a original; a escolha das regras da cópia é a
/// mesma conta (a prova é a carreira e o modo Rust).
unsafe extern "thiscall" fn comparar_write_deadline_news_text(this: usize, noticia: usize, completo: u32, texto: usize) -> u32 {
    crate::comparison::branch("write_deadline_news_text");
    original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, usize) -> u32>(%%I0%%)(this, noticia, completo, texto)
}

/// Comparador: quando as regras da nação respondem, só a original (o método é delas); senão os dois devolvem 1.
unsafe extern "thiscall" fn comparar_can_end_loan(this: usize, clube: usize, jogador: usize, a: u32, b: u32, c: u32) -> u32 {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, u32) -> u32>(%%I1%%)(this, clube, jogador, a, b, c);
    if loan_transfer_rules(this, clube, jogador).is_some() {
        crate::comparison::branch("can_end_loan pelas regras");
        return r;
    }
    SCOREBOARD.record(r == 1, || format!("can_end_loan({:x}, {:x}) original={} rust=1", clube, jogador, r));
    r
}

unsafe extern "C" fn comparar_pow_double(base: f64, expoente: f64) -> f64 {
    let r = original_fn::<unsafe extern "C" fn(f64, f64) -> f64>(%%I2%%)(base, expoente);
    let n = pow_double(base, expoente);
    SCOREBOARD.record(r.to_bits() == n.to_bits(), || format!("pow_double({}, {}) original={} rust={}", base, expoente, r, n));
    r
}
