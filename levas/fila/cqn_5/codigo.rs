// ---------- %%LEVA%%a leva: o calendário real de 2001/02 da Série A e da Série B italianas (os jogos fixos pelo nome dos clubes) (ita_ser_a.cpp, ita_ser_b.cpp) ----------

unsafe fn cqn5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn5_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Quantos jogos a liga tem: (clubes +0x3e / 2, com sinal, para zero) × rodadas +0xa9.
unsafe fn cqn5_jogos(liga: usize) -> i32 {
    let clubes = core::ptr::read_unaligned((liga + 0x3e) as *const i16) as i32;
    let rodadas = core::ptr::read_unaligned((liga + 0xa9) as *const i16) as i32;
    (clubes / 2).wrapping_mul(rodadas)
}

/// O corpo das duas (set_up_fixed_fixtures; thiscall, ret 0, quadro SEH só para o construtor de vetor): o vetor dos
/// jogos fixos (operator new 0x944cf1 de n × 16 + 4: push; add esp, 4 -> cdecl; a conta n no primeiro dword e cada jogo
/// pelo construtor 0x645800, com o destrutor 0x5a53c0 para a exceção: `eh vector constructor iterator` 0x9463af, push
/// 0x5a53c0; push 0x645800; push n; push 0x10; push vetor -> stdcall, ret 0x14) em +0xea; sem memória, a caixa da
/// linha `linha` do fonte `arquivo` e fim. Depois cada jogo da `tabela`, na ordem, no seu lugar do vetor
/// (fixed_fixture_set_by_club_names 0x68aff0: push o 6º (fora, pelo mapa); push o 5º (casa); push ano; push mês (0 =
/// janeiro); push dia; push liga; mov ecx, [+0xea] + 16 × k -> thiscall, ret 0x18: a data e os clubes achados pelo
/// nome, −1 em +8/+0xc se não acha). Por fim, se algum dos n jogos ficou com um clube sem achar (+8 ou +0xc = −1), o
/// vetor é destruído (`eh vector destructor iterator` 0x9462bb: push 0x5a53c0; push conta; push 0x10; push vetor ->
/// stdcall, ret 0x10; operator delete 0x944c94 do bloco: push; add esp, 4 -> cdecl) e +0xea = 0: a liga volta ao
/// calendário sorteado.
unsafe fn cqn5_fixos(this: usize, tabela: &[(u8, u8, u16, u32, u32)], arquivo: usize, linha: u32) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let destruir: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let jogo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x68aff0usize);
    let n = cqn5_jogos(this);
    let bloco = novo((n as u32).wrapping_mul(16).wrapping_add(4));
    let vetor = if bloco != 0 {
        cqn5_pd(bloco, n as usize);
        construir(bloco + 4, 0x10, n as u32, 0x645800, 0x5a53c0);
        bloco + 4
    } else {
        0
    };
    cqn5_pd(this + 0xea, vetor);
    if vetor == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, &(dia, mes, ano, casa, fora)) in tabela.iter().enumerate() {
        jogo(cqn5_d(this + 0xea) + k * 0x10, this, dia as u32, mes as u32, ano as u32, casa, fora); // cqn5: cada jogo fixo
    }
    crate::comparacao::ramo("cqn_5 calendario fixo");
    let n = cqn5_jogos(this);
    if n <= 0 {
        return;
    }
    let v = cqn5_d(this + 0xea);
    for i in 0..n as usize {
        let f = v + i * 0x10;
        if cqn5_d(f + 8) == 0xffff_ffff || cqn5_d(f + 0xc) == 0xffff_ffff { // cqn5: um clube sem achar derruba a lista
            if v != 0 {
                destruir(v, 0x10, cqn5_d(v - 4) as u32, 0x5a53c0);
                apagar(v - 4);
            }
            cqn5_pd(this + 0xea, 0);
            return;
        }
    }
}

/// { 0x642a70 } (thiscall, ret 0, ita_ser_a.cpp, 11.650 bytes): `ItaSerA::create_fixed_fixtures()`: os 306 jogos da
/// Série A 2001/02 (34 rodadas de 9, com o dia, o mês (0 = janeiro) e o ano que a original empurra). Fonte 0x9ea2a8, linha 0x60f.
unsafe extern "thiscall" fn create_fixed_fixtures(this: usize) {
    cqn5_fixos(this, &CQN5_SERIE_A, 0x9ea2a8, 0x60f)
}

/// { 0x647230 } (thiscall, ret 0, ita_ser_b.cpp, 14.388 bytes): `ItaSerB::set_up_fixed_fixtures()`: os 380 jogos da
/// Série B 2001/02 (38 rodadas de 10). Fonte 0x9ea384, linha 0x39a.
unsafe extern "thiscall" fn ita_ser_a_set_up_fixed_fixtures(this: usize) {
    cqn5_fixos(this, &CQN5_SERIE_B, 0x9ea384, 0x39a)
}

/// O comparador das duas: alocam: só a original, conferindo o que ela garante: ou a lista ficou (+0xea com a conta n e
/// cada jogo com os dois clubes achados), ou foi derrubada (+0xea = 0).
unsafe fn cqn5_comparar(nome: &str, this: usize, original: impl FnOnce()) {
    original();
    let v = cqn5_d(this + 0xea);
    let n = cqn5_jogos(this);
    let ok = v == 0 || (cqn5_d(v - 4) as i32 == n && (0..n.max(0) as usize).all(|i| cqn5_d(v + i * 0x10 + 8) != 0xffff_ffff && cqn5_d(v + i * 0x10 + 0xc) != 0xffff_ffff));
    PLACAR.registrar(ok, || format!("{}({:#x}): a lista dos jogos fixos (+0xea = {:#x}) fora do esperado", nome, this, v));
}

unsafe extern "thiscall" fn comparar_create_fixed_fixtures(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    cqn5_comparar("create_fixed_fixtures", this, || o(this))
}

unsafe extern "thiscall" fn comparar_ita_ser_a_set_up_fixed_fixtures(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqn5_comparar("ita_ser_a_set_up_fixed_fixtures", this, || o(this))
}

/// Os jogos fixos (dia, mês com 0 = janeiro, ano, o 5º argumento (casa, pelo mapa), o 6º (fora)), os nomes no .rdata do
/// exe; na ordem da original (extraídos da desmontagem e conferidos: os lugares do vetor vão de 16 em 16).
const CQN5_SERIE_A: [(u8, u8, u16, u32, u32); 306] = [
    (28, 7, 2001, 0x9ea370, 0x9ea378), // Bologna × Atalanta
    (28, 7, 2001, 0x9ea360, 0x9ea368), // Brescia × Milan
    (28, 7, 2001, 0x9ea34c, 0x9ea358), // Fiorentina × Chievo
    (28, 7, 2001, 0x9ea33c, 0x9ea344), // Inter × Perugia
    (28, 7, 2001, 0x9ea328, 0x9ea334), // Juventus × Venezia
    (28, 7, 2001, 0x9ea314, 0x9ea31c), // Lazio × Piacenza
    (28, 7, 2001, 0x9ea304, 0x9ea30c), // Lecce × Parma
    (28, 7, 2001, 0x9ea2f4, 0x9ea2fc), // Udinese × Torino
    (28, 7, 2001, 0x9ea2e4, 0x9ea2ec), // Verona × Roma
    (9, 8, 2001, 0x9ea378, 0x9ea328), // Atalanta × Juventus
    (9, 8, 2001, 0x9ea358, 0x9ea370), // Chievo × Bologna
    (9, 8, 2001, 0x9ea368, 0x9ea34c), // Milan × Fiorentina
    (9, 8, 2001, 0x9ea30c, 0x9ea33c), // Parma × Inter
    (9, 8, 2001, 0x9ea344, 0x9ea314), // Perugia × Lazio
    (9, 8, 2001, 0x9ea31c, 0x9ea304), // Piacenza × Lecce
    (9, 8, 2001, 0x9ea2ec, 0x9ea2f4), // Roma × Udinese
    (9, 8, 2001, 0x9ea2fc, 0x9ea360), // Torino × Brescia
    (9, 8, 2001, 0x9ea334, 0x9ea2e4), // Venezia × Verona
    (16, 8, 2001, 0x9ea370, 0x9ea30c), // Bologna × Parma
    (16, 8, 2001, 0x9ea360, 0x9ea304), // Brescia × Lecce
    (16, 8, 2001, 0x9ea34c, 0x9ea378), // Fiorentina × Atalanta
    (16, 8, 2001, 0x9ea33c, 0x9ea334), // Inter × Venezia
    (16, 8, 2001, 0x9ea328, 0x9ea358), // Juventus × Chievo
    (16, 8, 2001, 0x9ea314, 0x9ea2fc), // Lazio × Torino
    (16, 8, 2001, 0x9ea31c, 0x9ea2ec), // Piacenza × Roma
    (16, 8, 2001, 0x9ea2f4, 0x9ea368), // Udinese × Milan
    (16, 8, 2001, 0x9ea2e4, 0x9ea344), // Verona × Perugia
    (23, 8, 2001, 0x9ea378, 0x9ea2e4), // Atalanta × Verona
    (23, 8, 2001, 0x9ea358, 0x9ea31c), // Chievo × Piacenza
    (23, 8, 2001, 0x9ea304, 0x9ea328), // Lecce × Juventus
    (23, 8, 2001, 0x9ea368, 0x9ea314), // Milan × Lazio
    (23, 8, 2001, 0x9ea30c, 0x9ea360), // Parma × Brescia
    (23, 8, 2001, 0x9ea344, 0x9ea2f4), // Perugia × Udinese
    (23, 8, 2001, 0x9ea2ec, 0x9ea34c), // Roma × Fiorentina
    (23, 8, 2001, 0x9ea2fc, 0x9ea33c), // Torino × Inter
    (23, 8, 2001, 0x9ea334, 0x9ea370), // Venezia × Bologna
    (30, 8, 2001, 0x9ea360, 0x9ea378), // Brescia × Atalanta
    (30, 8, 2001, 0x9ea34c, 0x9ea334), // Fiorentina × Venezia
    (30, 8, 2001, 0x9ea33c, 0x9ea370), // Inter × Bologna
    (30, 8, 2001, 0x9ea328, 0x9ea2ec), // Juventus × Roma
    (30, 8, 2001, 0x9ea314, 0x9ea30c), // Lazio × Parma
    (30, 8, 2001, 0x9ea344, 0x9ea368), // Perugia × Milan
    (30, 8, 2001, 0x9ea31c, 0x9ea2fc), // Piacenza × Torino
    (30, 8, 2001, 0x9ea2f4, 0x9ea358), // Udinese × Chievo
    (30, 8, 2001, 0x9ea2e4, 0x9ea304), // Verona × Lecce
    (10, 9, 2001, 0x9ea378, 0x9ea368), // Atalanta × Milan
    (10, 9, 2001, 0x9ea370, 0x9ea2f4), // Bologna × Udinese
    (10, 9, 2001, 0x9ea358, 0x9ea314), // Chievo × Lazio
    (10, 9, 2001, 0x9ea34c, 0x9ea328), // Fiorentina × Juventus
    (10, 9, 2001, 0x9ea33c, 0x9ea2e4), // Inter × Verona
    (10, 9, 2001, 0x9ea304, 0x9ea344), // Lecce × Perugia
    (10, 9, 2001, 0x9ea2ec, 0x9ea360), // Roma × Brescia
    (10, 9, 2001, 0x9ea2fc, 0x9ea30c), // Torino × Parma
    (10, 9, 2001, 0x9ea334, 0x9ea31c), // Venezia × Piacenza
    (14, 9, 2001, 0x9ea360, 0x9ea358), // Brescia × Chievo
    (14, 9, 2001, 0x9ea328, 0x9ea2fc), // Juventus × Torino
    (14, 9, 2001, 0x9ea314, 0x9ea378), // Lazio × Atalanta
    (14, 9, 2001, 0x9ea304, 0x9ea34c), // Lecce × Fiorentina
    (14, 9, 2001, 0x9ea368, 0x9ea334), // Milan × Venezia
    (14, 9, 2001, 0x9ea30c, 0x9ea31c), // Parma × Piacenza
    (14, 9, 2001, 0x9ea344, 0x9ea2ec), // Perugia × Roma
    (14, 9, 2001, 0x9ea2f4, 0x9ea33c), // Udinese × Inter
    (14, 9, 2001, 0x9ea2e4, 0x9ea370), // Verona × Bologna
    (21, 9, 2001, 0x9ea378, 0x9ea2f4), // Atalanta × Udinese
    (21, 9, 2001, 0x9ea370, 0x9ea328), // Bologna × Juventus
    (21, 9, 2001, 0x9ea358, 0x9ea30c), // Chievo × Parma
    (21, 9, 2001, 0x9ea34c, 0x9ea2e4), // Fiorentina × Verona
    (21, 9, 2001, 0x9ea33c, 0x9ea368), // Inter × Milan
    (21, 9, 2001, 0x9ea31c, 0x9ea360), // Piacenza × Brescia
    (21, 9, 2001, 0x9ea2ec, 0x9ea304), // Roma × Lecce
    (21, 9, 2001, 0x9ea2fc, 0x9ea344), // Torino × Perugia
    (21, 9, 2001, 0x9ea334, 0x9ea314), // Venezia × Lazio
    (28, 9, 2001, 0x9ea360, 0x9ea334), // Brescia × Venezia
    (28, 9, 2001, 0x9ea358, 0x9ea2fc), // Chievo × Torino
    (28, 9, 2001, 0x9ea328, 0x9ea33c), // Juventus × Inter
    (28, 9, 2001, 0x9ea304, 0x9ea378), // Lecce × Atalanta
    (28, 9, 2001, 0x9ea368, 0x9ea370), // Milan × Bologna
    (28, 9, 2001, 0x9ea30c, 0x9ea2e4), // Parma × Verona
    (28, 9, 2001, 0x9ea344, 0x9ea31c), // Perugia × Piacenza
    (28, 9, 2001, 0x9ea2ec, 0x9ea314), // Roma × Lazio
    (28, 9, 2001, 0x9ea2f4, 0x9ea34c), // Udinese × Fiorentina
    (4, 10, 2001, 0x9ea378, 0x9ea2ec), // Atalanta × Roma
    (4, 10, 2001, 0x9ea370, 0x9ea34c), // Bologna × Fiorentina
    (4, 10, 2001, 0x9ea33c, 0x9ea304), // Inter × Lecce
    (4, 10, 2001, 0x9ea314, 0x9ea360), // Lazio × Brescia
    (4, 10, 2001, 0x9ea30c, 0x9ea344), // Parma × Perugia
    (4, 10, 2001, 0x9ea31c, 0x9ea2f4), // Piacenza × Udinese
    (4, 10, 2001, 0x9ea2fc, 0x9ea368), // Torino × Milan
    (4, 10, 2001, 0x9ea334, 0x9ea358), // Venezia × Chievo
    (4, 10, 2001, 0x9ea2e4, 0x9ea328), // Verona × Juventus
    (18, 10, 2001, 0x9ea378, 0x9ea334), // Atalanta × Venezia
    (18, 10, 2001, 0x9ea34c, 0x9ea2fc), // Fiorentina × Torino
    (18, 10, 2001, 0x9ea328, 0x9ea30c), // Juventus × Parma
    (18, 10, 2001, 0x9ea304, 0x9ea370), // Lecce × Bologna
    (18, 10, 2001, 0x9ea368, 0x9ea31c), // Milan × Piacenza
    (18, 10, 2001, 0x9ea344, 0x9ea360), // Perugia × Brescia
    (18, 10, 2001, 0x9ea2ec, 0x9ea33c), // Roma × Inter
    (18, 10, 2001, 0x9ea2f4, 0x9ea314), // Udinese × Lazio
    (18, 10, 2001, 0x9ea2e4, 0x9ea358), // Verona × Chievo
    (25, 10, 2001, 0x9ea370, 0x9ea2ec), // Bologna × Roma
    (25, 10, 2001, 0x9ea360, 0x9ea2f4), // Brescia × Udinese
    (25, 10, 2001, 0x9ea358, 0x9ea344), // Chievo × Perugia
    (25, 10, 2001, 0x9ea33c, 0x9ea34c), // Inter × Fiorentina
    (25, 10, 2001, 0x9ea314, 0x9ea328), // Lazio × Juventus
    (25, 10, 2001, 0x9ea30c, 0x9ea368), // Parma × Milan
    (25, 10, 2001, 0x9ea31c, 0x9ea378), // Piacenza × Atalanta
    (25, 10, 2001, 0x9ea2fc, 0x9ea2e4), // Torino × Verona
    (25, 10, 2001, 0x9ea334, 0x9ea304), // Venezia × Lecce
    (2, 11, 2001, 0x9ea378, 0x9ea33c), // Atalanta × Inter
    (2, 11, 2001, 0x9ea370, 0x9ea2fc), // Bologna × Torino
    (2, 11, 2001, 0x9ea34c, 0x9ea31c), // Fiorentina × Piacenza
    (2, 11, 2001, 0x9ea328, 0x9ea344), // Juventus × Perugia
    (2, 11, 2001, 0x9ea304, 0x9ea314), // Lecce × Lazio
    (2, 11, 2001, 0x9ea368, 0x9ea358), // Milan × Chievo
    (2, 11, 2001, 0x9ea2ec, 0x9ea334), // Roma × Venezia
    (2, 11, 2001, 0x9ea2f4, 0x9ea30c), // Udinese × Parma
    (2, 11, 2001, 0x9ea2e4, 0x9ea360), // Verona × Brescia
    (9, 11, 2001, 0x9ea360, 0x9ea33c), // Brescia × Inter
    (9, 11, 2001, 0x9ea358, 0x9ea304), // Chievo × Lecce
    (9, 11, 2001, 0x9ea314, 0x9ea34c), // Lazio × Fiorentina
    (9, 11, 2001, 0x9ea368, 0x9ea328), // Milan × Juventus
    (9, 11, 2001, 0x9ea30c, 0x9ea2ec), // Parma × Roma
    (9, 11, 2001, 0x9ea344, 0x9ea334), // Perugia × Venezia
    (9, 11, 2001, 0x9ea31c, 0x9ea370), // Piacenza × Bologna
    (9, 11, 2001, 0x9ea2fc, 0x9ea378), // Torino × Atalanta
    (9, 11, 2001, 0x9ea2f4, 0x9ea2e4), // Udinese × Verona
    (16, 11, 2001, 0x9ea378, 0x9ea30c), // Atalanta × Parma
    (16, 11, 2001, 0x9ea370, 0x9ea344), // Bologna × Perugia
    (16, 11, 2001, 0x9ea34c, 0x9ea360), // Fiorentina × Brescia
    (16, 11, 2001, 0x9ea33c, 0x9ea358), // Inter × Chievo
    (16, 11, 2001, 0x9ea328, 0x9ea31c), // Juventus × Piacenza
    (16, 11, 2001, 0x9ea304, 0x9ea2fc), // Lecce × Torino
    (16, 11, 2001, 0x9ea2ec, 0x9ea368), // Roma × Milan
    (16, 11, 2001, 0x9ea334, 0x9ea2f4), // Venezia × Udinese
    (16, 11, 2001, 0x9ea2e4, 0x9ea314), // Verona × Lazio
    (6, 0, 2002, 0x9ea360, 0x9ea328), // Brescia × Juventus
    (6, 0, 2002, 0x9ea358, 0x9ea2ec), // Chievo × Roma
    (6, 0, 2002, 0x9ea314, 0x9ea370), // Lazio × Bologna
    (6, 0, 2002, 0x9ea368, 0x9ea2e4), // Milan × Verona
    (6, 0, 2002, 0x9ea30c, 0x9ea34c), // Parma × Fiorentina
    (6, 0, 2002, 0x9ea344, 0x9ea378), // Perugia × Atalanta
    (6, 0, 2002, 0x9ea31c, 0x9ea33c), // Piacenza × Inter
    (6, 0, 2002, 0x9ea2fc, 0x9ea334), // Torino × Venezia
    (6, 0, 2002, 0x9ea2f4, 0x9ea304), // Udinese × Lecce
    (13, 0, 2002, 0x9ea378, 0x9ea358), // Atalanta × Chievo
    (13, 0, 2002, 0x9ea370, 0x9ea360), // Bologna × Brescia
    (13, 0, 2002, 0x9ea34c, 0x9ea344), // Fiorentina × Perugia
    (13, 0, 2002, 0x9ea33c, 0x9ea314), // Inter × Lazio
    (13, 0, 2002, 0x9ea328, 0x9ea2f4), // Juventus × Udinese
    (13, 0, 2002, 0x9ea304, 0x9ea368), // Lecce × Milan
    (13, 0, 2002, 0x9ea2ec, 0x9ea2fc), // Roma × Torino
    (13, 0, 2002, 0x9ea334, 0x9ea30c), // Venezia × Parma
    (13, 0, 2002, 0x9ea2e4, 0x9ea31c), // Verona × Piacenza
    (20, 0, 2002, 0x9ea378, 0x9ea370), // Atalanta × Bologna
    (20, 0, 2002, 0x9ea368, 0x9ea360), // Milan × Brescia
    (20, 0, 2002, 0x9ea358, 0x9ea34c), // Chievo × Fiorentina
    (20, 0, 2002, 0x9ea344, 0x9ea33c), // Perugia × Inter
    (20, 0, 2002, 0x9ea334, 0x9ea328), // Venezia × Juventus
    (20, 0, 2002, 0x9ea31c, 0x9ea314), // Piacenza × Lazio
    (20, 0, 2002, 0x9ea30c, 0x9ea304), // Parma × Lecce
    (20, 0, 2002, 0x9ea2fc, 0x9ea2f4), // Torino × Udinese
    (20, 0, 2002, 0x9ea2ec, 0x9ea2e4), // Roma × Verona
    (27, 0, 2002, 0x9ea328, 0x9ea378), // Juventus × Atalanta
    (27, 0, 2002, 0x9ea370, 0x9ea358), // Bologna × Chievo
    (27, 0, 2002, 0x9ea34c, 0x9ea368), // Fiorentina × Milan
    (27, 0, 2002, 0x9ea33c, 0x9ea30c), // Inter × Parma
    (27, 0, 2002, 0x9ea314, 0x9ea344), // Lazio × Perugia
    (27, 0, 2002, 0x9ea304, 0x9ea31c), // Lecce × Piacenza
    (27, 0, 2002, 0x9ea2f4, 0x9ea2ec), // Udinese × Roma
    (27, 0, 2002, 0x9ea360, 0x9ea2fc), // Brescia × Torino
    (27, 0, 2002, 0x9ea2e4, 0x9ea334), // Verona × Venezia
    (3, 1, 2002, 0x9ea30c, 0x9ea370), // Parma × Bologna
    (3, 1, 2002, 0x9ea304, 0x9ea360), // Lecce × Brescia
    (3, 1, 2002, 0x9ea378, 0x9ea34c), // Atalanta × Fiorentina
    (3, 1, 2002, 0x9ea334, 0x9ea33c), // Venezia × Inter
    (3, 1, 2002, 0x9ea358, 0x9ea328), // Chievo × Juventus
    (3, 1, 2002, 0x9ea2fc, 0x9ea314), // Torino × Lazio
    (3, 1, 2002, 0x9ea2ec, 0x9ea31c), // Roma × Piacenza
    (3, 1, 2002, 0x9ea368, 0x9ea2f4), // Milan × Udinese
    (3, 1, 2002, 0x9ea344, 0x9ea2e4), // Perugia × Verona
    (10, 1, 2002, 0x9ea2e4, 0x9ea378), // Verona × Atalanta
    (10, 1, 2002, 0x9ea31c, 0x9ea358), // Piacenza × Chievo
    (10, 1, 2002, 0x9ea328, 0x9ea304), // Juventus × Lecce
    (10, 1, 2002, 0x9ea314, 0x9ea368), // Lazio × Milan
    (10, 1, 2002, 0x9ea360, 0x9ea30c), // Brescia × Parma
    (10, 1, 2002, 0x9ea2f4, 0x9ea344), // Udinese × Perugia
    (10, 1, 2002, 0x9ea34c, 0x9ea2ec), // Fiorentina × Roma
    (10, 1, 2002, 0x9ea33c, 0x9ea2fc), // Inter × Torino
    (10, 1, 2002, 0x9ea370, 0x9ea334), // Bologna × Venezia
    (17, 1, 2002, 0x9ea378, 0x9ea360), // Atalanta × Brescia
    (17, 1, 2002, 0x9ea334, 0x9ea34c), // Venezia × Fiorentina
    (17, 1, 2002, 0x9ea370, 0x9ea33c), // Bologna × Inter
    (17, 1, 2002, 0x9ea2ec, 0x9ea328), // Roma × Juventus
    (17, 1, 2002, 0x9ea30c, 0x9ea314), // Parma × Lazio
    (17, 1, 2002, 0x9ea368, 0x9ea344), // Milan × Perugia
    (17, 1, 2002, 0x9ea2fc, 0x9ea31c), // Torino × Piacenza
    (17, 1, 2002, 0x9ea358, 0x9ea2f4), // Chievo × Udinese
    (17, 1, 2002, 0x9ea304, 0x9ea2e4), // Lecce × Verona
    (24, 1, 2002, 0x9ea368, 0x9ea378), // Milan × Atalanta
    (24, 1, 2002, 0x9ea2f4, 0x9ea370), // Udinese × Bologna
    (24, 1, 2002, 0x9ea314, 0x9ea358), // Lazio × Chievo
    (24, 1, 2002, 0x9ea328, 0x9ea34c), // Juventus × Fiorentina
    (24, 1, 2002, 0x9ea2e4, 0x9ea33c), // Verona × Inter
    (24, 1, 2002, 0x9ea344, 0x9ea304), // Perugia × Lecce
    (24, 1, 2002, 0x9ea360, 0x9ea2ec), // Brescia × Roma
    (24, 1, 2002, 0x9ea30c, 0x9ea2fc), // Parma × Torino
    (24, 1, 2002, 0x9ea31c, 0x9ea334), // Piacenza × Venezia
    (3, 2, 2002, 0x9ea358, 0x9ea360), // Chievo × Brescia
    (3, 2, 2002, 0x9ea2fc, 0x9ea328), // Torino × Juventus
    (3, 2, 2002, 0x9ea378, 0x9ea314), // Atalanta × Lazio
    (3, 2, 2002, 0x9ea34c, 0x9ea304), // Fiorentina × Lecce
    (3, 2, 2002, 0x9ea334, 0x9ea368), // Venezia × Milan
    (3, 2, 2002, 0x9ea31c, 0x9ea30c), // Piacenza × Parma
    (3, 2, 2002, 0x9ea2ec, 0x9ea344), // Roma × Perugia
    (3, 2, 2002, 0x9ea33c, 0x9ea2f4), // Inter × Udinese
    (3, 2, 2002, 0x9ea370, 0x9ea2e4), // Bologna × Verona
    (10, 2, 2002, 0x9ea2f4, 0x9ea378), // Udinese × Atalanta
    (10, 2, 2002, 0x9ea328, 0x9ea370), // Juventus × Bologna
    (10, 2, 2002, 0x9ea30c, 0x9ea358), // Parma × Chievo
    (10, 2, 2002, 0x9ea2e4, 0x9ea34c), // Verona × Fiorentina
    (10, 2, 2002, 0x9ea368, 0x9ea33c), // Milan × Inter
    (10, 2, 2002, 0x9ea360, 0x9ea31c), // Brescia × Piacenza
    (10, 2, 2002, 0x9ea304, 0x9ea2ec), // Lecce × Roma
    (10, 2, 2002, 0x9ea344, 0x9ea2fc), // Perugia × Torino
    (10, 2, 2002, 0x9ea314, 0x9ea334), // Lazio × Venezia
    (17, 2, 2002, 0x9ea334, 0x9ea360), // Venezia × Brescia
    (17, 2, 2002, 0x9ea2fc, 0x9ea358), // Torino × Chievo
    (17, 2, 2002, 0x9ea33c, 0x9ea328), // Inter × Juventus
    (17, 2, 2002, 0x9ea378, 0x9ea304), // Atalanta × Lecce
    (17, 2, 2002, 0x9ea370, 0x9ea368), // Bologna × Milan
    (17, 2, 2002, 0x9ea2e4, 0x9ea30c), // Verona × Parma
    (17, 2, 2002, 0x9ea31c, 0x9ea344), // Piacenza × Perugia
    (17, 2, 2002, 0x9ea314, 0x9ea2ec), // Lazio × Roma
    (17, 2, 2002, 0x9ea34c, 0x9ea2f4), // Fiorentina × Udinese
    (24, 2, 2002, 0x9ea2ec, 0x9ea378), // Roma × Atalanta
    (24, 2, 2002, 0x9ea34c, 0x9ea370), // Fiorentina × Bologna
    (24, 2, 2002, 0x9ea304, 0x9ea33c), // Lecce × Inter
    (24, 2, 2002, 0x9ea360, 0x9ea314), // Brescia × Lazio
    (24, 2, 2002, 0x9ea344, 0x9ea30c), // Perugia × Parma
    (24, 2, 2002, 0x9ea2f4, 0x9ea31c), // Udinese × Piacenza
    (24, 2, 2002, 0x9ea368, 0x9ea2fc), // Milan × Torino
    (24, 2, 2002, 0x9ea358, 0x9ea334), // Chievo × Venezia
    (24, 2, 2002, 0x9ea328, 0x9ea2e4), // Juventus × Verona
    (30, 2, 2002, 0x9ea334, 0x9ea378), // Venezia × Atalanta
    (30, 2, 2002, 0x9ea2fc, 0x9ea34c), // Torino × Fiorentina
    (30, 2, 2002, 0x9ea30c, 0x9ea328), // Parma × Juventus
    (30, 2, 2002, 0x9ea370, 0x9ea304), // Bologna × Lecce
    (30, 2, 2002, 0x9ea31c, 0x9ea368), // Piacenza × Milan
    (30, 2, 2002, 0x9ea360, 0x9ea344), // Brescia × Perugia
    (30, 2, 2002, 0x9ea33c, 0x9ea2ec), // Inter × Roma
    (30, 2, 2002, 0x9ea314, 0x9ea2f4), // Lazio × Udinese
    (30, 2, 2002, 0x9ea358, 0x9ea2e4), // Chievo × Verona
    (7, 3, 2002, 0x9ea2ec, 0x9ea370), // Roma × Bologna
    (7, 3, 2002, 0x9ea2f4, 0x9ea360), // Udinese × Brescia
    (7, 3, 2002, 0x9ea344, 0x9ea358), // Perugia × Chievo
    (7, 3, 2002, 0x9ea34c, 0x9ea33c), // Fiorentina × Inter
    (7, 3, 2002, 0x9ea328, 0x9ea314), // Juventus × Lazio
    (7, 3, 2002, 0x9ea368, 0x9ea30c), // Milan × Parma
    (7, 3, 2002, 0x9ea378, 0x9ea31c), // Atalanta × Piacenza
    (7, 3, 2002, 0x9ea2e4, 0x9ea2fc), // Verona × Torino
    (7, 3, 2002, 0x9ea304, 0x9ea334), // Lecce × Venezia
    (14, 3, 2002, 0x9ea33c, 0x9ea378), // Inter × Atalanta
    (14, 3, 2002, 0x9ea2fc, 0x9ea370), // Torino × Bologna
    (14, 3, 2002, 0x9ea31c, 0x9ea34c), // Piacenza × Fiorentina
    (14, 3, 2002, 0x9ea344, 0x9ea328), // Perugia × Juventus
    (14, 3, 2002, 0x9ea314, 0x9ea304), // Lazio × Lecce
    (14, 3, 2002, 0x9ea358, 0x9ea368), // Chievo × Milan
    (14, 3, 2002, 0x9ea334, 0x9ea2ec), // Venezia × Roma
    (14, 3, 2002, 0x9ea30c, 0x9ea2f4), // Parma × Udinese
    (14, 3, 2002, 0x9ea360, 0x9ea2e4), // Brescia × Verona
    (21, 3, 2002, 0x9ea33c, 0x9ea360), // Inter × Brescia
    (21, 3, 2002, 0x9ea304, 0x9ea358), // Lecce × Chievo
    (21, 3, 2002, 0x9ea34c, 0x9ea314), // Fiorentina × Lazio
    (21, 3, 2002, 0x9ea328, 0x9ea368), // Juventus × Milan
    (21, 3, 2002, 0x9ea2ec, 0x9ea30c), // Roma × Parma
    (21, 3, 2002, 0x9ea334, 0x9ea344), // Venezia × Perugia
    (21, 3, 2002, 0x9ea370, 0x9ea31c), // Bologna × Piacenza
    (21, 3, 2002, 0x9ea378, 0x9ea2fc), // Atalanta × Torino
    (21, 3, 2002, 0x9ea2e4, 0x9ea2f4), // Verona × Udinese
    (28, 3, 2002, 0x9ea30c, 0x9ea378), // Parma × Atalanta
    (28, 3, 2002, 0x9ea344, 0x9ea370), // Perugia × Bologna
    (28, 3, 2002, 0x9ea360, 0x9ea34c), // Brescia × Fiorentina
    (28, 3, 2002, 0x9ea358, 0x9ea33c), // Chievo × Inter
    (28, 3, 2002, 0x9ea31c, 0x9ea328), // Piacenza × Juventus
    (28, 3, 2002, 0x9ea2fc, 0x9ea304), // Torino × Lecce
    (28, 3, 2002, 0x9ea368, 0x9ea2ec), // Milan × Roma
    (28, 3, 2002, 0x9ea2f4, 0x9ea334), // Udinese × Venezia
    (28, 3, 2002, 0x9ea314, 0x9ea2e4), // Lazio × Verona
    (5, 4, 2002, 0x9ea328, 0x9ea360), // Juventus × Brescia
    (5, 4, 2002, 0x9ea2ec, 0x9ea358), // Roma × Chievo
    (5, 4, 2002, 0x9ea370, 0x9ea314), // Bologna × Lazio
    (5, 4, 2002, 0x9ea2e4, 0x9ea368), // Verona × Milan
    (5, 4, 2002, 0x9ea34c, 0x9ea30c), // Fiorentina × Parma
    (5, 4, 2002, 0x9ea378, 0x9ea344), // Atalanta × Perugia
    (5, 4, 2002, 0x9ea33c, 0x9ea31c), // Inter × Piacenza
    (5, 4, 2002, 0x9ea334, 0x9ea2fc), // Venezia × Torino
    (5, 4, 2002, 0x9ea304, 0x9ea2f4), // Lecce × Udinese
    (12, 4, 2002, 0x9ea358, 0x9ea378), // Chievo × Atalanta
    (12, 4, 2002, 0x9ea360, 0x9ea370), // Brescia × Bologna
    (12, 4, 2002, 0x9ea344, 0x9ea34c), // Perugia × Fiorentina
    (12, 4, 2002, 0x9ea314, 0x9ea33c), // Lazio × Inter
    (12, 4, 2002, 0x9ea2f4, 0x9ea328), // Udinese × Juventus
    (12, 4, 2002, 0x9ea368, 0x9ea304), // Milan × Lecce
    (12, 4, 2002, 0x9ea2fc, 0x9ea2ec), // Torino × Roma
    (12, 4, 2002, 0x9ea30c, 0x9ea334), // Parma × Venezia
    (12, 4, 2002, 0x9ea31c, 0x9ea2e4), // Piacenza × Verona
];

const CQN5_SERIE_B: [(u8, u8, u16, u32, u32); 380] = [
    (26, 7, 2001, 0x9ea464, 0x9ea46c), // Ancona × Reggina
    (26, 7, 2001, 0x9ea450, 0x9ea45c), // Cagliari × Messina
    (26, 7, 2001, 0x9ea440, 0x9ea448), // Como × Crotone
    (26, 7, 2001, 0x9ea42c, 0x9ea434), // Cosenza × Cittadella
    (26, 7, 2001, 0x9ea41c, 0x9ea424), // Empoli × Palermo
    (26, 7, 2001, 0x9ea40c, 0x9ea414), // Genoa × Napoli
    (26, 7, 2001, 0x9ea3fc, 0x9ea404), // Modena × Bari
    (26, 7, 2001, 0x9ea3e4, 0x9ea3f0), // Salernitana × Sampdoria
    (26, 7, 2001, 0x9ea3d4, 0x9ea3dc), // Ternana × Siena
    (26, 7, 2001, 0x9ea3c0, 0x9ea3c8), // Vicenza × Pistoiese
    (2, 8, 2001, 0x9ea404, 0x9ea41c), // Bari × Empoli
    (2, 8, 2001, 0x9ea434, 0x9ea3e4), // Cittadella × Salernitana
    (2, 8, 2001, 0x9ea448, 0x9ea3fc), // Crotone × Modena
    (2, 8, 2001, 0x9ea45c, 0x9ea3d4), // Messina × Ternana
    (2, 8, 2001, 0x9ea414, 0x9ea464), // Napoli × Ancona
    (2, 8, 2001, 0x9ea424, 0x9ea450), // Palermo × Cagliari
    (2, 8, 2001, 0x9ea3c8, 0x9ea40c), // Pistoiese × Genoa
    (2, 8, 2001, 0x9ea46c, 0x9ea440), // Reggina × Como
    (2, 8, 2001, 0x9ea3f0, 0x9ea42c), // Sampdoria × Cosenza
    (2, 8, 2001, 0x9ea3dc, 0x9ea3c0), // Siena × Vicenza
    (9, 8, 2001, 0x9ea464, 0x9ea3dc), // Ancona × Siena
    (9, 8, 2001, 0x9ea450, 0x9ea404), // Cagliari × Bari
    (9, 8, 2001, 0x9ea42c, 0x9ea3d4), // Cosenza × Ternana
    (9, 8, 2001, 0x9ea41c, 0x9ea45c), // Empoli × Messina
    (9, 8, 2001, 0x9ea40c, 0x9ea440), // Genoa × Como
    (9, 8, 2001, 0x9ea3fc, 0x9ea414), // Modena × Napoli
    (9, 8, 2001, 0x9ea424, 0x9ea3c8), // Palermo × Pistoiese
    (9, 8, 2001, 0x9ea46c, 0x9ea3f0), // Reggina × Sampdoria
    (9, 8, 2001, 0x9ea3e4, 0x9ea448), // Salernitana × Crotone
    (9, 8, 2001, 0x9ea3c0, 0x9ea434), // Vicenza × Cittadella
    (16, 8, 2001, 0x9ea404, 0x9ea424), // Bari × Palermo
    (16, 8, 2001, 0x9ea434, 0x9ea46c), // Cittadella × Reggina
    (16, 8, 2001, 0x9ea440, 0x9ea3e4), // Como × Salernitana
    (16, 8, 2001, 0x9ea448, 0x9ea450), // Crotone × Cagliari
    (16, 8, 2001, 0x9ea45c, 0x9ea464), // Messina × Ancona
    (16, 8, 2001, 0x9ea414, 0x9ea3c0), // Napoli × Vicenza
    (16, 8, 2001, 0x9ea3c8, 0x9ea42c), // Pistoiese × Cosenza
    (16, 8, 2001, 0x9ea3f0, 0x9ea41c), // Sampdoria × Empoli
    (16, 8, 2001, 0x9ea3dc, 0x9ea40c), // Siena × Genoa
    (16, 8, 2001, 0x9ea3d4, 0x9ea3fc), // Ternana × Modena
    (23, 8, 2001, 0x9ea464, 0x9ea448), // Ancona × Crotone
    (23, 8, 2001, 0x9ea450, 0x9ea3f0), // Cagliari × Sampdoria
    (23, 8, 2001, 0x9ea42c, 0x9ea440), // Cosenza × Como
    (23, 8, 2001, 0x9ea41c, 0x9ea414), // Empoli × Napoli
    (23, 8, 2001, 0x9ea40c, 0x9ea45c), // Genoa × Messina
    (23, 8, 2001, 0x9ea3fc, 0x9ea3dc), // Modena × Siena
    (23, 8, 2001, 0x9ea424, 0x9ea434), // Palermo × Cittadella
    (23, 8, 2001, 0x9ea46c, 0x9ea404), // Reggina × Bari
    (23, 8, 2001, 0x9ea3e4, 0x9ea3c8), // Salernitana × Pistoiese
    (23, 8, 2001, 0x9ea3c0, 0x9ea3d4), // Vicenza × Ternana
    (30, 8, 2001, 0x9ea404, 0x9ea448), // Bari × Crotone
    (30, 8, 2001, 0x9ea434, 0x9ea41c), // Cittadella × Empoli
    (30, 8, 2001, 0x9ea440, 0x9ea464), // Como × Ancona
    (30, 8, 2001, 0x9ea42c, 0x9ea46c), // Cosenza × Reggina
    (30, 8, 2001, 0x9ea45c, 0x9ea3c0), // Messina × Vicenza
    (30, 8, 2001, 0x9ea414, 0x9ea450), // Napoli × Cagliari
    (30, 8, 2001, 0x9ea3c8, 0x9ea3fc), // Pistoiese × Modena
    (30, 8, 2001, 0x9ea3f0, 0x9ea424), // Sampdoria × Palermo
    (30, 8, 2001, 0x9ea3dc, 0x9ea3e4), // Siena × Salernitana
    (30, 8, 2001, 0x9ea3d4, 0x9ea40c), // Ternana × Genoa
    (7, 9, 2001, 0x9ea464, 0x9ea40c), // Ancona × Genoa
    (7, 9, 2001, 0x9ea450, 0x9ea3dc), // Cagliari × Siena
    (7, 9, 2001, 0x9ea448, 0x9ea434), // Crotone × Cittadella
    (7, 9, 2001, 0x9ea41c, 0x9ea3c8), // Empoli × Pistoiese
    (7, 9, 2001, 0x9ea3fc, 0x9ea440), // Modena × Como
    (7, 9, 2001, 0x9ea46c, 0x9ea424), // Reggina × Palermo
    (7, 9, 2001, 0x9ea3e4, 0x9ea45c), // Salernitana × Messina
    (7, 9, 2001, 0x9ea3f0, 0x9ea404), // Sampdoria × Bari
    (7, 9, 2001, 0x9ea3d4, 0x9ea414), // Ternana × Napoli
    (7, 9, 2001, 0x9ea3c0, 0x9ea42c), // Vicenza × Cosenza
    (10, 9, 2001, 0x9ea464, 0x9ea3f0), // Ancona × Sampdoria
    (10, 9, 2001, 0x9ea440, 0x9ea3d4), // Como × Ternana
    (10, 9, 2001, 0x9ea42c, 0x9ea3fc), // Cosenza × Modena
    (10, 9, 2001, 0x9ea41c, 0x9ea450), // Empoli × Cagliari
    (10, 9, 2001, 0x9ea40c, 0x9ea434), // Genoa × Cittadella
    (10, 9, 2001, 0x9ea45c, 0x9ea3dc), // Messina × Siena
    (10, 9, 2001, 0x9ea414, 0x9ea3e4), // Napoli × Salernitana
    (10, 9, 2001, 0x9ea424, 0x9ea448), // Palermo × Crotone
    (10, 9, 2001, 0x9ea3c8, 0x9ea46c), // Pistoiese × Reggina
    (10, 9, 2001, 0x9ea3c0, 0x9ea404), // Vicenza × Bari
    (14, 9, 2001, 0x9ea404, 0x9ea40c), // Bari × Genoa
    (14, 9, 2001, 0x9ea434, 0x9ea414), // Cittadella × Napoli
    (14, 9, 2001, 0x9ea448, 0x9ea3c0), // Crotone × Vicenza
    (14, 9, 2001, 0x9ea45c, 0x9ea424), // Messina × Palermo
    (14, 9, 2001, 0x9ea3fc, 0x9ea450), // Modena × Cagliari
    (14, 9, 2001, 0x9ea46c, 0x9ea41c), // Reggina × Empoli
    (14, 9, 2001, 0x9ea3e4, 0x9ea42c), // Salernitana × Cosenza
    (14, 9, 2001, 0x9ea3f0, 0x9ea3c8), // Sampdoria × Pistoiese
    (14, 9, 2001, 0x9ea3dc, 0x9ea440), // Siena × Como
    (14, 9, 2001, 0x9ea3d4, 0x9ea464), // Ternana × Ancona
    (28, 9, 2001, 0x9ea464, 0x9ea434), // Ancona × Cittadella
    (28, 9, 2001, 0x9ea450, 0x9ea46c), // Cagliari × Reggina
    (28, 9, 2001, 0x9ea440, 0x9ea45c), // Como × Messina
    (28, 9, 2001, 0x9ea42c, 0x9ea3dc), // Cosenza × Siena
    (28, 9, 2001, 0x9ea41c, 0x9ea448), // Empoli × Crotone
    (28, 9, 2001, 0x9ea40c, 0x9ea3e4), // Genoa × Salernitana
    (28, 9, 2001, 0x9ea414, 0x9ea3f0), // Napoli × Sampdoria
    (28, 9, 2001, 0x9ea424, 0x9ea3d4), // Palermo × Ternana
    (28, 9, 2001, 0x9ea3c8, 0x9ea404), // Pistoiese × Bari
    (28, 9, 2001, 0x9ea3c0, 0x9ea3fc), // Vicenza × Modena
    (4, 10, 2001, 0x9ea404, 0x9ea414), // Bari × Napoli
    (4, 10, 2001, 0x9ea434, 0x9ea450), // Cittadella × Cagliari
    (4, 10, 2001, 0x9ea440, 0x9ea3c0), // Como × Vicenza
    (4, 10, 2001, 0x9ea448, 0x9ea46c), // Crotone × Reggina
    (4, 10, 2001, 0x9ea40c, 0x9ea3f0), // Genoa × Sampdoria
    (4, 10, 2001, 0x9ea45c, 0x9ea42c), // Messina × Cosenza
    (4, 10, 2001, 0x9ea3fc, 0x9ea41c), // Modena × Empoli
    (4, 10, 2001, 0x9ea3e4, 0x9ea464), // Salernitana × Ancona
    (4, 10, 2001, 0x9ea3dc, 0x9ea424), // Siena × Palermo
    (4, 10, 2001, 0x9ea3d4, 0x9ea3c8), // Ternana × Pistoiese
    (11, 10, 2001, 0x9ea404, 0x9ea3dc), // Bari × Siena
    (11, 10, 2001, 0x9ea450, 0x9ea3d4), // Cagliari × Ternana
    (11, 10, 2001, 0x9ea42c, 0x9ea40c), // Cosenza × Genoa
    (11, 10, 2001, 0x9ea41c, 0x9ea3e4), // Empoli × Salernitana
    (11, 10, 2001, 0x9ea414, 0x9ea448), // Napoli × Crotone
    (11, 10, 2001, 0x9ea424, 0x9ea3fc), // Palermo × Modena
    (11, 10, 2001, 0x9ea3c8, 0x9ea440), // Pistoiese × Como
    (11, 10, 2001, 0x9ea46c, 0x9ea45c), // Reggina × Messina
    (11, 10, 2001, 0x9ea3f0, 0x9ea434), // Sampdoria × Cittadella
    (11, 10, 2001, 0x9ea3c0, 0x9ea464), // Vicenza × Ancona
    (18, 10, 2001, 0x9ea464, 0x9ea42c), // Ancona × Cosenza
    (18, 10, 2001, 0x9ea434, 0x9ea404), // Cittadella × Bari
    (18, 10, 2001, 0x9ea440, 0x9ea424), // Como × Palermo
    (18, 10, 2001, 0x9ea448, 0x9ea3f0), // Crotone × Sampdoria
    (18, 10, 2001, 0x9ea40c, 0x9ea3c0), // Genoa × Vicenza
    (18, 10, 2001, 0x9ea45c, 0x9ea414), // Messina × Napoli
    (18, 10, 2001, 0x9ea3fc, 0x9ea46c), // Modena × Reggina
    (18, 10, 2001, 0x9ea3e4, 0x9ea450), // Salernitana × Cagliari
    (18, 10, 2001, 0x9ea3dc, 0x9ea3c8), // Siena × Pistoiese
    (18, 10, 2001, 0x9ea3d4, 0x9ea41c), // Ternana × Empoli
    (25, 10, 2001, 0x9ea404, 0x9ea440), // Bari × Como
    (25, 10, 2001, 0x9ea450, 0x9ea464), // Cagliari × Ancona
    (25, 10, 2001, 0x9ea434, 0x9ea3d4), // Cittadella × Ternana
    (25, 10, 2001, 0x9ea448, 0x9ea42c), // Crotone × Cosenza
    (25, 10, 2001, 0x9ea41c, 0x9ea3dc), // Empoli × Siena
    (25, 10, 2001, 0x9ea424, 0x9ea40c), // Palermo × Genoa
    (25, 10, 2001, 0x9ea3c8, 0x9ea45c), // Pistoiese × Messina
    (25, 10, 2001, 0x9ea46c, 0x9ea414), // Reggina × Napoli
    (25, 10, 2001, 0x9ea3e4, 0x9ea3c0), // Salernitana × Vicenza
    (25, 10, 2001, 0x9ea3f0, 0x9ea3fc), // Sampdoria × Modena
    (2, 11, 2001, 0x9ea464, 0x9ea3c8), // Ancona × Pistoiese
    (2, 11, 2001, 0x9ea440, 0x9ea450), // Como × Cagliari
    (2, 11, 2001, 0x9ea42c, 0x9ea404), // Cosenza × Bari
    (2, 11, 2001, 0x9ea40c, 0x9ea448), // Genoa × Crotone
    (2, 11, 2001, 0x9ea45c, 0x9ea3f0), // Messina × Sampdoria
    (2, 11, 2001, 0x9ea3fc, 0x9ea434), // Modena × Cittadella
    (2, 11, 2001, 0x9ea414, 0x9ea424), // Napoli × Palermo
    (2, 11, 2001, 0x9ea3dc, 0x9ea46c), // Siena × Reggina
    (2, 11, 2001, 0x9ea3d4, 0x9ea3e4), // Ternana × Salernitana
    (2, 11, 2001, 0x9ea3c0, 0x9ea41c), // Vicenza × Empoli
    (9, 11, 2001, 0x9ea404, 0x9ea464), // Bari × Ancona
    (9, 11, 2001, 0x9ea450, 0x9ea3c8), // Cagliari × Pistoiese
    (9, 11, 2001, 0x9ea434, 0x9ea45c), // Cittadella × Messina
    (9, 11, 2001, 0x9ea448, 0x9ea3dc), // Crotone × Siena
    (9, 11, 2001, 0x9ea41c, 0x9ea440), // Empoli × Como
    (9, 11, 2001, 0x9ea3fc, 0x9ea40c), // Modena × Genoa
    (9, 11, 2001, 0x9ea414, 0x9ea42c), // Napoli × Cosenza
    (9, 11, 2001, 0x9ea424, 0x9ea3c0), // Palermo × Vicenza
    (9, 11, 2001, 0x9ea46c, 0x9ea3e4), // Reggina × Salernitana
    (9, 11, 2001, 0x9ea3f0, 0x9ea3d4), // Sampdoria × Ternana
    (16, 11, 2001, 0x9ea464, 0x9ea3fc), // Ancona × Modena
    (16, 11, 2001, 0x9ea440, 0x9ea3f0), // Como × Sampdoria
    (16, 11, 2001, 0x9ea42c, 0x9ea450), // Cosenza × Cagliari
    (16, 11, 2001, 0x9ea40c, 0x9ea41c), // Genoa × Empoli
    (16, 11, 2001, 0x9ea45c, 0x9ea404), // Messina × Bari
    (16, 11, 2001, 0x9ea3c8, 0x9ea414), // Pistoiese × Napoli
    (16, 11, 2001, 0x9ea3e4, 0x9ea424), // Salernitana × Palermo
    (16, 11, 2001, 0x9ea3dc, 0x9ea434), // Siena × Cittadella
    (16, 11, 2001, 0x9ea3d4, 0x9ea448), // Ternana × Crotone
    (16, 11, 2001, 0x9ea3c0, 0x9ea46c), // Vicenza × Reggina
    (23, 11, 2001, 0x9ea404, 0x9ea3e4), // Bari × Salernitana
    (23, 11, 2001, 0x9ea450, 0x9ea40c), // Cagliari × Genoa
    (23, 11, 2001, 0x9ea434, 0x9ea440), // Cittadella × Como
    (23, 11, 2001, 0x9ea448, 0x9ea3c8), // Crotone × Pistoiese
    (23, 11, 2001, 0x9ea41c, 0x9ea464), // Empoli × Ancona
    (23, 11, 2001, 0x9ea3fc, 0x9ea45c), // Modena × Messina
    (23, 11, 2001, 0x9ea414, 0x9ea3dc), // Napoli × Siena
    (23, 11, 2001, 0x9ea424, 0x9ea42c), // Palermo × Cosenza
    (23, 11, 2001, 0x9ea46c, 0x9ea3d4), // Reggina × Ternana
    (23, 11, 2001, 0x9ea3f0, 0x9ea3c0), // Sampdoria × Vicenza
    (6, 0, 2002, 0x9ea464, 0x9ea424), // Ancona × Palermo
    (6, 0, 2002, 0x9ea440, 0x9ea414), // Como × Napoli
    (6, 0, 2002, 0x9ea42c, 0x9ea41c), // Cosenza × Empoli
    (6, 0, 2002, 0x9ea40c, 0x9ea46c), // Genoa × Reggina
    (6, 0, 2002, 0x9ea45c, 0x9ea448), // Messina × Crotone
    (6, 0, 2002, 0x9ea3c8, 0x9ea434), // Pistoiese × Cittadella
    (6, 0, 2002, 0x9ea3e4, 0x9ea3fc), // Salernitana × Modena
    (6, 0, 2002, 0x9ea3dc, 0x9ea3f0), // Siena × Sampdoria
    (6, 0, 2002, 0x9ea3d4, 0x9ea404), // Ternana × Bari
    (6, 0, 2002, 0x9ea3c0, 0x9ea450), // Vicenza × Cagliari
    (13, 0, 2002, 0x9ea46c, 0x9ea464), // Reggina × Ancona
    (13, 0, 2002, 0x9ea45c, 0x9ea450), // Messina × Cagliari
    (13, 0, 2002, 0x9ea448, 0x9ea440), // Crotone × Como
    (13, 0, 2002, 0x9ea434, 0x9ea42c), // Cittadella × Cosenza
    (13, 0, 2002, 0x9ea424, 0x9ea41c), // Palermo × Empoli
    (13, 0, 2002, 0x9ea414, 0x9ea40c), // Napoli × Genoa
    (13, 0, 2002, 0x9ea404, 0x9ea3fc), // Bari × Modena
    (13, 0, 2002, 0x9ea3f0, 0x9ea3e4), // Sampdoria × Salernitana
    (13, 0, 2002, 0x9ea3dc, 0x9ea3d4), // Siena × Ternana
    (13, 0, 2002, 0x9ea3c8, 0x9ea3c0), // Pistoiese × Vicenza
    (20, 0, 2002, 0x9ea41c, 0x9ea404), // Empoli × Bari
    (20, 0, 2002, 0x9ea3e4, 0x9ea434), // Salernitana × Cittadella
    (20, 0, 2002, 0x9ea3fc, 0x9ea448), // Modena × Crotone
    (20, 0, 2002, 0x9ea3d4, 0x9ea45c), // Ternana × Messina
    (20, 0, 2002, 0x9ea464, 0x9ea414), // Ancona × Napoli
    (20, 0, 2002, 0x9ea450, 0x9ea424), // Cagliari × Palermo
    (20, 0, 2002, 0x9ea40c, 0x9ea3c8), // Genoa × Pistoiese
    (20, 0, 2002, 0x9ea440, 0x9ea46c), // Como × Reggina
    (20, 0, 2002, 0x9ea42c, 0x9ea3f0), // Cosenza × Sampdoria
    (20, 0, 2002, 0x9ea3c0, 0x9ea3dc), // Vicenza × Siena
    (3, 1, 2002, 0x9ea3dc, 0x9ea464), // Siena × Ancona
    (3, 1, 2002, 0x9ea404, 0x9ea450), // Bari × Cagliari
    (3, 1, 2002, 0x9ea3d4, 0x9ea42c), // Ternana × Cosenza
    (3, 1, 2002, 0x9ea45c, 0x9ea41c), // Messina × Empoli
    (3, 1, 2002, 0x9ea440, 0x9ea40c), // Como × Genoa
    (3, 1, 2002, 0x9ea414, 0x9ea3fc), // Napoli × Modena
    (3, 1, 2002, 0x9ea3c8, 0x9ea424), // Pistoiese × Palermo
    (3, 1, 2002, 0x9ea3f0, 0x9ea46c), // Sampdoria × Reggina
    (3, 1, 2002, 0x9ea448, 0x9ea3e4), // Crotone × Salernitana
    (3, 1, 2002, 0x9ea434, 0x9ea3c0), // Cittadella × Vicenza
    (10, 1, 2002, 0x9ea424, 0x9ea404), // Palermo × Bari
    (10, 1, 2002, 0x9ea46c, 0x9ea434), // Reggina × Cittadella
    (10, 1, 2002, 0x9ea3e4, 0x9ea440), // Salernitana × Como
    (10, 1, 2002, 0x9ea450, 0x9ea448), // Cagliari × Crotone
    (10, 1, 2002, 0x9ea464, 0x9ea45c), // Ancona × Messina
    (10, 1, 2002, 0x9ea3c0, 0x9ea414), // Vicenza × Napoli
    (10, 1, 2002, 0x9ea42c, 0x9ea3c8), // Cosenza × Pistoiese
    (10, 1, 2002, 0x9ea41c, 0x9ea3f0), // Empoli × Sampdoria
    (10, 1, 2002, 0x9ea40c, 0x9ea3dc), // Genoa × Siena
    (10, 1, 2002, 0x9ea3fc, 0x9ea3d4), // Modena × Ternana
    (17, 1, 2002, 0x9ea448, 0x9ea464), // Crotone × Ancona
    (17, 1, 2002, 0x9ea3f0, 0x9ea450), // Sampdoria × Cagliari
    (17, 1, 2002, 0x9ea440, 0x9ea42c), // Como × Cosenza
    (17, 1, 2002, 0x9ea414, 0x9ea41c), // Napoli × Empoli
    (17, 1, 2002, 0x9ea45c, 0x9ea40c), // Messina × Genoa
    (17, 1, 2002, 0x9ea3dc, 0x9ea3fc), // Siena × Modena
    (17, 1, 2002, 0x9ea434, 0x9ea424), // Cittadella × Palermo
    (17, 1, 2002, 0x9ea404, 0x9ea46c), // Bari × Reggina
    (17, 1, 2002, 0x9ea3c8, 0x9ea3e4), // Pistoiese × Salernitana
    (17, 1, 2002, 0x9ea3d4, 0x9ea3c0), // Ternana × Vicenza
    (24, 1, 2002, 0x9ea448, 0x9ea404), // Crotone × Bari
    (24, 1, 2002, 0x9ea41c, 0x9ea434), // Empoli × Cittadella
    (24, 1, 2002, 0x9ea464, 0x9ea440), // Ancona × Como
    (24, 1, 2002, 0x9ea46c, 0x9ea42c), // Reggina × Cosenza
    (24, 1, 2002, 0x9ea3c0, 0x9ea45c), // Vicenza × Messina
    (24, 1, 2002, 0x9ea450, 0x9ea414), // Cagliari × Napoli
    (24, 1, 2002, 0x9ea3fc, 0x9ea3c8), // Modena × Pistoiese
    (24, 1, 2002, 0x9ea424, 0x9ea3f0), // Palermo × Sampdoria
    (24, 1, 2002, 0x9ea3e4, 0x9ea3dc), // Salernitana × Siena
    (24, 1, 2002, 0x9ea40c, 0x9ea3d4), // Genoa × Ternana
    (3, 2, 2002, 0x9ea40c, 0x9ea464), // Genoa × Ancona
    (3, 2, 2002, 0x9ea3dc, 0x9ea450), // Siena × Cagliari
    (3, 2, 2002, 0x9ea434, 0x9ea448), // Cittadella × Crotone
    (3, 2, 2002, 0x9ea3c8, 0x9ea41c), // Pistoiese × Empoli
    (3, 2, 2002, 0x9ea440, 0x9ea3fc), // Como × Modena
    (3, 2, 2002, 0x9ea424, 0x9ea46c), // Palermo × Reggina
    (3, 2, 2002, 0x9ea45c, 0x9ea3e4), // Messina × Salernitana
    (3, 2, 2002, 0x9ea404, 0x9ea3f0), // Bari × Sampdoria
    (3, 2, 2002, 0x9ea414, 0x9ea3d4), // Napoli × Ternana
    (3, 2, 2002, 0x9ea42c, 0x9ea3c0), // Cosenza × Vicenza
    (10, 2, 2002, 0x9ea3f0, 0x9ea464), // Sampdoria × Ancona
    (10, 2, 2002, 0x9ea3d4, 0x9ea440), // Ternana × Como
    (10, 2, 2002, 0x9ea3fc, 0x9ea42c), // Modena × Cosenza
    (10, 2, 2002, 0x9ea450, 0x9ea41c), // Cagliari × Empoli
    (10, 2, 2002, 0x9ea434, 0x9ea40c), // Cittadella × Genoa
    (10, 2, 2002, 0x9ea3dc, 0x9ea45c), // Siena × Messina
    (10, 2, 2002, 0x9ea3e4, 0x9ea414), // Salernitana × Napoli
    (10, 2, 2002, 0x9ea448, 0x9ea424), // Crotone × Palermo
    (10, 2, 2002, 0x9ea46c, 0x9ea3c8), // Reggina × Pistoiese
    (10, 2, 2002, 0x9ea404, 0x9ea3c0), // Bari × Vicenza
    (17, 2, 2002, 0x9ea40c, 0x9ea404), // Genoa × Bari
    (17, 2, 2002, 0x9ea414, 0x9ea434), // Napoli × Cittadella
    (17, 2, 2002, 0x9ea3c0, 0x9ea448), // Vicenza × Crotone
    (17, 2, 2002, 0x9ea424, 0x9ea45c), // Palermo × Messina
    (17, 2, 2002, 0x9ea450, 0x9ea3fc), // Cagliari × Modena
    (17, 2, 2002, 0x9ea41c, 0x9ea46c), // Empoli × Reggina
    (17, 2, 2002, 0x9ea42c, 0x9ea3e4), // Cosenza × Salernitana
    (17, 2, 2002, 0x9ea3c8, 0x9ea3f0), // Pistoiese × Sampdoria
    (17, 2, 2002, 0x9ea440, 0x9ea3dc), // Como × Siena
    (17, 2, 2002, 0x9ea464, 0x9ea3d4), // Ancona × Ternana
    (30, 2, 2002, 0x9ea434, 0x9ea464), // Cittadella × Ancona
    (30, 2, 2002, 0x9ea46c, 0x9ea450), // Reggina × Cagliari
    (30, 2, 2002, 0x9ea45c, 0x9ea440), // Messina × Como
    (30, 2, 2002, 0x9ea3dc, 0x9ea42c), // Siena × Cosenza
    (30, 2, 2002, 0x9ea448, 0x9ea41c), // Crotone × Empoli
    (30, 2, 2002, 0x9ea3e4, 0x9ea40c), // Salernitana × Genoa
    (30, 2, 2002, 0x9ea3f0, 0x9ea414), // Sampdoria × Napoli
    (30, 2, 2002, 0x9ea3d4, 0x9ea424), // Ternana × Palermo
    (30, 2, 2002, 0x9ea404, 0x9ea3c8), // Bari × Pistoiese
    (30, 2, 2002, 0x9ea3fc, 0x9ea3c0), // Modena × Vicenza
    (7, 3, 2002, 0x9ea414, 0x9ea404), // Napoli × Bari
    (7, 3, 2002, 0x9ea450, 0x9ea434), // Cagliari × Cittadella
    (7, 3, 2002, 0x9ea3c0, 0x9ea440), // Vicenza × Como
    (7, 3, 2002, 0x9ea46c, 0x9ea448), // Reggina × Crotone
    (7, 3, 2002, 0x9ea3f0, 0x9ea40c), // Sampdoria × Genoa
    (7, 3, 2002, 0x9ea42c, 0x9ea45c), // Cosenza × Messina
    (7, 3, 2002, 0x9ea41c, 0x9ea3fc), // Empoli × Modena
    (7, 3, 2002, 0x9ea464, 0x9ea3e4), // Ancona × Salernitana
    (7, 3, 2002, 0x9ea424, 0x9ea3dc), // Palermo × Siena
    (7, 3, 2002, 0x9ea3c8, 0x9ea3d4), // Pistoiese × Ternana
    (14, 3, 2002, 0x9ea3dc, 0x9ea404), // Siena × Bari
    (14, 3, 2002, 0x9ea3d4, 0x9ea450), // Ternana × Cagliari
    (14, 3, 2002, 0x9ea40c, 0x9ea42c), // Genoa × Cosenza
    (14, 3, 2002, 0x9ea3e4, 0x9ea41c), // Salernitana × Empoli
    (14, 3, 2002, 0x9ea448, 0x9ea414), // Crotone × Napoli
    (14, 3, 2002, 0x9ea3fc, 0x9ea424), // Modena × Palermo
    (14, 3, 2002, 0x9ea440, 0x9ea3c8), // Como × Pistoiese
    (14, 3, 2002, 0x9ea45c, 0x9ea46c), // Messina × Reggina
    (14, 3, 2002, 0x9ea434, 0x9ea3f0), // Cittadella × Sampdoria
    (14, 3, 2002, 0x9ea464, 0x9ea3c0), // Ancona × Vicenza
    (21, 3, 2002, 0x9ea42c, 0x9ea464), // Cosenza × Ancona
    (21, 3, 2002, 0x9ea404, 0x9ea434), // Bari × Cittadella
    (21, 3, 2002, 0x9ea424, 0x9ea440), // Palermo × Como
    (21, 3, 2002, 0x9ea3f0, 0x9ea448), // Sampdoria × Crotone
    (21, 3, 2002, 0x9ea3c0, 0x9ea40c), // Vicenza × Genoa
    (21, 3, 2002, 0x9ea414, 0x9ea45c), // Napoli × Messina
    (21, 3, 2002, 0x9ea46c, 0x9ea3fc), // Reggina × Modena
    (21, 3, 2002, 0x9ea450, 0x9ea3e4), // Cagliari × Salernitana
    (21, 3, 2002, 0x9ea3c8, 0x9ea3dc), // Pistoiese × Siena
    (21, 3, 2002, 0x9ea41c, 0x9ea3d4), // Empoli × Ternana
    (28, 3, 2002, 0x9ea440, 0x9ea404), // Como × Bari
    (28, 3, 2002, 0x9ea464, 0x9ea450), // Ancona × Cagliari
    (28, 3, 2002, 0x9ea3d4, 0x9ea434), // Ternana × Cittadella
    (28, 3, 2002, 0x9ea42c, 0x9ea448), // Cosenza × Crotone
    (28, 3, 2002, 0x9ea3dc, 0x9ea41c), // Siena × Empoli
    (28, 3, 2002, 0x9ea40c, 0x9ea424), // Genoa × Palermo
    (28, 3, 2002, 0x9ea45c, 0x9ea3c8), // Messina × Pistoiese
    (28, 3, 2002, 0x9ea414, 0x9ea46c), // Napoli × Reggina
    (28, 3, 2002, 0x9ea3c0, 0x9ea3e4), // Vicenza × Salernitana
    (28, 3, 2002, 0x9ea3fc, 0x9ea3f0), // Modena × Sampdoria
    (5, 4, 2002, 0x9ea3c8, 0x9ea464), // Pistoiese × Ancona
    (5, 4, 2002, 0x9ea450, 0x9ea440), // Cagliari × Como
    (5, 4, 2002, 0x9ea404, 0x9ea42c), // Bari × Cosenza
    (5, 4, 2002, 0x9ea448, 0x9ea40c), // Crotone × Genoa
    (5, 4, 2002, 0x9ea3f0, 0x9ea45c), // Sampdoria × Messina
    (5, 4, 2002, 0x9ea434, 0x9ea3fc), // Cittadella × Modena
    (5, 4, 2002, 0x9ea424, 0x9ea414), // Palermo × Napoli
    (5, 4, 2002, 0x9ea46c, 0x9ea3dc), // Reggina × Siena
    (5, 4, 2002, 0x9ea3e4, 0x9ea3d4), // Salernitana × Ternana
    (5, 4, 2002, 0x9ea41c, 0x9ea3c0), // Empoli × Vicenza
    (12, 4, 2002, 0x9ea464, 0x9ea404), // Ancona × Bari
    (12, 4, 2002, 0x9ea3c8, 0x9ea450), // Pistoiese × Cagliari
    (12, 4, 2002, 0x9ea45c, 0x9ea434), // Messina × Cittadella
    (12, 4, 2002, 0x9ea3dc, 0x9ea448), // Siena × Crotone
    (12, 4, 2002, 0x9ea440, 0x9ea41c), // Como × Empoli
    (12, 4, 2002, 0x9ea40c, 0x9ea3fc), // Genoa × Modena
    (12, 4, 2002, 0x9ea42c, 0x9ea414), // Cosenza × Napoli
    (12, 4, 2002, 0x9ea3c0, 0x9ea424), // Vicenza × Palermo
    (12, 4, 2002, 0x9ea3e4, 0x9ea46c), // Salernitana × Reggina
    (12, 4, 2002, 0x9ea3d4, 0x9ea3f0), // Ternana × Sampdoria
    (19, 4, 2002, 0x9ea3fc, 0x9ea464), // Modena × Ancona
    (19, 4, 2002, 0x9ea3f0, 0x9ea440), // Sampdoria × Como
    (19, 4, 2002, 0x9ea450, 0x9ea42c), // Cagliari × Cosenza
    (19, 4, 2002, 0x9ea41c, 0x9ea40c), // Empoli × Genoa
    (19, 4, 2002, 0x9ea404, 0x9ea45c), // Bari × Messina
    (19, 4, 2002, 0x9ea414, 0x9ea3c8), // Napoli × Pistoiese
    (19, 4, 2002, 0x9ea424, 0x9ea3e4), // Palermo × Salernitana
    (19, 4, 2002, 0x9ea434, 0x9ea3dc), // Cittadella × Siena
    (19, 4, 2002, 0x9ea448, 0x9ea3d4), // Crotone × Ternana
    (19, 4, 2002, 0x9ea46c, 0x9ea3c0), // Reggina × Vicenza
    (26, 4, 2002, 0x9ea3e4, 0x9ea404), // Salernitana × Bari
    (26, 4, 2002, 0x9ea40c, 0x9ea450), // Genoa × Cagliari
    (26, 4, 2002, 0x9ea440, 0x9ea434), // Como × Cittadella
    (26, 4, 2002, 0x9ea3c8, 0x9ea448), // Pistoiese × Crotone
    (26, 4, 2002, 0x9ea464, 0x9ea41c), // Ancona × Empoli
    (26, 4, 2002, 0x9ea45c, 0x9ea3fc), // Messina × Modena
    (26, 4, 2002, 0x9ea3dc, 0x9ea414), // Siena × Napoli
    (26, 4, 2002, 0x9ea42c, 0x9ea424), // Cosenza × Palermo
    (26, 4, 2002, 0x9ea3d4, 0x9ea46c), // Ternana × Reggina
    (26, 4, 2002, 0x9ea3c0, 0x9ea3f0), // Vicenza × Sampdoria
    (2, 5, 2002, 0x9ea424, 0x9ea464), // Palermo × Ancona
    (2, 5, 2002, 0x9ea414, 0x9ea440), // Napoli × Como
    (2, 5, 2002, 0x9ea41c, 0x9ea42c), // Empoli × Cosenza
    (2, 5, 2002, 0x9ea46c, 0x9ea40c), // Reggina × Genoa
    (2, 5, 2002, 0x9ea448, 0x9ea45c), // Crotone × Messina
    (2, 5, 2002, 0x9ea434, 0x9ea3c8), // Cittadella × Pistoiese
    (2, 5, 2002, 0x9ea3fc, 0x9ea3e4), // Modena × Salernitana
    (2, 5, 2002, 0x9ea3f0, 0x9ea3dc), // Sampdoria × Siena
    (2, 5, 2002, 0x9ea404, 0x9ea3d4), // Bari × Ternana
    (2, 5, 2002, 0x9ea450, 0x9ea3c0), // Cagliari × Vicenza
];

