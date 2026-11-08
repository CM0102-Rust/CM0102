// ---------- %%LEVA%%a leva: o sorteio da rodada da copa, o nome da fase e da rodada e o clube da divisão mais alta (Cup.cpp, comp_util.cpp) ----------

unsafe fn cqa3_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqa3_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqa3_b(a: usize) -> u8 {
    *(a as *const u8)
}

/// O registro de 0x68 bytes da rodada `r` (+0xa3); `r` como a original o usa (o dword inteiro).
unsafe fn round_record_by_dword(copa: usize, r: u32) -> usize {
    cqa3_d(copa + 0xa3).wrapping_add((r as usize).wrapping_mul(0x68))
}

/// Os confrontos da rodada `r` (+0xad: um vetor de confrontos de 0x35 bytes por rodada).
unsafe fn round_ties_array(copa: usize, r: u32) -> usize {
    cqa3_d(cqa3_d(copa + 0xad).wrapping_add((r as usize).wrapping_mul(4)))
}

/// Algum time definido (≠ −1) de algum confronto da rodada ainda sem clube (o vencedor de um confronto anterior em
/// aberto): a rodada não pode ser sorteada.
unsafe fn round_has_open_team(copa: usize, r: u32) -> bool {
    let reg = round_record_by_dword(copa, r);
    let mut t = round_ties_array(copa, r);
    let mut k = 0i32;
    while k < cqa3_w(reg + 0x1a) as i32 {
        for lado in [0usize, 2] {
            let x = cqa3_w(t + lado);
            if x != -1 && cqa3_d(cqa3_d(copa + 0xa7).wrapping_add((x as isize * 6) as usize)) == 0 {
                return true;
            }
        }
        t += 0x35;
        k += 1;
    }
    false
}

/// A data do sorteio da rodada (+0 o dia, +2 o ano em byte a partir de +0x40 da copa), numa Data local (0x549ef0:
/// push ano; push dia; ecx = a local; sem add esp -> thiscall, ret 8; os dois em registradores com lixo nos bits altos,
/// a Rust só lê as palavras).
unsafe fn round_draw_date(copa: usize, reg: usize, d: *mut [u8; 8]) {
    let nova: unsafe extern "thiscall" fn(*mut [u8; 8], u32, u32) -> usize = core::mem::transmute(0x549ef0usize);
    let ano = (cqa3_b(reg + 2) as i8 as i16).wrapping_add(cqa3_w(copa + 0x40));
    nova(d, cqa3_w(reg) as u16 as u32, ano as u16 as u32);
}

/// Grava a data `d` como a do sorteio da rodada: a palavra do dia e o byte do ano menos o byte baixo de +0x40.
unsafe fn set_round_draw_date(copa: usize, reg: usize, d: &[u8; 8]) {
    core::ptr::write_unaligned(reg as *mut [u8; 2], [d[0], d[1]]);
    *((reg + 2) as *mut u8) = d[2].wrapping_sub(cqa3_b(copa + 0x40));
}

/// A data `a` vem depois da `b` (o ano sem sinal, depois o dia com sinal), como a comparação da 0x51c410.
fn cup_date_is_after(a: &[u8; 8], b: &[u8; 8]) -> bool {
    let (aa, ab) = (u16::from_le_bytes([a[2], a[3]]), u16::from_le_bytes([b[2], b[3]]));
    aa > ab || (aa == ab && i16::from_le_bytes([a[0], a[1]]) > i16::from_le_bytes([b[0], b[1]]))
}

/// A competição não leva a notícia do sorteio: menos de 2 confrontos, regra 0x200 (+0xd), competição especial das
/// seleções (0x76d940 em [0xb74318]: push comp; ecx = as seleções; sem add esp -> ret 4; a cópia Rust é stdcall(comp))
/// ou amistosa (0x5c6710 em [0xae2c74]: push comp; ecx = o Friendly -> thiscall, ret 4).
unsafe fn skips_draw_news(copa: usize, r: u32) -> bool {
    let reg = round_record_by_dword(copa, r);
    if cqa3_w(reg + 0x1a) < 2 || cqa3_w(reg + 0xd) & 0x200 != 0 {
        return true;
    }
    if cqa3_d(0xb74318) != 0 {
        let especial: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x76d940usize);
        if especial(cqa3_d(copa + 4)) != 0 {
            return true;
        }
    }
    if cqa3_d(0xae2c74) != 0 {
        let amistosa: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5c6710usize);
        if amistosa(cqa3_d(0xae2c74), cqa3_d(copa + 4)) != 0 {
            return true;
        }
    }
    false
}

/// { 0x51c410 } (thiscall, ret 4, Cup.cpp, quadro SEH sem destrutor no caminho normal): `Cup::draw_round(rodada)`;
/// devolve 1. Sem +0x45 ou com a rodada já sorteada (+0x34 do primeiro confronto): nada. Com um time de algum confronto
/// ainda sem clube: a data do sorteio anda 1 dia (0x54c5b0, 0x417c10) e as das rodadas seguintes passam a ser, no
/// mínimo, essa. Senão sorteia: regra 0x400 (+0xd) pela semente (0x51d2d0), regra 4 (+0xb) separando as nações
/// (0x51d9f0), sem 0x200 o sorteio livre (0x51e220); os mandantes (0x51e870 com −1); estádio compartilhado (+0x21 = 2,
/// 0x522610); marca os confrontos sorteados (+0x34 = 1). Com notícia (skips_draw_news): rodada que não é a do
/// terceiro lugar (+7 ≠ 0x8c) soma 1 em +0xac; a notícia 0x27 (a competição, +0x3a, o tipo, +0x44, a rodada, 0, e 1
/// se +0xab + 1 < +0xac) vai a cada um dos times ainda vivos (desfecho 0) entre os primeiros (soma das entradas +0x1c
/// das rodadas até esta) que é de técnico humano, com o id dele no dado 3.
unsafe extern "thiscall" fn draw_round(copa: usize, rodada: u32) -> u32 {
    if cqa3_d(copa + 0x45) == 0 || cqa3_b(round_ties_array(copa, rodada) + 0x34) != 0 {
        return 1;
    }
    let reg = round_record_by_dword(copa, rodada);
    if round_has_open_team(copa, rodada) {
        let mais: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], u32) -> *mut [u8; 8] = core::mem::transmute(0x54c5b0usize);
        let copia: unsafe extern "thiscall" fn(*mut [u8; 8], *mut [u8; 8], *mut [u8; 8]) -> usize = core::mem::transmute(0x417c10usize);
        let (mut a, mut b, mut c) = ([0u8; 8], [0u8; 8], [0u8; 8]);
        round_draw_date(copa, reg, &mut a);
        crate::comparison::branch("cqa_3 sorteio adiado");
        let r = mais(&mut a, &mut b, 1); // cqa3: o sorteio adiado
        copia(&mut a, &mut c, r); // push &b; push &c; ecx = &a -> thiscall, ret 8
        set_round_draw_date(copa, reg, &a);
        let mut k = rodada.wrapping_add(1);
        while (k as i16) < cqa3_w(copa + 0x3c) {
            let rk = round_record_by_dword(copa, k as i16 as i32 as u32);
            round_draw_date(copa, rk, &mut b);
            if cup_date_is_after(&a, &b) {
                set_round_draw_date(copa, rk, &a);
            }
            k = k.wrapping_add(1);
        }
        return 1;
    }
    // os sorteios: push rodada; ecx = a copa; sem add esp -> thiscall, ret 4 (as três)
    let sorteio = |f: usize| {
        let g: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(f);
        g(copa, rodada)
    };
    let regra = cqa3_w(reg + 0xd);
    if regra & 0x400 != 0 {
        crate::comparison::branch("cqa_3 sorteio pela semente");
        sorteio(0x51d2d0); // cqa3: pela semente
    } else if cqa3_b(reg + 0xb) & 4 != 0 {
        sorteio(0x51d9f0);
    } else if regra & 0x200 == 0 {
        crate::comparison::branch("cqa_3 sorteio livre");
        sorteio(0x51e220); // cqa3: o sorteio livre
    }
    // push −1; push rodada; ecx = a copa -> thiscall, ret 8
    let mandantes: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x51e870usize);
    mandantes(copa, rodada, u32::MAX);
    if cqa3_b(round_record_by_dword(copa, rodada) + 0x21) == 2 {
        sorteio(0x522610);
    }
    let mut k = 0i32;
    while k < cqa3_w(round_record_by_dword(copa, rodada) + 0x1a) as i32 {
        *((round_ties_array(copa, rodada) + k as usize * 0x35 + 0x34) as *mut u8) = 1;
        k += 1;
    }
    if skips_draw_news(copa, rodada) {
        return 1;
    }
    let reg = round_record_by_dword(copa, rodada);
    if cqa3_w(reg + 7) != 0x8c {
        *((copa + 0xac) as *mut u8) = cqa3_b(copa + 0xac).wrapping_add(1);
    }
    let mut soma: i16 = 0;
    if (rodada as i32) >= 0 {
        for i in 0..=rodada {
            soma = soma.wrapping_add(cqa3_w(round_record_by_dword(copa, i) + 0x1c));
        }
    }
    let mut local = [0u32; 0x38];
    let n = local.as_mut_ptr() as usize;
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let humano: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x7875e0usize);
    construir(n, 0x27, 0); // push 0; push 0x27; ecx = a local -> thiscall, ret 8
    gravar(n, 0, cqa3_d(cqa3_d(copa + 4)) as u32); // push valor; push k; ecx = a local -> thiscall, ret 8
    gravar(n, 1, cqa3_w(copa + 0x3a) as i32 as u32);
    gravar(n, 2, cqa3_w(round_record_by_dword(copa, rodada) + 7) as i32 as u32);
    gravar(n, 4, cqa3_b(copa + 0x44) as i8 as i32 as u32);
    gravar(n, 5, rodada);
    gravar(n, 6, 0);
    gravar(n, 7, ((cqa3_b(copa + 0xab) as i8 as i32 + 1) < cqa3_b(copa + 0xac) as i8 as i32) as u32);
    for i in 0..soma.max(0) as usize {
        let e = cqa3_d(copa + 0xa7) + 6 * i;
        if cqa3_b(e + 5) != 0 || cqa3_d(e) == 0 {
            continue;
        }
        // 0x7875e0: push clube; ecx = [0xb7431c] -> ret 4 (a cópia Rust é stdcall(clube))
        if humano(cqa3_d(e)) == 0 {
            continue;
        }
        gravar(n, 3, cqa3_d(cqa3_d(cqa3_d(copa + 0xa7) + 6 * i)) as u32);
        // push clube; push &local; ecx = [0xb7431c]; sem add esp -> thiscall, ret 8
        entregar(cqa3_d(0xb7431c), n, cqa3_d(cqa3_d(copa + 0xa7) + 6 * i));
    }
    1
}

/// Comparador da 0x51c410: só a original (os sorteios sorteiam, a notícia aloca). O plano, feito antes: sem sorteio
/// (sem +0x45 ou já sorteada), nada muda nos confrontos nem nas datas; com time em aberto, as datas previstas das
/// rodadas (a desta mais 1 dia, as seguintes no mínimo essa); sorteada, todos os confrontos com +0x34 = 1 e +0xac com
/// a soma prevista (as consultas da notícia não mudam com o sorteio).
unsafe extern "thiscall" fn compare_draw_round(copa: usize, rodada: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> u32>(%%I0%%);
    if cqa3_d(copa + 0x45) == 0 || cqa3_b(round_ties_array(copa, rodada) + 0x34) != 0 {
        return o(copa, rodada); // ponytail: nada a sortear, a original não faz nada
    }
    let n = cqa3_w(copa + 0x3c);
    if round_has_open_team(copa, rodada) {
        // o plano das datas: (registro, palavra do dia, byte do ano) de cada rodada daqui em diante
        let mut a = [0u8; 8];
        let (mut dia, mut ano) = (cqa3_w(round_record_by_dword(copa, rodada)), (cqa3_b(round_record_by_dword(copa, rodada) + 2) as i8 as i16).wrapping_add(cqa3_w(copa + 0x40)) as u16);
        // a 0x54c5b0 (mais 1): normalizar para a frente, 365 dias no bissexto (crate::data::bissexto)
        dia = dia.wrapping_add(1);
        let tam = |y: u16| if crate::date::bissexto(y) == 1 { 365i16 } else { 364 };
        while dia > tam(ano) {
            dia = dia.wrapping_sub(tam(ano) + 1);
            ano = ano.wrapping_add(1);
        }
        a[0..2].copy_from_slice(&dia.to_le_bytes());
        a[2..4].copy_from_slice(&ano.to_le_bytes());
        let mut plano = vec![(round_record_by_dword(copa, rodada), dia, a[2].wrapping_sub(cqa3_b(copa + 0x40)))];
        let mut k = rodada.wrapping_add(1);
        while (k as i16) < n {
            let rk = round_record_by_dword(copa, k as i16 as i32 as u32);
            let mut b = [0u8; 8];
            let anob = (cqa3_b(rk + 2) as i8 as i16).wrapping_add(cqa3_w(copa + 0x40)) as u16;
            b[0..2].copy_from_slice(&cqa3_w(rk).to_le_bytes());
            b[2..4].copy_from_slice(&anob.to_le_bytes());
            plano.push(if cup_date_is_after(&a, &b) { (rk, dia, a[2].wrapping_sub(cqa3_b(copa + 0x40))) } else { (rk, cqa3_w(rk), cqa3_b(rk + 2)) });
            k = k.wrapping_add(1);
        }
        let r = o(copa, rodada);
        let ok = plano.iter().all(|&(rk, d, y)| cqa3_w(rk) == d && cqa3_b(rk + 2) == y);
        SCOREBOARD.record(ok, || format!("draw_round(rodada {}) as datas adiadas diferem do plano", rodada as i32));
        return r;
    }
    let antes_ac = cqa3_b(copa + 0xac);
    let r = o(copa, rodada);
    let mut ok = r == 1;
    let reg = round_record_by_dword(copa, rodada);
    for k in 0..cqa3_w(reg + 0x1a).max(0) as usize {
        ok &= cqa3_b(round_ties_array(copa, rodada) + k * 0x35 + 0x34) == 1;
    }
    let soma = if !skips_draw_news(copa, rodada) && cqa3_w(reg + 7) != 0x8c { 1 } else { 0 };
    ok &= cqa3_b(copa + 0xac) == antes_ac.wrapping_add(soma);
    SCOREBOARD.record(ok, || format!("draw_round(rodada {}) marcas dos confrontos ou +0xac ({} -> {}) diferem do plano", rodada as i32, antes_ac, cqa3_b(copa + 0xac)));
    r
}

/// { 0x4bffc0 } (cdecl, comp_util.cpp): `comp_util_format_stage_round_name(fase, rodada, saida, tamanho, estilo)`:
/// sem saída ou com menos de 200 bytes, a caixa (linha 0x26a1) e 0. Zera a saída; com fase e rodada, "{fase} {rodada}"
/// (0x66f4e0 com o modelo 0x9a7cfc, os nomes de 100 bytes da 0x4b4850 e os gêneros da 0x4ba8f0); só uma delas, o nome
/// dela direto na saída (0x4b4850 com o tamanho); nenhuma, só o zero. Devolve 1 (0 na caixa).
unsafe extern "C" fn comp_util_format_stage_round_name(fase: u32, rodada: u32, saida: *mut u8, tamanho: i32, estilo: u32) -> i32 {
    if saida.is_null() || tamanho < 0xc8 {
        return original_fn::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(%%I1%%)(fase, rodada, saida, tamanho, estilo); // a caixa
    }
    // 0x4b4850: 4 pushes (estilo, id, tamanho, saída); add esp -> cdecl; 0x4ba8f0: push estilo; push id -> cdecl
    let nome: unsafe extern "C" fn(*mut u8, i32, u32, u32) -> usize = core::mem::transmute(0x4b4850usize);
    let genero: unsafe extern "C" fn(u32, u32) -> u8 = core::mem::transmute(0x4ba8f0usize);
    *saida = 0;
    if fase != 0 && rodada != 0 {
        let (mut a, mut b) = ([0u8; 0x64], [0u8; 0x64]);
        nome(a.as_mut_ptr(), 0x64, fase, estilo);
        let ga = genero(fase, estilo) as i8 as i32 as u32;
        nome(b.as_mut_ptr(), 0x64, rodada, estilo);
        let gb = genero(rodada, estilo) as i8 as i32 as u32;
        // 0x66f4e0: 8 pushes; add esp, 0x20 -> cdecl (saída, modelo, ga, ga, gb, gb, &a, &b)
        let texto: unsafe extern "C" fn(*mut u8, usize, u32, u32, u32, u32, *const u8, *const u8) -> usize = core::mem::transmute(0x66f4e0usize);
        texto(saida, 0x9a7cfc, ga, ga, gb, gb, a.as_ptr(), b.as_ptr());
    } else if fase != 0 {
        nome(saida, tamanho, fase, estilo);
    } else if rodada != 0 {
        nome(saida, tamanho, rodada, estilo);
    } else {
        return 0; // nenhuma das duas: 0 (0x4c0136)
    }
    1
}

/// Comparador da 0x4bffc0: lado a lado na saída (até o tamanho, no máximo 0x400 bytes) e na devolução.
unsafe extern "C" fn compare_comp_util_format_stage_round_name(fase: u32, rodada: u32, saida: *mut u8, tamanho: i32, estilo: u32) -> i32 {
    let o = original_fn::<unsafe extern "C" fn(u32, u32, *mut u8, i32, u32) -> i32>(%%I1%%);
    if saida.is_null() || tamanho < 0xc8 {
        return o(fase, rodada, saida, tamanho, estilo); // a caixa: só a original
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&[(saida as usize, (tamanho as usize).min(0x400))],
        || o(fase, rodada, saida, tamanho, estilo), || comp_util_format_stage_round_name(fase, rodada, saida, tamanho, estilo));
    SCOREBOARD.record(igual && r == n, || format!("comp_util_format_stage_round_name({}, {}) {} x {} {:x?} x {:x?}", fase as i16, rodada as i16, r, n, mo, mr));
    r
}

/// O nível da divisão do clube: a palavra +0x69 da divisão (+0x57), ou 0 sem divisão.
unsafe fn division_level(clube: usize) -> i16 {
    let d = cqa3_d(clube + 0x57);
    if d != 0 {
        cqa3_w(d + 0x69)
    } else {
        0
    }
}

/// { 0x522360 } (thiscall, ret 8, Cup.cpp; o this não é lido; a virtual +0x90 da copa base):
/// `Cup::club_in_higher_division(a, b)`: o clube de nível menor (+0x57 → +0x69, com sinal; 0 sem divisão); empate, 0.
unsafe extern "thiscall" fn club_in_higher_division(_this: usize, a: usize, b: usize) -> usize {
    let (na, nb) = (division_level(a), division_level(b));
    if na < nb {
        a
    } else if na == nb {
        0
    } else {
        b
    }
}

/// Comparador da 0x522360: pelo valor (só lê).
unsafe extern "thiscall" fn compare_club_in_higher_division(this: usize, a: usize, b: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, usize, usize) -> usize>(%%I2%%)(this, a, b);
    let n = club_in_higher_division(this, a, b);
    SCOREBOARD.record(r == n, || format!("club_in_higher_division({:x}, {:x}) original={:x} rust={:x}", a, b, r, n));
    r
}
