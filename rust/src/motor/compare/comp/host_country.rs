use super::*;

// The comparers of the routines in ../comp/host_country.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/host_country.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `init_default_table` (comparison mode: see the header of motor.rs).
///
/// [pt] Lado a lado (a cópia sem contar os sorteios) pela tabela (conta +4 × 0x22 bytes), pelo estado do sorteador e por
/// [0xb67a34].
pub(crate) unsafe extern "thiscall" fn comparar_init_default_table(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_INIT_DEFAULT_TABLE);
    let n = read_u16(this + 4) as i16;
    if read_u32_as_usize(this) == 0 || n <= 0 {
        return o(this);
    }
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    reg.extend([(read_u32_as_usize(this), n as usize * 0x22), (0xb67a34, 4)]);
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this), || crate::aleatorio::sem_contar(|| init_default_table(this)));
    PLACAR.registrar(igual, || format!("init_default_table: {:x?} x {:x?}", mo, mr).chars().take(4000).collect());
}

/// Comparer for `post_host_news` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_post_host_news(this: usize, list: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_POST_HOST_NEWS)(this, list)
}

/// Comparer for `load_temp_file` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_load_temp_file(this: usize) -> u32 {
    original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_LOAD_TEMP_FILE)(this)
}

/// Comparer for `find_hosts` (comparison mode: see the header of motor.rs).
///
/// [pt] A devolução (o al) e as duas saídas (a rotina só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_hosts(this: usize, comp: u32, year: u32, a: *mut u32, b: *mut u32, default: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, *mut u32, *mut u32, u32) -> u32>(IDX_FIND_HOSTS);
    let (va, vb) = (*a, *b);
    let r = o(this, comp, year, a, b, default);
    let (oa, ob) = (*a, *b);
    let (mut ca, mut cb) = (va, vb);
    let c = find_hosts(this, comp, year, &mut ca, &mut cb, default);
    PLACAR.registrar(r as u8 == c as u8 && oa == ca && ob == cb, || format!("find_hosts({:x}, {}): original {} {:x} {:x} rust {} {:x} {:x}", comp, year as u16, r as u8, oa, ob, c as u8, ca, cb));
    r
}

/// Comparer for `find_next_hosts` (comparison mode: see the header of motor.rs).
///
/// [pt] A devolução (o al) e as duas saídas (a rotina só lê).
pub(crate) unsafe extern "thiscall" fn comparar_find_next_hosts(this: usize, comp: u32, a: *mut u32, b: *mut u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut u32, *mut u32) -> u32>(IDX_FIND_NEXT_HOSTS);
    let (va, vb) = (*a, *b);
    let r = o(this, comp, a, b);
    let (oa, ob) = (*a, *b);
    let (mut ca, mut cb) = (va, vb);
    let c = find_next_hosts(this, comp, &mut ca, &mut cb);
    PLACAR.registrar(r as u8 == c as u8 && oa == ca && ob == cb, || format!("find_next_hosts({:x}): original {} {:x} {:x} rust {} {:x} {:x}", comp, r as u8, oa, ob, c as u8, ca, cb));
    r
}

/// Comparer for `host_country_daily_update` (comparison mode: see the header of motor.rs).
///
/// [pt] Só a original (sorteia, monta a lista local e entrega notícias); a cópia leva os contadores.
pub(crate) unsafe extern "thiscall" fn comparar_host_country_daily_update(this: usize, today: usize) {
    // simplification: entrega notícias pela 0x5faec0 e mexe na lista: não dá para rodar duas vezes
    original_fn::<unsafe extern "thiscall" fn(usize, usize)>(IDX_HOST_COUNTRY_DAILY_UPDATE)(this, today)
}

/// Comparer for `save_temp_file` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x5fb320: só a original.
pub(crate) unsafe extern "thiscall" fn comparar_save_temp_file(this: usize) {
    // simplification: grava um arquivo em CM3_TEMP (não pode rodar duas vezes); a cópia é provada pela prova do save
    original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_SAVE_TEMP_FILE)(this)
}
