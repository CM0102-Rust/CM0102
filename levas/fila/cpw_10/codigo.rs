// ---------- %%LEVA%%a leva: a fila dos regens: os regens jovens que vencem e os técnicos de reserva refeitos; o veículo da notícia pelo clube e pela pessoa (player_regen.cpp, media.cpp) ----------

unsafe fn cpw10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw10_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpw10_por16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

unsafe fn cpw10_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// A data da entrada `e` da fila dos regens (+4) é antes de hoje (0x456770)?
unsafe fn cpw10_vencida(e: usize) -> bool {
    let anterior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    anterior(e + 4, cpw10_d(0xae2c90) as u32, cpw10_d(0xae2c94) as u32) != 0 // cpw10: a entrada vencida
}

/// Uma nota do técnico refeito (um dos blocos da 0x7a65d0): P = pow(+0x22 da pessoa, 0x968950) (vezes `fator`, quando
/// há), o maior entre P e o valor `v` da ficha, vezes `k`, no máximo `teto` (f64 em 0x...), truncado.
unsafe fn cpw10_nota(pessoa: usize, v: i16, fator: Option<usize>, k: usize, teto: usize) -> u16 {
    let mut p = pow_do_jogo(*((pessoa + 0x22) as *const u8) as f64, f64_em(0x968950));
    if let Some(f) = fator {
        p *= f64_em(f);
    }
    let v = v as f64;
    let m = if p > v { p } else { v };
    let mk = m * f64_em(k);
    let t = f64_em(teto);
    ftol(if !(mk >= t) { mk } else { t }) as u16
}

/// O cargo do técnico refeito no clube da máquina (0x7a6757..0x7a68d8): reputação do jogador (+0xb da ficha) abaixo da
/// do clube (+0x80) vezes 0x968968 e menos de 5 treinadores (0x53c8d0): as vagas de treinador vazias (+0x19f, 5) ficam
/// com ele (cargo 0xf); sem vaga, auxiliar (+0xd3, cargo 0xd) se o clube não tiver (0x53cb80). Senão, abaixo da do clube
/// vezes 0x969040, auxiliar do mesmo jeito.
unsafe fn cpw10_cargo_no_clube(s: usize) {
    let clube = cpw10_d(s + 0x39);
    if clube == 0 {
        return;
    }
    let humano: unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x600a50usize);
    if humano(clube, 1, 1, 0, 0) != 0 {
        return;
    }
    let treinadores: unsafe extern "C" fn(usize) -> u8 = core::mem::transmute(0x53c8d0usize);
    let auxiliar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x53cb80usize);
    let rep = |k: usize| {
        let f = cpw10_w(cpw10_d(s + 0x61) + 0xb) as f64;
        let c = cpw10_w(cpw10_d(s + 0x39) + 0x80) as f64 * f64_em(k);
        c > f
    };
    if rep(0x968968) && (treinadores(cpw10_d(s + 0x39)) as i8) < 5 {
        for k in 0..5usize {
            let v = cpw10_d(s + 0x39) + 0x19f + 4 * k;
            if cpw10_d(v) == 0 {
                core::ptr::write_unaligned(v as *mut usize, s);
                *((s + 0x3d) as *mut u8) = 0xf;
            }
        }
        if *((s + 0x3d) as *const u8) == 0xf || auxiliar(cpw10_d(s + 0x39)) != 0 {
            return;
        }
        core::ptr::write_unaligned((cpw10_d(s + 0x39) + 0xd3) as *mut usize, s);
        *((s + 0x3d) as *mut u8) = 0xd;
        return;
    }
    if !rep(0x969040) || auxiliar(cpw10_d(s + 0x39)) != 0 {
        return;
    }
    core::ptr::write_unaligned((cpw10_d(s + 0x39) + 0xd3) as *mut usize, s);
    *((s + 0x3d) as *mut u8) = 0xd;
}

/// { 0x7a65d0 } (thiscall, ret 0, player_regen.cpp): `PLAYER_REGEN::process_regen_queue()`, pelo laço do dia (0x5cccd0),
/// todo dia. A fila dos regens jovens ([+0x10], 12 bytes: +0 id, +4 data; conta +0x14): da frente, enquanto a data da
/// primeira for antes de hoje (0x456770) e +0x40 não for zero, o regen jovem (0x7abac0) da pessoa (id fora de 0..[0xae2364]:
/// caixa "Invalid staff_id for regen" e fim), as seguintes sobem uma casa (0x7a8b70) e a última fica com id −1. Depois, com
/// mais de 2 reservas de técnico ([+0x1c], conta +0x20), para conta − 2 delas: a pessoa que empresta os dados (0x7a6e60
/// pela nação +4 da primeira) ganha o registro de não jogador da reserva (+0x69, [0xae23c0] + 0x44 × id); ex-jogador: a
/// troca de +0x10/+0x11 da nota 20, a normalização (0x539490), o cargo no clube da máquina e as três notas (+0xa, +8, +0xc)
/// pela reputação, +9 e +0xd da ficha (faixa acima de 7500, ou sorteio(+0x57) e as outras duas); não jogador: sorteio(1500)
/// e sorteio(500). A reserva sai (0x7a82b0).
unsafe extern "thiscall" fn fila_de_regens(this: usize) {
    let regen: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x7abac0usize);
    let copiar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7a8b70usize);
    let c0 = cpw10_i(this + 0x14);
    let primeira = cpw10_d(this + 0x10);
    let mut k = 0i32;
    while k < c0 {
        if !cpw10_vencida(primeira) || cpw10_i(this + 0x40) == 0 {
            break;
        }
        let id = cpw10_i(primeira);
        if id < 0 || id > cpw10_i(0xae2364) {
            let formatar: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x944c9fusize);
            formatar(0xde1f64, 0xa6bfcc, id);
            aviso_em(0xa6bf9c, 0x36e);
            return;
        }
        crate::comparacao::ramo("cpw_10 regen da fila");
        regen(this, cpw10_d(0xae23bc).wrapping_add((id as u32).wrapping_mul(0x6e) as usize));
        let mut j = 0i32;
        while j < cpw10_i(this + 0x14) - 1 {
            let e = cpw10_d(this + 0x10).wrapping_add((j as u32).wrapping_mul(0xc) as usize);
            copiar(e, e + 0xc);
            j += 1;
        }
        let n = cpw10_i(this + 0x14);
        let ultima = cpw10_d(this + 0x10).wrapping_add((n as u32).wrapping_mul(0xc) as usize).wrapping_sub(0xc);
        core::ptr::write_unaligned(ultima as *mut i32, -1);
        cpw10_por16(ultima + 4, 0);
        cpw10_por16(ultima + 6, 0);
        core::ptr::write_unaligned((this + 0x14) as *mut i32, n - 1); // cpw10: a entrada sai
        k += 1;
    }
    let total = cpw10_i(this + 0x20);
    if total <= 2 {
        return;
    }
    let doador: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x7a6e60usize);
    let normalizar: unsafe extern "C" fn(usize) = core::mem::transmute(0x539490usize);
    let tirar: unsafe extern "thiscall" fn(usize, i32) = core::mem::transmute(0x7a82b0usize);
    for _ in 0..total - 2 {
        let s = doador(this, cpw10_d(cpw10_d(this + 0x1c) + 4));
        if s == 0 {
            return;
        }
        crate::comparacao::ramo("cpw_10 tecnico de reserva");
        let nj = cpw10_d(0xae23c0).wrapping_add((cpw10_d(cpw10_d(this + 0x1c)) as u32).wrapping_mul(0x44) as usize);
        core::ptr::write_unaligned((s + 0x69) as *mut usize, nj);
        let ficha = cpw10_d(s + 0x61);
        if ficha == 0 {
            let a = cpw10_sortear(0x5dc).wrapping_add(1) as u16;
            cpw10_por16(cpw10_d(s + 0x69) + 8, a);
            let b = cpw10_sortear(0x5dc).wrapping_add(1) as u16;
            cpw10_por16(cpw10_d(s + 0x69) + 0xa, b);
            let c = cpw10_sortear(0x1f4).wrapping_add(1) as u16;
            cpw10_por16(cpw10_d(s + 0x69) + 0xc, c);
        } else {
            if *((ficha + 0xf) as *const i8) == 0x14 {
                let (cl, dl) = (*((nj + 0x10) as *const i8), *((nj + 0x11) as *const i8));
                if cl > dl {
                    *((nj + 0x11) as *mut i8) = cl;
                    *((cpw10_d(s + 0x69) + 0x10) as *mut i8) = dl;
                    if cpw10_sortear(4) == 0 {
                        let r = cpw10_sortear(0x32) as u8;
                        *((cpw10_d(s + 0x69) + 0x11) as *mut u8) = r.wrapping_add(0x32);
                    }
                }
            }
            normalizar(s);
            cpw10_cargo_no_clube(s);
            let rep = cpw10_w(cpw10_d(s + 0x61) + 0xb);
            let (k, teto) = if rep > 0x1d4c {
                (0x9689d8, 0x9690b8)
            } else if cpw10_sortear(*((s + 0x57) as *const i8) as i32) != 0 {
                (0x969040, 0x96d300)
            } else {
                (0x968968, 0x96e288)
            };
            let a = cpw10_nota(s, rep, None, k, teto);
            cpw10_por16(cpw10_d(s + 0x69) + 0xa, a);
            let b = cpw10_nota(s, cpw10_w(cpw10_d(s + 0x61) + 9), None, k, teto);
            cpw10_por16(cpw10_d(s + 0x69) + 8, b);
            let c = cpw10_nota(s, cpw10_w(cpw10_d(s + 0x61) + 0xd), Some(0x969040), k, teto);
            cpw10_por16(cpw10_d(s + 0x69) + 0xc, c);
        }
        tirar(this, cpw10_i(cpw10_d(this + 0x1c)));
    }
}

/// O comparador: a 0x7abac0 (o regen jovem) aloca o contrato e manda notícia e as reservas de técnico refazem pessoas
/// achadas pelo caminho: só a original, conferindo o plano da fila. Sem entrada vencida e sem reservas a refazer, os dois
/// lado a lado no cabeçalho (nada muda); com vencidas, as k primeiras saem: as seguintes sobem k casas, as k últimas
/// ficam com id −1 e as palavras +4/+6 zeradas, a conta cai k. Com reservas (mais de 2) ou id inválido: só a original.
unsafe extern "thiscall" fn comparar_fila_de_regens(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    if cpw10_i(this + 0x20) > 2 {
        crate::comparacao::ramo("cpw_10 reservas so na original");
        return o(this);
    }
    let c0 = cpw10_i(this + 0x14);
    let base = cpw10_d(this + 0x10);
    let mut k = 0i32;
    if cpw10_i(this + 0x40) != 0 {
        while k < c0 {
            let e = base + k as usize * 0xc;
            if !cpw10_vencida(e) {
                break;
            }
            let id = cpw10_i(e);
            if id < 0 || id > cpw10_i(0xae2364) {
                return o(this); // ponytail: a caixa de erro só na original
            }
            k += 1;
        }
    }
    if k == 0 {
        let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x44)], || o(this), || fila_de_regens(this));
        PLACAR.registrar(igual, || format!("fila_de_regens({:x}) sem vencidas {:x?} x {:x?}", this, mo, mr));
        return;
    }
    let antes = core::slice::from_raw_parts(base as *const u8, c0 as usize * 0xc).to_vec();
    o(this);
    let (n, b2) = (cpw10_i(this + 0x14), cpw10_d(this + 0x10));
    let mut igual = n == c0 - k && b2 == base;
    if igual {
        let depois = core::slice::from_raw_parts(base as *const u8, c0 as usize * 0xc);
        let ficam = (c0 - k) as usize * 0xc;
        igual = depois[..ficam] == antes[k as usize * 0xc..];
        for j in (c0 - k) as usize..c0 as usize {
            let e = base + j * 0xc;
            igual &= cpw10_i(e) == -1 && cpw10_w(e + 4) == 0 && cpw10_w(e + 6) == 0;
        }
    }
    PLACAR.registrar(igual, || format!("fila_de_regens({:x}) {} vencidas de {}: conta {} base {:x}/{:x}", this, k, c0, n, base, b2));
}

/// O veículo pelo sorteio(100) dos grandes (0x757f30/0x757fa0): 0 → 6; até 7 → 3; até 14 → 5; até 39 → 2; até 59 → 0;
/// senão 1 (comparações do al com sinal).
fn cpw10_veiculo_grande(al: i8) -> u32 {
    if al == 0 {
        6
    } else if al < 8 {
        3
    } else if al < 0xf {
        5
    } else if al < 0x28 {
        2
    } else {
        (al >= 0x3c) as u32
    }
}

/// O veículo dos menores: sorteio(4) de 0 a 2 → 2, senão 4.
unsafe fn cpw10_veiculo_menor() -> u32 {
    let r = cpw10_sortear(4);
    if (0..=2).contains(&r) {
        2
    } else {
        4
    }
}

/// { 0x757f30 } (cdecl, media.cpp): `media_pick_outlet_type_for_club(clube)`: reputação (+0x80) acima de 6250: o
/// veículo dos grandes pelo sorteio(100); senão o dos menores (sorteio(4)). Pelas notícias do troféu (0x75c390) e da venda
/// (0x75cfc0).
unsafe extern "C" fn veiculo_pelo_clube(clube: usize) -> u32 {
    if cpw10_w(clube + 0x80) > 0x186a {
        cpw10_veiculo_grande(cpw10_sortear(100) as u8 as i8)
    } else {
        cpw10_veiculo_menor()
    }
}

unsafe extern "C" fn comparar_veiculo_pelo_clube(clube: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(%%I1%%);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(clube), || crate::aleatorio::sem_contar(|| veiculo_pelo_clube(clube)));
    PLACAR.registrar(igual && r == n, || format!("veiculo_pelo_clube({:x}) original={} rust={} sorteio igual={}", clube, r, n, igual));
    r
}

/// { 0x757fa0 } (cdecl, media.cpp): `media_pick_outlet_type_for_person(pessoa)`: como o do clube, pela reputação do
/// jogador (+0xb da ficha +0x61). Pelas histórias do jogador (0x72c1f0), do desabafo, do elogio, de querer jogar e da
/// comemoração da contratação (0x75b6e0, nas transferências).
unsafe extern "C" fn veiculo_pela_pessoa(pessoa: usize) -> u32 {
    if cpw10_w(cpw10_d(pessoa + 0x61) + 0xb) > 0x186a {
        cpw10_veiculo_grande(cpw10_sortear(100) as u8 as i8)
    } else {
        cpw10_veiculo_menor()
    }
}

unsafe extern "C" fn comparar_veiculo_pela_pessoa(pessoa: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(%%I2%%);
    let (r, n, igual, _, _) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(pessoa), || crate::aleatorio::sem_contar(|| veiculo_pela_pessoa(pessoa)));
    PLACAR.registrar(igual && r == n, || format!("veiculo_pela_pessoa({:x}) original={} rust={} sorteio igual={}", pessoa, r, n, igual));
    r
}
