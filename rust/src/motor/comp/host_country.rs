use super::*;

// ---------- 280a leva: as sedes dos torneios: a tabela padrão, a notícia das candidaturas e a carga do HostCountry.tmp (host_country.cpp) ----------





/// As datas de referência das decisões (Date::set 0x54a130: dia do mês, mês a partir de 0, ano 1997, sem dia da
/// semana): o dia do ano delas vai ao +0x1f das entradas do laço seguinte.
pub(crate) const CQO3_DATAS: [[u32; 4]; 7] = [
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
pub(crate) const CQO3_LACOS: [(u16, u8, u32, u16, u16, u8); 7] = [
    (0x0, 8, KEY_CLUB_COMP_FIFA_WORLD_CUP as u32, 1998, 4, 0x20),
    (0x110, 8, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP as u32, 2000, 4, 0x20), // cqo3: a Eurocopa
    (0x220, 8, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS as u32, 2000, 2, 0x10), // cqo3: a Copa Africana
    (0x330, 8, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES as u32, 2000, 4, 0x20),
    (0x462, 6, KEY_CLUB_COMP_COPA_AMERICA as u32, 1999, 2, 0xc), // cqo3: a Copa América
    (0x5fa, 6, KEY_CLUB_COMP_INTER_AMERICAN_CUP as u32, 1999, 1, 0x6), // cqo3: as edições anuais
    (0x6c6, 5, KEY_CLUB_COMP_ASIAN_CUP as u32, 2000, 4, 0x14),
];

/// As gravações da tabela padrão, na ordem da original (geradas da desmontagem de 0x5f999a..0x5fa640): (deslocamento,
/// tipo, valor); tipo 0 = dword do global `valor` (uma nação ou competição), 1 = dword `valor` (−1 = sem sede), 2 =
/// byte (o +0x1e de cada entrada: 0/1 a candidatura escolhida, −1 votação aberta, −2 fechada, −3 a sede padrão
/// fixa), 3 = word 0 (ano 0 e dia 0 das entradas sem edição), 4 = a data `CQO3_DATAS[valor]`, 5 = o laço
/// `CQO3_LACOS[valor]`.
pub(crate) const CQO3_TABELA: [(u16, u8, u32); 281] = [
    (0x0, 4, 0x0), (0x0, 5, 0x0), (0x6, 0, KEY_NATION_FRANCE as u32), (0xa, 1, 0xffffffff), (0xe, 0, KEY_NATION_SPAIN as u32), (0x12, 1, 0xffffffff),
    (0x16, 0, KEY_NATION_PORTUGAL as u32), (0x1a, 1, 0xffffffff), (0x1e, 2, 0x0), (0x28, 0, KEY_NATION_SOUTH_KOREA as u32), (0x2c, 0, KEY_NATION_JAPAN as u32), (0x30, 0, KEY_NATION_ENGLAND as u32),
    (0x34, 1, 0xffffffff), (0x38, 0, KEY_NATION_ITALY as u32), (0x3c, 1, 0xffffffff), (0x40, 2, 0x0), (0x4a, 0, KEY_NATION_GERMANY as u32), (0x4e, 1, 0xffffffff),
    (0x52, 0, KEY_NATION_ENGLAND as u32), (0x56, 1, 0xffffffff), (0x5a, 0, KEY_NATION_ARGENTINA as u32), (0x5e, 1, 0xffffffff), (0x62, 2, 0x0), (0x6c, 0, KEY_NATION_BRAZIL as u32),
    (0x70, 1, 0xffffffff), (0x74, 0, KEY_NATION_ARGENTINA as u32), (0x78, 1, 0xffffffff), (0x7c, 0, KEY_NATION_SOUTH_AFRICA as u32), (0x80, 1, 0xffffffff), (0x84, 2, 0xfe),
    (0x8e, 0, KEY_NATION_SWEDEN as u32), (0x92, 0, KEY_NATION_NORWAY as u32), (0x96, 0, KEY_NATION_SPAIN as u32), (0x9a, 1, 0xffffffff), (0x9e, 0, KEY_NATION_SWITZERLAND as u32), (0xa2, 1, 0xffffffff),
    (0xa6, 2, 0xfe), (0xb0, 0, KEY_NATION_MEXICO as u32), (0xb4, 1, 0xffffffff), (0xb8, 0, KEY_NATION_URUGUAY as u32), (0xbc, 1, 0xffffffff), (0xc0, 0, KEY_NATION_CANADA as u32),
    (0xc4, 1, 0xffffffff), (0xc8, 2, 0xfe), (0xd2, 0, KEY_NATION_ITALY as u32), (0xd6, 1, 0xffffffff), (0xda, 0, KEY_NATION_GERMANY as u32), (0xde, 1, 0xffffffff),
    (0xe2, 0, KEY_NATION_ENGLAND as u32), (0xe6, 1, 0xffffffff), (0xea, 2, 0xfe), (0xf4, 0, KEY_NATION_BRAZIL as u32), (0xf8, 1, 0xffffffff), (0xfc, 0, KEY_NATION_ARGENTINA as u32),
    (0x100, 1, 0xffffffff), (0x104, 0, KEY_NATION_UNITED_STATES as u32), (0x108, 1, 0xffffffff), (0x10c, 2, 0xfe), (0x0, 4, 0x1), (0x0, 5, 0x1),
    (0x116, 0, KEY_NATION_HOLLAND as u32), (0x11a, 0, KEY_NATION_BELGIUM as u32), (0x11e, 0, KEY_NATION_ITALY as u32), (0x122, 1, 0xffffffff), (0x126, 0, KEY_NATION_NORWAY as u32), (0x12a, 1, 0xffffffff),
    (0x12e, 2, 0x0), (0x138, 0, KEY_NATION_HUNGARY as u32), (0x13c, 0, KEY_NATION_AUSTRIA as u32), (0x140, 0, KEY_NATION_PORTUGAL as u32), (0x144, 1, 0xffffffff), (0x148, 0, KEY_NATION_SPAIN as u32),
    (0x14c, 1, 0xffffffff), (0x150, 2, 0x1), (0x15a, 0, KEY_NATION_SCOTLAND as u32), (0x15e, 0, KEY_NATION_WALES as u32), (0x162, 0, KEY_NATION_SWEDEN as u32), (0x166, 1, 0xffffffff),
    (0x16a, 0, KEY_NATION_SWITZERLAND as u32), (0x16e, 1, 0xffffffff), (0x172, 2, 0xfe), (0x17c, 0, KEY_NATION_FRANCE as u32), (0x180, 1, 0xffffffff), (0x184, 0, KEY_NATION_NORWAY as u32),
    (0x188, 1, 0xffffffff), (0x18c, 0, KEY_NATION_DENMARK as u32), (0x190, 1, 0xffffffff), (0x194, 2, 0xfe), (0x19e, 0, KEY_NATION_REPUBLIC_OF_IRELAND as u32), (0x1a2, 1, 0xffffffff),
    (0x1a6, 0, KEY_NATION_SWITZERLAND as u32), (0x1aa, 1, 0xffffffff), (0x1ae, 0, KEY_NATION_YUGOSLAVIA as u32), (0x1b2, 1, 0xffffffff), (0x1b6, 2, 0xfe), (0x1c0, 0, KEY_NATION_SWITZERLAND as u32),
    (0x1c4, 1, 0xffffffff), (0x1c8, 0, KEY_NATION_BELGIUM as u32), (0x1cc, 1, 0xffffffff), (0x1d0, 0, KEY_NATION_HOLLAND as u32), (0x1d4, 1, 0xffffffff), (0x1d8, 2, 0xfe),
    (0x1e2, 0, KEY_NATION_ITALY as u32), (0x1e6, 1, 0xffffffff), (0x1ea, 0, KEY_NATION_GERMANY as u32), (0x1ee, 1, 0xffffffff), (0x1f2, 0, KEY_NATION_ENGLAND as u32), (0x1f6, 1, 0xffffffff),
    (0x1fa, 2, 0xfe), (0x204, 0, KEY_NATION_GREECE as u32), (0x208, 1, 0xffffffff), (0x20c, 0, KEY_NATION_SPAIN as u32), (0x210, 1, 0xffffffff), (0x214, 0, KEY_NATION_FRANCE as u32),
    (0x218, 1, 0xffffffff), (0x21c, 2, 0xfe), (0x0, 4, 0x2), (0x0, 5, 0x2), (0x226, 0, KEY_NATION_GHANA as u32), (0x22a, 0, KEY_NATION_NIGERIA as u32),
    (0x23e, 2, 0xfd), (0x248, 0, KEY_NATION_EGYPT as u32), (0x24c, 1, 0xffffffff), (0x260, 2, 0xfd), (0x26a, 0, KEY_NATION_SUDAN as u32), (0x26e, 1, 0xffffffff),
    (0x282, 2, 0xfd), (0x28c, 0, KEY_NATION_TUNISIA as u32), (0x290, 1, 0xffffffff), (0x2a4, 2, 0xfd), (0x2ae, 0, KEY_NATION_CAMEROON as u32), (0x2b2, 1, 0xffffffff),
    (0x2c6, 2, 0xfd), (0x2d0, 0, KEY_NATION_NIGERIA as u32), (0x2d4, 1, 0xffffffff), (0x2e8, 2, 0xfd), (0x2f2, 0, KEY_NATION_MOROCCO as u32), (0x2f6, 1, 0xffffffff),
    (0x30a, 2, 0xfd), (0x314, 0, KEY_NATION_ALGERIA as u32), (0x318, 1, 0xffffffff), (0x32c, 2, 0xfd), (0x0, 4, 0x3), (0x0, 5, 0x3),
    (0x336, 0, KEY_NATION_AUSTRALIA as u32), (0x33a, 1, 0xffffffff), (0x34e, 2, 0xfd), (0x358, 0, KEY_NATION_GREECE as u32), (0x35c, 1, 0xffffffff), (0x370, 2, 0xfd),
    (0x37a, 0, KEY_NATION_GERMANY as u32), (0x37e, 1, 0xffffffff), (0x392, 2, 0xfd), (0x39c, 0, KEY_NATION_ITALY as u32), (0x3a0, 1, 0xffffffff), (0x3b4, 2, 0xfd),
    (0x3be, 0, KEY_NATION_JAPAN as u32), (0x3c2, 1, 0xffffffff), (0x3d6, 2, 0xfd), (0x3e0, 0, KEY_NATION_MEXICO as u32), (0x3e4, 1, 0xffffffff), (0x3f8, 2, 0xfd),
    (0x402, 0, KEY_NATION_CANADA as u32), (0x406, 1, 0xffffffff), (0x41a, 2, 0xfd), (0x424, 0, KEY_NATION_SPAIN as u32), (0x428, 1, 0xffffffff), (0x43c, 2, 0xfd),
    (0x440, 0, KEY_CLUB_COMP_WORLD_CLUB_CUP as u32), (0x444, 3, 0x0), (0x45f, 3, 0x0), (0x461, 2, 0x0), (0x446, 0, KEY_NATION_JAPAN as u32), (0x44a, 1, 0xffffffff),
    (0x45e, 2, 0xfd), (0x0, 4, 0x4), (0x0, 5, 0x4), (0x468, 0, KEY_NATION_PARAGUAY as u32), (0x46c, 1, 0xffffffff), (0x480, 2, 0xfd),
    (0x48a, 0, KEY_NATION_COLOMBIA as u32), (0x48e, 1, 0xffffffff), (0x4a2, 2, 0xfd), (0x4ac, 0, KEY_NATION_PERU as u32), (0x4b0, 1, 0xffffffff), (0x4c4, 2, 0xfd),
    (0x4ce, 0, KEY_NATION_VENEZUELA as u32), (0x4d2, 1, 0xffffffff), (0x4e6, 2, 0xfd), (0x4f0, 0, KEY_NATION_ARGENTINA as u32), (0x4f4, 1, 0xffffffff), (0x508, 2, 0xfd),
    (0x512, 0, KEY_NATION_BRAZIL as u32), (0x516, 1, 0xffffffff), (0x52a, 2, 0xfd), (0x52e, 0, KEY_CLUB_COMP_UEFA_EUROPEAN_CHAMPIONS_LEAGUE as u32), (0x532, 3, 0x0), (0x54d, 3, 0x0),
    (0x54f, 2, 0x0), (0x534, 0, KEY_NATION_SCOTLAND as u32), (0x538, 1, 0xffffffff), (0x54c, 2, 0xfd), (0x550, 0, KEY_CLUB_COMP_UEFA_CUP as u32), (0x554, 3, 0x0),
    (0x56f, 3, 0x0), (0x571, 2, 0x0), (0x556, 0, KEY_NATION_DENMARK as u32), (0x55a, 1, 0xffffffff), (0x56e, 2, 0xfd), (0x572, 0, KEY_CLUB_COMP_EUROPEAN_CUP_WINNERS_CUP as u32),
    (0x576, 3, 0x0), (0x591, 3, 0x0), (0x593, 2, 0x0), (0x578, 0, KEY_NATION_FRANCE as u32), (0x57c, 1, 0xffffffff), (0x590, 2, 0xfd),
    (0x594, 0, KEY_CLUB_COMP_FIFA_CLUB_WORLD_CHAMPIONSHIP as u32), (0x598, 3, 0x0), (0x5b3, 3, 0x0), (0x5b5, 2, 0x0), (0x59a, 0, KEY_NATION_SPAIN as u32), (0x59e, 1, 0xffffffff),
    (0x5b2, 2, 0xfd), (0x5b6, 0, KEY_CLUB_COMP_OCEANIA_CLUB_CHAMPIONSHIP as u32), (0x5ba, 3, 0x0), (0x5d5, 3, 0x0), (0x5d7, 2, 0x0), (0x5bc, 0, KEY_NATION_AUSTRALIA as u32),
    (0x5c0, 1, 0xffffffff), (0x5d4, 2, 0xfd), (0x5d8, 0, KEY_CLUB_COMP_WORLD_CUP_OCEANIA_QUALIFYING_SECTION as u32), (0x5dc, 3, 0x0), (0x5f7, 3, 0x0), (0x5f9, 2, 0x0),
    (0x5de, 0, KEY_NATION_AUSTRALIA as u32), (0x5e2, 0, KEY_NATION_NEW_ZEALAND as u32), (0x5f6, 2, 0xfd), (0x0, 4, 0x5), (0x0, 5, 0x5), (0x600, 0, KEY_NATION_PARAGUAY as u32),
    (0x604, 1, 0xffffffff), (0x618, 2, 0xfd), (0x622, 0, KEY_NATION_UNITED_STATES as u32), (0x626, 1, 0xffffffff), (0x63a, 2, 0xfd), (0x644, 0, KEY_NATION_ARGENTINA as u32),
    (0x648, 1, 0xffffffff), (0x65c, 2, 0xfd), (0x666, 0, KEY_NATION_MEXICO as u32), (0x66a, 1, 0xffffffff), (0x67e, 2, 0xfd), (0x688, 0, KEY_NATION_URUGUAY as u32),
    (0x68c, 1, 0xffffffff), (0x6a0, 2, 0xfd), (0x6aa, 0, KEY_NATION_BRAZIL as u32), (0x6ae, 1, 0xffffffff), (0x6c2, 2, 0xfd), (0x0, 4, 0x6),
    (0x0, 5, 0x6), (0x6cc, 0, KEY_NATION_LEBANON as u32), (0x6d0, 1, 0xffffffff), (0x6e4, 2, 0xfd), (0x6ee, 0, KEY_NATION_SINGAPORE as u32), (0x6f2, 1, 0xffffffff),
    (0x706, 2, 0xfd), (0x710, 0, KEY_NATION_JAPAN as u32), (0x714, 1, 0xffffffff), (0x728, 2, 0xfd), (0x732, 0, KEY_NATION_CHINA as u32), (0x736, 1, 0xffffffff),
    (0x74a, 2, 0xfd), (0x754, 0, KEY_NATION_SOUTH_KOREA as u32), (0x758, 1, 0xffffffff), (0x76c, 2, 0xfd), (0x770, 0, KEY_CLUB_COMP_CONFEDERATIONS_CUP as u32), (0x774, 3, 0x0),
    (0x78f, 3, 0x0), (0x791, 2, 0x0), (0x776, 0, KEY_NATION_SOUTH_KOREA as u32), (0x77a, 1, 0xffffffff), (0x78e, 2, 0xfd),
];

/// the default host table: 57 entries (World Cup 1998 to 2026 every 4 years, European Championship and Olympics 2000 to
/// 2028, African Cup 2000 to 2014 every 2 years, Copa América 1999 to 2009, Asian Cup 2000 to 2016 and one-off
/// editions), each with up to three bids of two nations, the chosen one (or open/closed voting, or a fixed host) and
/// the decision day (10 July for the World Cup, 20 July or 20 September for the others [inference: the month counts
/// from 0]) plus a random −35 to +34 days
/// Original: `HostCountry::init_default_table` (0x5f9970, host_country.cpp, thiscall, ret 0).
///
/// [pt] { 0x5f9970 } (thiscall, ret 0, host_country.cpp, `HostCountry::init_default_table`; pelo construtor 0x5f97c0, sem o
/// HostCountry.tmp; quadro SEH só pela Data local): a tabela padrão das sedes (57 entradas de 0x22 bytes em [this],
/// conta em +4: +0 a competição, +4 o ano, +6/+0xa, +0xe/+0x12, +0x16/+0x1a as três candidaturas (duas nações cada),
/// +0x1e a escolhida, +0x1f o dia da decisão, +0x21 o intervalo entre edições), pelas gravações de `CQO3_TABELA`;
/// depois o dia da decisão de cada entrada anda um sorteio de −35 a +34 (0x90cfc0(70)); a entrada que não é padrão
/// fixa (−3) e tem segunda ou terceira candidatura completa leva a caixa (linha 0x226) e perde as duas.
pub(crate) unsafe extern "thiscall" fn init_default_table(this: usize) {
    let definir: unsafe extern "thiscall" fn(*mut crate::data::Data, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_SET);
    let mut d = crate::data::Data { dia: 40, ano: 1972, bissexto: 1 };
    for &(off, t, v) in CQO3_TABELA.iter() {
        let e = read_u32_as_usize(this) + off as usize;
        match t {
            0 => write_u32(e, read_u32_as_usize(v as usize) as u32),
            1 => write_u32(e, v),
            2 => *(e as *mut u8) = v as u8,
            3 => write_u16(e, v as u16),
            4 => {
                let a = CQO3_DATAS[v as usize];
                definir(&mut d, a[0], a[1], a[2], a[3]);
            }
            _ => {
                let (ini, n, g, ano, passo, intervalo) = CQO3_LACOS[v as usize];
                let mut e = read_u32_as_usize(this) + ini as usize;
                let mut y = ano;
                for _ in 0..n {
                    write_u16(e + 4, y);
                    write_u32(e, read_u32_as_usize(g as usize) as u32);
                    write_u16(e + 0x1f, d.dia as u16);
                    *((e + 0x21) as *mut u8) = intervalo;
                    e += 0x22;
                    y = y.wrapping_add(passo);
                }
            }
        }
    }
    let sortear: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let n = read_u16(this + 4) as i16;
    let mut k: i16 = 0;
    while k < n {
        let o = (k as i32 * 0x22) as usize;
        let p = read_u32_as_usize(this) + o + 0x1f;
        crate::comparacao::ramo("cqo_3 sorteio do dia");
        let r = sortear(0x46); // cqo3: o sorteio do dia da decisão
        write_u16(p, read_u16(p).wrapping_add((r - 0x23) as u16));
        let e = read_u32_as_usize(this) + o;
        if *((e + 0x1e) as *const u8) != 0xfd && (read_u32_as_usize(e + 0x12) as u32 != u32::MAX || read_u32_as_usize(e + 0x1a) as u32 != u32::MAX) {
            aviso_em(0x9cf0d8, 0x226);
            write_u32(read_u32_as_usize(this) + o + 0x12, u32::MAX);
            write_u32(read_u32_as_usize(this) + o + 0x1a, u32::MAX);
        }
        k += 1;
    }
}

/// Lado a lado (a cópia sem contar os sorteios) pela tabela (conta +4 × 0x22 bytes), pelo estado do sorteador e por
/// [0xb67a34].
pub(crate) unsafe extern "thiscall" fn comparar_init_default_table(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_DEFAULT_TABLE);
    let n = read_u16(this + 4) as i16;
    if read_u32_as_usize(this) == 0 || n <= 0 {
        return o(this);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(read_u32_as_usize(this), n as usize * 0x22), (0xb67a34, 4)]);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| init_default_table(this)));
    PLACAR.registrar(igual, || format!("init_default_table: {:x?} x {:x?}", mo, mr).chars().take(4000).collect());
}

/// the news of changed hosts: with open voting, the bids (news 4); with a chosen bid, the hosts (news 5); to the
/// managers of the competition's continent or to everyone; except the Olympics, the 2006 World Cup in Germany and the
/// chosen bid of the edition 8 years ahead
/// Original: `HostCountry::post_host_news` (0x5faec0, host_country.cpp, thiscall, ret 4).
///
/// [pt] { 0x5faec0 } (thiscall, ret 4, host_country.cpp, `HostCountry::post_host_news(lista)`; pelo calendário das sedes
/// 0x5fa850; quadro SEH só pelas notícias locais, sem destrutor): para cada entrada da lista (cursor reposto, 0x68d1b0),
/// fora a candidatura escolhida (ou a padrão fixa) da edição de daqui a 8 anos ([0xae2c92] + 8), as Olimpíadas
/// ([0x9cf7a0]) e a Copa do Mundo de 2006 na Alemanha ([0x9cf79c], 0x7d6, [0x9cf314]): com a votação aberta (−1) a
/// notícia 4 (as candidaturas: competição, ano e as seis nações); senão a notícia 5 (a escolhida: competição, ano e as
/// duas sedes dela, a primeira na padrão fixa), montadas por 0x77bbe0/0x785790 e entregues aos técnicos do continente
/// da competição (tabela [0xae23d0], 0x6b bytes, +0x59; 0x786640) ou a todos (0x786780).
pub(crate) unsafe extern "thiscall" fn post_host_news(_this: usize, lista: usize) {
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_GET_NEXT);
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(FN_NEWSITEM_NEWSITEM);
    let dado: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let continente: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NEWS_SEND_TO_CONTINENT_MANAGERS);
    let todos: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
    crate::comparacao::ramo("cqo_3 noticia das sedes");
    write_u32(lista + 0xa, read_u32_as_usize(lista + 2) as u32);
    let mut it = proximo(lista);
    while it != 0 {
        let e = read_u32_as_usize(it);
        let k = *((e + 0x1e) as *const i8);
        let pular = ((k > 0 || k == -3) && read_u16(G_CURRENT_DATE_YEAR) as u32 == (read_u16(e + 4) as u32).wrapping_sub(8))
            || read_u32_as_usize(e) == read_u32_as_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES)
            || (read_u32_as_usize(e) == read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) && read_u16(e + 4) == 0x7d6 && read_u32_as_usize(e + 6) == read_u32_as_usize(KEY_NATION_GERMANY));
        if !pular {
            // a notícia local (0xe8 bytes, como nas outras cópias)
            let mut n = [0u8; 0xe8];
            let p = n.as_mut_ptr() as usize;
            if k == -1 {
                construir(p, 4, 0);
                dado(p, 0, read_u32_as_usize(read_u32_as_usize(it)) as u32);
                dado(p, 1, read_u16(read_u32_as_usize(it) + 4) as u32);
                for (i, off) in [6usize, 0xa, 0xe, 0x12, 0x16, 0x1a].iter().enumerate() {
                    dado(p, 2 + i as u32, read_u32_as_usize(read_u32_as_usize(it) + off) as u32);
                }
            } else {
                construir(p, 5, 0);
                let mut b = *((read_u32_as_usize(it) + 0x1e) as *const i8);
                if b == -3 {
                    b = 0;
                }
                dado(p, 0, read_u32_as_usize(read_u32_as_usize(it)) as u32);
                dado(p, 1, read_u16(read_u32_as_usize(it) + 4) as u32);
                let o = (b as i32 * 8) as usize;
                dado(p, 2, read_u32_as_usize(read_u32_as_usize(it).wrapping_add(o) + 6) as u32);
                dado(p, 3, read_u32_as_usize(read_u32_as_usize(it).wrapping_add(o) + 0xa) as u32);
            }
            let id = read_u32_as_usize(read_u32_as_usize(it));
            let c = read_u32_as_usize(read_u32_as_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b)) + 0x59);
            if c != 0 {
                continente(read_u32_as_usize(G_NEWS), p, c);
            } else {
                todos(read_u32_as_usize(G_NEWS), p);
            }
        }
        it = proximo(lista);
    }
}

// ponytail: entrega notícias (aloca); só a original. A cópia leva o ramo.
pub(crate) unsafe extern "thiscall" fn comparar_post_host_news(this: usize, lista: usize) {
    orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_HOST_NEWS)(this, lista)
}

/// reads HostCountry.tmp (the count and the 0x22-byte entries into a new block); without the file 0 (a new game builds
/// the default table); read errors with the error box
/// Original: `HostCountry::load_temp_file` (0x5fb160, host_country.cpp, thiscall, ret 0).
///
/// [pt] { 0x5fb160 } (thiscall, ret 0, host_country.cpp, `HostCountry::load_temp_file`; pelo construtor 0x5f97c0): lê o
/// HostCountry.tmp da pasta CM3_TEMP (0x90bde0, fopen 0x945b00 "rb"): a conta (word, +4) e as entradas de 0x22 bytes
/// num bloco novo ([this], 0x944e46); falhando a leitura, a caixa (linhas 0x448/0x452/0x45c), fecha (0x945a09) e, com
/// o bloco, o solta (0x9452ca). Devolve 1 com tudo lido, 0 sem o arquivo ou com erro.
pub(crate) unsafe extern "thiscall" fn load_temp_file(this: usize) -> u32 {
    let caminho: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_UTILS_BUILD_FILE_PATH);
    let abrir: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(FN_CRT_FOPEN);
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> u32 = core::mem::transmute(FN_CRT_FREAD);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(FN_CRT_FCLOSE);
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let f = abrir(caminho(0x987da0, 0x9cf10c), 0x98f928);
    if f == 0 {
        crate::comparacao::ramo("cqo_3 sem o arquivo");
        return 0;
    }
    if ler(this + 4, 2, 1, f) != 1 {
        aviso_em(0x9cf0d8, 0x448);
        fechar(f);
        return 0;
    }
    let n = read_u16(this + 4) as i16 as i32;
    let b = alocar((n * 0x22) as u32);
    write_u32(this, b as u32);
    if b == 0 {
        aviso_em(0x9cf0d8, 0x452);
        fechar(f);
        return 0;
    }
    let n = read_u16(this + 4) as i16 as i32 as u32;
    if ler(b, 0x22, n, f) != read_u16(this + 4) as i16 as i32 as u32 {
        aviso_em(0x9cf0d8, 0x45c);
        fechar(f);
        soltar(read_u32_as_usize(this));
        return 0;
    }
    fechar(f);
    1
}

// ponytail: abre o arquivo e aloca; só a original. A cópia leva os ramos.
pub(crate) unsafe extern "thiscall" fn comparar_load_temp_file(this: usize) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_LOAD_TEMP_FILE)(this)
}

// ---------- 273a leva: as sedes das competições: a sede de uma edição, a próxima sede e o calendário das candidaturas (host_country.cpp) ----------



/// As duas sedes da entrada `e` (registros de 0x22 bytes: +0 id da competição, +4 ano, +6 a sede padrão e a segunda
/// (+0xa), +6 + 8 × k as candidaturas k = 0..2 (sede e segunda), +0x1e a escolhida (0..2; −3 a padrão fixa, −1 a
/// votação aberta, −2 a votação ainda fechada), +0x1f o dia do ano das decisões, +0x21 o intervalo em anos) pela
/// candidatura `k` (a padrão com `k` < 0): grava as duas e devolve quantas valem (≠ −1).
pub(crate) unsafe fn cqj3_sedes(e: usize, k: i8, a: *mut u32, b: *mut u32) -> u8 {
    let off = if k >= 0 { k as i32 * 8 } else { 0 } as usize;
    *a = read_u32(e + 6 + off);
    let segunda = read_u32(e + 0xa + off);
    *b = segunda;
    (*a != u32::MAX) as u8 + (segunda != u32::MAX) as u8
}

/// the host (and the second host, for co-hosted editions) of a competition in a year: the winning bid, or the default
/// host when the host is fixed or the bidding is open (or closed, on request)
/// Original: `HostCountry::find_hosts` (0x5fa730, host_country.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x5fa730 } (thiscall, ret 0x14, host_country.cpp, `HostCountry::find_hosts`): as sedes da competição `comp` no ano
/// `ano` (short): a primeira entrada da competição com o ano (ou ano 0) que já tem a candidatura escolhida (as duas
/// sedes dela), a padrão fixa ou a votação aberta (a sede padrão), ou, com `padrao`, a votação ainda fechada (a padrão).
/// Grava as duas e devolve quantas valem; sem entrada, 0 sem gravar.
pub(crate) unsafe extern "thiscall" fn find_hosts(this: usize, comp: u32, ano: u32, a: *mut u32, b: *mut u32, padrao: u32) -> u32 {
    let n = read_u16(this + 4) as i16;
    let mut e = read_u32(this) as usize;
    let mut i = 0i16;
    while i < n {
        if read_u32(e) == comp {
            let ae = read_u16(e + 4);
            if ae == ano as u16 || ae == 0 {
                let k = *((e + 0x1e) as *const i8);
                if k > -1 {
                    return cqj3_sedes(e, k, a, b) as u32;
                }
                if k == -3 || k == -1 || padrao != 0 {
                    crate::comparacao::ramo("cqj_3 sede padrao");
                    return cqj3_sedes(e, -1, a, b) as u32; // cqj3: a sede padrão
                }
            }
        }
        e += 0x22;
        i += 1;
    }
    // ponytail: a original devolve o al = 0 com os bytes altos do eax (o ponteiro da entrada); quem chama lê o al
    0
}

/// A devolução (o al) e as duas saídas (a rotina só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_hosts(this: usize, comp: u32, ano: u32, a: *mut u32, b: *mut u32, padrao: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, *mut u32, *mut u32, u32) -> u32>(IDX_FIND_HOSTS);
    let (va, vb) = (*a, *b);
    let r = o(this, comp, ano, a, b, padrao);
    let (oa, ob) = (*a, *b);
    let (mut ca, mut cb) = (va, vb);
    let c = find_hosts(this, comp, ano, &mut ca, &mut cb, padrao);
    PLACAR.registrar(r as u8 == c as u8 && oa == ca && ob == cb, || format!("find_hosts({:x}, {}): original {} {:x} {:x} rust {} {:x} {:x}", comp, ano as u16, r as u8, oa, ob, c as u8, ca, cb));
    r
}

/// the hosts of the competition's next already decided edition (the winning bid or the fixed host)
/// Original: `HostCountry::find_next_hosts` (0x5fa7d0, host_country.cpp, thiscall, ret 0xc).
///
/// [pt] { 0x5fa7d0 } (thiscall, ret 0xc, host_country.cpp, `HostCountry::find_next_hosts`): as sedes da próxima edição da
/// competição `comp`: a primeira entrada dela com a candidatura escolhida (as duas sedes) ou com a padrão fixa (a
/// padrão). Grava as duas e devolve quantas valem; sem entrada, 0 sem gravar.
pub(crate) unsafe extern "thiscall" fn find_next_hosts(this: usize, comp: u32, a: *mut u32, b: *mut u32) -> u32 {
    let n = read_u16(this + 4) as i16;
    let mut e = read_u32(this) as usize;
    let mut i = 0i16;
    while i < n {
        if read_u32(e) == comp {
            let k = *((e + 0x1e) as *const i8);
            if k > -1 {
                return cqj3_sedes(e, k, a, b) as u32;
            }
            if k == -3 {
                return cqj3_sedes(e, -1, a, b) as u32;
            }
        }
        e += 0x22;
        i += 1;
    }
    // ponytail: a original devolve o al = 0 com os bytes altos do eax (o ponteiro da entrada); quem chama lê o al
    0
}

/// A devolução (o al) e as duas saídas (a rotina só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_next_hosts(this: usize, comp: u32, a: *mut u32, b: *mut u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32) -> u32>(IDX_FIND_NEXT_HOSTS);
    let (va, vb) = (*a, *b);
    let r = o(this, comp, a, b);
    let (oa, ob) = (*a, *b);
    let (mut ca, mut cb) = (va, vb);
    let c = find_next_hosts(this, comp, &mut ca, &mut cb);
    PLACAR.registrar(r as u8 == c as u8 && oa == ca && ob == cb, || format!("find_next_hosts({:x}): original {} {:x} {:x} rust {} {:x} {:x}", comp, r as u8, oa, ob, c as u8, ca, cb));
    r
}

/// A chave da busca (a 0x5fb3f0 compara o ano +4 e depois o dia +0x1f, sem sinal).
pub(crate) fn cqj3_compara(e: usize, ano: u16, dia: u16) -> i32 {
    unsafe {
        let ae = read_u16(e + 4);
        if ae != ano {
            ae as i32 - ano as i32
        } else {
            read_u16(e + 0x1f) as i32 - dia as i32
        }
    }
}

/// A primeira entrada (de `inicio` a `fim`, ordenadas por ano e dia) com o ano e o dia da chave (bsearch 0x946464 com a
/// 0x5fb3f0 e o recuo até a primeira igual), ou 0.
pub(crate) unsafe fn cqj3_primeira(inicio: usize, n: i16, ano: u16, dia: u16) -> usize {
    let busca: unsafe extern "C" fn(*const u8, usize, i32, u32, usize) -> usize = core::mem::transmute(FN_CRT_BSEARCH);
    let mut chave = [0u8; 0x22];
    chave[4..6].copy_from_slice(&ano.to_le_bytes());
    chave[0x1f..0x21].copy_from_slice(&dia.to_le_bytes());
    let mut p = busca(chave.as_ptr(), inicio, n as i32, 0x22, 0x5fb3f0);
    if p == 0 {
        return 0;
    }
    while p > inicio && cqj3_compara(p - 0x22, ano, dia) == 0 {
        p -= 0x22;
    }
    p
}

/// the hosting rule: each edition (World Cup, European Championship and the others in the table) has a decision date in
/// the year; 8 years ahead the bidding opens (news with the bids); 5 years ahead the host is drawn among the 3 bids
/// (news); on 1 January past editions jump to the next one (the competition's interval, e.g. 4 years) with the bidding
/// closed; outside 2001, on 1 July, the European finals hosts
/// Original: `HostCountry::daily_update` (0x5fa850, host_country.cpp, thiscall, ret 4).
///
/// [pt] { 0x5fa850 } (thiscall, ret 4, host_country.cpp, `HostCountry::daily_update`; quadro SEH pela lista local, destruída
/// no fim pela 0x68d040): o calendário das sedes, a cada dia `hoje` (+0 o dia do ano, +2 o ano). Fora de 2001, no dia
/// 1º de julho (0x549f70(1, 6, ano, −1)) as sedes das finais europeias (0x5fad20). As entradas com o dia de hoje e o ano
/// + 8: a votação abre (−2 vira −1) e vão à lista. As com o ano + 5: vão à lista e, com a votação aberta ou fechada,
/// a candidatura vencedora é sorteada (0x90cfc0(3)). No 1º de janeiro, as entradas de anos passados (≠ 0) andam o
/// intervalo (+0x21) e, fora da padrão fixa, a votação fecha (−2); a tabela é reordenada (qsort 0x945333 com 0x5fb3f0).
/// Por fim a notícia das entradas da lista (0x5faec0).
pub(crate) unsafe extern "thiscall" fn host_country_daily_update(this: usize, hoje: usize) {
    let lista_nova: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_LLIST_LLIST);
    let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LLIST_DESTRUCTOR_LLIST);
    let por: unsafe extern "thiscall" fn(usize, *const usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let data: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> *const [u16; 4] = core::mem::transmute(FN_DATE_DATE_549F70);
    let europeias: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_HOSTCOUNTRY_PICK_EUROPEAN_FINAL_HOSTS);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let ordenar: unsafe extern "C" fn(usize, i32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let noticia: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_HOSTCOUNTRY_POST_HOST_NEWS);
    let n = read_u16(this + 4) as i16;
    let inicio = read_u32(this) as usize;
    let fim = inicio.wrapping_add((n as i32 * 0x22 - 0x22) as usize);
    let mut lista = [0u8; 0x10];
    let l = lista.as_mut_ptr() as usize;
    lista_nova(l);
    let ano = read_u16(hoje + 2);
    let dia = read_u16(hoje);
    if ano != 0x7d1 {
        let mut d = [0u16; 4];
        let r = *data(&mut d, 1, 6, ano as u32, u32::MAX);
        if r[1] == ano && r[0] == dia {
            europeias(this);
        }
    }
    let mut p = cqj3_primeira(inicio, n, ano.wrapping_add(8), dia);
    if p != 0 {
        while p <= fim && cqj3_compara(p, ano.wrapping_add(8), dia) == 0 {
            let k = (p + 0x1e) as *mut u8;
            if *k == 0xfe {
                *k = 0xff; // cqj3: a votação abre
            }
            if *k == 0xff {
                crate::comparacao::ramo("cqj_3 votacao aberta");
                por(l, &p, 4);
            }
            p += 0x22;
        }
    }
    let mut p = cqj3_primeira(inicio, n, ano.wrapping_add(5), dia);
    if p != 0 {
        while p <= fim && cqj3_compara(p, ano.wrapping_add(5), dia) == 0 {
            por(l, &p, 4);
            let k = (p + 0x1e) as *mut u8;
            if *k == 0xff || *k == 0xfe {
                crate::comparacao::ramo("cqj_3 sede sorteada");
                *k = sorteio(3) as u8; // cqj3: a candidatura vencedora
            }
            p += 0x22;
        }
    }
    if dia == 0 {
        let mut i = 0i16;
        while i < read_u16(this + 4) as i16 {
            let e = (read_u32(this) as usize).wrapping_add(i as i32 as usize * 0x22);
            let ae = read_u16(e + 4);
            if ae < ano && ae != 0 {
                crate::comparacao::ramo("cqj_3 edicao seguinte");
                core::ptr::write_unaligned((e + 4) as *mut u16, ae.wrapping_add(*((e + 0x21) as *const i8) as i16 as u16)); // cqj3: o intervalo
                if *((e + 0x1e) as *const u8) != 0xfd {
                    *((e + 0x1e) as *mut u8) = 0xfe;
                }
            }
            i += 1;
        }
        ordenar(read_u32(this) as usize, read_u16(this + 4) as i16 as i32, 0x22, 0x5fb3f0);
    }
    noticia(this, l);
    destruir(l);
}

/// Só a original (sorteia, monta a lista local e entrega notícias); a cópia leva os contadores.
pub(crate) unsafe extern "thiscall" fn comparar_host_country_daily_update(this: usize, hoje: usize) {
    // ponytail: entrega notícias pela 0x5faec0 e mexe na lista: não dá para rodar duas vezes
    orig::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HOST_COUNTRY_DAILY_UPDATE)(this, hoje)
}

/// writes CM3_TEMP\HostCountry.tmp: the count (+4, u16) and 0x22 bytes per host from [+0], without checking the writes;
/// on open error the "Error" box
/// Original: `HostCountry::save_temp_file` (0x5fb320, host_country.cpp, thiscall).
///
/// [pt] { 0x5fb320 } (thiscall, ret 0, host_country.cpp): `HostCountry::save_temp_file`: grava `CM3_TEMP\HostCountry.tmp`:
/// a quantidade (+4, i16) e os registros de 0x22 bytes de [+0], sem conferir a gravação. Erro ao abrir: a caixa
/// "Error" (host_country.cpp 0x475).
pub(crate) unsafe extern "thiscall" fn save_temp_file(this: usize) {
    crate::comparacao::ramo("cpx_6 gravar paises anfitrioes");
    let f = cpx6_abrir(0x9cf10c);
    if f == 0 {
        aviso_em(0x9cf0d8, 0x475);
        return;
    }
    cpx6_escrever(this + 4, 2, 1, f);
    cpx6_escrever(read_usize(this), 0x22, *((this + 4) as *const i16) as i32 as u32, f); // cpx6: os anfitriões
    cpx6_fechar(f);
}

/// Comparador da 0x5fb320: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_save_temp_file(this: usize) {
    // ponytail: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    orig::<unsafe extern "thiscall" fn(usize)>(IDX_SAVE_TEMP_FILE)(this)
}
