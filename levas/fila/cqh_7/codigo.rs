// ---------- %%LEVA%%a leva: a ordem dos árbitros (por nação e qualidade, nos dois sentidos), a ordem das vagas pelo nível da competição e a cópia do registro do árbitro (officials_manager.cpp) ----------

unsafe fn cqh7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqh7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A qualidade do árbitro na ordem: o byte +0x28 × 50 + a palavra +0x22, em 16 bits.
unsafe fn cqh7_qualidade(r: usize) -> i16 {
    (*((r + 0x28) as *const i8) as i16).wrapping_mul(0x32).wrapping_add(cqh7_w(r + 0x22)) // cqh7: o peso da qualidade
}

/// O id da nação do árbitro ([+0x16] + 0).
unsafe fn cqh7_nacao(r: usize) -> i32 {
    cqh7_i(cqh7_i(r + 0x16) as u32 as usize)
}

/// { 0x79d4e0 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_desc(a, b)`, a ordem do
/// qsort da 0x79bf60 (os que ficam por nação): pela nação (id) e, na mesma, pela qualidade da maior para a menor; empate:
/// o índice +0 de `b` menos o de `a`.
unsafe extern "C" fn ordem_dos_arbitros_decrescente(a: usize, b: usize) -> i32 {
    let (na, nb) = (cqh7_nacao(a), cqh7_nacao(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (cqh7_qualidade(a), cqh7_qualidade(b));
    crate::comparacao::ramo("cqh_7 ordem decrescente");
    if ka == kb {
        return cqh7_i(b).wrapping_sub(cqh7_i(a)); // cqh7: o empate pelo índice
    }
    kb as i32 - ka as i32
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_dos_arbitros_decrescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = ordem_dos_arbitros_decrescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_arbitros_decrescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x79d480 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_asc(a, b)`, a ordem do
/// qsort da 0x79c5c0 (depois dos árbitros novos): pela nação e pela qualidade da menor para a maior; empate: o índice +0
/// de `a` menos o de `b`.
unsafe extern "C" fn ordem_dos_arbitros_crescente(a: usize, b: usize) -> i32 {
    let (na, nb) = (cqh7_nacao(a), cqh7_nacao(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (cqh7_qualidade(a), cqh7_qualidade(b));
    crate::comparacao::ramo("cqh_7 ordem crescente");
    if ka == kb {
        return cqh7_i(a).wrapping_sub(cqh7_i(b));
    }
    ka as i32 - kb as i32 // cqh7: a ordem crescente
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_dos_arbitros_crescente(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = ordem_dos_arbitros_crescente(a, b);
    PLACAR.registrar(r == n, || format!("ordem_dos_arbitros_crescente({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x79d410 } (cdecl, officials_manager.cpp): `officials_manager_compare_comp_level(a, b)`, a ordem das vagas de
/// competição de uma nação (qsort da 0x79bf60, entradas de 16 bytes com a competição em +0xc): competição de `a` fora de
/// 0..[0xae2378]: 1; a de `b` fora: −1; pelo nível (+0x69, palavra, da competição de 0x6b bytes em [0xae23d0]); mesmo
/// nível: a diferença dos ids.
unsafe extern "C" fn ordem_das_vagas_de_competicao(a: usize, b: usize) -> i32 {
    let (ca, cb) = (cqh7_i(a + 0xc), cqh7_i(b + 0xc));
    let max = cqh7_i(0xae2378);
    if ca < 0 || ca > max {
        return 1;
    }
    if cb < 0 || cb > max {
        return -1;
    }
    let base = cqh7_i(0xae23d0) as u32 as usize;
    let sa = cqh7_w(base.wrapping_add((ca as isize * 0x6b + 0x69) as usize));
    let sb = cqh7_w(base.wrapping_add((cb as isize * 0x6b + 0x69) as usize));
    if sa == sb {
        return ca.wrapping_sub(cb);
    }
    sa as i32 - sb as i32
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_ordem_das_vagas_de_competicao(a: usize, b: usize) -> i32 {
    let r = orig::<unsafe extern "C" fn(usize, usize) -> i32>(%%I2%%)(a, b);
    let n = ordem_das_vagas_de_competicao(a, b);
    PLACAR.registrar(r == n, || format!("ordem_das_vagas_de_competicao({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x79c530 } (thiscall, ret 4, officials_manager.cpp): `Official::operator=(origem)`: copia o registro de 0x2b bytes
/// campo a campo (o nascimento em +0xc pela atribuição da data 0x417c10) e devolve o destino.
unsafe extern "thiscall" fn copiar_arbitro(this: usize, o: usize) -> usize {
    core::ptr::copy_nonoverlapping(o as *const u8, this as *mut u8, 0xc);
    let mut tmp = [0u8; 8];
    let copia: unsafe extern "thiscall" fn(usize, usize, usize) -> usize = core::mem::transmute(0x417c10usize);
    copia(this + 0xc, tmp.as_mut_ptr() as usize, o + 0xc);
    core::ptr::copy_nonoverlapping((o + 0x14) as *const u8, (this + 0x14) as *mut u8, 0x2b - 0x14);
    this
}

/// Comparador: lado a lado no destino (0x2b bytes) e na devolução.
unsafe extern "thiscall" fn comparar_copiar_arbitro(this: usize, o: usize) -> usize {
    let f = orig::<unsafe extern "thiscall" fn(usize, usize) -> usize>(%%I3%%);
    if this == 0 || o == 0 {
        return f(this, o);
    }
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(&[(this, 0x2b)], || f(this, o), || copiar_arbitro(this, o));
    PLACAR.registrar(igual && r == n, || format!("copiar_arbitro({:x} <- {:x}) {:x?} x {:x?}", this, o, mo.0, mr.0));
    r
}
