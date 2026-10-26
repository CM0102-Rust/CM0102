// ---------- %%LEVA%%a leva: as qualidades e os defeitos do jogador (Database.cpp) ----------

unsafe fn dbs3_u32(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn dbs3_i16(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i16) as i32
}

/// A 0x7c1e30 das notas ([0xb74350]) do jogador (+0x61) da pessoa: `media(p, tipo, atributo)`, peso 6,5.
unsafe fn dbs3_media(pessoa: usize, tipo: u32, atributo: u32) -> f64 {
    let media: unsafe extern "thiscall" fn(usize, usize, u32, u32, f32) -> f64 = core::mem::transmute(0x7c1e30usize);
    media(dbs3_u32(0xb74350), dbs3_u32(pessoa + 0x61), tipo, atributo, 6.5)
}

/// O atributo calculado (0x543540) do jogador da pessoa, como byte com sinal.
unsafe fn dbs3_atributo(pessoa: usize, atributo: u32) -> i32 {
    let f: unsafe extern "C" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x543540usize);
    f(pessoa, dbs3_u32(pessoa + 0x61), atributo) as i8 as i32
}

/// Os bits das posições em que o jogador é natural (20), comuns às duas: goleiro (+0xf), lateral (+0x11, com +0x19
/// ou com +0x17/+0x18), zagueiros (+0x12/+0x13 sem +0x17/+0x18 a 15), meias (+0x14, +0x15). `bits` = os valores
/// de cada caso na ordem; mais o da soma das dez posições (+0x10..+0x19) acima de 110.
unsafe fn dbs3_posicoes(j: usize, bits: [u32; 7], soma: u32) -> u32 {
    let b = |k: usize| sb(j, k);
    let mut f = 0;
    if b(0xf) == 20 {
        f |= bits[0];
    }
    if b(0x11) == 20 {
        if b(0x19) == 20 {
            f += bits[1];
        }
        if b(0x17) == 20 || b(0x18) == 20 {
            f += bits[2];
        }
    }
    if b(0x12) == 20 && b(0x18) < 15 && b(0x17) < 15 {
        f += bits[3];
    }
    if b(0x13) == 20 && b(0x18) < 15 && b(0x17) < 15 {
        f += bits[4];
    }
    if b(0x14) == 20 {
        f += bits[5];
    }
    if b(0x15) == 20 {
        f += bits[6];
    }
    if (0x10..=0x19).map(b).sum::<i32>() > 110 {
        f += soma;
    }
    f
}

/// { 0x546ab0 } (cdecl): `staff_get_player_strength_flags(pessoa, outro)`: os bits das qualidades do jogador (+0x61)
/// para a mídia, olheiros, busca e transferências; `outro` só precisa não ser nulo. Posições naturais (1, 4, 2, 8, 0x10,
/// 0x20, 0x40), 0x1000000 se o nibble de [0xae2d0c] + 79 × id + 0xb passa um sorteio(3) e a 0x8cca10 ([0xde1f54]) dá 0,
/// 0x80 pela soma das posições; depois 19 sorteios (0x90cfc0) contra bytes do jogador, atributos (0x543540) e médias
/// (0x7c1e30 em [0xb74350], peso 6,5, x87 em precisão dupla com os `fstp dword` no meio): 0x100 (+0x3c), 0x200 (+0x37
/// mais o ajuste das médias 0x16/0xd/5/1), 0x800000 (média 0xe acima de 8), 0x400 (+0x39, +0x3e e a média 0xe ou o
/// atributo 0x17 com as médias 7 e 0x16), 0x800, 0x1000, 0x2000 (+0x2e, +0x40, +0x44), 0x4000, 0x20000, 0x100000,
/// 0x200000, 0x400000 (atributos 4, 0xd, 0x14, 0x15, 1), 0x8000 (+0x21), 0x10000 (atributo 0xb e a média 0xf), 0x40000
/// (reputação e as médias 6/2 e 6/1), 0x80000 (médias 0x15, 0x10, 2 e 1). A original guarda floats nos próprios
/// argumentos (gravados antes de lidos); a cópia não. Nulos: 0.
unsafe extern "C" fn forcas_do_jogador(pessoa: usize, outro: usize) -> u32 {
    if outro == 0 || pessoa == 0 {
        return 0;
    }
    let fora: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x8cca10usize);
    let j = || dbs3_u32(pessoa + 0x61);
    let b = |k: usize| sb(j(), k);
    let m = |tipo: u32, atributo: u32| dbs3_media(pessoa, tipo, atributo);
    let mut f = dbs3_posicoes(j(), [1, 4, 2, 8, 0x10, 0x20, 0x40], 0);
    let r = sortear(3);
    let id = dbs3_u32(pessoa) as u32;
    let nibble = (*(dbs3_u32(0xae2d0c).wrapping_add(id.wrapping_mul(79) as usize).wrapping_add(0xb) as *const u8) & 0xf) as i32;
    if nibble > r && dbs3_u32(0xde1f54) != 0 && fora(dbs3_u32(0xde1f54), pessoa, 0) == 0 {
        f += 0x1000000;
    }
    if j() != 0 && (0x10..=0x19).map(b).sum::<i32>() > 110 {
        f += 0x80;
    }
    if b(0x3c) > sortear(3) + 15 {
        f += 0x100;
    }
    let mut ajuste = 0i32;
    if m(2, 0x16) > f32_em(0x969570) as f64 {
        let a = ftol(m(2, 0xd)) as i8 as i32;
        if a > 10 {
            ajuste = ftol(a as f64 - f64_em(0x969568));
        }
        let x = (m(2, 5) * f32_em(0x969560) as f64) as f32; // fstp dword
        ajuste = ajuste.wrapping_add(ftol(x as f64 / m(2, 1)));
    }
    if b(0x37).wrapping_add(ajuste) > sortear(50) {
        f += 0x200;
    }
    let v14 = m(2, 0xe);
    let v14f = v14 as f32; // fst dword
    if v14 > f64_em(0x968f10) {
        f += 0x800000;
    }
    if b(0x39) > sortear(50) && b(0x3e) > sortear(50) {
        let mut sim = false;
        if v14f as f64 > f64_em(0x9689a0) {
            let r = sortear(10) + 5;
            sim = ftol(v14f as f64) as i8 as i32 > r;
        }
        if !sim && dbs3_atributo(pessoa, 0x17) > sortear(5) + 15 {
            let x = m(2, 7) * f64_em(0x968f50); // fstp qword
            sim = m(2, 0x16) > x;
        }
        if sim {
            f += 0x400;
        }
    }
    for (k, bit) in [(0x2e, 0x800), (0x40, 0x1000), (0x44, 0x2000)] {
        if b(k) > sortear(5) + 15 {
            f += bit;
        }
    }
    for (a, bit) in [(4, 0x4000), (0xd, 0x20000), (0x14, 0x100000), (0x15, 0x200000), (1, 0x400000)] {
        if dbs3_atributo(pessoa, a) > sortear(5) + 15 {
            f += bit;
        }
    }
    if b(0x21) > sortear(3) + 16 {
        f += 0x8000;
    }
    if dbs3_atributo(pessoa, 0xb) > sortear(4) + 14 {
        let x = m(2, 0xf); // fstp qword
        let r = sortear(20);
        if !(r as f64 * f64_em(0x967880) + f64_em(0x968eb0) >= x) {
            f += 0x10000;
        }
    }
    let v = m(2, 0x16);
    let limite = 1400 - dbs3_i16(j() + 0xb) / 10;
    if !(limite as f64 >= v) {
        let a = m(6, 2) as f32;
        let c = m(6, 1) as f32;
        let r = sortear(3);
        if !((r as f64 * f64_em(0x967880) + f64_em(0x968968)) * c as f64 >= a as f64) {
            f += 0x40000;
        }
    }
    if !(m(2, 0x15) >= f64_em(0x968930)) && m(2, 0x10) > f64_em(0x968ff8) {
        let x = m(2, 2) * f64_em(0x968930); // fstp qword
        if m(2, 1) > x {
            f += 0x80000;
        }
    }
    f
}

/// { 0x5470d0 } (cdecl): `staff_get_player_weakness_flags(pessoa, clube)`: os bits dos defeitos do jogador (+0x61) para
/// a mídia. Posições naturais (0x20, 0x10, 0x40, 0x80, 0x100, 0x200), 0x800000 pela soma das posições; depois os
/// sorteios (0x90cfc0, alguns com n = −50) contra bytes da pessoa e do jogador, atributos (0x543540), médias (0x7c1e30,
/// peso 6,5) e a palavra 0x62c3d0 ([0xb63d28]): 1 (+0x5d da pessoa e as médias 0xb, 0xa e 1), 2 (+0x3c contra um
/// sorteio de 4, 7, 8 ou 11 pelas +0x34 e 0x62c3d0), 4 (titular, vaga 0..10 da 0x8972c0 em [0xde1d08], com a nota
/// 0x6e6340 da casa 0x5b2ae0 abaixo de 150 + sorteio(40)), 0x400, 0x800, 8, 0x1000, 0x2000, 0x4000, 0x8000, 0x10000,
/// 0x20000, 0x40000, 0x80000, 0x100000, 0x200000, 0x400000. A original guarda bytes e floats nos próprios argumentos
/// (gravados antes de lidos; a vaga passada à 0x5b2ae0 é o argumento `clube` com o byte da vaga por cima, e a cópia a
/// monta igual); a cópia não grava neles. Nulos: 0. Na mídia (0x72c1f0, 0x487c70).
unsafe extern "C" fn fraquezas_do_jogador(pessoa: usize, clube: usize) -> u32 {
    if clube == 0 || pessoa == 0 {
        return 0;
    }
    let palavra: unsafe extern "thiscall" fn(usize, i32) -> i32 = core::mem::transmute(0x62c3d0usize);
    let vaga_de: unsafe extern "thiscall" fn(usize, usize, usize, u32) -> u8 = core::mem::transmute(0x8972c0usize);
    let formacao_de: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x896660usize);
    let casa_de: unsafe extern "thiscall" fn(usize, u32, u32) -> u16 = core::mem::transmute(0x5b2ae0usize);
    let nota: unsafe extern "C" fn(usize, u32, u32, usize, i32, usize) -> i32 = core::mem::transmute(0x6e6340usize);
    let j = || dbs3_u32(pessoa + 0x61);
    let b = |k: usize| sb(j(), k);
    let m = |tipo: u32, atributo: u32| dbs3_media(pessoa, tipo, atributo);
    let mut f = dbs3_posicoes(j(), [0, 0x20, 0x10, 0x40, 0x80, 0x100, 0x200], 0x800000);
    if (*((pessoa + 0x5d) as *const i8) as i32) < sortear(4) + 5 && (b(0x1c) > 15 || b(0x25) < 6) {
        let a = (m(2, 0xb) * f32_em(0x969584) as f64) as f32; // fstp dword
        let s = (m(2, 0xa) + a as f64) as f32; // fstp dword
        if !(m(2, 1) * f32_em(0x968f18) as f64 >= s as f64) {
            f += 1;
        }
    }
    let mut n = 4;
    let w = palavra(dbs3_u32(0xb63d28), dbs3_u32(pessoa) as i32) as i16 as i32;
    if w > sortear(1000) + 500 {
        n = 8;
    }
    if b(0x34) < sortear(12) {
        n += 3;
    }
    if b(0x3c) < sortear(n) + 8 && b(0xf) < 20 {
        f += 2;
    }
    let tatica = dbs3_u32(0xde1d08);
    let vaga = vaga_de(tatica, clube, pessoa, 0);
    if (vaga as i8) > -1 && (vaga as i8) < 11 {
        let formacao = formacao_de(tatica, clube, 0);
        let casa = casa_de(formacao, (clube as u32 & !0xff) | vaga as u32, 0); // o argumento com o byte da vaga
        let x = nota(pessoa, casa as u32, 0x800, 0, -1, 0);
        if x < sortear(40) + 150 {
            f += 4;
        }
    }
    let mut ajuste = 0i32;
    if m(2, 0x16) > f32_em(0x969570) as f64 {
        let a = ftol(m(2, 0xd)) as i8 as i32;
        if a > 10 {
            ajuste = 70 - a;
        }
        let x = (m(2, 5) * f32_em(0x969580) as f64) as f32; // fstp dword
        ajuste = ajuste.wrapping_add(ftol(x as f64 / m(2, 1)));
    }
    if b(0x37) < sortear(-50).wrapping_add(ajuste) {
        f += 0x400;
    }
    if b(0x39) < sortear(-50) && b(0x3e) < sortear(-50) {
        let v = m(2, 0xe);
        let vf = v as f32; // fst dword
        if v > f64_em(0x9689a0) {
            let r = sortear(10);
            if (ftol(vf as f64) as i8 as i32) < r {
                f += 0x800;
            }
        }
    }
    let bd = dbs3_u32(0xb63d28);
    if bd != 0 {
        let w = palavra(bd, dbs3_u32(pessoa) as i32) as i16 as i32;
        if w > sortear(500) + 750 {
            f += 8;
        }
    }
    if b(0x2e) < sortear(15) {
        f += 0x1000;
    }
    if b(0xf) < 20 { // os de linha
        if b(0x40) < sortear(10) {
            f += 0x2000;
        }
        if b(0x44) < sortear(12) {
            f += 0x4000;
        }
    }
    if b(0x1e) < sortear(-50) {
        f += 0x8000;
    }
    if b(0xf) < 20 && b(0x25) + b(0x1c) > sortear(20) + 25 {
        f += 0x10000;
    }
    if b(0x21) < 12 - sortear(10) {
        f += 0x20000;
    }
    if b(0x11) < 15 && dbs3_atributo(pessoa, 0xb) > sortear(4) + 14 {
        let v = m(2, 0xf);
        let vf = v as f32; // fst dword
        if v > f64_em(0x9689a0) && dbs3_atributo(pessoa, 0xb) as f64 * f64_em(0x969578) > vf as f64 {
            f += 0x40000;
        }
    }
    if b(0x28) > sortear(5) + 13 {
        f += 0x80000;
    }
    if m(2, 0x16) > f32_em(0x969570) as f64 {
        let a = dbs3_atributo(pessoa, 0xc);
        let t = (m(2, 0x10) + a as f64 - f32_em(0x968a78) as f64) as f32; // fstp dword
        if (sortear(20) + 20) as f64 > t as f64 {
            let x = m(6, 2); // fstp qword
            if m(6, 1) * f64_em(0x969510) > x {
                f += 0x100000;
            }
        }
    }
    if !(m(2, 0x15) >= f64_em(0x968930)) && m(2, 0x10) > f64_em(0x968ff8) {
        f += 0x200000;
    }
    if dbs3_atributo(pessoa, 0x14) < 7 || dbs3_atributo(pessoa, 0x14) < dbs3_i16(j() + 5) / 15 {
        f += 0x400000;
    }
    f
}

/// O lado a lado das duas, com o estado do sorteio. A 0x6e6340 (pela 0x62ed70) sorteia só na primeira consulta do
/// jogador e guarda: com essa memória fria a original sorteia a mais e a cópia, que roda depois, não, e os sorteios
/// seguintes saem diferentes. Então, divergindo, refaz uma vez a comparação a partir do mesmo estado (agora quente, a
/// original sem contar) e devolve o estado do sorteio da primeira, a que o jogo segue.
unsafe fn dbs3_comparar(o: impl Fn() -> u32, n: impl Fn() -> u32) -> (u32, u32, bool, String) {
    let antes = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (r, x, igual, _, _) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || o(),
        || crate::aleatorio::sem_contar(|| n()));
    if igual && r == x {
        return (r, x, true, String::new());
    }
    let depois = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    antes.escrever();
    let (r2, x2, igual2, mo, mr) = crate::comparacao::lado_a_lado_leve(&crate::aleatorio::REGIOES_DO_ESTADO, || crate::aleatorio::sem_contar(|| o()),
        || crate::aleatorio::sem_contar(|| n()));
    depois.escrever();
    (r, x, igual2 && r2 == x2, format!("1a {:x} x {:x}; 2a {:x} x {:x} {:x?} x {:x?}", r, x, r2, x2, mo, mr))
}

unsafe extern "C" fn comparar_forcas_do_jogador(pessoa: usize, outro: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(%%I0%%);
    let (r, _, igual, d) = dbs3_comparar(|| o(pessoa, outro), || forcas_do_jogador(pessoa, outro));
    PLACAR.registrar(igual, || format!("forcas_do_jogador({:x}, {:x}) {}", pessoa, outro, d));
    r
}

unsafe extern "C" fn comparar_fraquezas_do_jogador(pessoa: usize, clube: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize, usize) -> u32>(%%I1%%);
    let (r, _, igual, d) = dbs3_comparar(|| o(pessoa, clube), || fraquezas_do_jogador(pessoa, clube));
    PLACAR.registrar(igual, || format!("fraquezas_do_jogador({:x}, {:x}) {}", pessoa, clube, d));
    r
}
