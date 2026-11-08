use super::*;

/// The reference dates of the host decisions (Date::set 0x54a130: day of the month, month from 0, year 1997, no
/// weekday): their day of the year goes into +0x1f of the entries of the following loop.
///
/// [pt] As datas de referência das decisões (Date::set 0x54a130: dia do mês, mês a partir de 0, ano 1997, sem dia da
/// semana): o dia do ano delas vai ao +0x1f das entradas do laço seguinte.
pub(crate) const HOST_DECISION_DATES: [[u32; 4]; 7] = [
    [0xa, 0x6, 0x7cd, 0xffffffff], // cqo3: 10 July, the World Cup
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x6, 0x7cd, 0xffffffff],
    [0x14, 0x8, 0x7cd, 0xffffffff],
];

/// The entry loops of a competition: (start in the table, entries, the competition [global], first year, year step,
/// interval `+0x21`).
///
/// In order: the World Cup ([`0x9cf79c`], 1998..2026 every 4), the European Championship ([`0x9cf7a4`], 2000..2028),
/// the African Nations Cup ([`0x9cf7ac`], 2000..2014 every 2), the Olympics ([`0x9cf7a0`], 2000..2028), the Copa
/// América ([`0x9cf7a8`], 1999..2009 every 2), [`0x9cf95c`] (1999..2004, year by year) and the Asian Cup ([`0x9cf790`],
/// 2000..2016) [inference: the names from the community addresses, `docs/ENDERECOS_COMUNIDADE.md`].
///
/// [pt] Os laços das entradas de uma competição: (início na tabela, entradas, a competição [global], primeiro ano, passo
/// dos anos, intervalo +0x21). Na ordem: a Copa do Mundo ([0x9cf79c], 1998..2026 de 4 em 4), a Eurocopa ([0x9cf7a4],
/// 2000..2028), a Copa Africana ([0x9cf7ac], 2000..2014 de 2 em 2), as Olimpíadas ([0x9cf7a0], 2000..2028), a Copa
/// América ([0x9cf7a8], 1999..2009 de 2 em 2), [0x9cf95c] (1999..2004, ano a ano) e a Copa da Ásia ([0x9cf790],
/// 2000..2016) [inferência: os nomes pelos endereços da comunidade, docs/ENDERECOS_COMUNIDADE.md].
pub(crate) const HOST_COUNTRY_LOOPS: [(u16, u8, u32, u16, u16, u8); 7] = [
    (0x0, 8, KEY_CLUB_COMP_FIFA_WORLD_CUP as u32, 1998, 4, 0x20),
    (0x110, 8, KEY_CLUB_COMP_EUROPEAN_FOOTBALL_CHAMPIONSHIP as u32, 2000, 4, 0x20), // cqo3: a Eurocopa
    (0x220, 8, KEY_CLUB_COMP_AFRICAN_CUP_OF_NATIONS as u32, 2000, 2, 0x10), // cqo3: a Copa Africana
    (0x330, 8, KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES as u32, 2000, 4, 0x20),
    (0x462, 6, KEY_CLUB_COMP_COPA_AMERICA as u32, 1999, 2, 0xc), // cqo3: a Copa América
    (0x5fa, 6, KEY_CLUB_COMP_INTER_AMERICAN_CUP as u32, 1999, 1, 0x6), // cqo3: the annual editions
    (0x6c6, 5, KEY_CLUB_COMP_ASIAN_CUP as u32, 2000, 4, 0x14),
];

/// The writes of the default table, in the original routine's order (generated from the disassembly of
/// 0x5f999a..0x5fa640): (offset, type, value).
///
/// Type 0 = dword of the global `valor` (a nation or competition), 1 = dword `valor` (−1 = no host), 2 = byte (the
/// +0x1e of each entry: 0/1 the chosen bid, −1 voting open, −2 closed, −3 the fixed default host), 3 = word 0 (year 0
/// and day 0 of the entries without an edition), 4 = the date `HOST_DECISION_DATES[valor]`, 5 = the loop
/// `HOST_COUNTRY_LOOPS[valor]`.
///
/// [pt] As gravações da tabela padrão, na ordem da original (geradas da desmontagem de 0x5f999a..0x5fa640): (deslocamento,
/// tipo, valor); tipo 0 = dword do global `valor` (uma nação ou competição), 1 = dword `valor` (−1 = sem sede), 2 =
/// byte (o +0x1e de cada entrada: 0/1 a candidatura escolhida, −1 votação aberta, −2 fechada, −3 a sede padrão
/// fixa), 3 = word 0 (ano 0 e dia 0 das entradas sem edição), 4 = a data `HOST_DECISION_DATES[valor]`, 5 = o laço
/// `HOST_COUNTRY_LOOPS[valor]`.
pub(crate) const HOST_COUNTRY_TABLE: [(u16, u8, u32); 281] = [
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
/// +0x1e a escolhida, +0x1f o dia da decisão, +0x21 o intervalo entre edições), pelas gravações de `HOST_COUNTRY_TABLE`;
/// depois o dia da decisão de cada entrada anda um sorteio de −35 a +34 (0x90cfc0(70)); a entrada que não é padrão
/// fixa (−3) e tem segunda ou terceira candidatura completa leva a caixa (linha 0x226) e perde as duas.
pub(crate) unsafe extern "thiscall" fn init_default_table(this: usize) {
    let definir: unsafe extern "thiscall" fn(*mut crate::date::Data, u32, u32, u32, u32) -> usize = core::mem::transmute(FN_DATE_SET);
    let mut d = crate::date::Data { dia: 40, ano: 1972, bissexto: 1 };
    for &(off, t, v) in HOST_COUNTRY_TABLE.iter() {
        let e = read_usize(this) + off as usize;
        match t {
            0 => write_u32(e, read_usize(v as usize) as u32),
            1 => write_u32(e, v),
            2 => *(e as *mut u8) = v as u8,
            3 => write_u16(e, v as u16),
            4 => {
                let a = HOST_DECISION_DATES[v as usize];
                definir(&mut d, a[0], a[1], a[2], a[3]);
            }
            _ => {
                let (ini, n, g, ano, step_towards, intervalo) = HOST_COUNTRY_LOOPS[v as usize];
                let mut e = read_usize(this) + ini as usize;
                let mut y = ano;
                for _ in 0..n {
                    write_u16(e + 4, y);
                    write_u32(e, read_usize(g as usize) as u32);
                    write_u16(e + 0x1f, d.dia as u16);
                    *((e + 0x21) as *mut u8) = intervalo;
                    e += 0x22;
                    y = y.wrapping_add(step_towards);
                }
            }
        }
    }
    let sortear: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let n = read_u16(this + 4) as i16;
    let mut k: i16 = 0;
    while k < n {
        let o = (k as i32 * 0x22) as usize;
        let p = read_usize(this) + o + 0x1f;
        crate::comparison::branch("cqo_3 sorteio do dia");
        let r = sortear(0x46); // cqo3: the random draw of the decision day
        write_u16(p, read_u16(p).wrapping_add((r - 0x23) as u16));
        let e = read_usize(this) + o;
        if *((e + 0x1e) as *const u8) != 0xfd && (read_usize(e + 0x12) as u32 != u32::MAX || read_usize(e + 0x1a) as u32 != u32::MAX) {
            show_error_at(0x9cf0d8, 0x226);
            write_u32(read_usize(this) + o + 0x12, u32::MAX);
            write_u32(read_usize(this) + o + 0x1a, u32::MAX);
        }
        k += 1;
    }
}

/// `HostCountry::post_host_news(list)`: posts the news of changed hosts. Called by the hosts calendar 0x5fa850; SEH
/// frame only for the local news, no destructor.
///
/// For each entry of the list (cursor reset, 0x68d1b0), except the chosen bid (or the fixed default) of the edition 8
/// years ahead ([0xae2c92] + 8), the Olympics ([0x9cf7a0]) and the 2006 World Cup in Germany ([0x9cf79c], 0x7d6,
/// [0x9cf314]): with voting open (−1), news 4 (the bids: competition, year and the six nations); otherwise news 5 (the
/// chosen bid: competition, year and its two hosts, the first one in the fixed default), built by 0x77bbe0/0x785790 and
/// delivered to the managers of the competition's continent (table [0xae23d0], 0x6b bytes, +0x59; 0x786640) or to
/// everyone (0x786780).
/// Original: `HostCountry::post_host_news` (0x5faec0, host_country.cpp, thiscall, ret 4).
///
/// [pt] { 0x5faec0 } (thiscall, ret 4, host_country.cpp, `HostCountry::post_host_news(lista)`; pelo calendário das sedes
/// 0x5fa850; quadro SEH só pelas notícias locais, sem destrutor): para cada entrada da lista (cursor reposto, 0x68d1b0),
/// fora a candidatura escolhida (ou a padrão fixa) da edição de daqui a 8 anos ([0xae2c92] + 8), as Olimpíadas
/// ([0x9cf7a0]) e a Copa do Mundo de 2006 na Alemanha ([0x9cf79c], 0x7d6, [0x9cf314]): com a votação aberta (−1) a
/// notícia 4 (as candidaturas: competição, ano e as seis nações); senão a notícia 5 (a escolhida: competição, ano e as
/// duas sedes dela, a primeira na padrão fixa), montadas por 0x77bbe0/0x785790 e entregues aos técnicos do continente
/// da competição (tabela [0xae23d0], 0x6b bytes, +0x59; 0x786640) ou a todos (0x786780).
pub(crate) unsafe extern "thiscall" fn post_host_news(_this: usize, list: usize) {
    let given: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(FN_NEWSITEM_SET_SLOT);
    let continent: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NEWS_SEND_TO_CONTINENT_MANAGERS);
    let all: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_NEWS_SEND_TO_ALL_MANAGERS);
    crate::comparison::branch("cqo_3 noticia das sedes");
    write_u32(list + 0xa, read_usize(list + 2) as u32);
    let mut it = exe::llist_get_next(list);
    while it != 0 {
        let e = read_usize(it);
        let k = *((e + 0x1e) as *const i8);
        let skip = ((k > 0 || k == -3) && read_u16(G_CURRENT_DATE_YEAR) as u32 == (read_u16(e + 4) as u32).wrapping_sub(8))
            || read_usize(e) == read_usize(KEY_CLUB_COMP_FOOTBALL_TOURNAMENT_OF_THE_OLYMPIC_GAMES)
            || (read_usize(e) == read_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) && read_u16(e + 4) == 0x7d6 && read_usize(e + 6) == read_usize(KEY_NATION_GERMANY));
        if !skip {
            // the local news (0xe8 bytes, as in the other copies)
            let mut n = [0u8; 0xe8];
            let p = n.as_mut_ptr() as usize;
            if k == -1 {
                exe::new_newsitem(p, 4, 0);
                given(p, 0, read_usize(read_usize(it)) as u32);
                given(p, 1, read_u16(read_usize(it) + 4) as u32);
                for (i, off) in [6usize, 0xa, 0xe, 0x12, 0x16, 0x1a].iter().enumerate() {
                    given(p, 2 + i as u32, read_usize(read_usize(it) + off) as u32);
                }
            } else {
                exe::new_newsitem(p, 5, 0);
                let mut b = *((read_usize(it) + 0x1e) as *const i8);
                if b == -3 {
                    b = 0;
                }
                given(p, 0, read_usize(read_usize(it)) as u32);
                given(p, 1, read_u16(read_usize(it) + 4) as u32);
                let o = (b as i32 * 8) as usize;
                given(p, 2, read_usize(read_usize(it).wrapping_add(o) + 6) as u32);
                given(p, 3, read_usize(read_usize(it).wrapping_add(o) + 0xa) as u32);
            }
            let id = read_usize(read_usize(it));
            let c = read_usize(read_usize(G_CLUB_COMP_TABLE).wrapping_add(id.wrapping_mul(0x6b)) + 0x59);
            if c != 0 {
                continent(read_usize(G_NEWS), p, c);
            } else {
                all(read_usize(G_NEWS), p);
            }
        }
        it = exe::llist_get_next(list);
    }
}

// simplification: delivers news items (allocates); original only. The copy takes the branch.

/// `HostCountry::load_temp_file`: reads HostCountry.tmp. Called by the constructor 0x5f97c0.
///
/// Reads HostCountry.tmp from the CM3_TEMP folder (0x90bde0, fopen 0x945b00 "rb"): the count (word, +4) and the
/// 0x22-byte entries into a new block ([this], 0x944e46); if the read fails, the error box (lines 0x448/0x452/0x45c),
/// closes the file (0x945a09) and, if the block exists, frees it (0x9452ca). Returns 1 with everything read, 0 without
/// the file (a new game builds the default table) or on error.
/// Original: `HostCountry::load_temp_file` (0x5fb160, host_country.cpp, thiscall, ret 0).
///
/// [pt] { 0x5fb160 } (thiscall, ret 0, host_country.cpp, `HostCountry::load_temp_file`; pelo construtor 0x5f97c0): lê o
/// HostCountry.tmp da pasta CM3_TEMP (0x90bde0, fopen 0x945b00 "rb"): a conta (word, +4) e as entradas de 0x22 bytes
/// num bloco novo ([this], 0x944e46); falhando a leitura, a caixa (linhas 0x448/0x452/0x45c), fecha (0x945a09) e, com
/// o bloco, o solta (0x9452ca). Devolve 1 com tudo lido, 0 sem o arquivo ou com erro.
pub(crate) unsafe extern "thiscall" fn load_temp_file(this: usize) -> u32 {
    let f = exe::crt_fopen(exe::utils_build_file_path(0x987da0, 0x9cf10c), 0x98f928);
    if f == 0 {
        crate::comparison::branch("cqo_3 sem o arquivo");
        return 0;
    }
    if exe::crt_fread(this + 4, 2, 1, f) != 1 {
        show_error_at(0x9cf0d8, 0x448);
        exe::crt_fclose(f);
        return 0;
    }
    let n = read_u16(this + 4) as i16 as i32;
    let b = exe::crt_malloc((n * 0x22) as u32);
    write_u32(this, b as u32);
    if b == 0 {
        show_error_at(0x9cf0d8, 0x452);
        exe::crt_fclose(f);
        return 0;
    }
    let n = read_u16(this + 4) as i16 as i32 as u32;
    if exe::crt_fread(b, 0x22, n, f) != read_u16(this + 4) as i16 as i32 as u32 {
        show_error_at(0x9cf0d8, 0x45c);
        exe::crt_fclose(f);
        exe::crt_free(read_usize(this));
        return 0;
    }
    exe::crt_fclose(f);
    1
}

// simplification: opens the file and allocates; original only. The copy takes the branches.

/// Writes the host and second host of entry `e` for bid `k` (the default when `k` < 0); returns how many are valid (≠
/// −1).
///
/// The entries are 0x22-byte records: +0 competition id, +4 year, +6 the default host and the second one (+0xa), +6 + 8
/// × k the bids k = 0..2 (host and second host), +0x1e the chosen one (0..2; −3 the fixed default, −1 voting open, −2
/// voting still closed), +0x1f the day of the year of the decisions, +0x21 the interval in years.
///
/// [pt] As duas sedes da entrada `e` (registros de 0x22 bytes: +0 id da competição, +4 ano, +6 a sede padrão e a segunda
/// (+0xa), +6 + 8 × k as candidaturas k = 0..2 (sede e segunda), +0x1e a escolhida (0..2; −3 a padrão fixa, −1 a
/// votação aberta, −2 a votação ainda fechada), +0x1f o dia do ano das decisões, +0x21 o intervalo em anos) pela
/// candidatura `k` (a padrão com `k` < 0): grava as duas e devolve quantas valem (≠ −1).
pub(crate) unsafe fn host_pair(e: usize, k: i8, a: *mut u32, b: *mut u32) -> u8 {
    let off = if k >= 0 { k as i32 * 8 } else { 0 } as usize;
    *a = read_u32(e + 6 + off);
    let second = read_u32(e + 0xa + off);
    *b = second;
    (*a != u32::MAX) as u8 + (second != u32::MAX) as u8
}

/// `HostCountry::find_hosts`: the host (and the second host, for co-hosted editions) of competition `comp` in year
/// `year` (short).
///
/// The first entry of the competition with that year (or year 0) that already has the chosen bid (its two hosts), the
/// fixed default or open voting (the default host), or, with `default`, voting still closed (the default). Writes both
/// and returns how many are valid; with no entry, 0 without writing.
/// Original: `HostCountry::find_hosts` (0x5fa730, host_country.cpp, thiscall, ret 0x14).
///
/// [pt] { 0x5fa730 } (thiscall, ret 0x14, host_country.cpp, `HostCountry::find_hosts`): as sedes da competição `comp` no ano
/// `ano` (short): a primeira entrada da competição com o ano (ou ano 0) que já tem a candidatura escolhida (as duas
/// sedes dela), a padrão fixa ou a votação aberta (a sede padrão), ou, com `padrao`, a votação ainda fechada (a padrão).
/// Grava as duas e devolve quantas valem; sem entrada, 0 sem gravar.
pub(crate) unsafe extern "thiscall" fn find_hosts(this: usize, comp: u32, year: u32, a: *mut u32, b: *mut u32, default: u32) -> u32 {
    let n = read_u16(this + 4) as i16;
    let mut e = read_u32(this) as usize;
    let mut i = 0i16;
    while i < n {
        if read_u32(e) == comp {
            let ae = read_u16(e + 4);
            if ae == year as u16 || ae == 0 {
                let k = *((e + 0x1e) as *const i8);
                if k > -1 {
                    return host_pair(e, k, a, b) as u32;
                }
                if k == -3 || k == -1 || default != 0 {
                    crate::comparison::branch("cqj_3 sede padrao");
                    return host_pair(e, -1, a, b) as u32; // cqj3: a sede padrão
                }
            }
        }
        e += 0x22;
        i += 1;
    }
    // simplification: the original returns al = 0 with the high bytes of eax (the entry pointer); the caller reads al
    0
}

/// `HostCountry::find_next_hosts`: the hosts of the next already decided edition of competition `comp`.
///
/// The first entry of it with the chosen bid (the two hosts) or with the fixed default (the default). Writes both and
/// returns how many are valid; with no entry, 0 without writing.
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
                return host_pair(e, k, a, b) as u32;
            }
            if k == -3 {
                return host_pair(e, -1, a, b) as u32;
            }
        }
        e += 0x22;
        i += 1;
    }
    // simplification: the original returns al = 0 with the high bytes of eax (the entry pointer); the caller reads al
    0
}

/// Search key compare: year (+4) then day (+0x1f), unsigned.
///
/// [pt] A chave da busca (a 0x5fb3f0 compara o ano +4 e depois o dia +0x1f, sem sinal).
pub(crate) fn compare_year_day(e: usize, year: u16, day: u16) -> i32 {
    unsafe {
        let ae = read_u16(e + 4);
        if ae != year {
            ae as i32 - year as i32
        } else {
            read_u16(e + 0x1f) as i32 - day as i32
        }
    }
}

/// First entry with the key's year and day (bsearch, then back to the first equal), or 0.
///
/// [pt] A primeira entrada (de `inicio` a `fim`, ordenadas por ano e dia) com o ano e o dia da chave (bsearch 0x946464 com a
/// 0x5fb3f0 e o recuo até a primeira igual), ou 0.
pub(crate) unsafe fn first_entry_for_date(start: usize, n: i16, year: u16, day: u16) -> usize {
    let mut key = [0u8; 0x22];
    key[4..6].copy_from_slice(&year.to_le_bytes());
    key[0x1f..0x21].copy_from_slice(&day.to_le_bytes());
    let mut p = exe::crt_bsearch((key.as_ptr()) as usize, start, (n as i32) as u32, 0x22, 0x5fb3f0);
    if p == 0 {
        return 0;
    }
    while p > start && compare_year_day(p - 0x22, year, day) == 0 {
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
pub(crate) unsafe extern "thiscall" fn host_country_daily_update(this: usize, today: usize) {
    let por: unsafe extern "thiscall" fn(usize, *const usize, u32) = core::mem::transmute(FN_LLIST_ADD_TAIL);
    let data: unsafe extern "thiscall" fn(*mut [u16; 4], u32, u32, u32, u32) -> *const [u16; 4] = core::mem::transmute(FN_DATE_DATE_549F70);
    let n = read_u16(this + 4) as i16;
    let start = read_u32(this) as usize;
    let end = start.wrapping_add((n as i32 * 0x22 - 0x22) as usize);
    let mut list = [0u8; 0x10];
    let l = list.as_mut_ptr() as usize;
    exe::new_llist(l);
    let year = read_u16(today + 2);
    let day = read_u16(today);
    if year != 0x7d1 {
        let mut d = [0u16; 4];
        let r = *data(&mut d, 1, 6, year as u32, u32::MAX);
        if r[1] == year && r[0] == day {
            exe::hostcountry_pick_european_final_hosts(this);
        }
    }
    let mut p = first_entry_for_date(start, n, year.wrapping_add(8), day);
    if p != 0 {
        while p <= end && compare_year_day(p, year.wrapping_add(8), day) == 0 {
            let k = (p + 0x1e) as *mut u8;
            if *k == 0xfe {
                *k = 0xff; // cqj3: voting opens
            }
            if *k == 0xff {
                crate::comparison::branch("cqj_3 votacao aberta");
                por(l, &p, 4);
            }
            p += 0x22;
        }
    }
    let mut p = first_entry_for_date(start, n, year.wrapping_add(5), day);
    if p != 0 {
        while p <= end && compare_year_day(p, year.wrapping_add(5), day) == 0 {
            por(l, &p, 4);
            let k = (p + 0x1e) as *mut u8;
            if *k == 0xff || *k == 0xfe {
                crate::comparison::branch("cqj_3 sede sorteada");
                *k = exe::utils_random(3) as u8; // cqj3: a candidatura vencedora
            }
            p += 0x22;
        }
    }
    if day == 0 {
        let mut i = 0i16;
        while i < read_u16(this + 4) as i16 {
            let e = (read_u32(this) as usize).wrapping_add(i as i32 as usize * 0x22);
            let ae = read_u16(e + 4);
            if ae < year && ae != 0 {
                crate::comparison::branch("cqj_3 edicao seguinte");
                core::ptr::write_unaligned((e + 4) as *mut u16, ae.wrapping_add(*((e + 0x21) as *const i8) as i16 as u16)); // cqj3: o intervalo
                if *((e + 0x1e) as *const u8) != 0xfd {
                    *((e + 0x1e) as *mut u8) = 0xfe;
                }
            }
            i += 1;
        }
        exe::crt_qsort(read_u32(this) as usize, (read_u16(this + 4) as i16 as i32) as u32, 0x22, 0x5fb3f0);
    }
    exe::hostcountry_post_host_news(this, l);
    exe::drop_llist(l);
}

/// writes CM3_TEMP\HostCountry.tmp: the count (+4, u16) and 0x22 bytes per host from [+0], without checking the writes;
/// on open error the "Error" box
/// Original: `HostCountry::save_temp_file` (0x5fb320, host_country.cpp, thiscall).
///
/// [pt] { 0x5fb320 } (thiscall, ret 0, host_country.cpp): `HostCountry::save_temp_file`: grava `CM3_TEMP\HostCountry.tmp`:
/// a quantidade (+4, i16) e os registros de 0x22 bytes de [+0], sem conferir a gravação. Erro ao abrir: a caixa
/// "Error" (host_country.cpp 0x475).
pub(crate) unsafe extern "thiscall" fn save_temp_file(this: usize) {
    crate::comparison::branch("cpx_6 gravar paises anfitrioes");
    let f = open_temp_file_for_write(0x9cf10c);
    if f == 0 {
        show_error_at(0x9cf0d8, 0x475);
        return;
    }
    crt_fwrite(this + 4, 2, 1, f);
    crt_fwrite(read_usize(this), 0x22, *((this + 4) as *const i16) as i32 as u32, f); // cpx6: os anfitriões
    uefa_seeding_crt_fclose(f);
}

