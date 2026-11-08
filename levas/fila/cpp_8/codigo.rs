// ---------- %%LEVA%%a leva: as doenças e a pré-temporada: acrescentar o surto de doença no clube, espalhar os surtos pelos elencos e o jogador fora da temporada (injury.cpp) ----------

unsafe fn cpp8_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpp8_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpp8_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpp8_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cpp8_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O clube pelo índice (registro de 0x245 bytes em [0xae23b4]).
unsafe fn injury_club_by_index(i: i32) -> usize {
    cpp8_d(0xae23b4).wrapping_add((i as usize).wrapping_mul(0x245))
}

unsafe fn injury_is_national_team(clube: usize) -> bool {
    let is_national_team: unsafe extern "C" fn(usize) -> u32 = core::mem::transmute(0x53b900usize);
    is_national_team(clube) != 0
}

/// Um surto (17 bytes, vetor [this+0x1c], capacidade +0x20, conta +0x24): +0 a data (8 bytes), +8 o tipo da doença,
/// +9 o índice do clube, +0xd o id da partida (−1 sem).
const OUTBREAK_SIZE: usize = 0x11;

/// Se o surto entra (os sorteios da 0x62e820, na ordem): tipo 4 nunca; tipo 3 só com 1 em 15 (sorteio de 15 = 0); os
/// outros com 3 em 4 (sorteio de 4 ≠ 0); e nunca numa seleção (0x53b900).
unsafe fn outbreak_takes_hold(tipo: u8, clube_i: i32) -> bool {
    if tipo == 4 {
        return false;
    }
    if tipo == 3 {
        if cpp8_sorteio(0xf) != 0 {
            return false;
        }
    } else if cpp8_sorteio(4) == 0 { // 1 em 4 fica de fora
        return false;
    }
    !injury_is_national_team(injury_club_by_index(clube_i))
}

/// { 0x62e820 } (thiscall, ret 0x10, injury.cpp): `Injury::add_illness_outbreak(data, tipo, clube, partida)`: o tipo
/// é o byte baixo; se o surto entra (`outbreak_takes_hold`), cresce o vetor de 30 em 30 quando cheio (0x945667, ou
/// 0x944e46 sem vetor; sem memória, a caixa "Error" da linha 0xd01 e nada), grava o surto no fim (data pela 0x417c10)
/// e reordena pela data (qsort 0x945333 com 0x778e90).
unsafe extern "thiscall" fn add_illness_outbreak(this: usize, data: usize, tipo: u32, clube_i: i32, partida: usize) {
    let tipo = tipo as u8;
    if !outbreak_takes_hold(tipo, clube_i) {
        return;
    }
    crate::comparison::branch("cpp_8 surto novo");
    let cap = cpp8_i(this + 0x20);
    if cpp8_i(this + 0x24) >= cap {
        let cap = cap.wrapping_add(30);
        core::ptr::write_unaligned((this + 0x20) as *mut i32, cap);
        let tamanho = (cap as u32).wrapping_mul(OUTBREAK_SIZE as u32);
        let v = cpp8_d(this + 0x1c);
        let novo = if v != 0 {
            let realocar: unsafe extern "C" fn(usize, u32) -> usize = core::mem::transmute(0x945667usize);
            realocar(v, tamanho)
        } else {
            let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
            alocar(tamanho)
        };
        core::ptr::write_unaligned((this + 0x1c) as *mut usize, novo);
        if novo == 0 {
            show_error_at(0x9e7f28, 0xd01);
            return;
        }
    }
    let e = cpp8_d(this + 0x1c).wrapping_add((cpp8_i(this + 0x24) as usize).wrapping_mul(OUTBREAK_SIZE));
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut rascunho = [0u8; 8];
    copiar(e, rascunho.as_mut_ptr(), data);
    *((e + 8) as *mut u8) = tipo;
    core::ptr::write_unaligned((e + 9) as *mut i32, clube_i);
    let id = if partida != 0 { cpp8_i(partida) } else { -1 };
    core::ptr::write_unaligned((e + 0xd) as *mut i32, id);
    let n = cpp8_i(this + 0x24).wrapping_add(1);
    core::ptr::write_unaligned((this + 0x24) as *mut i32, n);
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(cpp8_d(this + 0x1c), n as u32, OUTBREAK_SIZE as u32, 0x778e90);
}

/// A data de cinco dias atrás (0x54c8e0 sobre a data do jogo).
unsafe fn five_days_ago() -> [u8; 8] {
    let datar: unsafe extern "thiscall" fn(usize, *mut [u8; 8], u32) = core::mem::transmute(0x54c8e0usize);
    let mut d = [0u8; 8];
    datar(0xae2c90, &mut d, 5); // cinco dias atrás
    d
}

/// A data `limite` (dia com sinal em +0, ano sem sinal em +2) é depois da do surto `e`?
unsafe fn is_after_outbreak(limit: &[u8; 8], e: usize) -> bool {
    let dia = i16::from_le_bytes([limit[0], limit[1]]);
    let ano = u16::from_le_bytes([limit[2], limit[3]]);
    let ano_e = cpp8_w(e + 2) as u16;
    ano > ano_e || (ano == ano_e && dia > cpp8_w(e))
}

/// { 0x62e9e0 } (thiscall, ret 0, injury.cpp, quadro SEH sem destrutor): `Injury::spread_illness_outbreaks()`: percorre
/// os surtos (ordenados pela data) até o primeiro depois de hoje. O de mais de cinco dias (0x54c8e0) conta para sair;
/// os outros espalham: tipo 3 só no próprio dia, com 1 a 4 doentes (sorteio de 4, +1); os outros 0 a 2 (sorteio de 3).
/// Fora das seleções (0x53b900), até 20 tentativas: uma das 50 vagas do elenco (sorteio de 50, +0xd7), jogador (id
/// abaixo de [0xae2364]) sem essa doença (+0x1d do registro de 31 bytes em [this]) fica doente (0x62d250: o tipo no
/// byte baixo da dword do deslocamento do surto, como a original; a partida do surto). No fim, os que saem vão
/// embora do começo do vetor (os outros descem) e a conta cai.
unsafe extern "thiscall" fn spread_illness_outbreaks(this: usize) {
    let cinco = five_days_ago();
    let injure: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, i32, u32, u32) -> u32 = core::mem::transmute(0x62d250usize);
    let mut removidos: i32 = 0;
    let mut k: i32 = 0;
    let mut off: usize = 0;
    while k < cpp8_i(this + 0x24) {
        let e = cpp8_d(this + 0x1c).wrapping_add(off);
        let hoje4 = cpp8_d(0xae2c90) as u32;
        let (ano_e, ano_h) = (cpp8_w(e + 2) as u16, (hoje4 >> 16) as u16);
        if ano_h < ano_e || (ano_h == ano_e && (hoje4 as u16 as i16) < cpp8_w(e)) {
            break; // o primeiro surto depois de hoje
        }
        if is_after_outbreak(&cinco, e) {
            crate::comparison::branch("cpp_8 surto antigo");
            removidos += 1;
        } else {
            let tipo = cpp8_b(e + 8);
            let quantos = if tipo == 3 {
                if cpp8_d(0xae2c90) as u32 == cpp8_d(e) as u32 { Some((cpp8_sorteio(4) as u8).wrapping_add(1)) } else { None }
            } else {
                Some(cpp8_sorteio(3) as u8) // 0 a 2 doentes
            };
            if let Some(mut quantos) = quantos {
                crate::comparison::branch("cpp_8 espalhar");
                let clube = injury_club_by_index(cpp8_i(cpp8_d(this + 0x1c).wrapping_add(off) + 9));
                if !injury_is_national_team(clube) {
                    let mut t = 0;
                    while t < 0x14 && (quantos as i8) > 0 {
                        let vaga = cpp8_sorteio(0x32) as i8 as isize;
                        let p = cpp8_d((clube + 0xd7).wrapping_add((vaga * 4) as usize));
                        if p != 0 && cpp8_i(p) < cpp8_i(0xae2364) {
                            let e = cpp8_d(this + 0x1c).wrapping_add(off);
                            let doenca = cpp8_b(e + 8);
                            let registro = cpp8_d(this).wrapping_add((cpp8_i(p) as usize).wrapping_mul(31));
                            if cpp8_b(registro + 0x1d) != doenca {
                                injure(this, (off as u32 & 0xffff_ff00) | doenca as u32, 0, p, 0, 0, cpp8_i(e + 0xd), 0, 0);
                                quantos = quantos.wrapping_sub(1);
                            }
                        }
                        t += 1;
                    }
                }
            }
        }
        k += 1;
        off += OUTBREAK_SIZE;
    }
    let n = cpp8_i(this + 0x24);
    if removidos != 0 && removidos != n {
        let base = cpp8_d(this + 0x1c);
        core::ptr::copy((base + removidos as usize * OUTBREAK_SIZE) as *const u8, base as *mut u8, (n - removidos) as usize * OUTBREAK_SIZE);
    }
    core::ptr::write_unaligned((this + 0x24) as *mut i32, n - removidos);
}

/// { 0x62ce30 } (stdcall, ret 4, injury.cpp, quadro SEH sem destrutor): `injury_player_in_close_season(pessoa)`: sem
/// pessoa, 1. Com seleção (+0x24), a data dela (0x76e040 sobre [0xb74318], a nação +0x53 da seleção, a classe da
/// 0x76d780 pelo bit 3 do +0x5e) depois de hoje conta como convocado. Sem clube (+0x39), convocado ou clube com +0x1db
/// ≠ −1: 0. Senão, fora da temporada da nação do clube (0x669bd0 com as duas datas) e faltando menos de 40 dias
/// (0x54cdb0 de hoje até a segunda data, 16 bits sem sinal): 1; senão 0.
unsafe extern "stdcall" fn injury_player_in_close_season(pessoa: usize) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut u32) -> usize = core::mem::transmute(0x54a110usize);
    let mut inicio = [0u32; 2];
    let mut fim = [0u32; 2];
    construir(inicio.as_mut_ptr());
    construir(fim.as_mut_ptr());
    if pessoa == 0 {
        return 1;
    }
    let mut convocado = false;
    let selecao = cpp8_d(pessoa + 0x24);
    if selecao != 0 {
        let classe: unsafe extern "stdcall" fn(usize, u32) -> u8 = core::mem::transmute(0x76d780usize);
        let campo: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x76e040usize);
        let k = classe(selecao, (cpp8_b(pessoa + 0x5e) & 8) as u32);
        // ponytail: a original empurra a dword do argumento com só o byte baixo trocado pelo al; a 0x76e040 lê só o byte
        let q = campo(cpp8_d(0xb74318), cpp8_d(selecao + 0x53), k as u32);
        let lo = cpp8_d(q) as u32;
        let (ano, ano_h) = ((lo >> 16) as u16, cpp8_w(0xae2c92) as u16);
        convocado = ano > ano_h || (ano == ano_h && (lo as u16 as i16) > cpp8_w(0xae2c90));
    }
    let clube = cpp8_d(pessoa + 0x39);
    if clube == 0 || convocado || cpp8_i(clube + 0x1db) != -1 {
        return 0;
    }
    let temporada: unsafe extern "C" fn(usize, *mut u32, *mut u32) -> u32 = core::mem::transmute(0x669bd0usize);
    if temporada(clube, inicio.as_mut_ptr(), fim.as_mut_ptr()) != 0 {
        return 0;
    }
    let dias: unsafe extern "thiscall" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x54cdb0usize);
    if (dias(0xae2c90, fim[0], fim[1]) as u16) < 0x28 {
        return 1;
    }
    0
}

/// Comparador: a original sorteia, aloca e reordena: o plano antes (os sorteios e a seleção, com o estado do sorteio
/// guardado e devolvido); a original roda. Confere o mesmo estado do sorteio e, se o surto entrou, a conta +1 e o surto
/// (data, tipo, clube, partida) no vetor; senão a conta igual.
unsafe extern "thiscall" fn comparar_add_illness_outbreak(this: usize, data: usize, tipo: u32, clube_i: i32, partida: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, i32, usize)>(%%I0%%);
    let estado = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    let entra = crate::rng::pause_trace(|| crate::rng::without_counting(|| outbreak_takes_hold(tipo as u8, clube_i)));
    let depois_do_plano = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS);
    estado.write_back();
    let n0 = cpp8_i(this + 0x24);
    let mut esperado = [0u8; OUTBREAK_SIZE];
    esperado[..8].copy_from_slice(&*(data as *const [u8; 8]));
    esperado[8] = tipo as u8;
    esperado[9..13].copy_from_slice(&clube_i.to_le_bytes());
    let id = if partida != 0 { cpp8_i(partida) } else { -1 };
    esperado[13..17].copy_from_slice(&id.to_le_bytes());
    o(this, data, tipo, clube_i, partida);
    let n = cpp8_i(this + 0x24);
    let mesmo_sorteio = crate::comparison::MemorySnapshot::read(&crate::rng::STATE_REGIONS) == depois_do_plano;
    let v = cpp8_d(this + 0x1c);
    let ok = if entra && v != 0 {
        n == n0 + 1 && (0..n.max(0) as usize).any(|k| *((v + k * OUTBREAK_SIZE) as *const [u8; OUTBREAK_SIZE]) == esperado)
    } else {
        n == n0
    };
    PLACAR.record(ok && mesmo_sorteio, || {
        format!("add_illness_outbreak(tipo {}, clube {}) plano entra={}, conta {} -> {}, mesmo sorteio={}", tipo as u8, clube_i, entra, n0, n, mesmo_sorteio)
    });
}

/// Comparador: a original sorteia e adoece (0x62d250, que pode acrescentar surtos): roda só ela e confere o plano
/// feito antes: quantos surtos tinham mais de cinco dias (os que saem) e, depois, nenhum surto com mais de cinco dias e
/// a conta pelo menos a de antes menos os que saem.
unsafe extern "thiscall" fn comparar_spread_illness_outbreaks(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let cinco = five_days_ago();
    let n0 = cpp8_i(this + 0x24);
    let hoje4 = cpp8_d(0xae2c90) as u32;
    let mut saem = 0;
    for k in 0..n0.max(0) as usize {
        let e = cpp8_d(this + 0x1c) + k * OUTBREAK_SIZE;
        let (ano_e, ano_h) = (cpp8_w(e + 2) as u16, (hoje4 >> 16) as u16);
        if ano_h < ano_e || (ano_h == ano_e && (hoje4 as u16 as i16) < cpp8_w(e)) {
            break;
        }
        if is_after_outbreak(&cinco, e) {
            saem += 1;
        }
    }
    o(this);
    let n = cpp8_i(this + 0x24);
    let velho = (0..n.max(0) as usize).find(|&k| is_after_outbreak(&cinco, cpp8_d(this + 0x1c) + k * OUTBREAK_SIZE));
    PLACAR.record(velho.is_none() && n >= n0 - saem, || {
        format!("spread_illness_outbreaks: conta {} -> {} (saem {}), surto velho {:?}", n0, n, saem, velho)
    });
}

/// Comparador: pelo valor (datas, a seleção e a temporada só calculam).
unsafe extern "stdcall" fn comparar_injury_player_in_close_season(pessoa: usize) -> u32 {
    let r = original_fn::<unsafe extern "stdcall" fn(usize) -> u32>(%%I2%%)(pessoa);
    let n = injury_player_in_close_season(pessoa);
    PLACAR.record(r == n, || format!("injury_player_in_close_season({:x}) original={} rust={}", pessoa, r, n));
    r
}
