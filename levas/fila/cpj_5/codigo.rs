// ---------- %%LEVA%%a leva: limpar as partidas assistidas que os humanos não podem ver, os alvos dos olheiros do clube e trocar o olheiro da busca ----------

unsafe fn cpj5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

/// A partida assistida do técnico humano `i` (79 bytes em [this] + 0x9ad4, 0xa6ef bytes por humano).
unsafe fn cpj5_assistida(this: usize, i: usize) -> usize {
    cpj5_d(this).wrapping_add(0x9ad4).wrapping_add(i.wrapping_mul(0xa6ef))
}

/// { 0x5fe8d0 } (thiscall, human_manager.cpp): `HUMAN_MANAGER::clear_invalid_watched_fixtures()`: para cada um dos 16
/// técnicos humanos (os últimos 16 de [0xae23bc]) ativo (+0x5f ≠ 0) que não pode mais ver a partida assistida dele
/// (0x5fe700), zera os 79 bytes dela. Devolve 1.
unsafe extern "thiscall" fn clear_invalid_watched_fixtures(this: usize) -> u32 {
    let pode_ver: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x5fe700usize);
    for i in 0..0x10usize {
        let p = cpj5_d(0xae23bc).wrapping_add(cpj5_d(0xae2364).wrapping_sub(0x10).wrapping_add(i).wrapping_mul(0x6e));
        if *((p + 0x5f) as *const u8) == 0 {
            continue;
        }
        let f = cpj5_assistida(this, i);
        if pode_ver(this, f, p) == 0 {
            core::ptr::write_bytes(f as *mut u8, 0, 79); // a partida sai
        }
    }
    1
}

/// Os pedidos de observação (0x11 bytes em [this+0x5c], [this+0x64] deles) do clube `id` (+0): as pessoas (+4).
unsafe fn cpj5_alvos(this: usize, id: u32) -> Vec<usize> {
    let mut v = Vec::new();
    let mut k = 0i32;
    while k < cpj5_d(this + 0x64) as i32 {
        let e = cpj5_d(this + 0x5c).wrapping_add(k as usize * 0x11);
        if cpj5_d(e) as u32 == id { // o pedido do clube
            v.push(cpj5_d(0xae23bc).wrapping_add(cpj5_d(e + 4).wrapping_mul(0x6e)));
        }
        k += 1;
    }
    v
}

/// { 0x7f9690 } (thiscall, ret 8, scout_manager.cpp): `SCOUT_MANAGER::collect_club_scout_targets(clube, lista)`: com
/// `lista`, esvazia-a (0x68d080) e acrescenta (0x68d0d0, 0x6e bytes) a pessoa de cada pedido de observação do clube
/// `*clube`. Devolve se há algum. Clube nulo: 0 (a lista fica vazia).
unsafe extern "thiscall" fn collect_club_scout_targets(this: usize, clube: *const u32, lista: usize) -> u32 {
    if lista != 0 {
        let esvaziar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d080usize);
        esvaziar(lista);
    }
    if clube.is_null() {
        return 0;
    }
    let alvos = cpj5_alvos(this, *clube);
    if lista != 0 {
        let acrescentar: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x68d0d0usize);
        for &p in &alvos {
            acrescentar(lista, p, 0x6e);
        }
    }
    (!alvos.is_empty()) as u32
}

/// { 0x7ae8a0 } (thiscall, ret 4, player_search.cpp): `PLAYER_SEARCH::set_scout(olheiro)`: se o olheiro muda (o atual
/// pela 0x7ae950), grava os ids dele em +0x174 (0x7e4920) ou, sem olheiro, o índice +0x180 = −1; e as missões de nação
/// (+0x18c), região (+0x194) e competição (+0x195) voltam a −1. Devolve 1.
unsafe extern "thiscall" fn set_scout(this: usize, olheiro: usize) -> u32 {
    if olheiro == get_scout(this) {
        return 1; // o mesmo olheiro
    }
    if olheiro != 0 {
        let gravar: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x7e4920usize);
        gravar(olheiro, this + 0x174);
    } else {
        core::ptr::write_unaligned((this + 0x180) as *mut u32, u32::MAX);
    }
    core::ptr::write_unaligned((this + 0x18c) as *mut u32, u32::MAX);
    *((this + 0x194) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x195) as *mut u32, u32::MAX);
    1
}

/// Comparador: lado a lado pelas 16 partidas assistidas (a 0x5fe700 só lê).
unsafe extern "thiscall" fn comparar_clear_invalid_watched_fixtures(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let reg: Vec<(usize, usize)> = (0..0x10).map(|i| (cpj5_assistida(this, i), 79)).collect();
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(this), || clear_invalid_watched_fixtures(this));
    PLACAR.registrar(igual && r == n, || format!("clear_invalid_watched_fixtures {:x?} x {:x?}", mo, mr));
    r
}

/// Comparador: os alvos da cópia antes; a original enche a lista; confere o valor e a lista (cada item, os 0x6e bytes
/// da pessoa).
unsafe extern "thiscall" fn comparar_collect_club_scout_targets(this: usize, clube: *const u32, lista: usize) -> u32 {
    let alvos = if clube.is_null() { Vec::new() } else { cpj5_alvos(this, *clube) };
    let r = orig::<unsafe extern "thiscall" fn(usize, *const u32, usize) -> u32>(%%I1%%)(this, clube, lista);
    let mut igual = r == (!alvos.is_empty()) as u32;
    if lista != 0 {
        let itens = cpd1_itens(lista);
        igual &= itens.len() == alvos.len()
            && itens.iter().zip(&alvos).all(|(&i, &p)| core::slice::from_raw_parts(i as *const u8, 0x6e) == core::slice::from_raw_parts(p as *const u8, 0x6e));
    }
    PLACAR.registrar(igual, || format!("collect_club_scout_targets original={} alvos {}", r, alvos.len()));
    r
}

/// Comparador: lado a lado pelo olheiro e as missões (+0x174 a +0x199; a 0x53e990 só lê).
unsafe extern "thiscall" fn comparar_set_scout(this: usize, olheiro: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize) -> u32>(%%I2%%);
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this + 0x174, 0x25)], || o(this, olheiro), || set_scout(this, olheiro));
    PLACAR.registrar(igual && r == n, || format!("set_scout({:x}) {:x?} x {:x?}", olheiro, mo, mr));
    r
}
