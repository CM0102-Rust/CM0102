// ---------- %%LEVA%%a leva: a Copa do Mundo: o sorteio dos grupos (com o laço da queda de 27/12), o sorteio feito no dia e o dia da Copa (world_cup.cpp) ----------

unsafe fn cqf4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqf4_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqf4_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqf4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqf4_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqf4_pb(a: usize, v: u8) {
    *(a as *mut u8) = v
}

/// As chaves das 32 nações do sorteio real de 2002 (os globais g_key_nation_*, índice na tabela de nações [0xae23a8],
/// 0x122 bytes cada), na ordem em que a original as grava na lista: grupos A a H, quatro por grupo.
const CQF4_SORTEIO_2002: [usize; 32] = [
    0x9cf300, 0x9cf460, 0x9cf4f4, 0x9cf2c0, // A: França, Senegal, Uruguai, Dinamarca
    0x9cf488, 0x9cf474, 0x9cf420, 0x9cf480, // B: Espanha, Eslovênia, Paraguai, África do Sul
    0x9cf260, 0x9cf4dc, 0x9cf298, 0x9cf2a8, // C: Brasil, Turquia, China, Costa Rica
    0x9cf484, 0x9cf430, 0x9cf4f8, 0x9cf434, // D: Coreia do Sul, Polônia, Estados Unidos, Portugal
    0x9cf314, 0x9cf458, 0x9cf2dc, 0x9cf27c, // E: Alemanha, Arábia Saudita, Irlanda, Camarões
    0x9cf218, 0x9cf400, 0x9cf2e4, 0x9cf4a8, // F: Argentina, Nigéria, Inglaterra, Suécia
    0x9cf360, 0x9cf2d4, 0x9cf2ac, 0x9cf3d0, // G: Itália, Equador, Croácia, México
    0x9cf36c, 0x9cf244, 0x9cf448, 0x9cf4d8, // H: Japão, Bélgica, Rússia, Tunísia
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
unsafe extern "thiscall" fn draw_groups(this: usize) -> usize {
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let sorteio: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    let principal: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53b3d0usize);
    let mut conta = [0i8; 8];
    let lista = zerada(0x20, 5);
    if lista == 0 {
        show_error_at(0xad5888, 0x4b5);
        return 0;
    }
    let mut s = 0i32;
    while s < 8 {
        conta[s as usize] = conta[s as usize].wrapping_add(1);
        if s == 0 {
            cqf4_pd(lista, cqf4_d(this + 0xee));
            cqf4_pb(lista + 4, 1);
            cqf4_pb(this + 0xf2, 0xff);
        } else if s == 2 {
            cqf4_pd(lista + 0xa, cqf4_d(this + 0xf3));
            cqf4_pb(lista + 0xe, 3);
            cqf4_pb(this + 0xf7, 0xff);
        } else {
            // o laço da queda de 27/12/2001 (0x92ea55): sem vaga de pé entre as 2..7, não termina
            let mut k = sorteio(6).wrapping_add(2) as i16 as i32;
            while cqf4_b(this.wrapping_add((k * 5) as usize) + 0xf2) == 0xff {
                k = sorteio(6).wrapping_add(2) as i16 as i32;
            }
            let c = this.wrapping_add((k * 5) as usize);
            let e = lista + s as usize * 5;
            cqf4_pd(e, cqf4_d(c + 0xee));
            cqf4_pb(e + 4, (s as u8).wrapping_add(1));
            cqf4_pb(c + 0xf2, 0xff);
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
        cqf4_pd(dst, cqf4_d(src));
        src += 5;
        cqf4_pb(dst + 4, (r as u8).wrapping_add(1));
        dst += 5;
        let g = r as i16 as usize;
        conta[g] = conta[g].wrapping_add(1);
        k += 1;
    }
    if cqf4_w(this + 0x40) != 0x7d2 {
        return lista;
    }
    crate::comparacao::ramo("cqf_4 sorteio de 2002");
    for (i, &chave) in CQF4_SORTEIO_2002.iter().enumerate() { // cqf4: as nações do sorteio real
        let nacao = cqf4_d(0xae23a8).wrapping_add((cqf4_d(chave) as i32).wrapping_mul(0x122) as usize);
        cqf4_pd(lista + i * 5, cqf4_d(principal(nacao)));
    }
    for i in 0..32usize {
        let e = lista + i * 5;
        let clube = cqf4_d(0xae23b4).wrapping_add(cqf4_d(e).wrapping_mul(0x245));
        cqf4_pd(cqf4_d(this + 0x14) + i * 6, clube);
        cqf4_pb(cqf4_d(this + 0x14) + i * 6 + 4, 6);
        cqf4_pb(e + 4, (i / 4) as u8 + 1);
    }
    cqf4_pb(cqf4_d(this + 0x14) + 4, 2);
    cqf4_pb(cqf4_d(this + 0x14) + 0x4c, 1);
    cqf4_pb(cqf4_d(this + 0x14) + 0xac, 1);
    cqf4_pw(this + 0x36, 0x20);
    lista
}

/// Lado a lado: a original e a cópia (sem contar os sorteios) a partir do mesmo estado do sorteador, das vagas
/// (+0xee, 0xa0 bytes), de +0x36, de [0xb67a34] e, na Copa de 2002, da tabela das nações ([+0x14], 32 × 6); as duas
/// listas alocadas têm de ser iguais byte a byte (a da cópia é liberada, 0x9452ca).
unsafe extern "thiscall" fn comparar_draw_groups(this: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> usize>(%%I0%%);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend_from_slice(&[(this + 0xee, 0xa0), (this + 0x36, 2), (0xb67a34, 4)]);
    if cqf4_w(this + 0x40) == 0x7d2 && cqf4_d(this + 0x14) != 0 {
        reg.push((cqf4_d(this + 0x14), 0xc0));
    }
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| draw_groups(this)));
    let mesma = if a != 0 && b != 0 {
        core::slice::from_raw_parts(a as *const u8, 0xa0) == core::slice::from_raw_parts(b as *const u8, 0xa0)
    } else {
        a == 0 && b == 0
    };
    if b != 0 {
        soltar(b);
    }
    PLACAR.registrar(igual && mesma, || format!("draw_groups: memória igual={} lista igual={}", igual, mesma));
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
unsafe extern "thiscall" fn run_group_draw(this: usize) {
    let sortear: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x92e940usize);
    let grupo_um: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x92d0f0usize);
    let criar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x6827d0usize);
    let fase: unsafe extern "thiscall" fn(usize, i32, usize) -> u32 = core::mem::transmute(0x92d540usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let cartoes: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7e9180usize);
    let lista = sortear(this);
    if grupo_um(this, lista) == 0 {
        show_error_at(0xad5888, 0x46d);
        return;
    }
    if criar(this) == 0 {
        show_error_at(0xad5888, 0x473);
        return;
    }
    if ordenar(this, 0) == 0 {
        show_error_at(0xad5888, 0x47a);
        return;
    }
    cqf4_pd(this + 0x45, 1);
    for g in 0..7 {
        fase(this, g, lista);
    }
    soltar(lista);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(cqf4_d(cqf4_d(this) + 0x5c));
    previstas(this);
    let orgao = cqf4_d(cqf4_d(cqf4_d(0xae28f0)).wrapping_add((cqf4_b(this + 0x50) as i8 as i32 * 4) as usize));
    cartoes(orgao, 0);
}

/// Só a original (o sorteio, os jogos, as notícias e as previsões não rodam duas vezes), conferindo o que ela garante:
/// terminada (+0x45 = 1), as sete fases dos grupos 2..8 existem e a fase atual (+0x30) é a última delas (6).
unsafe extern "thiscall" fn comparar_run_group_draw(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this);
    if cqf4_d(this + 0x45) == 1 {
        let fases = cqf4_d(this + 0xc);
        let ok = fases != 0 && (0..7).all(|g| cqf4_d(fases + g * 4) != 0) && cqf4_d(this + 0x30) == 6;
        PLACAR.registrar(ok, || format!("run_group_draw: fases dos grupos ou +0x30 ({}) fora do esperado", cqf4_d(this + 0x30) as i32));
    }
}

/// { 0x92f1a0 } (thiscall, ret 8, world_cup.cpp, método +0x14 da Copa do Mundo): `WorldCup::update_for_date(hoje,
/// completo)`, o dia da Copa pelo dia das competições. Na data das cabeças de chave (+0x18e o dia do ano, +0x190 o ano
/// relativo a +0x40: 15/12 do ano anterior, gravados pela 0x92d200) e com `completo`, semeia (0x92f230 mov ecx ->
/// thiscall, ret 0); na data do sorteio (+0xb6, +0xb8: 27/12 do ano anterior), com `completo` e sem tabela ainda (+0xb1
/// = 0), o sorteio (0x92e7f0 mov ecx -> thiscall, ret 0). Depois o dia de liga (0x6847c0 push completo; push hoje; mov
/// ecx -> thiscall, ret 8), cuja devolução (eax) ela repassa. A data: +0 o dia do ano, +2 o ano (sem sinal).
unsafe extern "thiscall" fn world_cup_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let semear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x92f230usize);
    let sortear: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x92e7f0usize);
    let liga: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x6847c0usize);
    let ano = core::ptr::read_unaligned((hoje + 2) as *const u16) as i32;
    let dia = core::ptr::read_unaligned(hoje as *const u16);
    if cqf4_w(this + 0x18e) as u16 == dia && cqf4_w(this + 0x190) as i32 + cqf4_w(this + 0x40) as i32 == ano && completo != 0 {
        semear(this);
    }
    let dia_do_sorteio = cqf4_w(this + 0xb6) as u16; // cqf4: o dia do sorteio
    if dia_do_sorteio == dia && cqf4_w(this + 0xb8) as i32 + cqf4_w(this + 0x40) as i32 == ano && completo != 0 && cqf4_d(this + 0xb1) == 0 {
        crate::comparacao::ramo("cqf_4 dia do sorteio");
        sortear(this);
    }
    liga(this, hoje, completo)
}

/// Só a original (a semeadura, o sorteio e o dia da liga não rodam duas vezes), conferindo o que ela garante: no dia do
/// sorteio (a condição da cópia), a Copa sai com a tabela do grupo 1 (+0xb1) e +0x45 = 1.
unsafe extern "thiscall" fn comparar_world_cup_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    let ano = core::ptr::read_unaligned((hoje + 2) as *const u16) as i32;
    let dia = core::ptr::read_unaligned(hoje as *const u16);
    let sorteia = cqf4_w(this + 0xb6) as u16 == dia && cqf4_w(this + 0xb8) as i32 + cqf4_w(this + 0x40) as i32 == ano && completo != 0 && cqf4_d(this + 0xb1) == 0;
    let r = o(this, hoje, completo);
    if sorteia {
        PLACAR.registrar(cqf4_d(this + 0xb1) != 0 && cqf4_d(this + 0x45) == 1, || format!("world_cup_update_for_date: dia do sorteio sem a tabela (+0xb1 = {:#x}, +0x45 = {})", cqf4_d(this + 0xb1), cqf4_d(this + 0x45)));
    }
    r
}
