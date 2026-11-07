// ---------- 270a leva: a Copa do Mundo: o sorteio dos grupos (com o laço da queda de 27/12), o sorteio feito no dia e o dia da Copa (world_cup.cpp) ----------







/// As chaves das 32 nações do sorteio real de 2002 (os globais g_key_nation_*, índice na tabela de nações [0xae23a8],
/// 0x122 bytes cada), na ordem em que a original as grava na lista: grupos A a H, quatro por grupo.
const CQF4_SORTEIO_2002: [usize; 32] = [
    KEY_NATION_FRANCE, KEY_NATION_SENEGAL, KEY_NATION_URUGUAY, KEY_NATION_DENMARK, // A: França, Senegal, Uruguai, Dinamarca
    KEY_NATION_SPAIN, KEY_NATION_SLOVENIA, KEY_NATION_PARAGUAY, KEY_NATION_SOUTH_AFRICA, // B: Espanha, Eslovênia, Paraguai, África do Sul
    KEY_NATION_BRAZIL, KEY_NATION_TURKEY, KEY_NATION_CHINA, KEY_NATION_COSTA_RICA, // C: Brasil, Turquia, China, Costa Rica
    KEY_NATION_SOUTH_KOREA, KEY_NATION_POLAND, KEY_NATION_UNITED_STATES, KEY_NATION_PORTUGAL, // D: Coreia do Sul, Polônia, Estados Unidos, Portugal
    KEY_NATION_GERMANY, KEY_NATION_SAUDI_ARABIA, KEY_NATION_REPUBLIC_OF_IRELAND, KEY_NATION_CAMEROON, // E: Alemanha, Arábia Saudita, Irlanda, Camarões
    KEY_NATION_ARGENTINA, KEY_NATION_NIGERIA, KEY_NATION_ENGLAND, KEY_NATION_SWEDEN, // F: Argentina, Nigéria, Inglaterra, Suécia
    KEY_NATION_ITALY, KEY_NATION_ECUADOR, KEY_NATION_CROATIA, KEY_NATION_MEXICO, // G: Itália, Equador, Croácia, México
    KEY_NATION_JAPAN, KEY_NATION_BELGIUM, KEY_NATION_RUSSIA, KEY_NATION_TUNISIA, // H: Japão, Bélgica, Rússia, Tunísia
];

/// { 0x92e940 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::draw_groups()`, do sorteio feito no dia
/// (0x92e7f0). A lista do sorteio: 32 entradas de 5 bytes (+0 o id do clube da seleção, +4 o grupo 1..8), zerada
/// (0x945501(0x20, 5): push 5; push 0x20; add esp, 8 -> cdecl; sem memória, a caixa da linha 0x4b5 e devolve 0).
/// 1) Os cabeças de chave (8 vagas de 5 bytes em +0xee: +0 o id, +4 o pote; 0xff = já tirada): a vaga 0 vai para o
///    grupo 1 e a vaga 1 (+0xf3) para o grupo 3; os grupos 2 e 4..8 tiram uma das vagas 2..7 (0x90cfc0(6) + 2),
///    sorteando de novo enquanto a vaga sorteada já saiu. ESTE É O LAÇO DA QUEDA DE 27/12/2001 (o jogo para sem erro,
///    contornado pelo Alan_v2 trocando o desvio em 0x92ea55): se menos de seis das vagas 2..7 estão de pé (numa carreira
///    começada antes de 2001, a seed_teams 0x92f230 não semeia a Copa de 2002 e as vagas ficam 0xff), o sorteio nunca
///    acha vaga e o laço não termina. A cópia repete o laço como a original, sem limite.
/// 2) Os outros 24 (as vagas de +0x116, na ordem) caem em grupos sorteados (0x90cfc0(8)), cada grupo com no máximo
///    (k / 8) + 1 times quando o k-ésimo entra (k de 8 a 31): sorteia de novo enquanto o grupo já tem o limite.
/// 3) Na Copa de 2002 (+0x40 = 2002), depois do sorteio, a lista vira o sorteio real (a ordem de `CQF4_SORTEIO_2002`:
///    o id do clube da seleção principal, 0x53b3d0 push nação; add esp -> cdecl), os grupos i/4 + 1, e a tabela das 32
///    nações da Copa ([+0x14], 6 bytes: +0 o clube, +4 a situação) recebe os clubes com a situação 6, a França (detentora)
///    2, a Coreia e o Japão (sedes) 1; +0x36 (quantas) = 32.
/// Devolve a lista.
unsafe extern "thiscall" fn mundial_sortear_grupos(this: usize) -> usize {
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(FN_CRT_CALLOC);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(FN_UTILS_RANDOM);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let mut conta = [0i8; 8];
    let lista = zerada(0x20, 5);
    if lista == 0 {
        aviso_em(0xad5888, 0x4b5);
        return 0;
    }
    let mut s = 0i32;
    while s < 8 {
        conta[s as usize] = conta[s as usize].wrapping_add(1);
        if s == 0 {
            write_u32_from_usize(lista, read_u32_as_usize(this + 0xee));
            write_u8(lista + 4, 1);
            write_u8(this + 0xf2, 0xff);
        } else if s == 2 {
            write_u32_from_usize(lista + 0xa, read_u32_as_usize(this + 0xf3));
            write_u8(lista + 0xe, 3);
            write_u8(this + 0xf7, 0xff);
        } else {
            // o laço da queda de 27/12/2001 (0x92ea55): sem vaga de pé entre as 2..7, não termina
            let mut k = sorteio(6).wrapping_add(2) as i16 as i32;
            while read_u8(this.wrapping_add((k * 5) as usize) + 0xf2) == 0xff {
                k = sorteio(6).wrapping_add(2) as i16 as i32;
            }
            let c = this.wrapping_add((k * 5) as usize);
            let e = lista + s as usize * 5;
            write_u32_from_usize(e, read_u32_as_usize(c + 0xee));
            write_u8(e + 4, (s as u8).wrapping_add(1));
            write_u8(c + 0xf2, 0xff);
        }
        s += 1;
    }
    let mut src = this + 0x116;
    let mut dst = lista + 0x28;
    let mut k = 8i32;
    for _ in 0..0x18 {
        let lim = ((k / 8) as i8).wrapping_add(1); // cqf4: o limite de times por grupo
        let mut r = sorteio(8);
        while conta[r as i16 as usize] >= lim {
            r = sorteio(8);
        }
        crate::comparacao::ramo("cqf_4 vaga no grupo");
        write_u32_from_usize(dst, read_u32_as_usize(src));
        src += 5;
        write_u8(dst + 4, (r as u8).wrapping_add(1));
        dst += 5;
        let g = r as i16 as usize;
        conta[g] = conta[g].wrapping_add(1);
        k += 1;
    }
    if read_i16(this + 0x40) != 0x7d2 {
        return lista;
    }
    crate::comparacao::ramo("cqf_4 sorteio de 2002");
    for (i, &chave) in CQF4_SORTEIO_2002.iter().enumerate() { // cqf4: as nações do sorteio real
        let nacao = read_u32_as_usize(0xae23a8).wrapping_add((read_u32_as_usize(chave) as i32).wrapping_mul(0x122) as usize);
        write_u32_from_usize(lista + i * 5, read_u32_as_usize(principal(nacao)));
    }
    for i in 0..32usize {
        let e = lista + i * 5;
        let clube = read_u32_as_usize(0xae23b4).wrapping_add(read_u32_as_usize(e).wrapping_mul(0x245));
        write_u32_from_usize(read_u32_as_usize(this + 0x14) + i * 6, clube);
        write_u8(read_u32_as_usize(this + 0x14) + i * 6 + 4, 6);
        write_u8(e + 4, (i / 4) as u8 + 1);
    }
    write_u8(read_u32_as_usize(this + 0x14) + 4, 2);
    write_u8(read_u32_as_usize(this + 0x14) + 0x4c, 1);
    write_u8(read_u32_as_usize(this + 0x14) + 0xac, 1);
    write_u16(this + 0x36, 0x20);
    lista
}

/// Lado a lado: a original e a cópia (sem contar os sorteios) a partir do mesmo estado do sorteador, das vagas
/// (+0xee, 0xa0 bytes), de +0x36, de [0xb67a34] e, na Copa de 2002, da tabela das nações ([+0x14], 32 × 6); as duas
/// listas alocadas têm de ser iguais byte a byte (a da cópia é liberada, 0x9452ca).
unsafe extern "thiscall" fn comparar_mundial_sortear_grupos(this: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> usize>(1603);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(&[(this + 0xee, 0xa0), (this + 0x36, 2), (0xb67a34, 4)]);
    if read_i16(this + 0x40) == 0x7d2 && read_u32_as_usize(this + 0x14) != 0 {
        reg.push((read_u32_as_usize(this + 0x14), 0xc0));
    }
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| mundial_sortear_grupos(this)));
    let mesma = if a != 0 && b != 0 {
        core::slice::from_raw_parts(a as *const u8, 0xa0) == core::slice::from_raw_parts(b as *const u8, 0xa0)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && mesma, || format!("mundial_sortear_grupos: memória igual={} lista igual={}", igual, mesma));
    a
}

/// { 0x92e7f0 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::run_group_draw()`, do dia do sorteio (0x92f1a0).
/// A lista do sorteio (0x92e940, mov ecx, this -> thiscall); o grupo 1 na tabela da própria Copa (0x92d0f0 push lista;
/// mov ecx -> thiscall, ret 4; 0: a caixa da linha 0x46d e fim); os jogos dele (0x6835c0 mov ecx -> thiscall, ret 0;
/// 0: a linha 0x473); a tabela ordenada (0x6827d0 push 0; mov ecx -> thiscall, ret 4; 0: a linha 0x47a). Com tudo
/// feito: +0x45 = 1, as fases dos grupos 2..8 (0x92d540 push lista; push g; mov ecx -> thiscall, ret 8, g de 0 a 6),
/// a lista liberada (0x9452ca push; add esp -> cdecl), as posições previstas (método +0x5c, 0x92f680) e os cartões
/// zerados no órgão da competição (0x7e9180 push 0; ecx = [[0xae28f0] + 4 × (char)+0x50] -> thiscall, ret 4).
/// Num erro a lista não é liberada (como a original).
unsafe extern "thiscall" fn mundial_sorteio_dos_grupos(this: usize) {
    let sortear: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(FN_WORLDCUP_DRAW_GROUPS);
    let grupo_um: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_WORLDCUP_CREATE_GROUP_TABLE);
    let criar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(FN_LEAGUE_CREATE_FIXTURES);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(FN_LEAGUE_SORT_TABLE);
    let fase: unsafe extern "thiscall" fn(usize, i32, usize) -> u32 = core::mem::transmute(FN_WORLDCUP_CREATE_GROUP_STAGE);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let cartoes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(FN_RULINGBODY_RESET_BOOKINGS_FOR_CLUB);
    let lista = sortear(this);
    if grupo_um(this, lista) == 0 {
        aviso_em(0xad5888, 0x46d);
        return;
    }
    if criar(this) == 0 {
        aviso_em(0xad5888, 0x473);
        return;
    }
    if ordenar(this, 0) == 0 {
        aviso_em(0xad5888, 0x47a);
        return;
    }
    write_u32_from_usize(this + 0x45, 1);
    for g in 0..7 {
        fase(this, g, lista);
    }
    soltar(lista);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x5c));
    previstas(this);
    let orgao = read_u32_as_usize(read_u32_as_usize(read_u32_as_usize(0xae28f0)).wrapping_add((read_u8(this + 0x50) as i8 as i32 * 4) as usize));
    cartoes(orgao, 0);
}

/// Só a original (o sorteio, os jogos, as notícias e as previsões não rodam duas vezes), conferindo o que ela garante:
/// terminada (+0x45 = 1), as sete fases dos grupos 2..8 existem e a fase atual (+0x30) é a última delas (6).
unsafe extern "thiscall" fn comparar_mundial_sorteio_dos_grupos(this: usize) {
    orig::<unsafe extern "thiscall" fn(usize)>(1604)(this);
    if read_u32_as_usize(this + 0x45) == 1 {
        let fases = read_u32_as_usize(this + 0xc);
        let ok = fases != 0 && (0..7).all(|g| read_u32_as_usize(fases + g * 4) != 0) && read_u32_as_usize(this + 0x30) == 6;
        PLACAR.registrar(ok, || format!("mundial_sorteio_dos_grupos: fases dos grupos ou +0x30 ({}) fora do esperado", read_u32_as_usize(this + 0x30) as i32));
    }
}

/// { 0x92f1a0 } (thiscall, ret 8, world_cup.cpp, método +0x14 da Copa do Mundo): `WorldCup::update_for_date(hoje,
/// completo)`, o dia da Copa pelo dia das competições. Na data das cabeças de chave (+0x18e o dia do ano, +0x190 o ano
/// relativo a +0x40: 15/12 do ano anterior, gravados pela 0x92d200) e com `completo`, semeia (0x92f230 mov ecx ->
/// thiscall, ret 0); na data do sorteio (+0xb6, +0xb8: 27/12 do ano anterior), com `completo` e sem tabela ainda (+0xb1
/// = 0), o sorteio (0x92e7f0 mov ecx -> thiscall, ret 0). Depois o dia de liga (0x6847c0 push completo; push hoje; mov
/// ecx -> thiscall, ret 8), cuja devolução (eax) ela repassa. A data: +0 o dia do ano, +2 o ano (sem sinal).
unsafe extern "thiscall" fn mundial_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let semear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_WORLDCUP_SEED_TEAMS);
    let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_WORLDCUP_RUN_GROUP_DRAW);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(FN_LEAGUE_UPDATE_FOR_DATE);
    let ano = core::ptr::read_unaligned((hoje + 2) as *const u16) as i32;
    let dia = core::ptr::read_unaligned(hoje as *const u16);
    if read_i16(this + 0x18e) as u16 == dia && read_i16(this + 0x190) as i32 + read_i16(this + 0x40) as i32 == ano && completo != 0 {
        semear(this);
    }
    let dia_do_sorteio = read_i16(this + 0xb6) as u16; // cqf4: o dia do sorteio
    if dia_do_sorteio == dia && read_i16(this + 0xb8) as i32 + read_i16(this + 0x40) as i32 == ano && completo != 0 && read_u32_as_usize(this + 0xb1) == 0 {
        crate::comparacao::ramo("cqf_4 dia do sorteio");
        sortear(this);
    }
    liga(this, hoje, completo)
}

/// Só a original (a semeadura, o sorteio e o dia da liga não rodam duas vezes), conferindo o que ela garante: no dia do
/// sorteio (a condição da cópia), a Copa sai com a tabela do grupo 1 (+0xb1) e +0x45 = 1.
unsafe extern "thiscall" fn comparar_mundial_dia(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(1605);
    let ano = core::ptr::read_unaligned((hoje + 2) as *const u16) as i32;
    let dia = core::ptr::read_unaligned(hoje as *const u16);
    let sorteia = read_i16(this + 0xb6) as u16 == dia && read_i16(this + 0xb8) as i32 + read_i16(this + 0x40) as i32 == ano && completo != 0 && read_u32_as_usize(this + 0xb1) == 0;
    let r = o(this, hoje, completo);
    if sorteia {
        PLACAR.registrar(read_u32_as_usize(this + 0xb1) != 0 && read_u32_as_usize(this + 0x45) == 1, || format!("mundial_dia: dia do sorteio sem a tabela (+0xb1 = {:#x}, +0x45 = {})", read_u32_as_usize(this + 0xb1), read_u32_as_usize(this + 0x45)));
    }
    r
}

// ---------- 270a leva: a Copa do Mundo: a tabela do grupo 1, as fases dos grupos 2 a 8 e as posições previstas (world_cup.cpp) ----------






/// O clube da entrada `i` da lista do sorteio (5 bytes: +0 o id): 0xae23b4 + id × 0x245.
unsafe fn cqf5_clube(lista: usize, i: usize) -> usize {
    read_u32_as_usize(0xae23b4).wrapping_add(read_u32_as_usize(lista + i * 5).wrapping_mul(0x245))
}

/// { 0x92d0f0 } (thiscall, ret 4, world_cup.cpp, sem SEH): `WorldCup::create_group_table(lista)`, do sorteio
/// (0x92e7f0). A própria Copa vira o grupo 1: +0x3e (times) = 4, a tabela [+0xb1] alocada com 0xec bytes (0x944e46 push;
/// add esp -> cdecl; sem memória, a caixa da linha 0x2b1 e devolve 0); as entradas da lista do grupo 1 (+4 = 1), na
/// ordem, entram nas linhas 0..3 (0x687430 push 0; push 0; push clube; push linha; mov ecx, this -> thiscall, ret 0x10;
/// a linha vai no cl e a original deixa lixo nos bytes altos do ecx, que a 0x687430 não lê: a cópia passa o byte
/// limpo), até 4 linhas ou o fim das 32 entradas; depois a notícia dos grupos (0x684230 mov ecx, this -> thiscall).
/// Devolve 1.
unsafe extern "thiscall" fn mundial_tabela_do_grupo_um(this: usize, lista: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let linha_da_tabela: unsafe extern "thiscall" fn(usize, u32, usize, u32, usize) = core::mem::transmute(FN_LEAGUE_INIT_TABLE_ROW);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    write_u16(this + 0x3e, 4);
    let t = alocar(0xec);
    write_u32_from_usize(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xad5888, 0x2b1);
        return 0;
    }
    let mut linha = 0i8;
    let mut i = 0i8;
    while linha < 4 {
        let e = i as usize; // cqf5: a entrada da lista
        if read_u8(lista + e * 5 + 4) == 1 {
            crate::comparacao::ramo("cqf_5 time do grupo um");
            let clube = cqf5_clube(lista, e);
            linha_da_tabela(this, linha as u8 as u32, clube, 0, 0);
            linha += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    noticia(this);
    1
}

/// Os clubes do grupo `g` (+4 = g) da lista, na ordem, até 4 (o que a 0x92d0f0 e a 0x92d540 põem nas linhas).
unsafe fn cqf5_do_grupo(lista: usize, g: i32) -> Vec<usize> {
    (0..32usize).filter(|&i| read_u8(lista + i * 5 + 4) as i8 as i32 == g).map(|i| cqf5_clube(lista, i)).take(4).collect()
}

/// Só a original (aloca a tabela e manda a notícia dos grupos), conferindo o plano feito antes: devolve 1, +0x3e = 4 e
/// as linhas da tabela (0x3b bytes, +0 o clube) com os clubes do grupo 1 da lista, na ordem.
unsafe extern "thiscall" fn comparar_mundial_tabela_do_grupo_um(this: usize, lista: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(1606);
    let plano = cqf5_do_grupo(lista, 1);
    let r = o(this, lista);
    if r == 1 {
        let t = read_u32_as_usize(this + 0xb1);
        let ok = read_i16(this + 0x3e) == 4 && plano.iter().enumerate().all(|(k, &c)| read_u32_as_usize(t + k * 0x3b) == c);
        PLACAR.registrar(ok, || format!("mundial_tabela_do_grupo_um: as linhas não têm os clubes do grupo 1 ({:x?})", plano));
    }
    r
}

/// { 0x92d540 } (thiscall, ret 8, world_cup.cpp, quadro SEH só para o new: sem destrutor no caminho normal):
/// `WorldCup::create_group_stage(g, lista)`, do sorteio (0x92e7f0, g de 0 a 6: os grupos 2 a 8). Os times do grupo g+2
/// (até 4, na ordem da lista) num vetor de +0x3e ponteiros (0x944e46; sem memória, a caixa da linha 0x366 e 0); a tabela
/// das rodadas pelo método +0x3c da Copa (push 0; push &id; push &rodadas; push g; mov ecx -> thiscall, ret 0x10: a
/// 0x92c290 grava as duas saídas para as fases 0..6); a fase (0x944cf1(0xee) push; add esp -> cdecl; o construtor
/// League_Stage 0x68b100, thiscall, ret 0x58: 22 argumentos, conferidos na leitura dele: a Copa, os times (word),
/// o vetor, 1, +4, a tabela das rodadas, as rodadas (word: +0xa9 da fase, a conta das datas), +0xc2, +0xc3, +0xc4,
/// &+0xc5, &+0xbe, o ano +0x40 (word), g (byte), o id (word: +0x3a da fase), +0x51, 4, 0, +0xd9 (word), −1, 0, 2; a original empurra os words e bytes com lixo nos bytes
/// altos, que o construtor não lê: a cópia passa os valores limpos) guardada em [+0xc][g]; o vetor e a tabela das
/// rodadas liberados (0x9452ca). Sem fase: a caixa da linha 0x380 e 0. Com ela: +0x30 = g e a notícia do grupo
/// (0x684230 na fase). Devolve 1.
unsafe extern "thiscall" fn mundial_fase_do_grupo(this: usize, g: i32, lista: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_OPERATOR_NEW);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let noticia: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_LEAGUE_SEND_GROUP_DRAW_NEWS);
    #[allow(clippy::type_complexity)]
    let construir: unsafe extern "thiscall" fn(usize, usize, u32, usize, u32, usize, usize, u32, u32, u32, u32, usize, usize, u32, u32, u32, u32, u32, u32, u32, i32, u32, u32) -> usize =
        core::mem::transmute(FN_LEAGUE_STAGE_LEAGUE_STAGE);
    let times = alocar(((read_i16(this + 0x3e) as i32) << 2) as u32);
    if times == 0 {
        aviso_em(0xad5888, 0x366);
        return 0;
    }
    let mut n = 0i16;
    let mut i = 0i16;
    while n < 4 {
        let e = lista + i as usize * 5;
        if read_u8(e + 4) as i8 as i32 == g + 2 {
            crate::comparacao::ramo("cqf_5 time do grupo");
            write_u32_from_usize(times + n as usize * 4, cqf5_clube(lista, i as usize)); // cqf5: o time na vaga do grupo
            n += 1;
        }
        i += 1;
        if i >= 0x20 {
            break;
        }
    }
    let rodadas_da_copa: unsafe extern "thiscall" fn(usize, i32, *mut i16, *mut i16, usize) -> usize = core::mem::transmute(read_u32_as_usize(read_u32_as_usize(this) + 0x3c));
    let (mut rodadas, mut id) = (0i16, 0i16);
    let tabela = rodadas_da_copa(this, g, &mut rodadas, &mut id, 0);
    let p = novo(0xee);
    let fase = if p != 0 {
        construir(
            p,
            this,
            read_i16(this + 0x3e) as u16 as u32,
            times,
            1,
            read_u32_as_usize(this + 4),
            tabela,
            rodadas as u16 as u32, // cqf5: as rodadas (as datas, +0xa9 da fase)
            read_u8(this + 0xc2) as u32,
            read_u8(this + 0xc3) as u32,
            read_u8(this + 0xc4) as u32,
            this + 0xc5,
            this + 0xbe,
            read_i16(this + 0x40) as u16 as u32,
            g as u8 as u32,
            id as u16 as u32,
            read_u8(this + 0x51) as u32,
            4,
            0,
            read_i16(this + 0xd9) as u16 as u32,
            -1,
            0,
            2,
        )
    } else {
        0
    };
    write_u32_from_usize(read_u32_as_usize(this + 0xc) + g as usize * 4, fase);
    soltar(times);
    soltar(tabela);
    if read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4) == 0 {
        aviso_em(0xad5888, 0x380);
        return 0;
    }
    write_u32_from_usize(this + 0x30, g as usize);
    noticia(read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4));
    1
}

/// Só a original (aloca a fase e manda a notícia), conferindo o plano: devolve 1 com a fase em [+0xc][g], +0x30 = g, a
/// fase com os times (+0x3e) da Copa, o ano dela e, na tabela da fase (+0xb1, linhas de 0x3b bytes), os clubes do
/// grupo g+2 da lista (em qualquer ordem: na comparação de 02/11 as linhas da fase não
/// saíram na ordem da lista [inferência: a fase ordena ou sorteia a tabela dela]).
unsafe extern "thiscall" fn comparar_mundial_fase_do_grupo(this: usize, g: i32, lista: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i32, usize) -> u32>(1607);
    let plano = cqf5_do_grupo(lista, g + 2);
    let r = o(this, g, lista);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
        let t = read_u32_as_usize(f + 0xb1);
        // as rodadas (3) na conta das datas +0xa9 e o id (0x420 + g) em +0x3a: a ordem dos argumentos do construtor
        let ok = f != 0 && read_u32_as_usize(this + 0x30) as i32 == g && read_i16(f + 0x3e) == read_i16(this + 0x3e) && read_i16(f + 0x40) == read_i16(this + 0x40)
            && read_i16(f + 0xa9) == 3 && read_i16(f + 0x3a) as i32 == 0x420 + g
            && (t == 0 || {
                // as linhas da fase não seguem a ordem da lista: confere o conjunto dos clubes
                let mut a: Vec<usize> = (0..plano.len()).map(|k| read_u32_as_usize(t + k * 0x3b)).collect();
                let mut b = plano.clone();
                a.sort_unstable();
                b.sort_unstable();
                a == b
            });
        PLACAR.registrar(ok, || format!("mundial_fase_do_grupo({}): a fase não bate com o plano ({:x?})", g, plano));
    }
    r
}

/// { 0x92f680 } (thiscall, ret 0, world_cup.cpp, método +0x5c da Copa do Mundo): `WorldCup::set_predicted_positions()`,
/// do sorteio (0x92e7f0). Sem estatística (+8): a caixa da linha 0x671 e volta. Menos de 32 nações (+0x36): a caixa da
/// linha 0x678 e +0x36 = 32. Os clubes da tabela das nações ([+0x14], 6 bytes) num vetor (0x944e46; sem memória, a
/// caixa da linha 0x680 e volta), ordenado pela reputação (qsort 0x945333 com 0x4b07d0: push; add esp, 0x10 -> cdecl),
/// e a posição prevista de cada um na estatística (0x4a2540 push posição; push clube; mov ecx, [+8] -> thiscall,
/// ret 8): 1, 2, 3, 4, quatro 5, oito 9, oito 17 e oito 25 (as faixas das fases finais); depois a competição atual
/// da nação de cada clube (+0x53) vira a Copa (0x775220 push [+4]; push nação; mov ecx, [0xb74318] -> thiscall, ret 8)
/// e o vetor é liberado (0x9452ca).
unsafe extern "thiscall" fn mundial_posicoes_previstas(this: usize) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let posicao: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(FN_COMP_STATS_SET_CLUB_FINAL_POSITION);
    let competicao: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(FN_NATIONALTEAMS_SET_CURRENT_COMPETITION);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    if read_u32_as_usize(this + 8) == 0 {
        aviso_em(0xad5888, 0x671);
        return;
    }
    if read_i16(this + 0x36) < 0x20 {
        aviso_em(0xad5888, 0x678);
        write_u16(this + 0x36, 0x20);
    }
    let v = alocar(((read_i16(this + 0x36) as i32) << 2) as u32);
    if v == 0 {
        aviso_em(0xad5888, 0x680);
        return;
    }
    let mut i = 0i16;
    while i < read_i16(this + 0x36) {
        write_u32_from_usize(v + i as usize * 4, read_u32_as_usize(read_u32_as_usize(this + 0x14) + i as usize * 6));
        i += 1;
    }
    ordenar(v, read_i16(this + 0x36) as i32 as u32, 4, 0x4b07d0);
    const FAIXAS: [u32; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    for (k, &p) in FAIXAS.iter().enumerate() {
        posicao(read_u32_as_usize(this + 8), read_u32_as_usize(v + k * 4), p);
    }
    let mut i = 0i16;
    while i < read_i16(this + 0x36) {
        crate::comparacao::ramo("cqf_5 competição da nação");
        competicao(read_u32_as_usize(0xb74318), read_u32_as_usize(read_u32_as_usize(v + i as usize * 4) + 0x53), read_u32_as_usize(this + 4));
        i += 1; // cqf5: a próxima nação
    }
    soltar(v);
}

/// Só a original (acrescenta registros à estatística e avisa o gerenciador dos técnicos), conferindo o plano feito
/// antes: o vetor ordenado como ela ordena (a mesma qsort, numa cópia) e, depois, a posição prevista (+0x72 do registro
/// achado por 0x4a4850) de cada um dos 32 primeiros.
unsafe extern "thiscall" fn comparar_mundial_posicoes_previstas(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1608);
    if read_u32_as_usize(this + 8) == 0 || read_i16(this + 0x36) < 0x20 || read_u32_as_usize(this + 0x14) == 0 {
        return o(this);
    }
    let ordenar: unsafe extern "C" fn(*mut usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let achar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(FN_COMP_STATS_FIND_CLUB_ENTRY);
    let n = read_i16(this + 0x36) as usize;
    let mut v: Vec<usize> = (0..n).map(|i| read_u32_as_usize(read_u32_as_usize(this + 0x14) + i * 6)).collect();
    ordenar(v.as_mut_ptr(), n as u32, 4, 0x4b07d0);
    o(this);
    const FAIXAS: [u8; 32] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 9, 9, 9, 9, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x11, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19, 0x19];
    let errado = (0..32).find(|&k| {
        let e = if v[k] != 0 { achar(read_u32_as_usize(this + 8), v[k]) } else { 0 };
        e == 0 || read_u8(e + 0x72) != FAIXAS[k]
    });
    PLACAR.registrar(errado.is_none(), || format!("mundial_posicoes_previstas: a posição prevista da vaga {:?} não bate", errado));
}

// ---------- 270a leva: a Copa do Mundo: a tabela das rodadas de cada fase, a nação classificada e as cabeças de chave (world_cup.cpp) ----------







/// Os jogos de cada fase de grupos na tabela das rodadas (0x68a1c0: rodada, jogo, a, b, c, valor), na ordem da
/// original, pela fase + 1 (a tabela de saltos 0x92ccd0: −1 é a própria Copa, o grupo 1; 0..6 os grupos 2..8).
const CQF6_JOGOS: [[(i32, i32, i32, i32, i32, u32); 6]; 8] = [
    [(0, 0, 2, 1, 3, 4), (0, 1, 2, 2, 3, 0x4000), (1, 0, 1, 1, 3, 0x1000), (1, 1, 1, 2, 3, 0x400), (2, 0, 1, 2, 3, 0x40), (2, 1, 1, 2, 3, 0x2000)],
    [(0, 0, 3, 1, 3, 0x1000), (0, 1, 3, 2, 3, 0x800), (1, 0, 2, 1, 3, 0x2000), (1, 1, 2, 2, 3, 0x4000), (2, 0, 1, 1, 3, 4), (2, 1, 1, 1, 3, 0x400)],
    [(0, 0, 4, 1, 3, 0x200), (0, 1, 4, 2, 3, 0x40), (1, 0, 3, 2, 3, 0x800), (1, 1, 3, 1, 3, 4), (2, 0, 2, 1, 3, 0x100), (2, 1, 2, 1, 3, 0x1000)],
    [(0, 0, 4, 1, 3, 0x4000), (0, 1, 5, 1, 3, 0x400), (1, 0, 5, 1, 3, 0x2000), (1, 1, 5, 2, 3, 0x80), (2, 0, 2, 2, 3, 0x200), (2, 1, 2, 2, 3, 0x800)],
    [(0, 0, 5, 1, 3, 0x100), (0, 1, 5, 2, 3, 4), (1, 0, 5, 1, 3, 0x1000), (1, 1, 5, 2, 3, 0x40), (2, 0, 3, 1, 3, 0x2000), (2, 1, 3, 1, 3, 0x80)],
    [(0, 0, 6, 1, 3, 0x2000), (0, 1, 0, 2, 3, 0x80), (1, 0, 6, 1, 3, 0x200), (1, 1, 6, 2, 3, 0x100), (2, 0, 3, 2, 3, 0x4000), (2, 1, 3, 2, 3, 0x400)],
    [(0, 0, 0, 1, 3, 0x40), (0, 1, 0, 2, 3, 0x100), (1, 0, 0, 1, 3, 0x4000), (1, 1, 0, 2, 3, 0x800), (2, 0, 4, 2, 3, 4), (2, 1, 4, 2, 3, 0x200)],
    [(0, 0, 6, 1, 3, 0x800), (0, 1, 6, 2, 3, 0x200), (1, 0, 5, 1, 3, 0x400), (1, 1, 6, 1, 3, 0x80), (2, 0, 4, 1, 3, 0x1000), (2, 1, 4, 1, 3, 0x100)],
];

/// { 0x92c290 } (thiscall, ret 0x10, world_cup.cpp, método +0x3c da Copa do Mundo): `WorldCup::create_schedule(fase,
/// &rodadas, &id, &é_liga)`, da criação das regras (0x92d200, fase −1) e das fases dos grupos (0x92d540, 0..6). Nas
/// fases de grupos (char < 7): é_liga (se pedido) = 1, rodadas = 3, id = 0x420 + fase; a tabela de 3 rodadas de 0x41
/// bytes (0x944e46(0xc3); sem memória, a caixa da linha 0xcc e devolve 0), as datas (0x68a160: 12, 19 e 24 de junho
/// do ano da Copa, levadas aos dias da semana 4, 4 e 2, código 1, valor 1), o jogo 2 de cada rodada vazio (−1) e os
/// dois jogos de cada rodada da fase (`CQF6_JOGOS`). O mata-mata (fase 7, com as rotinas da copa) e as fases acima:
/// a original. Devolve a tabela.
unsafe extern "thiscall" fn mundial_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let f = fase as u8 as i8;
    if f >= 7 {
        return orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1609)(this, fase, rodadas, id, liga);
    }
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(FN_CRT_MALLOC);
    let data: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, i32, u32, i32) -> usize = core::mem::transmute(FN_LEAGUE_ROUND_SET_DATE);
    let jogo: unsafe extern "C" fn(usize, i32, i32, i32, i32, i32, u32) = core::mem::transmute(FN_LEAGUE_ROUND_SET_MATCH_SLOT);
    if !liga.is_null() {
        *liga = 1;
    }
    *rodadas = 3; // cqf6: três rodadas
    *id = (f as i16).wrapping_add(0x420);
    let t = alocar(0xc3);
    if t == 0 {
        aviso_em(0xad5888, 0xcc);
        return 0;
    }
    crate::comparacao::ramo("cqf_6 tabela das rodadas");
    // o ano vai com lixo nos bytes altos na original (o cx do ano sobre o ecx de antes); a 0x68a160 só usa os 16 bits
    // de baixo (a 0x549f70 lê o ano como word e a palavra +2 guarda ano − base): a cópia passa o ano limpo
    let ano = read_i16(this + 0x40) as u16 as u32;
    data(t, 0, 0xc, 5, 0, 4, 1, ano, 1); // cqf6: a primeira rodada
    jogo(t, 0, 2, -1, -1, -1, 0);
    data(t, 1, 0x13, 5, 0, 4, 1, ano, 1); // cqf6: a segunda rodada
    jogo(t, 1, 2, -1, -1, -1, 0);
    data(t, 2, 0x18, 5, 0, 2, 1, ano, 1); // cqf6: a terceira rodada
    jogo(t, 2, 2, -1, -1, -1, 0);
    let k = (f as i32 + 1) as u32;
    if k <= 7 {
        for &(r, j, a, b, c, v) in CQF6_JOGOS[k as usize].iter() {
            jogo(t, r, j, a, b, c, v);
        }
    }
    t
}

/// Os bytes que a tabela das rodadas tem gravados (o resto da memória alocada não é escrito): de cada rodada, +0..+4 e
/// +0x3d..+0x40; de cada um dos 3 jogos, +5..+0xb.
unsafe fn cqf6_gravados(t: usize) -> Vec<u8> {
    let mut v = Vec::new();
    for r in 0..3usize {
        let e = t + r * 0x41;
        v.extend_from_slice(core::slice::from_raw_parts(e as *const u8, 5));
        v.extend_from_slice(core::slice::from_raw_parts((e + 0x3d) as *const u8, 4));
        for j in 0..3usize {
            v.extend_from_slice(core::slice::from_raw_parts((e + j * 7 + 5) as *const u8, 7));
        }
    }
    v
}

/// Lado a lado nas fases de grupos: a original com as saídas de quem chama e a cópia com saídas próprias, a partir do
/// mesmo [0xb67a34]; as saídas e os bytes gravados das duas tabelas alocadas têm de ser iguais (a da cópia é liberada).
/// O mata-mata: só a original.
unsafe extern "thiscall" fn comparar_mundial_rodadas_da_fase(this: usize, fase: u32, rodadas: *mut i16, id: *mut i16, liga: *mut i32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(1609);
    if fase as u8 as i8 >= 7 {
        return o(this, fase, rodadas, id, liga);
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let (mut r2, mut i2, mut l2) = (0i16, 0i16, 0i32);
    let reg = [(0xb67a34usize, 4usize)];
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(
        &reg,
        || o(this, fase, rodadas, id, liga),
        || mundial_rodadas_da_fase(this, fase, &mut r2, &mut i2, if liga.is_null() { core::ptr::null_mut() } else { &mut l2 }),
    );
    let saidas = *rodadas == r2 && *id == i2 && (liga.is_null() || *liga == l2);
    let tabela = if a != 0 && b != 0 { cqf6_gravados(a) == cqf6_gravados(b) } else { a == 0 && b == 0 };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && saidas && tabela, || format!("mundial_rodadas_da_fase({}): saídas {} tabela {}", fase as u8 as i8, saidas, tabela));
    a
}

/// { 0x92e740 } (thiscall, ret 4, world_cup.cpp, sem SEH): `WorldCup::add_qualified_team(clube)`, das eliminatórias
/// (0x90f4f0, 0x911e10, 0x91538e, 0x9184c0, 0x91e200, 0x920500): com a lista das nações (+0x36) abaixo do máximo
/// (+0x38), a vaga +0x36 da tabela [+0x14] (6 bytes) recebe a situação 6 e o clube, e +0x36 sobe; cheia, a caixa da
/// linha 0x456.
unsafe extern "thiscall" fn mundial_nacao_classificada(this: usize, clube: usize) {
    let n = read_i16(this + 0x36);
    if n >= read_i16(this + 0x38) {
        aviso_em(0xad5888, 0x456);
        return;
    }
    crate::comparacao::ramo("cqf_6 nação classificada");
    let e = read_u32_as_usize(this + 0x14).wrapping_add((n as i32 * 6) as usize);
    write_u8(e + 4, 6);
    write_u32_from_usize(read_u32_as_usize(this + 0x14).wrapping_add((read_i16(this + 0x36) as i32 * 6) as usize), clube);
    write_u16(this + 0x36, (read_i16(this + 0x36) as u16).wrapping_add(1));
}

/// Lado a lado (+0x36 e a vaga da tabela); lista cheia (a caixa): só a original.
unsafe extern "thiscall" fn comparar_mundial_nacao_classificada(this: usize, clube: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize)>(1610);
    let n = read_i16(this + 0x36);
    if n >= read_i16(this + 0x38) || read_u32_as_usize(this + 0x14) == 0 {
        return o(this, clube);
    }
    let reg = [(this + 0x36, 2usize), (read_u32_as_usize(this + 0x14).wrapping_add((n as i32 * 6) as usize), 6usize)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube), || mundial_nacao_classificada(this, clube));
    PLACAR.registrar(igual, || format!("mundial_nacao_classificada: {:x?} x {:x?}", mo, mr));
}

/// { 0x92f230 } (thiscall, ret 0, world_cup.cpp, sem SEH): `WorldCup::seed_teams()`, do dia da Copa (0x92f1a0, na
/// data das cabeças de chave). Na Copa de 2002 (+0x40 = 2002) volta sem fazer nada: as vagas do sorteio dela vêm do
/// banco de dados, e é por isso que uma Copa de 2002 criada por outro caminho trava o sorteio (o laço da 0x92e940, cqf_4).
/// Nas outras: +0x4b = 0; com menos de 32 nações (+0x36), a caixa da linha 0x59e e completa a lista com a seleção
/// principal (0x53b3d0) de cada nação ([0xae2350] nações de 0x122 bytes) que ainda não esteja nela, até 32; ordena a
/// tabela [+0x14] (6 bytes) pela reputação (qsort 0x945333 com 0x4b07b0); as sedes (0x5fa730 em [0xb5e134]: thiscall,
/// ret 0x14, com o id da Copa [0x9cf79c], o ano como word, as duas saídas e 1; saída −1 = sem sede) e a detentora (o
/// campeão mais recente, 0x4afd80 da competição [0xae23d0] + id × 0x6b) vão para as vagas 1, 2 e 0 (trocando de lugar),
/// e as outras são ordenadas de novo a partir da primeira livre; as 8 primeiras ficam cabeças de chave (pote 0,
/// situação 3) e as outras 24 vão para o pote do continente (+0x53 nação, +0x71 continente: [0x9cfa10] pote 1,
/// [0x9cfa18]/[0x9cfa0c]/[0x9cfa1c] pote 2, [0x9cfa08]/[0x9cfa14] pote 3; sem nação ou continente, a caixa da linha
/// 0x61e) com o id do clube nas vagas +0xee (5 bytes); por fim as 24 da tabela (a partir da 9ª) e as vagas +0x116 são
/// ordenadas pelo pote (0x92f660).
unsafe extern "thiscall" fn mundial_cabecas_de_chave(this: usize) {
    if read_i16(this + 0x40) == 0x7d2 {
        crate::comparacao::ramo("cqf_6 sem cabeças de chave em 2002");
        return;
    }
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(FN_DATABASE_GET_NATIONAL_TEAM);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(FN_CRT_QSORT);
    let sedes: unsafe extern "thiscall" fn(usize, usize, u32, *mut i32, *mut i32, u32) -> u8 = core::mem::transmute(FN_HOSTCOUNTRY_FIND_HOSTS);
    let campeao: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(FN_COMP_UTIL_GET_LATEST_PLACED_CLUB);
    write_u8(this + 0x4b, 0);
    if read_i16(this + 0x36) < 0x20 {
        aviso_em(0xad5888, 0x59e);
        let mut k = 0i32;
        let mut desloc = 0usize;
        if read_u32_as_usize(0xae2350) as i32 > 0 {
            while read_i16(this + 0x36) < 0x20 {
                let mut j = 0i16;
                let mut ja = false;
                while j < read_i16(this + 0x36) {
                    let c = principal(read_u32_as_usize(0xae23a8).wrapping_add(desloc));
                    if read_u32_as_usize(read_u32_as_usize(this + 0x14) + j as usize * 6) == c {
                        ja = true;
                        break;
                    }
                    j += 1;
                }
                if !ja {
                    let c = principal(read_u32_as_usize(0xae23a8).wrapping_add(desloc));
                    write_u32_from_usize(read_u32_as_usize(this + 0x14) + read_i16(this + 0x36) as usize * 6, c);
                    write_u16(this + 0x36, (read_i16(this + 0x36) as u16).wrapping_add(1));
                }
                k += 1;
                desloc += 0x122;
                if k >= read_u32_as_usize(0xae2350) as i32 {
                    break;
                }
            }
        }
    }
    ordenar(read_u32_as_usize(this + 0x14), read_i16(this + 0x36) as i32 as u32, 6, 0x4b07b0);
    // ponytail: as duas saídas começam em −1 (sem sede); a original as deixa com o resto da pilha quando a 0x5fa730 não
    // acha a Copa (ela só as grava achando)
    let (mut a, mut b) = (-1i32, -1i32);
    sedes(read_u32_as_usize(0xb5e134), read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP), read_i16(this + 0x40) as u16 as u32, &mut a, &mut b, 1);
    let sede1 = if a != -1 { principal(read_u32_as_usize(0xae23a8).wrapping_add(a.wrapping_mul(0x122) as usize)) } else { 0 };
    let sede2 = if b != -1 { principal(read_u32_as_usize(0xae23a8).wrapping_add(b.wrapping_mul(0x122) as usize)) } else { 0 };
    let detentor = campeao(read_u32_as_usize(0xae23d0).wrapping_add((read_u32_as_usize(KEY_CLUB_COMP_FIFA_WORLD_CUP) as i32).wrapping_mul(0x6b) as usize), 0);
    let mut fixos = 0i8;
    for i in 0..32usize {
        for (vaga, alvo) in [(0usize, detentor), (1, sede1), (2, sede2)] {
            let t = read_u32_as_usize(this + 0x14);
            let e = t + i * 6;
            if read_u32_as_usize(e) == alvo {
                let (c, w) = (read_u32_as_usize(e), read_i16(e + 4) as u16);
                let f = t + vaga * 6;
                write_u32_from_usize(e, read_u32_as_usize(f));
                write_u16(e + 4, read_i16(f + 4) as u16);
                let t = read_u32_as_usize(this + 0x14) + vaga * 6;
                write_u32_from_usize(t, c);
                write_u16(t + 4, w);
                if fixos < vaga as i8 + 1 {
                    fixos = vaga as i8 + 1;
                }
            }
        }
    }
    let mut n = fixos;
    if sede2 == sede1 || sede2 == detentor {
        n = 2;
    }
    if sede1 == detentor {
        n = 1;
    }
    ordenar(read_u32_as_usize(this + 0x14) + n as i32 as usize * 6, (0x20 - n as i32) as u32, 6, 0x4b07b0);
    for i in 0..0x20usize {
        let clube = read_u32_as_usize(read_u32_as_usize(this + 0x14) + i * 6);
        let pote = this + 0xf2 + i * 5;
        write_u32_from_usize(pote - 4, read_u32_as_usize(clube));
        if i < 8 {
            write_u8(pote, 0);
            write_u8(read_u32_as_usize(this + 0x14) + i * 6 + 4, 3);
            continue;
        }
        let nacao = read_u32_as_usize(clube + 0x53);
        let cont = if nacao != 0 { read_u32_as_usize(nacao + 0x71) } else { 0 };
        if cont == 0 {
            aviso_em(0xad5888, 0x61e);
            continue;
        }
        let c = read_u32_as_usize(cont);
        let (p, s) = if c == read_u32_as_usize(KEY_CONTINENT_EUROPE) {
            (1, 0xa)
        } else if c == read_u32_as_usize(KEY_CONTINENT_OCEANIA) || c == read_u32_as_usize(KEY_CONTINENT_ASIA) || c == read_u32_as_usize(KEY_CONTINENT_SOUTH_AMERICA) {
            (2, 0xb)
        } else if c == read_u32_as_usize(KEY_CONTINENT_AFRICA) || c == read_u32_as_usize(KEY_CONTINENT_NORTH_AMERICA) {
            (3, 0xc)
        } else {
            continue;
        };
        write_u8(pote, p);
        write_u8(read_u32_as_usize(this + 0x14) + i * 6 + 4, s);
    }
    ordenar(read_u32_as_usize(this + 0x14) + 0x30, 0x18, 6, 0x92f660);
    ordenar(this + 0x116, 0x18, 5, 0x92f660);
}

/// Na Copa de 2002 (volta sem gravar nada) lado a lado em +0x4b; nas outras, só a original (caixas, e as saídas da
/// 0x5fa730 que a original pode deixar sem gravar).
unsafe extern "thiscall" fn comparar_mundial_cabecas_de_chave(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(1611);
    if read_i16(this + 0x40) != 0x7d2 {
        return o(this);
    }
    let reg = [(this + 0x36, 0x16usize), (this + 0xee, 0xa0usize)];
    let (_, _, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || mundial_cabecas_de_chave(this));
    PLACAR.registrar(igual, || "mundial_cabecas_de_chave: a Copa de 2002 mudou".to_string());
}

