use super::*;

// The comparers of the routines in ../comp/eurocomp/conmebol_liber.rs (comparison mode: see the header of motor.rs).
// [pt] Os comparadores das rotinas de ../comp/eurocomp/conmebol_liber.rs (modo comparação: ver o cabeçalho do motor.rs).

/// Comparer for `conmebol_liber_update_for_date` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_liber_update_for_date(this: usize, hoje: usize, completo: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(IDX_CONMEBOL_LIBER_UPDATE_FOR_DATE);
    let sorteia = read_u32_as_usize(this + 0x45) == 0
        && completo != 0
        && (read_i16(this + 0xb8) as u16).wrapping_add(read_i16(this + 0x40) as u16) == core::ptr::read_unaligned((hoje + 2) as *const u16)
        && read_i16(this + 0xb6) as u16 == core::ptr::read_unaligned(hoje as *const u16);
    let r = o(this, hoje, completo);
    if sorteia {
        let fases = read_u32_as_usize(this + 0xc);
        let ok = read_u32_as_usize(this + 0x45) == 1
            && read_u32_as_usize(this + 0xee) == 0
            && read_u32_as_usize(this + 0xb1) != 0
            && fases != 0
            && (0..7).all(|g| read_u32_as_usize(fases + g * 4) != 0)
            && read_u32_as_usize(this + 0x30) == 6;
        PLACAR.registrar(ok, || format!("conmebol_liber_update_for_date({:#x}): o sorteio não montou os grupos (+0x45 {}, +0x30 {})", this, read_u32_as_usize(this + 0x45), read_u32_as_usize(this + 0x30) as i32));
    }
    r
}

/// Comparer for `build_first_group_table` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_build_first_group_table(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_BUILD_FIRST_GROUP_TABLE);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 {
        return o(this);
    }
    let previstos: [usize; 4] = core::array::from_fn(|i| read_u32_as_usize(lista + i * 4));
    let r = o(this);
    let t = read_u32_as_usize(this + 0xb1);
    let mut lidos = [(0usize, 0u8); 4];
    if t != 0 {
        for (i, l) in lidos.iter_mut().enumerate() {
            *l = (read_u32_as_usize(t + i * 0x3b), *((t + i * 0x3b + 4) as *const u8));
        }
    }
    let igual = r == 1 && read_i16(this + 0x3e) == 4 && t != 0 && (0..4).all(|i| lidos[i] == (previstos[i], i as u8));
    PLACAR.registrar(igual, || format!("build_first_group_table({:#x}) devolveu {} linhas {:x?} previstas {:x?}", this, r, lidos, previstos));
    r
}

/// Comparer for `conmebol_liber_create_group_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_liber_create_group_stage(this: usize, g: i32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, i32) -> u32>(IDX_CONMEBOL_LIBER_CREATE_GROUP_STAGE);
    let lista = read_u32_as_usize(this + 0xee);
    if lista == 0 || !(0..7).contains(&g) {
        return o(this, g);
    }
    let mut plano: Vec<usize> = (0..4).map(|k| read_u32_as_usize(lista + (g as usize * 4 + 4 + k) * 4)).collect();
    let r = o(this, g);
    if r == 1 {
        let f = read_u32_as_usize(read_u32_as_usize(this + 0xc) + g as usize * 4);
        let t = if f != 0 { read_u32_as_usize(f + 0xb1) } else { 0 };
        let mut a: Vec<usize> = if t != 0 { (0..4).map(|k| read_u32_as_usize(t + k * 0x3b)).collect() } else { Vec::new() };
        a.sort_unstable();
        plano.sort_unstable();
        let ok = f != 0 && read_u32_as_usize(this + 0x30) as i32 == g && read_i16(f + 0x3e) == 4 && read_i16(f + 0x40) == read_i16(this + 0x40) && a == plano;
        PLACAR.registrar(ok, || format!("conmebol_liber_create_group_stage({}): a fase não bate com o plano ({:x?} lidos {:x?})", g, plano, a));
    }
    r
}

/// Comparer for `conmebol_liber_set_up_rules` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_liber_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_LIBER_SET_UP_RULES);
    check_cup_rules("conmebol_liber_set_up_rules", this, &LIBERTADORES_RULES, || o(this))
}

/// Comparer for `conmebol_liber_select_qualified_clubs` (comparison mode: see the header of motor.rs).
///
/// [pt] Comparador da 0x4c3f40: lado a lado com as duas listas (a original aloca a dela, a cópia a sua): a original roda e a
/// lista dela fica; o estado do sorteador volta, a cópia roda sem contar e a lista dela é comparada com a da original e
/// liberada (0x9452ca: push; add esp, 4 -> cdecl). Com menos de 32 inscritos (as caixas): só a original.
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_liber_select_qualified_clubs(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(IDX_CONMEBOL_LIBER_SELECT_QUALIFIED_CLUBS);
    let mut v = [0usize; 32];
    if libertadores_entrants(this, 0x20, &mut v) != 0x20 {
        return o(this); // simplification: as caixas de erro: só a original
    }
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_CRT_FREE);
    let estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO);
    let (ee, f2) = (read_u32_as_usize(this + 0xee), read_u32_as_usize(this + 0xf2));
    let r = o(this);
    let (la, depois) = (read_u32_as_usize(this + 0xee), crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO));
    let a: [u32; 32] = if la != 0 { core::ptr::read_unaligned(la as *const [u32; 32]) } else { [0; 32] };
    estado.escrever();
    core::ptr::write_unaligned((this + 0xee) as *mut u32, ee as u32);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, f2 as u32);
    let rr = crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| conmebol_liber_select_qualified_clubs(this)));
    let lb = read_u32_as_usize(this + 0xee);
    let b: [u32; 32] = if lb != 0 { core::ptr::read_unaligned(lb as *const [u32; 32]) } else { [0; 32] };
    let igual_estado = crate::comparacao::Memoria::ler(&crate::aleatorio::REGIOES_DO_ESTADO) == depois;
    if lb != 0 && lb != la {
        soltar(lb);
    }
    core::ptr::write_unaligned((this + 0xee) as *mut u32, la as u32);
    core::ptr::write_unaligned((this + 0xf2) as *mut u32, 0x20);
    depois.escrever();
    PLACAR.registrar(r == rr && a == b && igual_estado, || format!("conmebol_liber_select_qualified_clubs({:#x}) devolveu {} x {} estado igual {} original {:x?} rust {:x?}", this, r, rr, igual_estado, a, b));
    r
}

/// Comparer for `conmebol_liber_update_staff_records_for_status` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_liber_update_staff_records_for_status(this: usize, clube: usize, desfecho: u32, fase: u32, a4: u32, jogo: usize, a6: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, usize, u32) -> u32>(IDX_CONMEBOL_LIBER_UPDATE_STAFF_RECORDS_FOR_STATUS);
    let titulo = fase as u8 == 7 && desfecho as u8 == 1;
    let mut vaga = 0usize;
    if titulo {
        let mundial = read_u32_as_usize(read_u32_as_usize(G_COMP_OBJECT_TABLE).wrapping_add(read_u32_as_usize(KEY_CLUB_COMP_WORLD_CLUB_CUP).wrapping_mul(4)));
        let t = read_u32_as_usize(mundial + 0xa7);
        let nacao = read_u32_as_usize(read_u32_as_usize(t) + 0x53);
        vaga = if nacao != 0 && read_u32_as_usize(read_u32_as_usize(nacao + nation::CONTINENT)) == read_u32_as_usize(KEY_CONTINENT_SOUTH_AMERICA) { t } else { t + 6 };
    }
    let r = o(this, clube, desfecho, fase, a4, jogo, a6);
    let igual = r == 0 && (!titulo || read_u32_as_usize(vaga) == clube);
    PLACAR.registrar(igual, || format!("conmebol_liber_update_staff_records_for_status({:#x}, {}, {}) devolveu {} titulo={} vaga {:#x}", clube, desfecho as u8, fase as u8, r, titulo, vaga));
    r
}

/// Comparer for `conmebol_liber_advance_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_liber_advance_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_LIBER_ADVANCE_STAGE);
    let f = read_u32_as_usize(this + 0x30) as i32;
    if f >= (read_u32_as_usize(this + 0x2c) as i32).wrapping_sub(1) {
        return o(this);
    }
    o(this);
    let depois = read_u32_as_usize(this + 0x30) as i32;
    PLACAR.registrar(depois == f + 1, || format!("conmebol_liber_advance_stage({:#x}) fase {} -> {}", this, f, depois));
}

/// Comparer for `conmebol_liber_create_schedule` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_liber_create_schedule(this: usize, fase: u32, rodadas: *mut i16, extra: *mut i16, liga: *mut i32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, *mut i16, *mut i16, *mut i32) -> usize>(IDX_CONMEBOL_LIBER_CREATE_SCHEDULE);
    compare_south_american_cup_schedule("conmebol_liber_create_schedule", fase, rodadas, extra, liga, 7, &[], || o(this, fase, rodadas, extra, liga),
        |r, e, l| conmebol_liber_create_schedule(this, fase, r, e, l))
}

/// Comparer for `conmebol_liber_create_knockout_stage` (comparison mode: see the header of motor.rs).
pub(crate) unsafe extern "thiscall" fn comparar_conmebol_liber_create_knockout_stage(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(IDX_CONMEBOL_LIBER_CREATE_KNOCKOUT_STAGE);
    con_merc_cup_compare_knockout_stage("conmebol_liber_create_knockout_stage", this, 0x10, 7, 7, || o(this))
}
