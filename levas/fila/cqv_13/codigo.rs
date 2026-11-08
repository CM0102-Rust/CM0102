// ---------- %%LEVA%%a leva: as posições previstas e o sorteio dos grupos do Campeonato de Clubes da Oceania; as posições previstas da Copa Africana de Nações e da eliminatória da Oceania (oceania_club_champ.cpp, african_nations.cpp, wc_oceania_league.cpp) ----------

unsafe fn cqv13_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqv13_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// As posições na ordem do vetor ordenado pela reputação (Comp_Stats::set_club_final_position 0x4a2540: push posição;
/// push clube; mov ecx, [+8] -> thiscall, ret 8).
unsafe fn cqv13_gravar(this: usize, v: usize, posicoes: &[u32]) {
    let gravar: unsafe extern "thiscall" fn(usize, usize, u32) -> usize = core::mem::transmute(0x4a2540usize);
    for (k, &p) in posicoes.iter().enumerate() {
        gravar(cqv13_d(this + 8), cqv13_d(v + k * 4), p);
    }
}

/// qsort 0x945333(v, n, 4, 0x4b07d0) (4 pushes; add esp -> cdecl): a reputação (+0x80) decrescente, nulos no fim.
unsafe fn cqv13_ordenar(v: usize, n: u32) {
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(v, n, 4, 0x4b07d0);
}

const CQV13_OCEANIA_CLUBES: [u32; 8] = [1, 2, 3, 3, 5, 5, 7, 7];

/// { 0x798640 } (thiscall, ret 0, oceania_club_champ.cpp; a virtual +0x5c): `OceaniaClubChamp::set_predicted_
/// positions()`: sem estatísticas (+8), a caixa da linha 0x3a0; senão os 8 escolhidos (+0xee, +0xf2 clubes) ordenados
/// pela reputação, as posições 1, 2, 3, 3, 5, 5, 7, 7 e a lista embaralhada (0x4afb40(lista, n): push n (word); push
/// lista; add esp, 8 -> cdecl: 3n trocas sorteadas). É esse embaralhamento que faz os grupos: os 4 primeiros vão ao
/// grupo A, os outros ao B.
unsafe extern "thiscall" fn cqv13_oceania_clubes_previstas(this: usize) {
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    if cqv13_d(this + 8) == 0 {
        return show_error_at(0xa6a068, 0x3a0);
    }
    let lista = cqv13_d(this + 0xee);
    cqv13_ordenar(lista, cqv13_d(this + 0xf2) as u32); // cqv13: os 8 pela reputação
    cqv13_gravar(this, cqv13_d(this + 0xee), &CQV13_OCEANIA_CLUBES);
    crate::comparison::branch("cqv_13 sorteio dos grupos da oceania");
    let n = cqv13_w(this + 0xf2) as u16 as u32;
    embaralhar(cqv13_d(this + 0xee), n); // cqv13: os 8 embaralhados
}

/// O plano das previstas: os clubes `clubes` (antes da original), cada um com a posição gravada (+0x72 do registro,
/// 0x4a4850) depois dela; pela reputação (+0x80) decrescente as posições formam `posicoes` (empates de reputação podem
/// trocar entre si: o qsort não é estável). Só com tantos clubes quantas posições e nenhum nulo.
unsafe fn cqv13_conferir(nome: &str, this: usize, clubes: &[usize], posicoes: &[u32]) {
    let est = cqv13_d(this + 8);
    if est == 0 || clubes.len() != posicoes.len() || clubes.iter().any(|&c| c == 0) {
        return;
    }
    let buscar: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4a4850usize);
    let mut v: Vec<(i16, u8)> = clubes
        .iter()
        .map(|&c| {
            let e = buscar(est, c);
            (cqv13_w(c + 0x80), if e != 0 { *((e + 0x72) as *const u8) } else { 0 })
        })
        .collect();
    v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
    let lidas: Vec<u32> = v.iter().map(|x| x.1 as u32).collect();
    SCOREBOARD.record(lidas == posicoes, || format!("{}({:#x}): posições pela reputação {:?}", nome, this, lidas));
}

// ponytail: Comp_Stats::set_club_final_position acrescenta registros e o embaralhamento sorteia: não roda duas vezes.
// Só a original; o plano: as posições pela reputação e a lista depois dela uma permutação da de antes.
unsafe extern "thiscall" fn comparar_cqv13_oceania_clubes_previstas(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let lista = cqv13_d(this + 0xee);
    let n = cqv13_w(this + 0xf2);
    if lista == 0 || !(0..=64).contains(&n) {
        return o(this);
    }
    let mut antes: Vec<usize> = (0..n as usize).map(|k| cqv13_d(lista + k * 4)).collect();
    o(this);
    let mut depois: Vec<usize> = (0..n as usize).map(|k| cqv13_d(cqv13_d(this + 0xee) + k * 4)).collect();
    cqv13_conferir("cqv13_oceania_clubes_previstas", this, &antes, &CQV13_OCEANIA_CLUBES);
    antes.sort_unstable();
    depois.sort_unstable();
    SCOREBOARD.record(antes == depois, || format!("cqv13_oceania_clubes_previstas({:#x}): a lista embaralhada não é a mesma", this));
}

/// O corpo das previstas com cópia (0x4027d0 e 0x91ff30): sem estatísticas (+8), a caixa `linhas.0`; uma cópia dos `n`
/// ponteiros da lista (0x944e46(4n): push; add esp, 4 -> cdecl; sem memória, a caixa `linhas.1`), ordenada pela
/// reputação; as posições pela ordem; a cópia liberada (0x9452ca).
unsafe fn cqv13_previstas_da_copia(this: usize, lista: usize, n: i32, posicoes: &[u32], arquivo: usize, linhas: (u32, u32)) {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqv13_d(this + 8) == 0 {
        return show_error_at(arquivo, linhas.0);
    }
    let v = alocar((n as u32).wrapping_mul(4));
    if v == 0 {
        return show_error_at(arquivo, linhas.1);
    }
    core::ptr::copy_nonoverlapping(lista as *const u8, v as *mut u8, (n as u32).wrapping_mul(4) as usize);
    cqv13_ordenar(v, n as u32);
    cqv13_gravar(this, v, posicoes);
    soltar(v);
}

const CQV13_AFRICANA: [u32; 16] = [1, 2, 3, 4, 5, 5, 5, 5, 9, 9, 9, 9, 13, 13, 13, 13];

/// { 0x4027d0 } (thiscall, ret 0, african_nations.cpp; a virtual +0x5c): `AfricanNations::set_predicted_positions()`:
/// as 16 finalistas da Copa Africana de Nações (+0xee, +0xf2 word) pela reputação: 1, 2, 3, 4, quatro 5, quatro 9 e
/// quatro 13 (caixas nas linhas 0x3f4 e 0x3fc).
unsafe extern "thiscall" fn cqv13_africana_previstas(this: usize) {
    cqv13_previstas_da_copia(this, cqv13_d(this + 0xee), cqv13_w(this + 0xf2) as i32, &CQV13_AFRICANA, 0x9870f8, (0x3f4, 0x3fc))
}

// ponytail: Comp_Stats::set_club_final_position acrescenta registros: não roda duas vezes. Só a original; o plano: as
// posições pela reputação.
unsafe extern "thiscall" fn comparar_cqv13_africana_previstas(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let (lista, n) = (cqv13_d(this + 0xee), cqv13_w(this + 0xf2));
    let clubes: Vec<usize> = if lista != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| cqv13_d(lista + k * 4)).collect() } else { Vec::new() };
    o(this);
    cqv13_conferir("cqv13_africana_previstas", this, &clubes, &CQV13_AFRICANA);
}

const CQV13_ELIMINATORIA: [u32; 10] = [1, 2, 3, 3, 5, 5, 7, 7, 9, 10];

/// { 0x91ff30 } (thiscall, ret 0, wc_oceania_league.cpp; a virtual +0x5c): `WcOceaniaLeague::set_predicted_positions()`:
/// as seleções da eliminatória da Oceania (+0xef, +0xf3 dword) pela força (a reputação +0x80 dos clubes das seleções):
/// 1, 2, 3, 3, 5, 5, 7, 7, 9 e, se forem 10, a décima 10 (caixas nas linhas 0x52a e 0x532).
unsafe extern "thiscall" fn cqv13_eliminatoria_oceania_previstas(this: usize) {
    let n = cqv13_d(this + 0xf3) as i32;
    let k = if n == 10 { 10 } else { 9 };
    cqv13_previstas_da_copia(this, cqv13_d(this + 0xef), n, &CQV13_ELIMINATORIA[..k], 0xad4afc, (0x52a, 0x532))
}

unsafe extern "thiscall" fn comparar_cqv13_eliminatoria_oceania_previstas(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let (lista, n) = (cqv13_d(this + 0xef), cqv13_d(this + 0xf3) as i32);
    let clubes: Vec<usize> = if lista != 0 && (0..=64).contains(&n) { (0..n as usize).map(|k| cqv13_d(lista + k * 4)).collect() } else { Vec::new() };
    o(this);
    let k = if n == 10 { 10 } else { 9 };
    cqv13_conferir("cqv13_eliminatoria_oceania_previstas", this, &clubes, &CQV13_ELIMINATORIA[..k]);
}
