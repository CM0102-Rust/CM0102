// ---------- %%LEVA%%a leva: as sedes dos torneios: a tabela padrão, a notícia das candidaturas e a carga do HostCountry.tmp (host_country.cpp) ----------

unsafe fn cqo3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqo3_pd(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

unsafe fn cqo3_w(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cqo3_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

/// As datas de referência das decisões (Date::set 0x54a130: dia do mês, mês a partir de 0, ano 1997, sem dia da
/// semana): o dia do ano delas vai ao +0x1f das entradas do laço seguinte.
const HOST_DECISION_DATES: [[u32; 4]; 7] = [
    [0xa, 0x6, 0x7cd, 0xffffffff], // cqo3: 10 de julho, a Copa do Mundo
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x8, 0x7cd, 0xffffffff],
];

/// Os laços das entradas de uma competição: (início na tabela, entradas, a competição [global], primeiro ano, passo
/// dos anos, intervalo +0x21). Na ordem: a Copa do Mundo ([0x9cf79c], 1998..2026 de 4 em 4), a Eurocopa ([0x9cf7a4],
/// 2000..2028), a Copa Africana ([0x9cf7ac], 2000..2014 de 2 em 2), as Olimpíadas ([0x9cf7a0], 2000..2028), a Copa
/// América ([0x9cf7a8], 1999..2009 de 2 em 2), [0x9cf95c] (1999..2004, ano a ano) e a Copa da Ásia ([0x9cf790],
/// 2000..2016) [inferência: os nomes pelos endereços da comunidade, docs/ENDERECOS_COMUNIDADE.md].
const HOST_COUNTRY_LOOPS: [(u16, u8, u32, u16, u16, u8); 7] = [
    (0x0, 8, 0x9cf79c, 1998, 4, 0x20),
    (0x110, 8, 0x9cf7a4, 2000, 4, 0x20), // cqo3: a Eurocopa
    (0x220, 8, 0x9cf7ac, 2000, 2, 0x10), // cqo3: a Copa Africana
    (0x330, 8, 0x9cf7a0, 2000, 4, 0x20),
    (0x462, 6, 0x9cf7a8, 1999, 2, 0xc), // cqo3: a Copa América
    (0x5fa, 6, 0x9cf95c, 1999, 1, 0x6), // cqo3: as edições anuais
    (0x6c6, 5, 0x9cf790, 2000, 4, 0x14),
];

/// As gravações da tabela padrão, na ordem da original (geradas da desmontagem de 0x5f999a..0x5fa640): (deslocamento,
/// tipo, valor); tipo 0 = dword do global `valor` (uma nação ou competição), 1 = dword `valor` (−1 = sem sede), 2 =
/// byte (o +0x1e de cada entrada: 0/1 a candidatura escolhida, −1 votação aberta, −2 fechada, −3 a sede padrão
/// fixa), 3 = word 0 (ano 0 e dia 0 das entradas sem edição), 4 = a data `HOST_DECISION_DATES[valor]`, 5 = o laço
/// `HOST_COUNTRY_LOOPS[valor]`.
const HOST_COUNTRY_TABLE: [(u16, u8, u32); 281] = [
    (0x0, 4, 0x0), (0x0, 5, 0x0), (0x6, 0, 0x9cf300), (0xa, 1, 0xffffffff), (0xe, 0, 0x9cf488), (0x12, 1, 0xffffffff),
    (0x16, 0, 0x9cf434), (0x1a, 1, 0xffffffff), (0x1e, 2, 0x0), (0x28, 0, 0x9cf484), (0x2c, 0, 0x9cf36c), (0x30, 0, 0x9cf2e4),
    (0x34, 1, 0xffffffff), (0x38, 0, 0x9cf360), (0x3c, 1, 0xffffffff), (0x40, 2, 0x0), (0x4a, 0, 0x9cf314), (0x4e, 1, 0xffffffff),
    (0x52, 0, 0x9cf2e4), (0x56, 1, 0xffffffff), (0x5a, 0, 0x9cf218), (0x5e, 1, 0xffffffff), (0x62, 2, 0x0), (0x6c, 0, 0x9cf260),
    (0x70, 1, 0xffffffff), (0x74, 0, 0x9cf218), (0x78, 1, 0xffffffff), (0x7c, 0, 0x9cf480), (0x80, 1, 0xffffffff), (0x84, 2, 0xfe),
    (0x8e, 0, 0x9cf4a8), (0x92, 0, 0x9cf408), (0x96, 0, 0x9cf488), (0x9a, 1, 0xffffffff), (0x9e, 0, 0x9cf4ac), (0xa2, 1, 0xffffffff),
    (0xa6, 2, 0xfe), (0xb0, 0, 0x9cf3d0), (0xb4, 1, 0xffffffff), (0xb8, 0, 0x9cf4f4), (0xbc, 1, 0xffffffff), (0xc0, 0, 0x9cf280),
    (0xc4, 1, 0xffffffff), (0xc8, 2, 0xfe), (0xd2, 0, 0x9cf360), (0xd6, 1, 0xffffffff), (0xda, 0, 0x9cf314), (0xde, 1, 0xffffffff),
    (0xe2, 0, 0x9cf2e4), (0xe6, 1, 0xffffffff), (0xea, 2, 0xfe), (0xf4, 0, 0x9cf260), (0xf8, 1, 0xffffffff), (0xfc, 0, 0x9cf218),
    (0x100, 1, 0xffffffff), (0x104, 0, 0x9cf4f8), (0x108, 1, 0xffffffff), (0x10c, 2, 0xfe), (0x0, 4, 0x1), (0x0, 5, 0x1),
    (0x116, 0, 0x9cf338), (0x11a, 0, 0x9cf244), (0x11e, 0, 0x9cf360), (0x122, 1, 0xffffffff), (0x126, 0, 0x9cf408), (0x12a, 1, 0xffffffff),
    (0x12e, 2, 0x0), (0x138, 0, 0x9cf344), (0x13c, 0, 0x9cf228), (0x140, 0, 0x9cf434), (0x144, 1, 0xffffffff), (0x148, 0, 0x9cf488),
    (0x14c, 1, 0xffffffff), (0x150, 2, 0x1), (0x15a, 0, 0x9cf45c), (0x15e, 0, 0x9cf510), (0x162, 0, 0x9cf4a8), (0x166, 1, 0xffffffff),
    (0x16a, 0, 0x9cf4ac), (0x16e, 1, 0xffffffff), (0x172, 2, 0xfe), (0x17c, 0, 0x9cf300), (0x180, 1, 0xffffffff), (0x184, 0, 0x9cf408),
    (0x188, 1, 0xffffffff), (0x18c, 0, 0x9cf2c0), (0x190, 1, 0xffffffff), (0x194, 2, 0xfe), (0x19e, 0, 0x9cf2dc), (0x1a2, 1, 0xffffffff),
    (0x1a6, 0, 0x9cf4ac), (0x1aa, 1, 0xffffffff), (0x1ae, 0, 0x9cf51c), (0x1b2, 1, 0xffffffff), (0x1b6, 2, 0xfe), (0x1c0, 0, 0x9cf4ac),
    (0x1c4, 1, 0xffffffff), (0x1c8, 0, 0x9cf244), (0x1cc, 1, 0xffffffff), (0x1d0, 0, 0x9cf338), (0x1d4, 1, 0xffffffff), (0x1d8, 2, 0xfe),
    (0x1e2, 0, 0x9cf360), (0x1e6, 1, 0xffffffff), (0x1ea, 0, 0x9cf314), (0x1ee, 1, 0xffffffff), (0x1f2, 0, 0x9cf2e4), (0x1f6, 1, 0xffffffff),
    (0x1fa, 2, 0xfe), (0x204, 0, 0x9cf31c), (0x208, 1, 0xffffffff), (0x20c, 0, 0x9cf488), (0x210, 1, 0xffffffff), (0x214, 0, 0x9cf300),
    (0x218, 1, 0xffffffff), (0x21c, 2, 0xfe), (0x0, 4, 0x2), (0x0, 5, 0x2), (0x226, 0, 0x9cf318), (0x22a, 0, 0x9cf400),
    (0x23e, 2, 0xfd), (0x248, 0, 0x9cf2d8), (0x24c, 1, 0xffffffff), (0x260, 2, 0xfd), (0x26a, 0, 0x9cf49c), (0x26e, 1, 0xffffffff),
    (0x282, 2, 0xfd), (0x28c, 0, 0x9cf4d8), (0x290, 1, 0xffffffff), (0x2a4, 2, 0xfd), (0x2ae, 0, 0x9cf27c), (0x2b2, 1, 0xffffffff),
    (0x2c6, 2, 0xfd), (0x2d0, 0, 0x9cf400), (0x2d4, 1, 0xffffffff), (0x2e8, 2, 0xfd), (0x2f2, 0, 0x9cf3d8), (0x2f6, 1, 0xffffffff),
    (0x30a, 2, 0xfd), (0x314, 0, 0x9cf208), (0x318, 1, 0xffffffff), (0x32c, 2, 0xfd), (0x0, 4, 0x3), (0x0, 5, 0x3),
    (0x336, 0, 0x9cf224), (0x33a, 1, 0xffffffff), (0x34e, 2, 0xfd), (0x358, 0, 0x9cf31c), (0x35c, 1, 0xffffffff), (0x370, 2, 0xfd),
    (0x37a, 0, 0x9cf314), (0x37e, 1, 0xffffffff), (0x392, 2, 0xfd), (0x39c, 0, 0x9cf360), (0x3a0, 1, 0xffffffff), (0x3b4, 2, 0xfd),
    (0x3be, 0, 0x9cf36c), (0x3c2, 1, 0xffffffff), (0x3d6, 2, 0xfd), (0x3e0, 0, 0x9cf3d0), (0x3e4, 1, 0xffffffff), (0x3f8, 2, 0xfd),
    (0x402, 0, 0x9cf280), (0x406, 1, 0xffffffff), (0x41a, 2, 0xfd), (0x424, 0, 0x9cf488), (0x428, 1, 0xffffffff), (0x43c, 2, 0xfd),
    (0x440, 0, 0x9cf6e4), (0x444, 3, 0x0), (0x45f, 3, 0x0), (0x461, 2, 0x0), (0x446, 0, 0x9cf36c), (0x44a, 1, 0xffffffff),
    (0x45e, 2, 0xfd), (0x0, 4, 0x4), (0x0, 5, 0x4), (0x468, 0, 0x9cf420), (0x46c, 1, 0xffffffff), (0x480, 2, 0xfd),
    (0x48a, 0, 0x9cf29c), (0x48e, 1, 0xffffffff), (0x4a2, 2, 0xfd), (0x4ac, 0, 0x9cf428), (0x4b0, 1, 0xffffffff), (0x4c4, 2, 0xfd),
    (0x4ce, 0, 0x9cf504), (0x4d2, 1, 0xffffffff), (0x4e6, 2, 0xfd), (0x4f0, 0, 0x9cf218), (0x4f4, 1, 0xffffffff), (0x508, 2, 0xfd),
    (0x512, 0, 0x9cf260), (0x516, 1, 0xffffffff), (0x52a, 2, 0xfd), (0x52e, 0, 0x9cf6e8), (0x532, 3, 0x0), (0x54d, 3, 0x0),
    (0x54f, 2, 0x0), (0x534, 0, 0x9cf45c), (0x538, 1, 0xffffffff), (0x54c, 2, 0xfd), (0x550, 0, 0x9cf6f0), (0x554, 3, 0x0),
    (0x56f, 3, 0x0), (0x571, 2, 0x0), (0x556, 0, 0x9cf2c0), (0x55a, 1, 0xffffffff), (0x56e, 2, 0xfd), (0x572, 0, 0x9cf6ec),
    (0x576, 3, 0x0), (0x591, 3, 0x0), (0x593, 2, 0x0), (0x578, 0, 0x9cf300), (0x57c, 1, 0xffffffff), (0x590, 2, 0xfd),
    (0x594, 0, 0x9cf7bc), (0x598, 3, 0x0), (0x5b3, 3, 0x0), (0x5b5, 2, 0x0), (0x59a, 0, 0x9cf488), (0x59e, 1, 0xffffffff),
    (0x5b2, 2, 0xfd), (0x5b6, 0, 0x9cf7cc), (0x5ba, 3, 0x0), (0x5d5, 3, 0x0), (0x5d7, 2, 0x0), (0x5bc, 0, 0x9cf224),
    (0x5c0, 1, 0xffffffff), (0x5d4, 2, 0xfd), (0x5d8, 0, 0x9cf76c), (0x5dc, 3, 0x0), (0x5f7, 3, 0x0), (0x5f9, 2, 0x0),
    (0x5de, 0, 0x9cf224), (0x5e2, 0, 0x9cf3f4), (0x5f6, 2, 0xfd), (0x0, 4, 0x5), (0x0, 5, 0x5), (0x600, 0, 0x9cf420),
    (0x604, 1, 0xffffffff), (0x618, 2, 0xfd), (0x622, 0, 0x9cf4f8), (0x626, 1, 0xffffffff), (0x63a, 2, 0xfd), (0x644, 0, 0x9cf218),
    (0x648, 1, 0xffffffff), (0x65c, 2, 0xfd), (0x666, 0, 0x9cf3d0), (0x66a, 1, 0xffffffff), (0x67e, 2, 0xfd), (0x688, 0, 0x9cf4f4),
    (0x68c, 1, 0xffffffff), (0x6a0, 2, 0xfd), (0x6aa, 0, 0x9cf260), (0x6ae, 1, 0xffffffff), (0x6c2, 2, 0xfd), (0x0, 4, 0x6),
    (0x0, 5, 0x6), (0x6cc, 0, 0x9cf390), (0x6d0, 1, 0xffffffff), (0x6e4, 2, 0xfd), (0x6ee, 0, 0x9cf46c), (0x6f2, 1, 0xffffffff),
    (0x706, 2, 0xfd), (0x710, 0, 0x9cf36c), (0x714, 1, 0xffffffff), (0x728, 2, 0xfd), (0x732, 0, 0x9cf298), (0x736, 1, 0xffffffff),
    (0x74a, 2, 0xfd), (0x754, 0, 0x9cf484), (0x758, 1, 0xffffffff), (0x76c, 2, 0xfd), (0x770, 0, 0x9cf964), (0x774, 3, 0x0),
    (0x78f, 3, 0x0), (0x791, 2, 0x0), (0x776, 0, 0x9cf484), (0x77a, 1, 0xffffffff), (0x78e, 2, 0xfd),
];

/// { 0x5f9970 } (thiscall, ret 0, host_country.cpp, `HostCountry::init_default_table`; pelo construtor 0x5f97c0, sem o
/// HostCountry.tmp; quadro SEH só pela Data local): a tabela padrão das sedes (57 entradas de 0x22 bytes em [this],
/// conta em +4: +0 a competição, +4 o ano, +6/+0xa, +0xe/+0x12, +0x16/+0x1a as três candidaturas (duas nações cada),
/// +0x1e a escolhida, +0x1f o dia da decisão, +0x21 o intervalo entre edições), pelas gravações de `HOST_COUNTRY_TABLE`;
/// depois o dia da decisão de cada entrada anda um sorteio de −35 a +34 (0x90cfc0(70)); a entrada que não é padrão
/// fixa (−3) e tem segunda ou terceira candidatura completa leva a caixa (linha 0x226) e perde as duas.
unsafe extern "thiscall" fn init_default_table(this: usize) {
    let definir: unsafe extern "thiscall" fn(*mut crate::date::Data, u32, u32, u32, u32) -> usize = core::mem::transmute(0x54a130usize);
    let mut d = crate::date::Data { dia: 40, ano: 1972, is_leap_year: 1 };
    for &(off, t, v) in HOST_COUNTRY_TABLE.iter() {
        let e = cqo3_d(this) + off as usize;
        match t {
            0 => cqo3_pd(e, cqo3_d(v as usize) as u32),
            1 => cqo3_pd(e, v),
            2 => *(e as *mut u8) = v as u8,
            3 => cqo3_pw(e, v as u16),
            4 => {
                let a = HOST_DECISION_DATES[v as usize];
                definir(&mut d, a[0], a[1], a[2], a[3]);
            }
            _ => {
                let (ini, n, g, ano, step_towards, intervalo) = HOST_COUNTRY_LOOPS[v as usize];
                let mut e = cqo3_d(this) + ini as usize;
                let mut y = ano;
                for _ in 0..n {
                    cqo3_pw(e + 4, y);
                    cqo3_pd(e, cqo3_d(g as usize) as u32);
                    cqo3_pw(e + 0x1f, d.dia as u16);
                    *((e + 0x21) as *mut u8) = intervalo;
                    e += 0x22;
                    y = y.wrapping_add(step_towards);
                }
            }
        }
    }
    let sortear: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let n = cqo3_w(this + 4) as i16;
    let mut k: i16 = 0;
    while k < n {
        let o = (k as i32 * 0x22) as usize;
        let p = cqo3_d(this) + o + 0x1f;
        crate::comparison::branch("cqo_3 day draw");
        let r = sortear(0x46); // cqo3: o sorteio do dia da decisão
        cqo3_pw(p, cqo3_w(p).wrapping_add((r - 0x23) as u16));
        let e = cqo3_d(this) + o;
        if *((e + 0x1e) as *const u8) != 0xfd && (cqo3_d(e + 0x12) as u32 != u32::MAX || cqo3_d(e + 0x1a) as u32 != u32::MAX) {
            show_error_at(0x9cf0d8, 0x226);
            cqo3_pd(cqo3_d(this) + o + 0x12, u32::MAX);
            cqo3_pd(cqo3_d(this) + o + 0x1a, u32::MAX);
        }
        k += 1;
    }
}

/// Lado a lado (a cópia sem contar os sorteios) pela tabela (conta +4 × 0x22 bytes), pelo estado do sorteador e por
/// [0xb67a34].
unsafe extern "thiscall" fn compare_init_default_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let n = cqo3_w(this + 4) as i16;
    if cqo3_d(this) == 0 || n <= 0 {
        return o(this);
    }
    let mut reg = crate::rng::STATE_REGIONS.to_vec();
    reg.extend([(cqo3_d(this), n as usize * 0x22), (0xb67a34, 4)]);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this), || crate::rng::without_counting(|| init_default_table(this)));
    SCOREBOARD.record(igual, || format!("init_default_table: {:x?} x {:x?}", mo, mr).chars().take(4000).collect());
}

/// { 0x5faec0 } (thiscall, ret 4, host_country.cpp, `HostCountry::post_host_news(lista)`; pelo calendário das sedes
/// 0x5fa850; quadro SEH só pelas notícias locais, sem destrutor): para cada entrada da lista (cursor reposto, 0x68d1b0),
/// fora a candidatura escolhida (ou a padrão fixa) da edição de daqui a 8 anos ([0xae2c92] + 8), as Olimpíadas
/// ([0x9cf7a0]) e a Copa do Mundo de 2006 na Alemanha ([0x9cf79c], 0x7d6, [0x9cf314]): com a votação aberta (−1) a
/// notícia 4 (as candidaturas: competição, ano e as seis nações); senão a notícia 5 (a escolhida: competição, ano e as
/// duas sedes dela, a primeira na padrão fixa), montadas por 0x77bbe0/0x785790 e entregues aos técnicos do continente
/// da competição (tabela [0xae23d0], 0x6b bytes, +0x59; 0x786640) ou a todos (0x786780).
unsafe extern "thiscall" fn post_host_news(_this: usize, lista: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let dado: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x785790usize);
    let continente: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x786640usize);
    let todos: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x786780usize);
    crate::comparison::branch("cqo_3 hosts news");
    cqo3_pd(lista + 0xa, cqo3_d(lista + 2) as u32);
    let mut it = proximo(lista);
    while it != 0 {
        let e = cqo3_d(it);
        let k = *((e + 0x1e) as *const i8);
        let pular = ((k > 0 || k == -3) && cqo3_w(0xae2c92) as u32 == (cqo3_w(e + 4) as u32).wrapping_sub(8))
            || cqo3_d(e) == cqo3_d(0x9cf7a0)
            || (cqo3_d(e) == cqo3_d(0x9cf79c) && cqo3_w(e + 4) == 0x7d6 && cqo3_d(e + 6) == cqo3_d(0x9cf314));
        if !pular {
            // a notícia local (0xe8 bytes, como nas outras cópias)
            let mut n = [0u8; 0xe8];
            let p = n.as_mut_ptr() as usize;
            if k == -1 {
                construir(p, 4, 0);
                dado(p, 0, cqo3_d(cqo3_d(it)) as u32);
                dado(p, 1, cqo3_w(cqo3_d(it) + 4) as u32);
                for (i, off) in [6usize, 0xa, 0xe, 0x12, 0x16, 0x1a].iter().enumerate() {
                    dado(p, 2 + i as u32, cqo3_d(cqo3_d(it) + off) as u32);
                }
            } else {
                construir(p, 5, 0);
                let mut b = *((cqo3_d(it) + 0x1e) as *const i8);
                if b == -3 {
                    b = 0;
                }
                dado(p, 0, cqo3_d(cqo3_d(it)) as u32);
                dado(p, 1, cqo3_w(cqo3_d(it) + 4) as u32);
                let o = (b as i32 * 8) as usize;
                dado(p, 2, cqo3_d(cqo3_d(it).wrapping_add(o) + 6) as u32);
                dado(p, 3, cqo3_d(cqo3_d(it).wrapping_add(o) + 0xa) as u32);
            }
            let id = cqo3_d(cqo3_d(it));
            let c = cqo3_d(cqo3_d(0xae23d0).wrapping_add(id.wrapping_mul(0x6b)) + 0x59);
            if c != 0 {
                continente(cqo3_d(0xb7431c), p, c);
            } else {
                todos(cqo3_d(0xb7431c), p);
            }
        }
        it = proximo(lista);
    }
}

// ponytail: entrega notícias (aloca); só a original. A cópia leva o ramo.
unsafe extern "thiscall" fn compare_post_host_news(this: usize, lista: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(%%I1%%)(this, lista)
}

/// { 0x5fb160 } (thiscall, ret 0, host_country.cpp, `HostCountry::load_temp_file`; pelo construtor 0x5f97c0): lê o
/// HostCountry.tmp da pasta CM3_TEMP (0x90bde0, fopen 0x945b00 "rb"): a conta (word, +4) e as entradas de 0x22 bytes
/// num bloco novo ([this], 0x944e46); falhando a leitura, a caixa (linhas 0x448/0x452/0x45c), fecha (0x945a09) e, com
/// o bloco, o solta (0x9452ca). Devolve 1 com tudo lido, 0 sem o arquivo ou com erro.
unsafe extern "thiscall" fn load_temp_file(this: usize) -> u32 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x90bde0usize);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945b00usize);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(0x945bfeusize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let f = abrir(caminho(0x987da0, 0x9cf10c), 0x98f928);
    if f == 0 {
        crate::comparison::branch("cqo_3 without the file");
        return 0;
    }
    if ler(this + 4, 2, 1, f) != 1 {
        show_error_at(0x9cf0d8, 0x448);
        fechar(f);
        return 0;
    }
    let n = cqo3_w(this + 4) as i16 as i32;
    let b = alocar((n * 0x22) as u32);
    cqo3_pd(this, b as u32);
    if b == 0 {
        show_error_at(0x9cf0d8, 0x452);
        fechar(f);
        return 0;
    }
    let n = cqo3_w(this + 4) as i16 as i32 as u32;
    if ler(b, 0x22, n, f) != cqo3_w(this + 4) as i16 as i32 as u32 {
        show_error_at(0x9cf0d8, 0x45c);
        fechar(f);
        soltar(cqo3_d(this));
        return 0;
    }
    fechar(f);
    1
}

// ponytail: abre o arquivo e aloca; só a original. A cópia leva os ramos.
unsafe extern "thiscall" fn compare_load_temp_file(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%)(this)
}
