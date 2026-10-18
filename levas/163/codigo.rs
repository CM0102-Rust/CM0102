// ---------- centesima sexagesima terceira leva: a formacao (marcas e casas) e o reinicio dos recordes do clube ----------

/// { 0x5b3c70 } (thiscall, ret 0x10, formation.cpp): a casa guardada da vaga `vaga` 0..10 com
/// `ataque` 0/1 e `faixa` (palavra baixa 0..2, alta 0..3): o dword +0x164 + 4×(alta + 4×(baixa +
/// 3×(ataque + 2×vaga))) vai para `*saida`. Devolve 1. Fora das faixas: a original (caixa de erro).
unsafe extern "thiscall" fn casa_guardada(t: usize, vaga: i8, ataque: i8, faixa: u32, saida: *mut u32) -> u32 {
    let baixo = faixa as u16 as i16;
    let alto = (faixa >> 16) as u16 as i16;
    if !(0..=10).contains(&vaga) || !(0..=1).contains(&ataque) || !(0..=2).contains(&baixo) || !(0..=3).contains(&alto) {
        return orig::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(361)(t, vaga, ataque, faixa, saida);
    }
    let i = alto as usize + 4 * (baixo as usize + 3 * (ataque as usize + 2 * vaga as usize));
    *saida = core::ptr::read_unaligned((t + 0x164 + 4 * i) as *const u32);
    1
}

/// { 0x5b50d0 } (thiscall, ret 4, formation.cpp): liga a marca `m` em +0x584 apagando antes as do
/// mesmo grupo (1/2/4/8; 0x10/0x20/0x40; 0x80/0x100; 0x200/0x400; 0x800/0x1000; 0x2000/0x4000;
/// 0x8000/0x10000/0x20000) e zera +0x187f. Devolve 1. Marca fora dos grupos: a original (caixa de
/// erro).
unsafe extern "thiscall" fn ligar_marca_da_formacao(t: usize, m: u32) -> u32 {
    let grupo: u32 = match m {
        1 | 2 | 4 | 8 => 0xf,
        0x10 | 0x20 | 0x40 => 0x70,
        0x80 | 0x100 => 0x180,
        0x200 | 0x400 => 0x600,
        0x800 | 0x1000 => 0x1800,
        0x2000 | 0x4000 => 0x6000,
        0x8000 | 0x10000 | 0x20000 => 0x38000,
        _ => return orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(362)(t, m),
    };
    let marcas = (t + 0x584) as *mut u32;
    let v = core::ptr::read_unaligned(marcas) & !grupo | m;
    core::ptr::write_unaligned((t + 0x187f) as *mut u32, 0);
    core::ptr::write_unaligned(marcas, v);
    1
}

/// { 0x452fe0 } (cdecl, club_records.cpp): reinicia os recordes do clube `r`: −1.0f em +0x1f1 e −1
/// nos campos de id (dwords, palavras e bytes). Devolve 1. `r` nulo: a original (caixa de erro).
unsafe extern "C" fn reiniciar_recordes(r: usize) -> u32 {
    if r == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(363)(r);
    }
    let d = |o: usize| core::ptr::write_unaligned((r + o) as *mut u32, u32::MAX);
    core::ptr::write_unaligned((r + 0x1f1) as *mut u32, 0xbf800000);
    for o in [1, 8, 0xe, 0x31, 0x54, 0x77, 0x85, 0xa4, 0xc3, 0xe2, 0x101, 0x118, 0x12c, 0x140, 0x154] {
        d(o);
    }
    for o in [0x168, 0x183, 0x19e, 0x1d6, 0x210, 0x22b] {
        *((r + o) as *mut u8) = 0xff;
    }
    for o in [0x247, 0x261] {
        core::ptr::write_unaligned((r + o) as *mut u16, 0xffff);
    }
    d(0x2a3);
    d(0x2e2);
    core::ptr::write_unaligned((r + 0x2f9) as *mut u16, 0xffff);
    for o in [0x313, 0x33c, 0x365, 0x36b, 0x37d] {
        d(o);
    }
    1
}

unsafe extern "thiscall" fn comparar_casa_guardada(t: usize, vaga: i8, ataque: i8, faixa: u32, saida: *mut u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, i8, i8, u32, *mut u32) -> u32>(361);
    let reg = [(saida as usize, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, vaga, ataque, faixa, saida), || casa_guardada(t, vaga, ataque, faixa, saida));
    PLACAR.registrar(igual && r == v, || format!("casa_guardada({:#x}, {}, {}, {:#x}) original={} {:x?} rust={} {:x?}", t, vaga, ataque, faixa, r, mo, v, mr));
    r
}

unsafe extern "thiscall" fn comparar_ligar_marca_da_formacao(t: usize, m: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> u32>(362);
    let reg = [(t + 0x584, 4), (t + 0x187f, 4)];
    let (r, v, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(t, m), || ligar_marca_da_formacao(t, m));
    PLACAR.registrar(igual && r == v, || format!("ligar_marca_da_formacao({:#x}, {:#x}) original={} {:x?} rust={} {:x?}", t, m, r, mo, v, mr));
    r
}

unsafe extern "C" fn comparar_reiniciar_recordes(r: usize) -> u32 {
    let o = orig::<unsafe extern "C" fn(usize) -> u32>(363);
    let reg = if r == 0 { vec![] } else { vec![(r, 0x381)] };
    let (a, v, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(r), || reiniciar_recordes(r));
    PLACAR.registrar(igual && a == v, || format!("reiniciar_recordes({:#x}) original={} {:x?} rust={} {:x?}", r, a, mo, v, mr));
    a
}
