// ---------- %%LEVA%%a leva: o clube e o seu time B: os fisioterapeutas, se é time B e as pessoas fora do elenco (Database.cpp) ----------

/// O time B ligado ao `clube` (club_get_linked_b_team 0x540a50 pelo endereço do jogo, com `ativo` = 1): o registro
/// do parceiro ou 0; `eh_a` recebe 1 (o clube é o A) ou 0 (o clube é o B) quando há parceiro. Recusado, a 0x540a50
/// zera a marca +0x244 do clube: na comparação a original vem antes e a cópia acha a marca já zerada (também 0).
unsafe fn dbt1_parceiro(clube: usize, eh_a: *mut u32) -> usize {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    parceiro(clube, eh_a, 1)
}

/// { 0x53c9b0 } (cdecl, Database.cpp): `database_count_club_physios(clube)`: quantos dos 3 fisioterapeutas
/// (+0x1cf) do clube e do time B ligado (0x540a50) são pessoas válidas (não nulas, id +0 < [0xae2364] com sinal).
/// Só `al`. Clube nulo: a original (caixa e 0). Nas telas da pessoa (staff_screens, 0x8740c8 e 0x87412b).
unsafe extern "C" fn database_count_club_physios(clube: usize) -> u8 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(clube);
    }
    let validos = |c: usize| {
        (0..3).filter(|k| {
            let p = core::ptr::read_unaligned((c + 0x1cf + 4 * k) as *const usize);
            p != 0 && *(p as *const i32) < *(0xae2364 as *const i32)
        }).count() as u8
    };
    let b = dbt1_parceiro(clube, core::ptr::null_mut());
    validos(clube) + if b != 0 { validos(b) } else { 0 } // os do clube e os do time B (dbt)
}

unsafe extern "C" fn comparar_database_count_club_physios(clube: usize) -> u8 {
    let r = orig::<unsafe extern "C" fn(usize) -> u8>(%%I0%%)(clube);
    if clube == 0 {
        return r; // caixa: só a original
    }
    let n = database_count_club_physios(clube);
    PLACAR.registrar(r == n, || format!("database_count_club_physios({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x5448c0 } (cdecl, Database.cpp): `club_is_b_team(clube)`: 1 se o clube tem parceiro (0x540a50 com `ativo`) e
/// é o B dele (`eh_a` = 0), senão 0 (o eax inteiro). A local `eh_a` só é lida com parceiro, e então a 0x540a50 a
/// escreveu. Clube nulo: a original (caixa e 0). Na tela da pessoa (staff_screens, 0x87272d).
unsafe extern "C" fn club_is_b_team(clube: usize) -> u32 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(clube);
    }
    let mut eh_a = 0u32;
    (dbt1_parceiro(clube, &mut eh_a) != 0 && eh_a == 0) as u32 // é o B (dbt)
}

unsafe extern "C" fn comparar_club_is_b_team(clube: usize) -> u32 {
    let r = orig::<unsafe extern "C" fn(usize) -> u32>(%%I1%%)(clube);
    if clube == 0 {
        return r;
    }
    let n = club_is_b_team(clube);
    PLACAR.registrar(r == n, || format!("club_is_b_team({:x}) original={} rust={}", clube, r, n));
    r
}

/// { 0x544c70 } (cdecl, Database.cpp): `club_collect_non_playing_staff(clube, saida, max)`: põe em `saida` as
/// pessoas fora do elenco do clube e depois as do time B ligado (0x540a50 com `ativo`), na ordem dos cargos: os 6
/// de +0xbf (técnico, auxiliar, ...) e os 15 de +0x19f (treinadores, olheiros, fisioterapeutas), as não nulas, até
/// `max` (byte com sinal); devolve quantas em `al`. Clube nulo: a original (caixa e 0). Na aba de informações da
/// tela do clube (club_screen_draw_info_tab 0x460590, 0x4616fa).
unsafe extern "C" fn club_collect_non_playing_staff(clube: usize, saida: *mut usize, max: u32) -> u8 {
    if clube == 0 {
        return orig::<unsafe extern "C" fn(usize, *mut usize, u32) -> u8>(%%I2%%)(clube, saida, max);
    }
    let max = max as i8;
    let mut n = 0i8;
    let b = dbt1_parceiro(clube, core::ptr::null_mut());
    for c in [clube, b] {
        if c == 0 {
            continue;
        }
        for d in (0..6).map(|k| 0xbf + 4 * k).chain((0..15).map(|k| 0x19f + 4 * k)) {
            let p = core::ptr::read_unaligned((c + d) as *const usize);
            if p != 0 && n < max {
                *saida.add(n as usize) = p; // (dbt)
                n += 1;
            }
        }
    }
    n as u8
}

unsafe extern "C" fn comparar_club_collect_non_playing_staff(clube: usize, saida: *mut usize, max: u32) -> u8 {
    let o = orig::<unsafe extern "C" fn(usize, *mut usize, u32) -> u8>(%%I2%%);
    if clube == 0 {
        return o(clube, saida, max);
    }
    let reg = [(saida as usize, 4 * (max as i8).max(0) as usize)];
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&reg, || o(clube, saida, max), || club_collect_non_playing_staff(clube, saida, max));
    PLACAR.registrar(igual && r == n, || format!("club_collect_non_playing_staff({:x}, {:x}, {}) original={} rust={} {:x?} x {:x?}", clube, saida as usize, max as i8, r, n, mo, mr));
    r
}
