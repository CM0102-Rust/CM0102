// ---------- %%LEVA%%a leva: o adversário do amistoso e o da excursão (friendly.cpp) ----------

unsafe fn cpe1_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpe1_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpe1_b(a: usize) -> i8 {
    *(a as *const i8)
}

/// O clube de id `id` na tabela [0xae23b4] (registros de 0x245 bytes).
unsafe fn cpe1_clube(id: usize) -> usize {
    cpe1_d(0xae23b4).wrapping_add(id.wrapping_mul(0x245))
}

/// O candidato `c` já tem o candidato `clube` (pelo id, +0) na lista de adversários (+0xa)? Volta o cursor da lista
/// ao começo (+0xa = +2) e percorre com a 0x68d1b0, como as duas originais.
unsafe fn cpe1_na_lista(c: usize, clube: usize) -> bool {
    let l = cpe1_d(c + 0xa);
    if l == 0 {
        return false;
    }
    core::ptr::write_unaligned((l + 0xa) as *mut usize, cpe1_d(l + 2));
    let proximo: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x68d1b0usize);
    loop {
        let x = proximo(cpe1_d(c + 0xa));
        if x == 0 {
            return false;
        }
        if cpe1_d(x) == cpe1_d(clube) {
            return true;
        }
    }
}

/// Fora pela data: no ano do amistoso ([0xae2c90], com sinal, contra +0x34e8 do Friendly, sem sinal) o clube `op`
/// com +0x1db igual ao float de [0x9cf6f8] (comparado como inteiro).
unsafe fn cpe1_fora_pela_data(this: usize, op: usize) -> bool {
    *(0xae2c90 as *const i16) as i32 == core::ptr::read_unaligned((this + 0x34e8) as *const u16) as i32
        && cpe1_d(op + 0x1db) as u32 == *(0x9cf6f8 as *const u32)
}

/// Os clubes são rivais (+0xb3, +0xb7, +0xbb, nos dois sentidos)?
unsafe fn cpe1_rivais(a: usize, b: usize) -> bool {
    [0xb3usize, 0xb7, 0xbb].iter().any(|&o| a == cpe1_d(b + o) || b == cpe1_d(a + o))
}

/// As nações (+0x53) dos clubes são rivais (+0x110, +0x114, +0x118, nos dois sentidos)?
unsafe fn cpe1_nacoes_rivais(a: usize, b: usize) -> bool {
    let (na, nb) = (cpe1_d(a + 0x53), cpe1_d(b + 0x53));
    [0x110usize, 0x114, 0x118].iter().any(|&o| na == cpe1_d(nb + o) || nb == cpe1_d(na + o))
}

unsafe fn cpe1_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// { 0x5c4cb0 } (thiscall, ret 0x14, friendly.cpp): `Friendly::find_friendly_opponent(clube, lista, n, faixa,
/// excluir)`: a faixa (byte com sinal) presa em 1..=20; busca (bsearch 0x946464 com 0x5c5270, pelo byte +5) na lista
/// de candidatos (0x16 bytes, `n`) um da faixa, subindo a faixa até achar (a partir de 20: o começo da lista). Dali
/// até o penúltimo candidato (o último fica de fora, como na original), pula: o próprio clube, os do vetor `excluir`
/// (4 ponteiros, pelo id), os que já têm o clube na lista (+0xa, o cursor volta ao começo), os fora pela data, os de
/// faixas cruzadas (+0xe/+0x10, −1 = qualquer), os rivais, os de nações rivais (meio a meio: sorteio(2) = 0), os da
/// mesma cidade (+0x57) e os acima da faixa do sorteio (+4 > sorteio(3) + 3). Pela reputação (+0x80) do clube: abaixo
/// de 2000 a mesma nação (+0x53); de 2000 a 3999 o mesmo +0x75 da nação; de 4000 a 7999 o mesmo +0x71; com o +4 do
/// clube acima de sorteio(2), a mesma nação. Fica com o candidato quando sorteio(5) = 1; nenhum: 0. Sem SEH, sem caixa.
unsafe extern "thiscall" fn adversario_do_amistoso(this: usize, clube: usize, lista: usize, n: u32, faixa: u32, excluir: usize) -> usize {
    let mut faixa = faixa as i8;
    if faixa < 1 {
        faixa = 1;
    } else if faixa > 0x14 {
        faixa = 0x14;
    }
    let cp = cpe1_clube(cpe1_d(clube));
    let mut chave = [0u8; 0x16];
    chave[5] = faixa as u8;
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let mut c;
    loop {
        c = bsearch(chave.as_ptr() as usize, lista, n, 0x16, 0x5c5270);
        if chave[5] as i8 >= 0x14 {
            c = lista;
        }
        chave[5] = chave[5].wrapping_add(1);
        if c != 0 {
            break;
        }
    }
    let ultimo = lista.wrapping_add((n as usize).wrapping_mul(0x16)).wrapping_sub(0x16);
    while c < ultimo {
        'proximo: {
            let id = cpe1_d(c);
            let op = cpe1_clube(id);
            if cp == op {
                break 'proximo;
            }
            if excluir != 0 {
                for k in 0..4usize {
                    let e = cpe1_d(excluir + 4 * k);
                    if e != 0 && id == cpe1_d(e) {
                        break 'proximo;
                    }
                }
            }
            if cpe1_na_lista(c, clube) || cpe1_fora_pela_data(this, op) {
                break 'proximo;
            }
            let (fc, fcl) = (cpe1_w(c + 0xe), cpe1_w(clube + 0xe));
            if fc != -1 && fcl != -1 && (fc > cpe1_w(clube + 0x10) || fcl > cpe1_w(c + 0x10)) {
                break 'proximo;
            }
            if cpe1_rivais(cp, op) || (cpe1_nacoes_rivais(cp, op) && cpe1_sorteio(2) == 0) {
                break 'proximo;
            }
            if cpe1_d(cp + 0x57) == cpe1_d(op + 0x57) {
                break 'proximo;
            }
            if cpe1_b(c + 4) > (cpe1_sorteio(3) as i8).wrapping_add(3) {
                break 'proximo;
            }
            let rep = cpe1_w(cp + 0x80);
            let (nc, no) = (cpe1_d(cp + 0x53), cpe1_d(op + 0x53));
            let longe = if rep < 2000 {
                no != nc
            } else if rep < 4000 {
                cpe1_b(no + 0x75) != cpe1_b(nc + 0x75)
            } else if rep < 8000 {
                cpe1_d(no + 0x71) != cpe1_d(nc + 0x71)
            } else {
                false
            };
            if longe {
                break 'proximo;
            }
            if cpe1_b(clube + 4) as i32 > cpe1_sorteio(2) && no != nc {
                break 'proximo;
            }
            if cpe1_sorteio(5) == 1 { // o adversário do amistoso
                return c;
            }
        }
        c = c.wrapping_add(0x16);
    }
    0
}

/// As regiões dos dois comparadores: o estado do sorteio e o cursor (+0xa) da lista de cada candidato.
unsafe fn cpe1_regioes(candidatos: impl Iterator<Item = usize>) -> Vec<(usize, usize)> {
    let mut reg = crate::aleatorio::REGIOES_DO_ESTADO.to_vec();
    for c in candidatos.filter(|&c| c != 0) {
        let l = cpe1_d(c + 0xa);
        if l != 0 {
            reg.push((l + 0xa, 4));
        }
    }
    reg
}

unsafe extern "thiscall" fn comparar_adversario_do_amistoso(this: usize, clube: usize, lista: usize, n: u32, faixa: u32, excluir: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(%%I0%%);
    let reg = cpe1_regioes((0..(n as i32).max(0) as usize).map(|i| lista + 0x16 * i));
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, lista, n, faixa, excluir),
        || crate::aleatorio::sem_contar(|| adversario_do_amistoso(this, clube, lista, n, faixa, excluir)));
    PLACAR.registrar(igual && r == m, || format!("adversario_do_amistoso({:x}, {:x}, n {}, faixa {}) original={:x} rust={:x} {:x?} x {:x?}",
        clube, lista, n, faixa as i8, r, m, mo, mr));
    r
}

/// { 0x5c5040 } (thiscall, ret 0x14, friendly.cpp): `Friendly::find_tour_opponent(clube, ordenados, n, faixa_max,
/// nacao)`: busca (bsearch 0x946464 com 0x5c5290, pelo +6) no vetor de `n` ponteiros para candidatos, ordenado pela
/// nação, um da nação (o id, [nacao]) e volta ao primeiro dela. Dali até o penúltimo: o primeiro cujo clube (+0x53)
/// não é da nação encerra (0); pula os acima da faixa (+5 > faixa_max), os fora pela data, o próprio clube, os que já
/// têm o clube na lista (+0xa), os rivais, os de nações rivais (sorteio(2) = 0), os da mesma cidade (+0x57) e os acima
/// da faixa do sorteio (+4 > sorteio(3) + 3); fica com o candidato quando sorteio(2) = 1. Nenhum: 0. Sem SEH, sem caixa.
unsafe extern "thiscall" fn adversario_da_excursao(this: usize, clube: usize, ordenados: usize, n: u32, faixa_max: u32, nacao: usize) -> usize {
    let cp = cpe1_clube(cpe1_d(clube));
    let mut chave = [0u8; 0x16];
    chave[5] = 0xff;
    core::ptr::write_unaligned(chave.as_mut_ptr().add(6) as *mut usize, cpe1_d(nacao));
    let pchave = chave.as_ptr() as usize;
    let bsearch: unsafe extern "C" fn(usize, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let mut s = bsearch(&pchave as *const usize as usize, ordenados, n, 4, 0x5c5290);
    if s == 0 {
        return 0;
    }
    if s > ordenados {
        loop {
            let anterior = cpe1_d(s - 4);
            s -= 4;
            if cpe1_d(anterior + 6) != cpe1_d(nacao) {
                s += 4;
                break;
            }
            if s <= ordenados {
                break;
            }
        }
    }
    let ultimo = ordenados.wrapping_add((n as usize).wrapping_mul(4)).wrapping_sub(4);
    while s < ultimo {
        let c = cpe1_d(s);
        let op = cpe1_clube(cpe1_d(c));
        if cpe1_d(op + 0x53) != nacao {
            return 0;
        }
        'proximo: {
            if cpe1_b(c + 5) > faixa_max as i8 || cpe1_fora_pela_data(this, op) || cp == op || cpe1_na_lista(c, clube) {
                break 'proximo;
            }
            if cpe1_rivais(cp, op) || (cpe1_nacoes_rivais(cp, op) && cpe1_sorteio(2) == 0) {
                break 'proximo;
            }
            if cpe1_d(cp + 0x57) == cpe1_d(op + 0x57) {
                break 'proximo;
            }
            if cpe1_b(c + 4) > (cpe1_sorteio(3) as i8).wrapping_add(3) {
                break 'proximo;
            }
            if cpe1_sorteio(2) == 1 { // o adversário da excursão
                return c;
            }
        }
        s += 4;
    }
    0
}

unsafe extern "thiscall" fn comparar_adversario_da_excursao(this: usize, clube: usize, ordenados: usize, n: u32, faixa_max: u32, nacao: usize) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, u32, usize) -> usize>(%%I1%%);
    let reg = cpe1_regioes((0..(n as i32).max(0) as usize).map(|i| cpe1_d(ordenados + 4 * i)));
    let (r, m, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(this, clube, ordenados, n, faixa_max, nacao),
        || crate::aleatorio::sem_contar(|| adversario_da_excursao(this, clube, ordenados, n, faixa_max, nacao)));
    PLACAR.registrar(igual && r == m, || format!("adversario_da_excursao({:x}, {:x}, n {}, faixa {}, nacao {:x}) original={:x} rust={:x} {:x?} x {:x?}",
        clube, ordenados, n, faixa_max as i8, nacao, r, m, mo, mr));
    r
}
