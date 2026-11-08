// ---------- %%LEVA%%a leva: as casas da formação: gravar a coordenada da casa e escolher as posições, a posição repetida e o tipo de marcação (formation.cpp) ----------

unsafe fn cpu11_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpu11_pw(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v);
}

/// { 0x5b3560 } (thiscall, ret 0x10, formation.cpp): `Formation::set_slot_coordinate(casa, fase, faixa, coord)`:
/// grava a coordenada (dword: x 0..0x1a, y 0..0x23) da casa na fase e na faixa da bola (+0x164, 11 × 2 × 3 × 4
/// dwords) e, para cada fase, acha as posições da tabela 0x9cc368 (palavras até 0xffff) mais perto das 12 coordenadas
/// da casa nessa fase (|Δx·9/27| + |Δy·12/36| contra a casa de cada posição pela 0x5b7e60; dobrada nas posições com o
/// bit 2; os empates entram na lista da fase, até 11 em [esp+0x64] contíguas). A posição geral (+0x12d) fica se
/// estiver numa das listas; senão vira a primeira da fase de menor distância (a 0x5b59a0 refaz a máscara, a casa
/// ganha a marca +0x143 e as outras casas com a mesma posição a perdem). A posição extra (+0x14e) sai da lista da
/// outra fase: a mesma posição, ou a de mesmo lado (0x5b73a0), ou a primeira, ou com duas o meio dos bits das duas.
/// Zera +0x187f. Argumentos fora da faixa: a original (caixa e 0). Devolve 1.
unsafe extern "thiscall" fn set_slot_coordinate(this: usize, casa: u32, fase: u32, band: u32, coord: u32) -> u32 {
    let (s, f) = (casa as u8 as i8, fase as u8 as i8);
    let (zx, zy) = (band as u16 as i16, (band >> 16) as u16 as i16);
    let (cx, cy) = (coord as u16 as i16, (coord >> 16) as u16 as i16);
    if !(0..=10).contains(&s) || !(0..=1).contains(&f) || !(0..=2).contains(&zx) || !(0..=3).contains(&zy)
        || !(0..=0x1a).contains(&cx) || !(0..=0x23).contains(&cy)
    {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(%%I0%%)(this, casa, fase, band, coord);
    }
    let casa_no: unsafe extern "C" fn(u32, i8, u32, *mut i16) -> u32 = core::mem::transmute(0x5b7e60usize);
    let separar: unsafe extern "C" fn(u32, *mut u16, *mut u16) -> u32 = core::mem::transmute(0x5b73a0usize);
    let mascara: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5b59a0usize);
    let su = s as usize;
    let cel = this + 0x164 + 4 * (zy as usize + 4 * (zx as usize + 3 * (f as usize + 2 * su)));
    crate::comparison::branch("cpu_11 coordenada gravada");
    core::ptr::write_unaligned(cel as *mut u32, coord); // cpu11: a coordenada
    // a saída da 0x5b7e60 é o próprio argumento coord na original (uma vaga só, para a função inteira)
    let mut saida: [i16; 2] = [cx, cy];
    // ponytail: as listas das duas fases ficam contíguas como em [esp+0x64]; os índices além de 0xea não acontecem
    let mut cand = [0u16; 0xea];
    let mut conta = [0i8; 2]; // na original sem valor inicial; a primeira posição sempre a zera
    let mut melhor = [0i32; 2];
    let mut ultimo_dy: i32 = 0; // [esp+0x60], lido só pelos índices fora da tabela no fim
    for p in 0..2usize {
        melhor[p] = 0x7fffffff;
        let mut lista = 0x9cc368usize;
        while cpu11_w(lista) != -1 {
            let pos = cpu11_w(lista) as u16;
            let base = this + 0x164 + su * 0x60 + p * 0x30;
            let mut dist: i32 = 0;
            for zx in 0..3u32 {
                for zy in 0..4u32 {
                    // a máscara vai com a altura da faixa nos 16 bits de cima (o ecx da original); a 0x5b7e60 não os lê
                    casa_no(pos as u32 | zy << 16, p as i8, zx | zy << 16, saida.as_mut_ptr());
                    let c = base + 4 * (zx as usize * 4 + zy as usize);
                    let dy = ((cpu11_w(c + 2) as i32 * 12) / 36 - (saida[1] as i32 * 12) / 36).wrapping_abs();
                    let dx = ((cpu11_w(c) as i32 * 9) / 27 - (saida[0] as i32 * 9) / 27).wrapping_abs();
                    ultimo_dy = dy;
                    dist = dx.wrapping_add(dist).wrapping_add(dy);
                }
            }
            if pos & 2 != 0 {
                crate::comparison::branch("cpu_11 distância dobrada");
                dist = dist.wrapping_add(dist); // cpu11: o dobro
            }
            if dist <= melhor[p] { // cpu11: o empate entra na lista
                if dist < melhor[p] {
                    conta[p] = 0;
                } else {
                    crate::comparison::branch("cpu_11 empate na lista");
                }
                melhor[p] = dist;
                cand[(conta[p] as i32 + p as i32 * 11) as usize] = pos;
                conta[p] = conta[p].wrapping_add(1);
            }
            lista += 2;
        }
    }
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    let geral = this + 0x12d + su * 2;
    let mut sf: i8 = (melhor[0] > melhor[1]) as i8;
    let mut si: i8 = 0;
    for q in 0..2i8 {
        let mut a: i8 = 0;
        while a < conta[q as usize] {
            if cand[(a as i32 + q as i32 * 11) as usize] == cpu11_w(geral) as u16 {
                sf = q;
                si = a;
                break;
            }
            a += 1;
        }
    }
    let novo = cand[(sf as i32 * 11 + si as i32) as usize];
    if cpu11_w(geral) as u16 != novo {
        cpu11_pw(geral, novo);
        mascara(this);
        *((this + 0x143 + su) as *mut u8) = 1;
        for k in 0..11i8 {
            if k != s && cpu11_w(this + 0x12d + k as usize * 2) as u16 == novo {
                *((this + 0x143 + k as usize) as *mut u8) = 0;
            }
        }
    }
    let extra = this + 0x14e + su * 2;
    let o = (1 - sf as i32) * 11; // o começo da lista da outra fase
    let no = conta[(1 - sf) as usize];
    let atual = cpu11_w(geral) as u16;
    for a in 0..no.max(0) as i32 {
        if cand[(o + a) as usize] == atual {
            cpu11_pw(extra, cand[(o + a) as usize]);
            return 1;
        }
    }
    // ponytail: as saídas da 0x5b73a0 ficam sem valor inicial na original se ela falhar (posição sem lado); aqui 0
    let (mut setor, mut lado_c, mut lado_a) = (0u16, 0u16, 0u16);
    for b in 0..no.max(0) as i32 {
        separar(cand[(o + b) as usize] as u32, &mut setor, &mut lado_c);
        separar(cpu11_w(geral) as u16 as u32, &mut setor, &mut lado_a);
        if lado_c == lado_a {
            cpu11_pw(extra, cand[(o + b) as usize]);
            return 1;
        }
    }
    if no != 2 {
        cpu11_pw(extra, cand[o as usize]);
        return 1;
    }
    // duas na outra fase: o meio dos bits de setor (1..0x40) e de lado (0x80..0x800) das duas. As palavras do quadro a
    // partir de [esp+0x38]: a tabela dos lados e 0xffff, a faixa (3) e 0, o ponteiro da melhor distância (sem valor
    // reproduzível: 0), this, a tabela dos setores e 0xffff, o último |Δy| e as listas.
    let (c0, c1) = (cand[o as usize], cand[(o + 1) as usize]);
    let mut q = [0u16; 0x100];
    q[..6].copy_from_slice(&[0x80, 0x100, 0x200, 0x400, 0x800, 0xffff]);
    q[6] = 3;
    q[10] = this as u16;
    q[11] = (this >> 16) as u16;
    q[12..20].copy_from_slice(&[1, 2, 4, 8, 0x10, 0x20, 0x40, 0xffff]);
    q[20] = ultimo_dy as u16;
    q[21] = (ultimo_dy >> 16) as u16;
    q[22..].copy_from_slice(&cand[..0xea]);
    let mut v1a: i8 = conta[0]; // [esp+0x1a]
    let mut v12: i8 = no; // [esp+0x12]
    for k in 0..7usize {
        if q[12 + k] & c0 != 0 {
            v1a = k as i8;
        }
        if q[12 + k] & c1 != 0 {
            v12 = k as i8;
        }
    }
    let mut v13: i8 = si; // [esp+0x13]
    let mut dl: i8 = v1a;
    for k in 0..5usize {
        if q[k] & c0 != 0 {
            v13 = k as i8;
        }
        if q[k] & c1 != 0 {
            dl = k as i8;
        }
    }
    let ib = (v13 as i32 + dl as i32) / 2;
    let ia = (v12 as i32 + v1a as i32) / 2;
    cpu11_pw(extra, q[ib as usize & 0xff] | q[(12 + ia) as usize & 0xff]);
    1
}

/// { 0x5b3460 } (thiscall, ret 8, formation.cpp): `Formation::check_duplicate_position(casa, *saida)`: zera a saída;
/// a primeira outra casa com a mesma posição geral (+0x12d) decide: com marca (+0x143, byte com sinal) menor que a da
/// casa, saída 1 e devolve 1; senão devolve 0. Sem repetida, 1. Casa fora de 0..10: a original (caixa e 0).
unsafe extern "thiscall" fn check_duplicate_position(this: usize, casa: u32, saida: *mut u32) -> u32 {
    let s = casa as u8 as i8;
    if !(0..=10).contains(&s) {
        return original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(%%I1%%)(this, casa, saida);
    }
    *saida = 0;
    let su = s as usize;
    for k in 0..11usize {
        if k != su && cpu11_w(this + 0x12d + k * 2) == cpu11_w(this + 0x12d + su * 2) {
            if *((this + 0x143 + k) as *const i8) < *((this + 0x143 + su) as *const i8) {
                *saida = 1;
                return 1;
            }
            return 0;
        }
    }
    1
}

/// { 0x5b57d0 } (thiscall, ret 8, formation.cpp): `Formation::set_marking_type(casa, marca)`: as marcas 1, 2 e 4
/// trocam os bits 0..2 do byte +0x5b4 da casa; 8 e 0x10, os bits 3..4; zera +0x187f e devolve 1. Outra marca: a
/// original (caixa e 0). A casa não é conferida.
unsafe extern "thiscall" fn set_marking_type(this: usize, casa: u32, marca: u32) -> u32 {
    let m = marca as u8;
    let limpa = match m {
        1 | 2 | 4 => 0xf8u8,
        8 | 0x10 => 0xe7,
        _ => return original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I2%%)(this, casa, marca),
    };
    let b = (this as isize + casa as u8 as i8 as isize + 0x5b4) as *mut u8;
    *b = *b & limpa | m;
    core::ptr::write_unaligned((this + 0x187f) as *mut u32, 0);
    1
}

/// As regiões da formação que as três gravam: as posições, as marcas, as extras e as coordenadas (+0x12d..+0x584),
/// os tipos de marcação (+0x5b4, 11) e a máscara com +0x187f (+0x187b, 8).
fn formation_regions(this: usize) -> [(usize, usize); 3] {
    [(this + 0x12d, 0x584 - 0x12d), (this + 0x5b4, 11), (this + 0x187b, 8)]
}

/// Comparador: grava só a formação (a 0x5b7e60 e a 0x5b73a0 são puras; a 0x5b59a0 refaz a máscara): lado a lado.
unsafe extern "thiscall" fn compare_set_slot_coordinate(this: usize, casa: u32, fase: u32, band: u32, coord: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32>(%%I0%%);
    let reg = formation_regions(this);
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, casa, fase, band, coord), || set_slot_coordinate(this, casa, fase, band, coord));
    SCOREBOARD.record(igual && r == n, || format!("set_slot_coordinate({:x}, {}, {}, {:x}, {:x}) {}/{} {:x?} x {:x?}", this, casa as u8 as i8, fase as u8 as i8, band, coord, r, n, mo, mr));
    r
}

/// Comparador: pura (grava só a saída): as duas sobre saídas próprias.
unsafe extern "thiscall" fn compare_check_duplicate_position(this: usize, casa: u32, saida: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u32) -> u32>(%%I1%%);
    if !(0..=10).contains(&(casa as u8 as i8)) {
        return o(this, casa, saida); // ponytail: a caixa, só a original
    }
    let (mut so, mut sr) = (0xdead_u32, 0xbeef_u32);
    let r = o(this, casa, &mut so);
    let n = check_duplicate_position(this, casa, &mut sr);
    SCOREBOARD.record(r == n && so == sr, || format!("check_duplicate_position({:x}, {}) {}/{} saída {}/{}", this, casa as u8 as i8, r, n, so, sr));
    *saida = so;
    r
}

/// Comparador: grava um byte e +0x187f: lado a lado.
unsafe extern "thiscall" fn compare_set_marking_type(this: usize, casa: u32, marca: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(%%I2%%);
    if !matches!(marca as u8, 1 | 2 | 4 | 8 | 0x10) {
        return o(this, casa, marca); // ponytail: a caixa, só a original
    }
    let reg = [(this as isize + casa as u8 as i8 as isize + 0x5b4) as usize, 1usize];
    let reg = [(reg[0], reg[1]), (this + 0x187f, 4)];
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, casa, marca), || set_marking_type(this, casa, marca));
    SCOREBOARD.record(igual && r == n, || format!("set_marking_type({:x}, {}, {}) {}/{} {:x?} x {:x?}", this, casa as u8 as i8, marca as u8, r, n, mo, mr));
    r
}
