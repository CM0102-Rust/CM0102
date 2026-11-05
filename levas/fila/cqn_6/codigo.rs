// ---------- %%LEVA%%a leva: o calendário real de 2001/02 dos dois grupos da Série C1 italiana (os jogos fixos pelo nome dos clubes) e os dois tocos que devolvem 1 (ita_ser_c1a.cpp, ita_ser_c1b.cpp, fra_first.cpp) ----------

unsafe fn cqn6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// Quantos jogos a liga tem: (clubes +0x3e / 2, com sinal, para zero) × rodadas +0xa9.
unsafe fn cqn6_jogos(liga: usize) -> i32 {
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
unsafe fn cqn6_fixos(this: usize, tabela: &[(u8, u8, u16, u32, u32)], arquivo: usize, linha: u32) {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let construir: unsafe extern "stdcall" fn(usize, u32, u32, usize, usize) = core::mem::transmute(0x9463afusize);
    let destruir: unsafe extern "stdcall" fn(usize, u32, u32, usize) = core::mem::transmute(0x9462bbusize);
    let apagar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let jogo: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32, u32) = core::mem::transmute(0x68aff0usize);
    let n = cqn6_jogos(this);
    let bloco = novo((n as u32).wrapping_mul(16).wrapping_add(4));
    let vetor = if bloco != 0 {
        cqn6_pd(bloco, n as usize);
        construir(bloco + 4, 0x10, n as u32, 0x645800, 0x5a53c0);
        bloco + 4
    } else {
        0
    };
    cqn6_pd(this + 0xea, vetor);
    if vetor == 0 {
        aviso_em(arquivo, linha);
        return;
    }
    for (k, &(dia, mes, ano, casa, fora)) in tabela.iter().enumerate() {
        jogo(cqn6_d(this + 0xea) + k * 0x10, this, dia as u32, mes as u32, ano as u32, casa, fora); // cqn6: cada jogo fixo
    }
    crate::comparacao::ramo("cqn_6 calendario fixo");
    let n = cqn6_jogos(this);
    if n <= 0 {
        return;
    }
    let v = cqn6_d(this + 0xea);
    for i in 0..n as usize {
        let f = v + i * 0x10;
        if cqn6_d(f + 8) == 0xffff_ffff || cqn6_d(f + 0xc) == 0xffff_ffff { // cqn6: um clube sem achar derruba a lista
            if v != 0 {
                destruir(v, 0x10, cqn6_d(v - 4) as u32, 0x5a53c0);
                apagar(v - 4);
            }
            cqn6_pd(this + 0xea, 0);
            return;
        }
    }
}

/// { 0x64be90 } (thiscall, ret 0, ita_ser_c1a.cpp, 11.650 bytes): `ItaSerC1a::set_up_fixed_fixtures()`: os 306 jogos
/// da C1/A 2001/02 (34 rodadas de 9). Fonte 0x9ea474, linha 0x3fd.
unsafe extern "thiscall" fn calendario_fixo_da_c1a(this: usize) {
    cqn6_fixos(this, &CQN6_C1A, 0x9ea474, 0x3fd)
}

/// { 0x650100 } (thiscall, ret 0, ita_ser_c1b.cpp, 11.650 bytes): `ItaSerC1b::set_up_fixed_fixtures()`: os 306 jogos
/// da C1/B 2001/02. Fonte 0x9ea558, linha 0x404.
unsafe extern "thiscall" fn calendario_fixo_da_c1b(this: usize) {
    cqn6_fixos(this, &CQN6_C1B, 0x9ea558, 0x404)
}
/// O comparador das duas: alocam: só a original, conferindo o que ela garante: ou a lista ficou (+0xea com a conta n e
/// cada jogo com os dois clubes achados), ou foi derrubada (+0xea = 0).
unsafe fn cqn6_comparar(nome: &str, this: usize, original: impl FnOnce()) {
    original();
    let v = cqn6_d(this + 0xea);
    let n = cqn6_jogos(this);
    let ok = v == 0 || (cqn6_d(v - 4) as i32 == n && (0..n.max(0) as usize).all(|i| cqn6_d(v + i * 0x10 + 8) != 0xffff_ffff && cqn6_d(v + i * 0x10 + 0xc) != 0xffff_ffff));
    PLACAR.registrar(ok, || format!("{}({:#x}): a lista dos jogos fixos (+0xea = {:#x}) fora do esperado", nome, this, v));
}

unsafe extern "thiscall" fn comparar_calendario_fixo_da_c1a(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    cqn6_comparar("calendario_fixo_da_c1a", this, || o(this))
}

unsafe extern "thiscall" fn comparar_calendario_fixo_da_c1b(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    cqn6_comparar("calendario_fixo_da_c1b", this, || o(this))
}

/// { 0x5bb540 } (thiscall, ret 8, fra_first.cpp, 8 bytes): `stub_return_one_ret8(a, b)`, o toco das competições que não
/// jogam partidas no lugar do add_result (o slot +4): ignora os dois e devolve 1.
unsafe extern "thiscall" fn toco_um_com_dois(_this: usize, _a: u32, _b: u32) -> u32 {
    1 // cqn6: o toco devolve 1
}

/// { 0x5bb550 } (thiscall, ret 4, fra_first.cpp, 8 bytes): `stub_return_one_ret4(a)`, o padrão do slot 25 das
/// competições e do slot 6 do órgão disciplinar: ignora o argumento e devolve 1.
unsafe extern "thiscall" fn toco_um_com_um(_this: usize, _a: u32) -> u32 {
    1
}

/// Os dois tocos: compara a devolução.
unsafe extern "thiscall" fn comparar_toco_um_com_dois(this: usize, a: u32, b: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I2%%)(this, a, b);
    let n = toco_um_com_dois(this, a, b);
    PLACAR.registrar(r == n, || format!("toco_um_com_dois: original={} rust={}", r, n));
    r
}

unsafe extern "thiscall" fn comparar_toco_um_com_um(this: usize, a: u32) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I3%%)(this, a);
    let n = toco_um_com_um(this, a);
    PLACAR.registrar(r == n, || format!("toco_um_com_um: original={} rust={}", r, n));
    r
}

/// Os jogos fixos (dia, mês com 0 = janeiro, ano, o 5º argumento (casa, pelo mapa), o 6º (fora)), os nomes no .rdata do
/// exe; na ordem da original (extraídos da desmontagem e conferidos: os lugares do vetor vão de 16 em 16).
const CQN6_C1A: [(u8, u8, u16, u32, u32); 306] = [
    (2, 8, 2001, 0x9ea540, 0x9ea54c), // Albinoleffe × Carrarese
    (2, 8, 2001, 0x9ea530, 0x9ea538), // Alzano × Arezzo
    (2, 8, 2001, 0x9ea520, 0x9ea528), // Cesena × Livorno
    (2, 8, 2001, 0x9ea50c, 0x9ea518), // Lumezzane × Lecco
    (2, 8, 2001, 0x9ea4fc, 0x9ea504), // Pisa × Treviso
    (2, 8, 2001, 0x9ea4e4, 0x9ea4f0), // Reggiana × Triestina
    (2, 8, 2001, 0x9ea4d4, 0x9ea4dc), // Spal × Padova
    (2, 8, 2001, 0x9ea4c4, 0x9ea4cc), // Spezia × Monza
    (2, 8, 2001, 0x9ea4b0, 0x9ea4b8), // Varese × Lucchese
    (9, 8, 2001, 0x9ea538, 0x9ea4fc), // Arezzo × Pisa
    (9, 8, 2001, 0x9ea54c, 0x9ea4d4), // Carrarese × Spal
    (9, 8, 2001, 0x9ea518, 0x9ea540), // Lecco × Albinoleffe
    (9, 8, 2001, 0x9ea528, 0x9ea4b0), // Livorno × Varese
    (9, 8, 2001, 0x9ea4b8, 0x9ea50c), // Lucchese × Lumezzane
    (9, 8, 2001, 0x9ea4cc, 0x9ea520), // Monza × Cesena
    (9, 8, 2001, 0x9ea4dc, 0x9ea4e4), // Padova × Reggiana
    (9, 8, 2001, 0x9ea504, 0x9ea4c4), // Treviso × Spezia
    (9, 8, 2001, 0x9ea4f0, 0x9ea530), // Triestina × Alzano
    (16, 8, 2001, 0x9ea540, 0x9ea4f0), // Albinoleffe × Triestina
    (16, 8, 2001, 0x9ea530, 0x9ea4c4), // Alzano × Spezia
    (16, 8, 2001, 0x9ea538, 0x9ea528), // Arezzo × Livorno
    (16, 8, 2001, 0x9ea54c, 0x9ea4dc), // Carrarese × Padova
    (16, 8, 2001, 0x9ea520, 0x9ea4e4), // Cesena × Reggiana
    (16, 8, 2001, 0x9ea50c, 0x9ea4cc), // Lumezzane × Monza
    (16, 8, 2001, 0x9ea4fc, 0x9ea4b8), // Pisa × Lucchese
    (16, 8, 2001, 0x9ea4d4, 0x9ea518), // Spal × Lecco
    (16, 8, 2001, 0x9ea4b0, 0x9ea504), // Varese × Treviso
    (23, 8, 2001, 0x9ea518, 0x9ea520), // Lecco × Cesena
    (23, 8, 2001, 0x9ea528, 0x9ea4d4), // Livorno × Spal
    (23, 8, 2001, 0x9ea4b8, 0x9ea54c), // Lucchese × Carrarese
    (23, 8, 2001, 0x9ea4cc, 0x9ea4fc), // Monza × Pisa
    (23, 8, 2001, 0x9ea4dc, 0x9ea540), // Padova × Albinoleffe
    (23, 8, 2001, 0x9ea4e4, 0x9ea530), // Reggiana × Alzano
    (23, 8, 2001, 0x9ea4c4, 0x9ea4b0), // Spezia × Varese
    (23, 8, 2001, 0x9ea504, 0x9ea50c), // Treviso × Lumezzane
    (23, 8, 2001, 0x9ea4f0, 0x9ea538), // Triestina × Arezzo
    (30, 8, 2001, 0x9ea540, 0x9ea4e4), // Albinoleffe × Reggiana
    (30, 8, 2001, 0x9ea530, 0x9ea4cc), // Alzano × Monza
    (30, 8, 2001, 0x9ea538, 0x9ea4dc), // Arezzo × Padova
    (30, 8, 2001, 0x9ea54c, 0x9ea528), // Carrarese × Livorno
    (30, 8, 2001, 0x9ea520, 0x9ea504), // Cesena × Treviso
    (30, 8, 2001, 0x9ea518, 0x9ea4f0), // Lecco × Triestina
    (30, 8, 2001, 0x9ea4fc, 0x9ea4c4), // Pisa × Spezia
    (30, 8, 2001, 0x9ea4d4, 0x9ea4b8), // Spal × Lucchese
    (30, 8, 2001, 0x9ea4b0, 0x9ea50c), // Varese × Lumezzane
    (7, 9, 2001, 0x9ea530, 0x9ea4b0), // Alzano × Varese
    (7, 9, 2001, 0x9ea4b8, 0x9ea518), // Lucchese × Lecco
    (7, 9, 2001, 0x9ea50c, 0x9ea520), // Lumezzane × Cesena
    (7, 9, 2001, 0x9ea4cc, 0x9ea538), // Monza × Arezzo
    (7, 9, 2001, 0x9ea4dc, 0x9ea4f0), // Padova × Triestina
    (7, 9, 2001, 0x9ea4fc, 0x9ea540), // Pisa × Albinoleffe
    (7, 9, 2001, 0x9ea4e4, 0x9ea528), // Reggiana × Livorno
    (7, 9, 2001, 0x9ea4c4, 0x9ea54c), // Spezia × Carrarese
    (7, 9, 2001, 0x9ea504, 0x9ea4d4), // Treviso × Spal
    (14, 9, 2001, 0x9ea540, 0x9ea4b8), // Albinoleffe × Lucchese
    (14, 9, 2001, 0x9ea538, 0x9ea4b0), // Arezzo × Varese
    (14, 9, 2001, 0x9ea54c, 0x9ea4e4), // Carrarese × Reggiana
    (14, 9, 2001, 0x9ea520, 0x9ea4c4), // Cesena × Spezia
    (14, 9, 2001, 0x9ea518, 0x9ea530), // Lecco × Alzano
    (14, 9, 2001, 0x9ea528, 0x9ea4fc), // Livorno × Pisa
    (14, 9, 2001, 0x9ea4dc, 0x9ea504), // Padova × Treviso
    (14, 9, 2001, 0x9ea4d4, 0x9ea4cc), // Spal × Monza
    (14, 9, 2001, 0x9ea4f0, 0x9ea50c), // Triestina × Lumezzane
    (21, 9, 2001, 0x9ea520, 0x9ea538), // Cesena × Arezzo
    (21, 9, 2001, 0x9ea50c, 0x9ea540), // Lumezzane × Albinoleffe
    (21, 9, 2001, 0x9ea4cc, 0x9ea4dc), // Monza × Padova
    (21, 9, 2001, 0x9ea4fc, 0x9ea530), // Pisa × Alzano
    (21, 9, 2001, 0x9ea4e4, 0x9ea4b8), // Reggiana × Lucchese
    (21, 9, 2001, 0x9ea4c4, 0x9ea518), // Spezia × Lecco
    (21, 9, 2001, 0x9ea504, 0x9ea54c), // Treviso × Carrarese
    (21, 9, 2001, 0x9ea4f0, 0x9ea528), // Triestina × Livorno
    (21, 9, 2001, 0x9ea4b0, 0x9ea4d4), // Varese × Spal
    (28, 9, 2001, 0x9ea540, 0x9ea504), // Albinoleffe × Treviso
    (28, 9, 2001, 0x9ea530, 0x9ea520), // Alzano × Cesena
    (28, 9, 2001, 0x9ea538, 0x9ea50c), // Arezzo × Lumezzane
    (28, 9, 2001, 0x9ea54c, 0x9ea4cc), // Carrarese × Monza
    (28, 9, 2001, 0x9ea518, 0x9ea4e4), // Lecco × Reggiana
    (28, 9, 2001, 0x9ea528, 0x9ea4dc), // Livorno × Padova
    (28, 9, 2001, 0x9ea4b8, 0x9ea4f0), // Lucchese × Triestina
    (28, 9, 2001, 0x9ea4d4, 0x9ea4c4), // Spal × Spezia
    (28, 9, 2001, 0x9ea4b0, 0x9ea4fc), // Varese × Pisa
    (4, 10, 2001, 0x9ea520, 0x9ea4b0), // Cesena × Varese
    (4, 10, 2001, 0x9ea528, 0x9ea4b8), // Livorno × Lucchese
    (4, 10, 2001, 0x9ea50c, 0x9ea4fc), // Lumezzane × Pisa
    (4, 10, 2001, 0x9ea4cc, 0x9ea540), // Monza × Albinoleffe
    (4, 10, 2001, 0x9ea4dc, 0x9ea518), // Padova × Lecco
    (4, 10, 2001, 0x9ea4e4, 0x9ea4d4), // Reggiana × Spal
    (4, 10, 2001, 0x9ea4c4, 0x9ea538), // Spezia × Arezzo
    (4, 10, 2001, 0x9ea504, 0x9ea530), // Treviso × Alzano
    (4, 10, 2001, 0x9ea4f0, 0x9ea54c), // Triestina × Carrarese
    (11, 10, 2001, 0x9ea540, 0x9ea528), // Albinoleffe × Livorno
    (11, 10, 2001, 0x9ea530, 0x9ea50c), // Alzano × Lumezzane
    (11, 10, 2001, 0x9ea538, 0x9ea504), // Arezzo × Treviso
    (11, 10, 2001, 0x9ea518, 0x9ea54c), // Lecco × Carrarese
    (11, 10, 2001, 0x9ea4b8, 0x9ea4dc), // Lucchese × Padova
    (11, 10, 2001, 0x9ea4fc, 0x9ea520), // Pisa × Cesena
    (11, 10, 2001, 0x9ea4d4, 0x9ea4f0), // Spal × Triestina
    (11, 10, 2001, 0x9ea4c4, 0x9ea4e4), // Spezia × Reggiana
    (11, 10, 2001, 0x9ea4b0, 0x9ea4cc), // Varese × Monza
    (18, 10, 2001, 0x9ea540, 0x9ea4d4), // Albinoleffe × Spal
    (18, 10, 2001, 0x9ea54c, 0x9ea4fc), // Carrarese × Pisa
    (18, 10, 2001, 0x9ea528, 0x9ea518), // Livorno × Lecco
    (18, 10, 2001, 0x9ea4b8, 0x9ea538), // Lucchese × Arezzo
    (18, 10, 2001, 0x9ea50c, 0x9ea4c4), // Lumezzane × Spezia
    (18, 10, 2001, 0x9ea4dc, 0x9ea530), // Padova × Alzano
    (18, 10, 2001, 0x9ea4e4, 0x9ea4b0), // Reggiana × Varese
    (18, 10, 2001, 0x9ea504, 0x9ea4cc), // Treviso × Monza
    (18, 10, 2001, 0x9ea4f0, 0x9ea520), // Triestina × Cesena
    (25, 10, 2001, 0x9ea530, 0x9ea540), // Alzano × Albinoleffe
    (25, 10, 2001, 0x9ea538, 0x9ea4e4), // Arezzo × Reggiana
    (25, 10, 2001, 0x9ea520, 0x9ea4dc), // Cesena × Padova
    (25, 10, 2001, 0x9ea50c, 0x9ea54c), // Lumezzane × Carrarese
    (25, 10, 2001, 0x9ea4cc, 0x9ea4b8), // Monza × Lucchese
    (25, 10, 2001, 0x9ea4fc, 0x9ea4d4), // Pisa × Spal
    (25, 10, 2001, 0x9ea4c4, 0x9ea528), // Spezia × Livorno
    (25, 10, 2001, 0x9ea504, 0x9ea4f0), // Treviso × Triestina
    (25, 10, 2001, 0x9ea4b0, 0x9ea518), // Varese × Lecco
    (2, 11, 2001, 0x9ea540, 0x9ea4b0), // Albinoleffe × Varese
    (2, 11, 2001, 0x9ea54c, 0x9ea530), // Carrarese × Alzano
    (2, 11, 2001, 0x9ea518, 0x9ea538), // Lecco × Arezzo
    (2, 11, 2001, 0x9ea528, 0x9ea4cc), // Livorno × Monza
    (2, 11, 2001, 0x9ea4b8, 0x9ea504), // Lucchese × Treviso
    (2, 11, 2001, 0x9ea4dc, 0x9ea4fc), // Padova × Pisa
    (2, 11, 2001, 0x9ea4e4, 0x9ea50c), // Reggiana × Lumezzane
    (2, 11, 2001, 0x9ea4d4, 0x9ea520), // Spal × Cesena
    (2, 11, 2001, 0x9ea4f0, 0x9ea4c4), // Triestina × Spezia
    (9, 11, 2001, 0x9ea530, 0x9ea4d4), // Alzano × Spal
    (9, 11, 2001, 0x9ea538, 0x9ea540), // Arezzo × Albinoleffe
    (9, 11, 2001, 0x9ea520, 0x9ea54c), // Cesena × Carrarese
    (9, 11, 2001, 0x9ea50c, 0x9ea528), // Lumezzane × Livorno
    (9, 11, 2001, 0x9ea4cc, 0x9ea518), // Monza × Lecco
    (9, 11, 2001, 0x9ea4fc, 0x9ea4f0), // Pisa × Triestina
    (9, 11, 2001, 0x9ea4c4, 0x9ea4b8), // Spezia × Lucchese
    (9, 11, 2001, 0x9ea504, 0x9ea4e4), // Treviso × Reggiana
    (9, 11, 2001, 0x9ea4b0, 0x9ea4dc), // Varese × Padova
    (16, 11, 2001, 0x9ea540, 0x9ea520), // Albinoleffe × Cesena
    (16, 11, 2001, 0x9ea54c, 0x9ea538), // Carrarese × Arezzo
    (16, 11, 2001, 0x9ea518, 0x9ea4fc), // Lecco × Pisa
    (16, 11, 2001, 0x9ea528, 0x9ea504), // Livorno × Treviso
    (16, 11, 2001, 0x9ea4b8, 0x9ea530), // Lucchese × Alzano
    (16, 11, 2001, 0x9ea4dc, 0x9ea4c4), // Padova × Spezia
    (16, 11, 2001, 0x9ea4e4, 0x9ea4cc), // Reggiana × Monza
    (16, 11, 2001, 0x9ea4d4, 0x9ea50c), // Spal × Lumezzane
    (16, 11, 2001, 0x9ea4f0, 0x9ea4b0), // Triestina × Varese
    (23, 11, 2001, 0x9ea530, 0x9ea528), // Alzano × Livorno
    (23, 11, 2001, 0x9ea538, 0x9ea4d4), // Arezzo × Spal
    (23, 11, 2001, 0x9ea520, 0x9ea4b8), // Cesena × Lucchese
    (23, 11, 2001, 0x9ea50c, 0x9ea4dc), // Lumezzane × Padova
    (23, 11, 2001, 0x9ea4cc, 0x9ea4f0), // Monza × Triestina
    (23, 11, 2001, 0x9ea4fc, 0x9ea4e4), // Pisa × Reggiana
    (23, 11, 2001, 0x9ea4c4, 0x9ea540), // Spezia × Albinoleffe
    (23, 11, 2001, 0x9ea504, 0x9ea518), // Treviso × Lecco
    (23, 11, 2001, 0x9ea4b0, 0x9ea54c), // Varese × Carrarese
    (6, 0, 2002, 0x9ea54c, 0x9ea540), // Carrarese × Albinoleffe
    (6, 0, 2002, 0x9ea538, 0x9ea530), // Arezzo × Alzano
    (6, 0, 2002, 0x9ea528, 0x9ea520), // Livorno × Cesena
    (6, 0, 2002, 0x9ea518, 0x9ea50c), // Lecco × Lumezzane
    (6, 0, 2002, 0x9ea504, 0x9ea4fc), // Treviso × Pisa
    (6, 0, 2002, 0x9ea4f0, 0x9ea4e4), // Triestina × Reggiana
    (6, 0, 2002, 0x9ea4dc, 0x9ea4d4), // Padova × Spal
    (6, 0, 2002, 0x9ea4cc, 0x9ea4c4), // Monza × Spezia
    (6, 0, 2002, 0x9ea4b8, 0x9ea4b0), // Lucchese × Varese
    (13, 0, 2002, 0x9ea4fc, 0x9ea538), // Pisa × Arezzo
    (13, 0, 2002, 0x9ea4d4, 0x9ea54c), // Spal × Carrarese
    (13, 0, 2002, 0x9ea540, 0x9ea518), // Albinoleffe × Lecco
    (13, 0, 2002, 0x9ea4b0, 0x9ea528), // Varese × Livorno
    (13, 0, 2002, 0x9ea50c, 0x9ea4b8), // Lumezzane × Lucchese
    (13, 0, 2002, 0x9ea520, 0x9ea4cc), // Cesena × Monza
    (13, 0, 2002, 0x9ea4e4, 0x9ea4dc), // Reggiana × Padova
    (13, 0, 2002, 0x9ea4c4, 0x9ea504), // Spezia × Treviso
    (13, 0, 2002, 0x9ea530, 0x9ea4f0), // Alzano × Triestina
    (20, 0, 2002, 0x9ea4f0, 0x9ea540), // Triestina × Albinoleffe
    (20, 0, 2002, 0x9ea4c4, 0x9ea530), // Spezia × Alzano
    (20, 0, 2002, 0x9ea528, 0x9ea538), // Livorno × Arezzo
    (20, 0, 2002, 0x9ea4dc, 0x9ea54c), // Padova × Carrarese
    (20, 0, 2002, 0x9ea4e4, 0x9ea520), // Reggiana × Cesena
    (20, 0, 2002, 0x9ea4cc, 0x9ea50c), // Monza × Lumezzane
    (20, 0, 2002, 0x9ea4b8, 0x9ea4fc), // Lucchese × Pisa
    (20, 0, 2002, 0x9ea518, 0x9ea4d4), // Lecco × Spal
    (20, 0, 2002, 0x9ea504, 0x9ea4b0), // Treviso × Varese
    (27, 0, 2002, 0x9ea520, 0x9ea518), // Cesena × Lecco
    (27, 0, 2002, 0x9ea4d4, 0x9ea528), // Spal × Livorno
    (27, 0, 2002, 0x9ea54c, 0x9ea4b8), // Carrarese × Lucchese
    (27, 0, 2002, 0x9ea4fc, 0x9ea4cc), // Pisa × Monza
    (27, 0, 2002, 0x9ea540, 0x9ea4dc), // Albinoleffe × Padova
    (27, 0, 2002, 0x9ea530, 0x9ea4e4), // Alzano × Reggiana
    (27, 0, 2002, 0x9ea4b0, 0x9ea4c4), // Varese × Spezia
    (27, 0, 2002, 0x9ea50c, 0x9ea504), // Lumezzane × Treviso
    (27, 0, 2002, 0x9ea538, 0x9ea4f0), // Arezzo × Triestina
    (3, 1, 2002, 0x9ea4e4, 0x9ea540), // Reggiana × Albinoleffe
    (3, 1, 2002, 0x9ea4cc, 0x9ea530), // Monza × Alzano
    (3, 1, 2002, 0x9ea4dc, 0x9ea538), // Padova × Arezzo
    (3, 1, 2002, 0x9ea528, 0x9ea54c), // Livorno × Carrarese
    (3, 1, 2002, 0x9ea504, 0x9ea520), // Treviso × Cesena
    (3, 1, 2002, 0x9ea4f0, 0x9ea518), // Triestina × Lecco
    (3, 1, 2002, 0x9ea4c4, 0x9ea4fc), // Spezia × Pisa
    (3, 1, 2002, 0x9ea4b8, 0x9ea4d4), // Lucchese × Spal
    (3, 1, 2002, 0x9ea50c, 0x9ea4b0), // Lumezzane × Varese
    (10, 1, 2002, 0x9ea4b0, 0x9ea530), // Varese × Alzano
    (10, 1, 2002, 0x9ea518, 0x9ea4b8), // Lecco × Lucchese
    (10, 1, 2002, 0x9ea520, 0x9ea50c), // Cesena × Lumezzane
    (10, 1, 2002, 0x9ea538, 0x9ea4cc), // Arezzo × Monza
    (10, 1, 2002, 0x9ea4f0, 0x9ea4dc), // Triestina × Padova
    (10, 1, 2002, 0x9ea540, 0x9ea4fc), // Albinoleffe × Pisa
    (10, 1, 2002, 0x9ea528, 0x9ea4e4), // Livorno × Reggiana
    (10, 1, 2002, 0x9ea54c, 0x9ea4c4), // Carrarese × Spezia
    (10, 1, 2002, 0x9ea4d4, 0x9ea504), // Spal × Treviso
    (17, 1, 2002, 0x9ea4b8, 0x9ea540), // Lucchese × Albinoleffe
    (17, 1, 2002, 0x9ea4b0, 0x9ea538), // Varese × Arezzo
    (17, 1, 2002, 0x9ea4e4, 0x9ea54c), // Reggiana × Carrarese
    (17, 1, 2002, 0x9ea4c4, 0x9ea520), // Spezia × Cesena
    (17, 1, 2002, 0x9ea530, 0x9ea518), // Alzano × Lecco
    (17, 1, 2002, 0x9ea4fc, 0x9ea528), // Pisa × Livorno
    (17, 1, 2002, 0x9ea504, 0x9ea4dc), // Treviso × Padova
    (17, 1, 2002, 0x9ea4cc, 0x9ea4d4), // Monza × Spal
    (17, 1, 2002, 0x9ea50c, 0x9ea4f0), // Lumezzane × Triestina
    (3, 2, 2002, 0x9ea538, 0x9ea520), // Arezzo × Cesena
    (3, 2, 2002, 0x9ea540, 0x9ea50c), // Albinoleffe × Lumezzane
    (3, 2, 2002, 0x9ea4dc, 0x9ea4cc), // Padova × Monza
    (3, 2, 2002, 0x9ea530, 0x9ea4fc), // Alzano × Pisa
    (3, 2, 2002, 0x9ea4b8, 0x9ea4e4), // Lucchese × Reggiana
    (3, 2, 2002, 0x9ea518, 0x9ea4c4), // Lecco × Spezia
    (3, 2, 2002, 0x9ea54c, 0x9ea504), // Carrarese × Treviso
    (3, 2, 2002, 0x9ea528, 0x9ea4f0), // Livorno × Triestina
    (3, 2, 2002, 0x9ea4d4, 0x9ea4b0), // Spal × Varese
    (10, 2, 2002, 0x9ea504, 0x9ea540), // Treviso × Albinoleffe
    (10, 2, 2002, 0x9ea520, 0x9ea530), // Cesena × Alzano
    (10, 2, 2002, 0x9ea50c, 0x9ea538), // Lumezzane × Arezzo
    (10, 2, 2002, 0x9ea4cc, 0x9ea54c), // Monza × Carrarese
    (10, 2, 2002, 0x9ea4e4, 0x9ea518), // Reggiana × Lecco
    (10, 2, 2002, 0x9ea4dc, 0x9ea528), // Padova × Livorno
    (10, 2, 2002, 0x9ea4f0, 0x9ea4b8), // Triestina × Lucchese
    (10, 2, 2002, 0x9ea4c4, 0x9ea4d4), // Spezia × Spal
    (10, 2, 2002, 0x9ea4fc, 0x9ea4b0), // Pisa × Varese
    (17, 2, 2002, 0x9ea4b0, 0x9ea520), // Varese × Cesena
    (17, 2, 2002, 0x9ea4b8, 0x9ea528), // Lucchese × Livorno
    (17, 2, 2002, 0x9ea4fc, 0x9ea50c), // Pisa × Lumezzane
    (17, 2, 2002, 0x9ea540, 0x9ea4cc), // Albinoleffe × Monza
    (17, 2, 2002, 0x9ea518, 0x9ea4dc), // Lecco × Padova
    (17, 2, 2002, 0x9ea4d4, 0x9ea4e4), // Spal × Reggiana
    (17, 2, 2002, 0x9ea538, 0x9ea4c4), // Arezzo × Spezia
    (17, 2, 2002, 0x9ea530, 0x9ea504), // Alzano × Treviso
    (17, 2, 2002, 0x9ea54c, 0x9ea4f0), // Carrarese × Triestina
    (24, 2, 2002, 0x9ea528, 0x9ea540), // Livorno × Albinoleffe
    (24, 2, 2002, 0x9ea50c, 0x9ea530), // Lumezzane × Alzano
    (24, 2, 2002, 0x9ea504, 0x9ea538), // Treviso × Arezzo
    (24, 2, 2002, 0x9ea54c, 0x9ea518), // Carrarese × Lecco
    (24, 2, 2002, 0x9ea4dc, 0x9ea4b8), // Padova × Lucchese
    (24, 2, 2002, 0x9ea520, 0x9ea4fc), // Cesena × Pisa
    (24, 2, 2002, 0x9ea4f0, 0x9ea4d4), // Triestina × Spal
    (24, 2, 2002, 0x9ea4e4, 0x9ea4c4), // Reggiana × Spezia
    (24, 2, 2002, 0x9ea4cc, 0x9ea4b0), // Monza × Varese
    (30, 2, 2002, 0x9ea4d4, 0x9ea540), // Spal × Albinoleffe
    (30, 2, 2002, 0x9ea4fc, 0x9ea54c), // Pisa × Carrarese
    (30, 2, 2002, 0x9ea518, 0x9ea528), // Lecco × Livorno
    (30, 2, 2002, 0x9ea538, 0x9ea4b8), // Arezzo × Lucchese
    (30, 2, 2002, 0x9ea4c4, 0x9ea50c), // Spezia × Lumezzane
    (30, 2, 2002, 0x9ea530, 0x9ea4dc), // Alzano × Padova
    (30, 2, 2002, 0x9ea4b0, 0x9ea4e4), // Varese × Reggiana
    (30, 2, 2002, 0x9ea4cc, 0x9ea504), // Monza × Treviso
    (30, 2, 2002, 0x9ea520, 0x9ea4f0), // Cesena × Triestina
    (7, 3, 2002, 0x9ea540, 0x9ea530), // Albinoleffe × Alzano
    (7, 3, 2002, 0x9ea4e4, 0x9ea538), // Reggiana × Arezzo
    (7, 3, 2002, 0x9ea4dc, 0x9ea520), // Padova × Cesena
    (7, 3, 2002, 0x9ea54c, 0x9ea50c), // Carrarese × Lumezzane
    (7, 3, 2002, 0x9ea4b8, 0x9ea4cc), // Lucchese × Monza
    (7, 3, 2002, 0x9ea4d4, 0x9ea4fc), // Spal × Pisa
    (7, 3, 2002, 0x9ea528, 0x9ea4c4), // Livorno × Spezia
    (7, 3, 2002, 0x9ea4f0, 0x9ea504), // Triestina × Treviso
    (7, 3, 2002, 0x9ea518, 0x9ea4b0), // Lecco × Varese
    (14, 3, 2002, 0x9ea4b0, 0x9ea540), // Varese × Albinoleffe
    (14, 3, 2002, 0x9ea530, 0x9ea54c), // Alzano × Carrarese
    (14, 3, 2002, 0x9ea538, 0x9ea518), // Arezzo × Lecco
    (14, 3, 2002, 0x9ea4cc, 0x9ea528), // Monza × Livorno
    (14, 3, 2002, 0x9ea504, 0x9ea4b8), // Treviso × Lucchese
    (14, 3, 2002, 0x9ea4fc, 0x9ea4dc), // Pisa × Padova
    (14, 3, 2002, 0x9ea50c, 0x9ea4e4), // Lumezzane × Reggiana
    (14, 3, 2002, 0x9ea520, 0x9ea4d4), // Cesena × Spal
    (14, 3, 2002, 0x9ea4c4, 0x9ea4f0), // Spezia × Triestina
    (21, 3, 2002, 0x9ea4d4, 0x9ea530), // Spal × Alzano
    (21, 3, 2002, 0x9ea540, 0x9ea538), // Albinoleffe × Arezzo
    (21, 3, 2002, 0x9ea54c, 0x9ea520), // Carrarese × Cesena
    (21, 3, 2002, 0x9ea528, 0x9ea50c), // Livorno × Lumezzane
    (21, 3, 2002, 0x9ea518, 0x9ea4cc), // Lecco × Monza
    (21, 3, 2002, 0x9ea4f0, 0x9ea4fc), // Triestina × Pisa
    (21, 3, 2002, 0x9ea4b8, 0x9ea4c4), // Lucchese × Spezia
    (21, 3, 2002, 0x9ea4e4, 0x9ea504), // Reggiana × Treviso
    (21, 3, 2002, 0x9ea4dc, 0x9ea4b0), // Padova × Varese
    (28, 3, 2002, 0x9ea520, 0x9ea540), // Cesena × Albinoleffe
    (28, 3, 2002, 0x9ea538, 0x9ea54c), // Arezzo × Carrarese
    (28, 3, 2002, 0x9ea4fc, 0x9ea518), // Pisa × Lecco
    (28, 3, 2002, 0x9ea504, 0x9ea528), // Treviso × Livorno
    (28, 3, 2002, 0x9ea530, 0x9ea4b8), // Alzano × Lucchese
    (28, 3, 2002, 0x9ea4c4, 0x9ea4dc), // Spezia × Padova
    (28, 3, 2002, 0x9ea4cc, 0x9ea4e4), // Monza × Reggiana
    (28, 3, 2002, 0x9ea50c, 0x9ea4d4), // Lumezzane × Spal
    (28, 3, 2002, 0x9ea4b0, 0x9ea4f0), // Varese × Triestina
    (5, 4, 2002, 0x9ea528, 0x9ea530), // Livorno × Alzano
    (5, 4, 2002, 0x9ea4d4, 0x9ea538), // Spal × Arezzo
    (5, 4, 2002, 0x9ea4b8, 0x9ea520), // Lucchese × Cesena
    (5, 4, 2002, 0x9ea4dc, 0x9ea50c), // Padova × Lumezzane
    (5, 4, 2002, 0x9ea4f0, 0x9ea4cc), // Triestina × Monza
    (5, 4, 2002, 0x9ea4e4, 0x9ea4fc), // Reggiana × Pisa
    (5, 4, 2002, 0x9ea540, 0x9ea4c4), // Albinoleffe × Spezia
    (5, 4, 2002, 0x9ea518, 0x9ea504), // Lecco × Treviso
    (5, 4, 2002, 0x9ea54c, 0x9ea4b0), // Carrarese × Varese
];

const CQN6_C1B: [(u8, u8, u16, u32, u32); 306] = [
    (2, 8, 2001, 0x9ea638, 0x9ea640), // Ascoli × Castel di Sangro
    (2, 8, 2001, 0x9ea620, 0x9ea62c), // Avellino × L'Aquila
    (2, 8, 2001, 0x9ea60c, 0x9ea614), // Catania × Benevento
    (2, 8, 2001, 0x9ea5f8, 0x9ea604), // Giulianova × Chieti
    (2, 8, 2001, 0x9ea5e0, 0x9ea5ec), // Lodigiani × Lanciano
    (2, 8, 2001, 0x9ea5d0, 0x9ea5d8), // Pescara × Fermana
    (2, 8, 2001, 0x9ea5bc, 0x9ea5c4), // Sora × Nocerina
    (2, 8, 2001, 0x9ea5a8, 0x9ea5b0), // Taranto × Vis Pesaro
    (2, 8, 2001, 0x9ea594, 0x9ea5a0), // Viterbese × Torres
    (9, 8, 2001, 0x9ea614, 0x9ea594), // Benevento × Viterbese
    (9, 8, 2001, 0x9ea640, 0x9ea5f8), // Castel di Sangro × Giulianova
    (9, 8, 2001, 0x9ea604, 0x9ea5d0), // Chieti × Pescara
    (9, 8, 2001, 0x9ea5d8, 0x9ea5e0), // Fermana × Lodigiani
    (9, 8, 2001, 0x9ea62c, 0x9ea5bc), // L'Aquila × Sora
    (9, 8, 2001, 0x9ea5ec, 0x9ea5a8), // Lanciano × Taranto
    (9, 8, 2001, 0x9ea5c4, 0x9ea620), // Nocerina × Avellino
    (9, 8, 2001, 0x9ea5a0, 0x9ea60c), // Torres × Catania
    (9, 8, 2001, 0x9ea5b0, 0x9ea638), // Vis Pesaro × Ascoli
    (16, 8, 2001, 0x9ea638, 0x9ea604), // Ascoli × Chieti
    (16, 8, 2001, 0x9ea620, 0x9ea5a0), // Avellino × Torres
    (16, 8, 2001, 0x9ea60c, 0x9ea5ec), // Catania × Lanciano
    (16, 8, 2001, 0x9ea5f8, 0x9ea594), // Giulianova × Viterbese
    (16, 8, 2001, 0x9ea5e0, 0x9ea614), // Lodigiani × Benevento
    (16, 8, 2001, 0x9ea5d0, 0x9ea62c), // Pescara × L'Aquila
    (16, 8, 2001, 0x9ea5bc, 0x9ea5d8), // Sora × Fermana
    (16, 8, 2001, 0x9ea5a8, 0x9ea640), // Taranto × Castel di Sangro
    (16, 8, 2001, 0x9ea5b0, 0x9ea5c4), // Vis Pesaro × Nocerina
    (23, 8, 2001, 0x9ea614, 0x9ea5d0), // Benevento × Pescara
    (23, 8, 2001, 0x9ea640, 0x9ea620), // Castel di Sangro × Avellino
    (23, 8, 2001, 0x9ea604, 0x9ea5a8), // Chieti × Taranto
    (23, 8, 2001, 0x9ea5d8, 0x9ea60c), // Fermana × Catania
    (23, 8, 2001, 0x9ea62c, 0x9ea5f8), // L'Aquila × Giulianova
    (23, 8, 2001, 0x9ea5ec, 0x9ea638), // Lanciano × Ascoli
    (23, 8, 2001, 0x9ea5c4, 0x9ea5e0), // Nocerina × Lodigiani
    (23, 8, 2001, 0x9ea5a0, 0x9ea5b0), // Torres × Vis Pesaro
    (23, 8, 2001, 0x9ea594, 0x9ea5bc), // Viterbese × Sora
    (30, 8, 2001, 0x9ea638, 0x9ea5a0), // Ascoli × Torres
    (30, 8, 2001, 0x9ea620, 0x9ea5ec), // Avellino × Lanciano
    (30, 8, 2001, 0x9ea640, 0x9ea604), // Castel di Sangro × Chieti
    (30, 8, 2001, 0x9ea5f8, 0x9ea5c4), // Giulianova × Nocerina
    (30, 8, 2001, 0x9ea5e0, 0x9ea594), // Lodigiani × Viterbese
    (30, 8, 2001, 0x9ea5d0, 0x9ea60c), // Pescara × Catania
    (30, 8, 2001, 0x9ea5bc, 0x9ea614), // Sora × Benevento
    (30, 8, 2001, 0x9ea5a8, 0x9ea5d8), // Taranto × Fermana
    (30, 8, 2001, 0x9ea5b0, 0x9ea62c), // Vis Pesaro × L'Aquila
    (7, 9, 2001, 0x9ea620, 0x9ea5bc), // Avellino × Sora
    (7, 9, 2001, 0x9ea614, 0x9ea5f8), // Benevento × Giulianova
    (7, 9, 2001, 0x9ea60c, 0x9ea5e0), // Catania × Lodigiani
    (7, 9, 2001, 0x9ea604, 0x9ea5c4), // Chieti × Nocerina
    (7, 9, 2001, 0x9ea5d8, 0x9ea5b0), // Fermana × Vis Pesaro
    (7, 9, 2001, 0x9ea62c, 0x9ea638), // L'Aquila × Ascoli
    (7, 9, 2001, 0x9ea5ec, 0x9ea640), // Lanciano × Castel di Sangro
    (7, 9, 2001, 0x9ea5a0, 0x9ea5a8), // Torres × Taranto
    (7, 9, 2001, 0x9ea594, 0x9ea5d0), // Viterbese × Pescara
    (14, 9, 2001, 0x9ea638, 0x9ea614), // Ascoli × Benevento
    (14, 9, 2001, 0x9ea640, 0x9ea5a0), // Castel di Sangro × Torres
    (14, 9, 2001, 0x9ea5f8, 0x9ea5d8), // Giulianova × Fermana
    (14, 9, 2001, 0x9ea5ec, 0x9ea604), // Lanciano × Chieti
    (14, 9, 2001, 0x9ea5e0, 0x9ea5d0), // Lodigiani × Pescara
    (14, 9, 2001, 0x9ea5c4, 0x9ea594), // Nocerina × Viterbese
    (14, 9, 2001, 0x9ea5bc, 0x9ea60c), // Sora × Catania
    (14, 9, 2001, 0x9ea5a8, 0x9ea62c), // Taranto × L'Aquila
    (14, 9, 2001, 0x9ea5b0, 0x9ea620), // Vis Pesaro × Avellino
    (21, 9, 2001, 0x9ea620, 0x9ea604), // Avellino × Chieti
    (21, 9, 2001, 0x9ea614, 0x9ea5c4), // Benevento × Nocerina
    (21, 9, 2001, 0x9ea60c, 0x9ea5b0), // Catania × Vis Pesaro
    (21, 9, 2001, 0x9ea5d8, 0x9ea640), // Fermana × Castel di Sangro
    (21, 9, 2001, 0x9ea62c, 0x9ea5e0), // L'Aquila × Lodigiani
    (21, 9, 2001, 0x9ea5d0, 0x9ea5f8), // Pescara × Giulianova
    (21, 9, 2001, 0x9ea5a0, 0x9ea5ec), // Torres × Lanciano
    (21, 9, 2001, 0x9ea5bc, 0x9ea638), // Sora × Ascoli
    (21, 9, 2001, 0x9ea594, 0x9ea5a8), // Viterbese × Taranto
    (28, 9, 2001, 0x9ea638, 0x9ea620), // Ascoli × Avellino
    (28, 9, 2001, 0x9ea640, 0x9ea5e0), // Castel di Sangro × Lodigiani
    (28, 9, 2001, 0x9ea604, 0x9ea5b0), // Chieti × Vis Pesaro
    (28, 9, 2001, 0x9ea5f8, 0x9ea5bc), // Giulianova × Sora
    (28, 9, 2001, 0x9ea5ec, 0x9ea62c), // Lanciano × L'Aquila
    (28, 9, 2001, 0x9ea5c4, 0x9ea5d0), // Nocerina × Pescara
    (28, 9, 2001, 0x9ea5a0, 0x9ea614), // Torres × Benevento
    (28, 9, 2001, 0x9ea5a8, 0x9ea60c), // Taranto × Catania
    (28, 9, 2001, 0x9ea594, 0x9ea5d8), // Viterbese × Fermana
    (4, 10, 2001, 0x9ea620, 0x9ea5a8), // Avellino × Taranto
    (4, 10, 2001, 0x9ea614, 0x9ea640), // Benevento × Castel di Sangro
    (4, 10, 2001, 0x9ea60c, 0x9ea594), // Catania × Viterbese
    (4, 10, 2001, 0x9ea5d8, 0x9ea5c4), // Fermana × Nocerina
    (4, 10, 2001, 0x9ea62c, 0x9ea604), // L'Aquila × Chieti
    (4, 10, 2001, 0x9ea5e0, 0x9ea5f8), // Lodigiani × Giulianova
    (4, 10, 2001, 0x9ea5d0, 0x9ea638), // Pescara × Ascoli
    (4, 10, 2001, 0x9ea5bc, 0x9ea5a0), // Sora × Torres
    (4, 10, 2001, 0x9ea5b0, 0x9ea5ec), // Vis Pesaro × Lanciano
    (11, 10, 2001, 0x9ea638, 0x9ea5a8), // Ascoli × Taranto
    (11, 10, 2001, 0x9ea614, 0x9ea5d8), // Benevento × Fermana
    (11, 10, 2001, 0x9ea640, 0x9ea5b0), // Castel di Sangro × Vis Pesaro
    (11, 10, 2001, 0x9ea604, 0x9ea5e0), // Chieti × Lodigiani
    (11, 10, 2001, 0x9ea5f8, 0x9ea60c), // Giulianova × Catania
    (11, 10, 2001, 0x9ea5c4, 0x9ea5ec), // Nocerina × Lanciano
    (11, 10, 2001, 0x9ea5d0, 0x9ea5bc), // Pescara × Sora
    (11, 10, 2001, 0x9ea5a0, 0x9ea62c), // Torres × L'Aquila
    (11, 10, 2001, 0x9ea594, 0x9ea620), // Viterbese × Avellino
    (18, 10, 2001, 0x9ea620, 0x9ea614), // Avellino × Benevento
    (18, 10, 2001, 0x9ea60c, 0x9ea5c4), // Catania × Nocerina
    (18, 10, 2001, 0x9ea604, 0x9ea5a0), // Chieti × Torres
    (18, 10, 2001, 0x9ea5d8, 0x9ea638), // Fermana × Ascoli
    (18, 10, 2001, 0x9ea62c, 0x9ea640), // L'Aquila × Castel di Sangro
    (18, 10, 2001, 0x9ea5ec, 0x9ea5d0), // Lanciano × Pescara
    (18, 10, 2001, 0x9ea5e0, 0x9ea5bc), // Lodigiani × Sora
    (18, 10, 2001, 0x9ea5a8, 0x9ea5f8), // Taranto × Giulianova
    (18, 10, 2001, 0x9ea5b0, 0x9ea594), // Vis Pesaro × Viterbese
    (25, 10, 2001, 0x9ea614, 0x9ea5a8), // Benevento × Taranto
    (25, 10, 2001, 0x9ea60c, 0x9ea638), // Catania × Ascoli
    (25, 10, 2001, 0x9ea5d8, 0x9ea604), // Fermana × Chieti
    (25, 10, 2001, 0x9ea5f8, 0x9ea5a0), // Giulianova × Torres
    (25, 10, 2001, 0x9ea5e0, 0x9ea5b0), // Lodigiani × Vis Pesaro
    (25, 10, 2001, 0x9ea5c4, 0x9ea62c), // Nocerina × L'Aquila
    (25, 10, 2001, 0x9ea5d0, 0x9ea620), // Pescara × Avellino
    (25, 10, 2001, 0x9ea5bc, 0x9ea5ec), // Sora × Lanciano
    (25, 10, 2001, 0x9ea594, 0x9ea640), // Viterbese × Castel di Sangro
    (2, 11, 2001, 0x9ea638, 0x9ea5c4), // Ascoli × Nocerina
    (2, 11, 2001, 0x9ea620, 0x9ea5d8), // Avellino × Fermana
    (2, 11, 2001, 0x9ea640, 0x9ea5bc), // Castel di Sangro × Sora
    (2, 11, 2001, 0x9ea604, 0x9ea60c), // Chieti × Catania
    (2, 11, 2001, 0x9ea62c, 0x9ea594), // L'Aquila × Viterbese
    (2, 11, 2001, 0x9ea5ec, 0x9ea614), // Lanciano × Benevento
    (2, 11, 2001, 0x9ea5a0, 0x9ea5e0), // Torres × Lodigiani
    (2, 11, 2001, 0x9ea5a8, 0x9ea5d0), // Taranto × Pescara
    (2, 11, 2001, 0x9ea5b0, 0x9ea5f8), // Vis Pesaro × Giulianova
    (9, 11, 2001, 0x9ea614, 0x9ea5b0), // Benevento × Vis Pesaro
    (9, 11, 2001, 0x9ea60c, 0x9ea62c), // Catania × L'Aquila
    (9, 11, 2001, 0x9ea5d8, 0x9ea5a0), // Fermana × Torres
    (9, 11, 2001, 0x9ea5f8, 0x9ea620), // Giulianova × Avellino
    (9, 11, 2001, 0x9ea5e0, 0x9ea638), // Lodigiani × Ascoli
    (9, 11, 2001, 0x9ea5c4, 0x9ea5a8), // Nocerina × Taranto
    (9, 11, 2001, 0x9ea5d0, 0x9ea640), // Pescara × Castel di Sangro
    (9, 11, 2001, 0x9ea5bc, 0x9ea604), // Sora × Chieti
    (9, 11, 2001, 0x9ea594, 0x9ea5ec), // Viterbese × Lanciano
    (16, 11, 2001, 0x9ea638, 0x9ea5f8), // Ascoli × Giulianova
    (16, 11, 2001, 0x9ea620, 0x9ea5e0), // Avellino × Lodigiani
    (16, 11, 2001, 0x9ea640, 0x9ea60c), // Castel di Sangro × Catania
    (16, 11, 2001, 0x9ea604, 0x9ea594), // Chieti × Viterbese
    (16, 11, 2001, 0x9ea62c, 0x9ea614), // L'Aquila × Benevento
    (16, 11, 2001, 0x9ea5ec, 0x9ea5d8), // Lanciano × Fermana
    (16, 11, 2001, 0x9ea5a0, 0x9ea5c4), // Torres × Nocerina
    (16, 11, 2001, 0x9ea5a8, 0x9ea5bc), // Taranto × Sora
    (16, 11, 2001, 0x9ea5b0, 0x9ea5d0), // Vis Pesaro × Pescara
    (23, 11, 2001, 0x9ea614, 0x9ea604), // Benevento × Chieti
    (23, 11, 2001, 0x9ea60c, 0x9ea620), // Catania × Avellino
    (23, 11, 2001, 0x9ea5d8, 0x9ea62c), // Fermana × L'Aquila
    (23, 11, 2001, 0x9ea5f8, 0x9ea5ec), // Giulianova × Lanciano
    (23, 11, 2001, 0x9ea5e0, 0x9ea5a8), // Lodigiani × Taranto
    (23, 11, 2001, 0x9ea5c4, 0x9ea640), // Nocerina × Castel di Sangro
    (23, 11, 2001, 0x9ea5d0, 0x9ea5a0), // Pescara × Torres
    (23, 11, 2001, 0x9ea5bc, 0x9ea5b0), // Sora × Vis Pesaro
    (23, 11, 2001, 0x9ea594, 0x9ea638), // Viterbese × Ascoli
    (6, 0, 2002, 0x9ea640, 0x9ea638), // Castel di Sangro × Ascoli
    (6, 0, 2002, 0x9ea62c, 0x9ea620), // L'Aquila × Avellino
    (6, 0, 2002, 0x9ea614, 0x9ea60c), // Benevento × Catania
    (6, 0, 2002, 0x9ea604, 0x9ea5f8), // Chieti × Giulianova
    (6, 0, 2002, 0x9ea5ec, 0x9ea5e0), // Lanciano × Lodigiani
    (6, 0, 2002, 0x9ea5d8, 0x9ea5d0), // Fermana × Pescara
    (6, 0, 2002, 0x9ea5c4, 0x9ea5bc), // Nocerina × Sora
    (6, 0, 2002, 0x9ea5b0, 0x9ea5a8), // Vis Pesaro × Taranto
    (6, 0, 2002, 0x9ea5a0, 0x9ea594), // Torres × Viterbese
    (13, 0, 2002, 0x9ea594, 0x9ea614), // Viterbese × Benevento
    (13, 0, 2002, 0x9ea5f8, 0x9ea640), // Giulianova × Castel di Sangro
    (13, 0, 2002, 0x9ea5d0, 0x9ea604), // Pescara × Chieti
    (13, 0, 2002, 0x9ea5e0, 0x9ea5d8), // Lodigiani × Fermana
    (13, 0, 2002, 0x9ea5bc, 0x9ea62c), // Sora × L'Aquila
    (13, 0, 2002, 0x9ea5a8, 0x9ea5ec), // Taranto × Lanciano
    (13, 0, 2002, 0x9ea620, 0x9ea5c4), // Avellino × Nocerina
    (13, 0, 2002, 0x9ea60c, 0x9ea5a0), // Catania × Torres
    (13, 0, 2002, 0x9ea638, 0x9ea5b0), // Ascoli × Vis Pesaro
    (20, 0, 2002, 0x9ea604, 0x9ea638), // Chieti × Ascoli
    (20, 0, 2002, 0x9ea5a0, 0x9ea620), // Torres × Avellino
    (20, 0, 2002, 0x9ea5ec, 0x9ea60c), // Lanciano × Catania
    (20, 0, 2002, 0x9ea594, 0x9ea5f8), // Viterbese × Giulianova
    (20, 0, 2002, 0x9ea614, 0x9ea5e0), // Benevento × Lodigiani
    (20, 0, 2002, 0x9ea62c, 0x9ea5d0), // L'Aquila × Pescara
    (20, 0, 2002, 0x9ea5d8, 0x9ea5bc), // Fermana × Sora
    (20, 0, 2002, 0x9ea640, 0x9ea5a8), // Castel di Sangro × Taranto
    (20, 0, 2002, 0x9ea5c4, 0x9ea5b0), // Nocerina × Vis Pesaro
    (27, 0, 2002, 0x9ea5d0, 0x9ea614), // Pescara × Benevento
    (27, 0, 2002, 0x9ea620, 0x9ea640), // Avellino × Castel di Sangro
    (27, 0, 2002, 0x9ea5a8, 0x9ea604), // Taranto × Chieti
    (27, 0, 2002, 0x9ea60c, 0x9ea5d8), // Catania × Fermana
    (27, 0, 2002, 0x9ea5f8, 0x9ea62c), // Giulianova × L'Aquila
    (27, 0, 2002, 0x9ea638, 0x9ea5ec), // Ascoli × Lanciano
    (27, 0, 2002, 0x9ea5e0, 0x9ea5c4), // Lodigiani × Nocerina
    (27, 0, 2002, 0x9ea5b0, 0x9ea5a0), // Vis Pesaro × Torres
    (27, 0, 2002, 0x9ea5bc, 0x9ea594), // Sora × Viterbese
    (3, 1, 2002, 0x9ea5a0, 0x9ea638), // Torres × Ascoli
    (3, 1, 2002, 0x9ea5ec, 0x9ea620), // Lanciano × Avellino
    (3, 1, 2002, 0x9ea604, 0x9ea640), // Chieti × Castel di Sangro
    (3, 1, 2002, 0x9ea5c4, 0x9ea5f8), // Nocerina × Giulianova
    (3, 1, 2002, 0x9ea594, 0x9ea5e0), // Viterbese × Lodigiani
    (3, 1, 2002, 0x9ea60c, 0x9ea5d0), // Catania × Pescara
    (3, 1, 2002, 0x9ea614, 0x9ea5bc), // Benevento × Sora
    (3, 1, 2002, 0x9ea5d8, 0x9ea5a8), // Fermana × Taranto
    (3, 1, 2002, 0x9ea62c, 0x9ea5b0), // L'Aquila × Vis Pesaro
    (10, 1, 2002, 0x9ea5bc, 0x9ea620), // Sora × Avellino
    (10, 1, 2002, 0x9ea5f8, 0x9ea614), // Giulianova × Benevento
    (10, 1, 2002, 0x9ea5e0, 0x9ea60c), // Lodigiani × Catania
    (10, 1, 2002, 0x9ea5c4, 0x9ea604), // Nocerina × Chieti
    (10, 1, 2002, 0x9ea5b0, 0x9ea5d8), // Vis Pesaro × Fermana
    (10, 1, 2002, 0x9ea638, 0x9ea62c), // Ascoli × L'Aquila
    (10, 1, 2002, 0x9ea640, 0x9ea5ec), // Castel di Sangro × Lanciano
    (10, 1, 2002, 0x9ea5a8, 0x9ea5a0), // Taranto × Torres
    (10, 1, 2002, 0x9ea5d0, 0x9ea594), // Pescara × Viterbese
    (17, 1, 2002, 0x9ea614, 0x9ea638), // Benevento × Ascoli
    (17, 1, 2002, 0x9ea5a0, 0x9ea640), // Torres × Castel di Sangro
    (17, 1, 2002, 0x9ea5d8, 0x9ea5f8), // Fermana × Giulianova
    (17, 1, 2002, 0x9ea604, 0x9ea5ec), // Chieti × Lanciano
    (17, 1, 2002, 0x9ea5d0, 0x9ea5e0), // Pescara × Lodigiani
    (17, 1, 2002, 0x9ea594, 0x9ea5c4), // Viterbese × Nocerina
    (17, 1, 2002, 0x9ea60c, 0x9ea5bc), // Catania × Sora
    (17, 1, 2002, 0x9ea62c, 0x9ea5a8), // L'Aquila × Taranto
    (17, 1, 2002, 0x9ea620, 0x9ea5b0), // Avellino × Vis Pesaro
    (3, 2, 2002, 0x9ea604, 0x9ea620), // Chieti × Avellino
    (3, 2, 2002, 0x9ea5c4, 0x9ea614), // Nocerina × Benevento
    (3, 2, 2002, 0x9ea5b0, 0x9ea60c), // Vis Pesaro × Catania
    (3, 2, 2002, 0x9ea640, 0x9ea5d8), // Castel di Sangro × Fermana
    (3, 2, 2002, 0x9ea5e0, 0x9ea62c), // Lodigiani × L'Aquila
    (3, 2, 2002, 0x9ea5f8, 0x9ea5d0), // Giulianova × Pescara
    (3, 2, 2002, 0x9ea5ec, 0x9ea5a0), // Lanciano × Torres
    (3, 2, 2002, 0x9ea638, 0x9ea5bc), // Ascoli × Sora
    (3, 2, 2002, 0x9ea5a8, 0x9ea594), // Taranto × Viterbese
    (10, 2, 2002, 0x9ea620, 0x9ea638), // Avellino × Ascoli
    (10, 2, 2002, 0x9ea5e0, 0x9ea640), // Lodigiani × Castel di Sangro
    (10, 2, 2002, 0x9ea5b0, 0x9ea604), // Vis Pesaro × Chieti
    (10, 2, 2002, 0x9ea5bc, 0x9ea5f8), // Sora × Giulianova
    (10, 2, 2002, 0x9ea62c, 0x9ea5ec), // L'Aquila × Lanciano
    (10, 2, 2002, 0x9ea5d0, 0x9ea5c4), // Pescara × Nocerina
    (10, 2, 2002, 0x9ea614, 0x9ea5a0), // Benevento × Torres
    (10, 2, 2002, 0x9ea60c, 0x9ea5a8), // Catania × Taranto
    (10, 2, 2002, 0x9ea5d8, 0x9ea594), // Fermana × Viterbese
    (17, 2, 2002, 0x9ea5a8, 0x9ea620), // Taranto × Avellino
    (17, 2, 2002, 0x9ea640, 0x9ea614), // Castel di Sangro × Benevento
    (17, 2, 2002, 0x9ea594, 0x9ea60c), // Viterbese × Catania
    (17, 2, 2002, 0x9ea5c4, 0x9ea5d8), // Nocerina × Fermana
    (17, 2, 2002, 0x9ea604, 0x9ea62c), // Chieti × L'Aquila
    (17, 2, 2002, 0x9ea5f8, 0x9ea5e0), // Giulianova × Lodigiani
    (17, 2, 2002, 0x9ea638, 0x9ea5d0), // Ascoli × Pescara
    (17, 2, 2002, 0x9ea5a0, 0x9ea5bc), // Torres × Sora
    (17, 2, 2002, 0x9ea5ec, 0x9ea5b0), // Lanciano × Vis Pesaro
    (24, 2, 2002, 0x9ea5a8, 0x9ea638), // Taranto × Ascoli
    (24, 2, 2002, 0x9ea5d8, 0x9ea614), // Fermana × Benevento
    (24, 2, 2002, 0x9ea5b0, 0x9ea640), // Vis Pesaro × Castel di Sangro
    (24, 2, 2002, 0x9ea5e0, 0x9ea604), // Lodigiani × Chieti
    (24, 2, 2002, 0x9ea60c, 0x9ea5f8), // Catania × Giulianova
    (24, 2, 2002, 0x9ea5ec, 0x9ea5c4), // Lanciano × Nocerina
    (24, 2, 2002, 0x9ea5bc, 0x9ea5d0), // Sora × Pescara
    (24, 2, 2002, 0x9ea62c, 0x9ea5a0), // L'Aquila × Torres
    (24, 2, 2002, 0x9ea620, 0x9ea594), // Avellino × Viterbese
    (30, 2, 2002, 0x9ea614, 0x9ea620), // Benevento × Avellino
    (30, 2, 2002, 0x9ea5c4, 0x9ea60c), // Nocerina × Catania
    (30, 2, 2002, 0x9ea5a0, 0x9ea604), // Torres × Chieti
    (30, 2, 2002, 0x9ea638, 0x9ea5d8), // Ascoli × Fermana
    (30, 2, 2002, 0x9ea640, 0x9ea62c), // Castel di Sangro × L'Aquila
    (30, 2, 2002, 0x9ea5d0, 0x9ea5ec), // Pescara × Lanciano
    (30, 2, 2002, 0x9ea5bc, 0x9ea5e0), // Sora × Lodigiani
    (30, 2, 2002, 0x9ea5f8, 0x9ea5a8), // Giulianova × Taranto
    (30, 2, 2002, 0x9ea594, 0x9ea5b0), // Viterbese × Vis Pesaro
    (7, 3, 2002, 0x9ea5a8, 0x9ea614), // Taranto × Benevento
    (7, 3, 2002, 0x9ea638, 0x9ea60c), // Ascoli × Catania
    (7, 3, 2002, 0x9ea604, 0x9ea5d8), // Chieti × Fermana
    (7, 3, 2002, 0x9ea5a0, 0x9ea5f8), // Torres × Giulianova
    (7, 3, 2002, 0x9ea5b0, 0x9ea5e0), // Vis Pesaro × Lodigiani
    (7, 3, 2002, 0x9ea62c, 0x9ea5c4), // L'Aquila × Nocerina
    (7, 3, 2002, 0x9ea620, 0x9ea5d0), // Avellino × Pescara
    (7, 3, 2002, 0x9ea5ec, 0x9ea5bc), // Lanciano × Sora
    (7, 3, 2002, 0x9ea640, 0x9ea594), // Castel di Sangro × Viterbese
    (14, 3, 2002, 0x9ea5c4, 0x9ea638), // Nocerina × Ascoli
    (14, 3, 2002, 0x9ea5d8, 0x9ea620), // Fermana × Avellino
    (14, 3, 2002, 0x9ea5bc, 0x9ea640), // Sora × Castel di Sangro
    (14, 3, 2002, 0x9ea60c, 0x9ea604), // Catania × Chieti
    (14, 3, 2002, 0x9ea594, 0x9ea62c), // Viterbese × L'Aquila
    (14, 3, 2002, 0x9ea614, 0x9ea5ec), // Benevento × Lanciano
    (14, 3, 2002, 0x9ea5e0, 0x9ea5a0), // Lodigiani × Torres
    (14, 3, 2002, 0x9ea5d0, 0x9ea5a8), // Pescara × Taranto
    (14, 3, 2002, 0x9ea5f8, 0x9ea5b0), // Giulianova × Vis Pesaro
    (21, 3, 2002, 0x9ea5b0, 0x9ea614), // Vis Pesaro × Benevento
    (21, 3, 2002, 0x9ea62c, 0x9ea60c), // L'Aquila × Catania
    (21, 3, 2002, 0x9ea5a0, 0x9ea5d8), // Torres × Fermana
    (21, 3, 2002, 0x9ea620, 0x9ea5f8), // Avellino × Giulianova
    (21, 3, 2002, 0x9ea638, 0x9ea5e0), // Ascoli × Lodigiani
    (21, 3, 2002, 0x9ea5a8, 0x9ea5c4), // Taranto × Nocerina
    (21, 3, 2002, 0x9ea640, 0x9ea5d0), // Castel di Sangro × Pescara
    (21, 3, 2002, 0x9ea604, 0x9ea5bc), // Chieti × Sora
    (21, 3, 2002, 0x9ea5ec, 0x9ea594), // Lanciano × Viterbese
    (28, 3, 2002, 0x9ea5f8, 0x9ea638), // Giulianova × Ascoli
    (28, 3, 2002, 0x9ea5e0, 0x9ea620), // Lodigiani × Avellino
    (28, 3, 2002, 0x9ea60c, 0x9ea640), // Catania × Castel di Sangro
    (28, 3, 2002, 0x9ea594, 0x9ea604), // Viterbese × Chieti
    (28, 3, 2002, 0x9ea614, 0x9ea62c), // Benevento × L'Aquila
    (28, 3, 2002, 0x9ea5d8, 0x9ea5ec), // Fermana × Lanciano
    (28, 3, 2002, 0x9ea5c4, 0x9ea5a0), // Nocerina × Torres
    (28, 3, 2002, 0x9ea5bc, 0x9ea5a8), // Sora × Taranto
    (28, 3, 2002, 0x9ea5d0, 0x9ea5b0), // Pescara × Vis Pesaro
    (5, 4, 2002, 0x9ea604, 0x9ea614), // Chieti × Benevento
    (5, 4, 2002, 0x9ea620, 0x9ea60c), // Avellino × Catania
    (5, 4, 2002, 0x9ea62c, 0x9ea5d8), // L'Aquila × Fermana
    (5, 4, 2002, 0x9ea5ec, 0x9ea5f8), // Lanciano × Giulianova
    (5, 4, 2002, 0x9ea5a8, 0x9ea5e0), // Taranto × Lodigiani
    (5, 4, 2002, 0x9ea640, 0x9ea5c4), // Castel di Sangro × Nocerina
    (5, 4, 2002, 0x9ea5a0, 0x9ea5d0), // Torres × Pescara
    (5, 4, 2002, 0x9ea5b0, 0x9ea5bc), // Vis Pesaro × Sora
    (5, 4, 2002, 0x9ea638, 0x9ea594), // Ascoli × Viterbese
];

