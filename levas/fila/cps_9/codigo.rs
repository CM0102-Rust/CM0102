// ---------- %%LEVA%%a leva: os gols na carreira do jogador, o registro do empréstimo e a ordem da fila das voltas aos treinos (player_stats.cpp, staff_records.cpp, injury.cpp) ----------

unsafe fn cps9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cps9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

/// { 0x7c3d20 } (stdcall, ret 4, player_stats.cpp; a original recebe o PlayerStats no ecx e não o lê):
/// `player_stats_get_career_goals(pessoa)` (pela MATCH_PLAYER::setup 0x6eaea0, que grava em +0x43 do jogador da
/// partida): pessoa nula, a caixa "Error" (linha 0xb4b) e 0; id a partir de [0xae2364], 0; sem a ficha de jogador
/// (+0x61), a caixa (linha 0xb5d) e 0. Senão a tabela da carreira (0x7c3e40 do [0xb74350]: entradas de 0x2b bytes,
/// seis por temporada e clube) e a soma, em 16 bits, do byte +8 da sexta entrada de cada grupo (+0xdf); a tabela é
/// liberada (0x9452ca, também quando nula). Quente (cada jogador de cada partida): sem contador.
unsafe extern "stdcall" fn player_stats_get_career_goals(pessoa: usize) -> u16 {
    if pessoa == 0 {
        show_error_at(0xa78310, 0xb4b);
        return 0;
    }
    if cps9_i(pessoa) >= cps9_i(0xae2364) {
        return 0;
    }
    if cps9_d(pessoa + 0x61) == 0 {
        show_error_at(0xa78310, 0xb5d);
        return 0;
    }
    let montar: unsafe extern "thiscall" fn(usize, usize, *mut i32) -> usize = core::mem::transmute(0x7c3e40usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let mut n = 0i32;
    let t = montar(cps9_d(0xb74350), pessoa, &mut n);
    let mut s = 0u16;
    if t != 0 {
        let mut i = 0i32; // cps9: do primeiro grupo
        while i < n {
            s = s.wrapping_add(*((t + 0xdf + (i / 6) as usize * 0x102) as *const u8) as u16);
            i += 6;
        }
    }
    liberar(t);
    s // cps9: os gols na carreira
}

/// Comparador: a devolução (a tabela é montada e liberada pelas duas), sem os casos de erro (só a original).
unsafe extern "stdcall" fn compare_player_stats_get_career_goals(pessoa: usize) -> u16 {
    let o = original_fn::<unsafe extern "stdcall" fn(usize) -> u16>(%%I0%%);
    if pessoa == 0 || (cps9_i(pessoa) < cps9_i(0xae2364) && cps9_d(pessoa + 0x61) == 0) {
        return o(pessoa);
    }
    let r = o(pessoa);
    let n = player_stats_get_career_goals(pessoa);
    SCOREBOARD.record(r == n, || format!("player_stats_get_career_goals(pessoa {:x}) original={} rust={}", pessoa, r, n));
    r
}

/// { 0x86da20 } (thiscall, ret 0xc, staff_records.cpp, SEH só da data local, de destrutor vazio):
/// `Staff_Records::add_loan_records(pessoa, de, clube)` (pela 0x8c2180, a transferência aceita): com o clube e o técnico
/// dele (+0xcf), o histórico do técnico (tipo 0x69: o clube, a pessoa e os três ids +0x33 de +4, +8 e +0xc dela, e o id
/// de `de`); depois, na mesma entrada, o da pessoa (tipo 0x6a: o clube e o id de `de`), no arquivo (0x58e340). O clube
/// é lido antes do teste da pessoa, como na original; pessoa nula: a caixa "Error" (linha 0x1010).
unsafe extern "thiscall" fn add_loan_records(this: usize, pessoa: usize, de: usize, clube: usize) {
    crate::comparison::branch("cps_9 emprestimo");
    let mut e = history_entry();
    if clube != 0 && cps9_d(clube + 0xcf) != 0 {
        let tecnico = cps9_d(clube + 0xcf);
        stamp_game_date(e.as_mut_ptr());
        e[0xc] = 0x69;
        put_i32(&mut e, 8, cps9_i(clube));
        put_person_keys(&mut e, 0xd, pessoa);
        put_i32(&mut e, 0x1d, cps9_i(de));
        append_person_history_by_id(true, this, cps9_d(tecnico), &e);
    }
    stamp_game_date(e.as_mut_ptr());
    e[0xc] = 0x6a;
    put_i32(&mut e, 8, cps9_i(clube));
    put_i32(&mut e, 0xd, cps9_i(de));
    if pessoa != 0 {
        append_person_history_by_id(true, this, cps9_d(pessoa), &e);
    } else {
        show_error_at(0xa83b78, 0x1010);
    }
}

// ponytail: o histórico vai para o arquivo (não roda duas vezes) e a cópia não grava mais nada: só a original; a
// cópia leva o ramo.
unsafe extern "thiscall" fn compare_add_loan_records(this: usize, pessoa: usize, de: usize, clube: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, usize)>(%%I1%%)(this, pessoa, de, clube)
}

/// { 0x632040 } (cdecl, injury.cpp): `injury_compare_entry_dates(chave, item)`, a ordem do bsearch (0x946464) da fila
/// das voltas aos treinos (entradas de 0x16 bytes) em Injury::post_due_recovery_news (0x62e010, todo dia): a data em
/// +8 da chave antes da do item (0x456770), −1; depois (0x456740), 1; igual, 0.
unsafe extern "C" fn injury_compare_entry_dates(chave: usize, item: usize) -> i32 {
    let antes: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    let depois: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    if antes(chave + 8, cps9_d(item + 8) as u32, cps9_d(item + 0xc) as u32) != 0 {
        return -1;
    }
    if depois(chave + 8, cps9_d(item + 8) as u32, cps9_d(item + 0xc) as u32) != 0 {
        return 1;
    }
    crate::comparison::branch("cps_9 data igual");
    0 // cps9: a volta de hoje achada
}

unsafe extern "C" fn compare_injury_compare_entry_dates(chave: usize, item: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(chave, item);
    let n = injury_compare_entry_dates(chave, item);
    SCOREBOARD.record(r == n, || format!("injury_compare_entry_dates(chave {:x}, item {:x}) original={} rust={}", chave, item, r, n));
    r
}
