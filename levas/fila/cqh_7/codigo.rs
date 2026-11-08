// ---------- %%LEVA%%a leva: a ordem dos árbitros por nação e qualidade, nos dois sentidos (officials_manager.cpp) ----------

unsafe fn cqh7_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cqh7_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A qualidade do árbitro na ordem: o byte +0x28 × 50 + a palavra +0x22, em 16 bits.
unsafe fn official_quality(r: usize) -> i16 {
    (*((r + 0x28) as *const i8) as i16).wrapping_mul(0x32).wrapping_add(cqh7_w(r + 0x22)) // cqh7: o peso da qualidade
}

/// O id da nação do árbitro ([+0x16] + 0).
unsafe fn official_nation_id(r: usize) -> i32 {
    cqh7_i(cqh7_i(r + 0x16) as u32 as usize)
}

/// { 0x79d4e0 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_desc(a, b)`, a ordem do
/// qsort da 0x79bf60 (os que ficam por nação): pela nação (id) e, na mesma, pela qualidade da maior para a menor; empate:
/// o índice +0 de `b` menos o de `a`.
unsafe extern "C" fn officials_manager_compare_nation_then_quality_desc(a: usize, b: usize) -> i32 {
    let (na, nb) = (official_nation_id(a), official_nation_id(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (official_quality(a), official_quality(b));
    crate::comparacao::ramo("cqh_7 ordem decrescente");
    if ka == kb {
        return cqh7_i(b).wrapping_sub(cqh7_i(a)); // cqh7: o empate pelo índice
    }
    kb as i32 - ka as i32
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_officials_manager_compare_nation_then_quality_desc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I0%%)(a, b);
    let n = officials_manager_compare_nation_then_quality_desc(a, b);
    PLACAR.registrar(r == n, || format!("officials_manager_compare_nation_then_quality_desc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}

/// { 0x79d480 } (cdecl, officials_manager.cpp): `officials_manager_compare_nation_then_quality_asc(a, b)`, a ordem do
/// qsort da 0x79c5c0 (depois dos árbitros novos): pela nação e pela qualidade da menor para a maior; empate: o índice +0
/// de `a` menos o de `b`.
unsafe extern "C" fn officials_manager_compare_nation_then_quality_asc(a: usize, b: usize) -> i32 {
    let (na, nb) = (official_nation_id(a), official_nation_id(b));
    if na != nb {
        return na.wrapping_sub(nb);
    }
    let (ka, kb) = (official_quality(a), official_quality(b));
    crate::comparacao::ramo("cqh_7 ordem crescente");
    if ka == kb {
        return cqh7_i(a).wrapping_sub(cqh7_i(b));
    }
    ka as i32 - kb as i32 // cqh7: a ordem crescente
}

/// Comparador: a devolução (só lê).
unsafe extern "C" fn comparar_officials_manager_compare_nation_then_quality_asc(a: usize, b: usize) -> i32 {
    let r = original_fn::<unsafe extern "C" fn(usize, usize) -> i32>(%%I1%%)(a, b);
    let n = officials_manager_compare_nation_then_quality_asc(a, b);
    PLACAR.registrar(r == n, || format!("officials_manager_compare_nation_then_quality_asc({:x}, {:x}) original={} rust={}", a, b, r, n));
    r
}
