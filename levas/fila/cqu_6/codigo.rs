// ---------- %%LEVA%%a leva: a Copa do Rei e a Supercopa espanhola: os dois construtores e os 83 inscritos da Copa do Rei (spa_cup.cpp, spa_super.cpp) ----------

unsafe fn cqu6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqu6_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqu6_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqu6_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqu6_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqu6_d(cqu6_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqu6_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqu6_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqu6_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqu6_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqu6_d(this) != vt || cqu6_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqu6_d(this), cqu6_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
    }
    if *((this + 0x44) as *const u8) != 0xff {
        return None;
    }
    for &(o, t, v) in campos {
        let x = match t {
            1 => *((this + o) as *const u8) as u32,
            2 => core::ptr::read_unaligned((this + o) as *const u16) as u32,
            _ => core::ptr::read_unaligned((this + o) as *const u32),
        };
        if x != v {
            return Some(format!("+{:#x} = {:#x}, o construtor grava {:#x}", o, x, v));
        }
    }
    if (rodadas && cqu6_d(this + 0xa3) == 0) || cqu6_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqu6_d(this + 0xa3), cqu6_d(this + 8)));
    }
    None
}

/// { 0x84cf50 } (thiscall, ret 8, spa_cup.cpp; da criação da carreira, n = 1): `SpaCup::SpaCup(ano, registro)`: o construtor de
/// a Copa do Rei
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96f9c8, o registro em +4, o ano em +0x40 e
/// os da tabela CQU6_CAMPOS_COPA_DO_REI); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x60 se devolver 0); as estatísticas em +8 (a caixa da linha 0x67 sem elas); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); 83 times; fundo de prêmios de 1.000.000; 7 reservas no banco e 3 substituições; regra de disciplina 27 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu6_copa_do_rei_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu6_p32(this + 4, dono as u32);
    cqu6_p32(this, 0x96f9c8);
    cqu6_p16(this + 0x40, ano as u16);
    cqu6_p8(this + 0xab, 0x0);
    cqu6_p8(this + 0x44, 0xff);
    cqu6_p32(this + 0x30, 0xffffffff);
    cqu6_p32(this + 0x2c, 0x0);
    cqu6_p8(this + 0x42, 0x1);
    cqu6_p8(this + 0x49, 0x7);
    cqu6_p8(this + 0x4a, 0x3);
    cqu6_p8(this + 0x50, 0x1b);
    cqu6_p8(this + 0xb1, 0x0);
    cqu6_p32(this + 0x45, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqu_6 copa_do_rei nova");
    cqu6_p16(this + 0x3e, 0x53);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let v = zerada(0x53, 6);
    cqu6_p32(this + 0xa7, v as u32);
    if v == 0 {
        show_error_at(0xa82810, 0x52);
        return this;
    }
    cqu6_p32(this + 0x28, 0xf4240);
    cqu6_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0xa82810, 0x60);
        return this;
    }
    if !cqu6_estatisticas(this) {
        show_error_at(0xa82810, 0x67);
        return this;
    }
    this
}

const CQU6_CAMPOS_COPA_DO_REI: [(usize, u8, u32); 12] = [(0x28, 4, 0xf4240), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x3e, 2, 0x53), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x45, 4, 0x0), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x1b), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu6_copa_do_rei(this: usize, ano: u32, dono: usize) -> usize {
    cqu6_copa_do_rei_corpo(this, ano, dono) // cqu6: copa_do_rei
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu6_plano).
unsafe extern "thiscall" fn comparar_cqu6_copa_do_rei(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqu6_plano(this, r, ano, dono, 0x96f9c8, &CQU6_CAMPOS_COPA_DO_REI, true);
    PLACAR.registrar(erro.is_none(), || format!("cqu6_copa_do_rei({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// { 0x8586b0 } (thiscall, ret 8, spa_super.cpp; da criação da carreira, n = 1): `SpaSuper::SpaSuper(ano, registro)`: o construtor de
/// a Supercopa espanhola
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x96fd38, o registro em +4, o ano em +0x40 e
/// os da tabela CQU6_CAMPOS_SUPERCOPA_ESPANHOLA); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os dois clubes (SpaSuper::select_teams) (0x858aa0, thiscall, ret 0; a caixa da linha 0x43 se devolver negativo); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x51 se devolver negativo); as estatísticas em +8 (a caixa da linha 0x58 sem elas); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); sem fundo de prêmios (+0x28 fica como a base deixou); 7 reservas no banco e 3 substituições; regra de disciplina 26 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqu6_supercopa_espanhola_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqu6_p32(this + 4, dono as u32);
    cqu6_p32(this, 0x96fd38);
    cqu6_p16(this + 0x40, ano as u16);
    cqu6_p8(this + 0xab, 0x0);
    cqu6_p8(this + 0x44, 0xff);
    cqu6_p32(this + 0x2c, 0x0);
    cqu6_p32(this + 0x30, 0xffffffff);
    cqu6_p8(this + 0x42, 0x1);
    cqu6_p8(this + 0x49, 0x7);
    cqu6_p8(this + 0x4a, 0x3);
    cqu6_p8(this + 0x50, 0x1a);
    cqu6_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqu_6 supercopa_espanhola nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x858aa0usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0xa82a58, 0x43);
        return this;
    }
    cqu6_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r < 0 {
        show_error_at(0xa82a58, 0x51);
        return this;
    }
    if !cqu6_estatisticas(this) {
        show_error_at(0xa82a58, 0x58);
        return this;
    }
    this
}

const CQU6_CAMPOS_SUPERCOPA_ESPANHOLA: [(usize, u8, u32); 9] = [(0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x1a), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqu6_supercopa_espanhola(this: usize, ano: u32, dono: usize) -> usize {
    cqu6_supercopa_espanhola_corpo(this, ano, dono) // cqu6: supercopa_espanhola
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqu6_plano).
unsafe extern "thiscall" fn comparar_cqu6_supercopa_espanhola(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I1%%)(this, ano, dono);
    let erro = cqu6_plano(this, r, ano, dono, 0x96fd38, &CQU6_CAMPOS_SUPERCOPA_ESPANHOLA, true);
    PLACAR.registrar(erro.is_none(), || format!("cqu6_supercopa_espanhola({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// O clube `c` não serve à Copa do Rei pela ligação com o time B? (club_get_linked_b_team 0x540a50: push 1; push &sênior;
/// push clube; add esp, 0xc -> cdecl): serve se não há ligação ou se a saída diz que é o principal (≠ 0).
unsafe fn cqu6_time_b(c: usize) -> bool {
    let ligado: unsafe extern "C" fn(usize, *mut i32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut principal = 0i32;
    let r = ligado(c, &mut principal, 1);
    r != 0 && principal == 0
}

/// A divisão do clube (+0x5b, ponteiro para o registro da competição; +0 o id) é a de [`g`]?
unsafe fn cqu6_div(c: usize, g: usize) -> bool {
    let l = cqu6_d(c + 0x5b);
    l != 0 && cqu6_d(l) == cqu6_d(g)
}

/// O clube é das divisões de baixo ([0x9cf7b8], [0x9cf62c], [0x9cf630], [0x9cf634], [0x9cf638]: a Segunda B e a
/// Terceira espanholas [inferência]), com nação (+0x53) e divisão?
unsafe fn cqu6_de_baixo(c: usize) -> bool {
    if cqu6_d(c + 0x53) == 0 || cqu6_d(c + 0x5b) == 0 {
        return false;
    }
    [0x9cf7b8usize, 0x9cf62c, 0x9cf630, 0x9cf634, 0x9cf638].iter().any(|&g| cqu6_div(c, g))
}

/// Grava o clube `c` na vaga `k` da tabela dos inscritos (+0xa7, registros de 6 bytes: o clube, +4 e +5 zerados).
unsafe fn cqu6_vaga(this: usize, k: i32, c: usize) {
    let t = cqu6_d(this + 0xa7) + k as usize * 6;
    cqu6_p32(t, c as u32);
    cqu6_p8(t + 4, 0);
    cqu6_p8(t + 5, 0);
}

/// Copia os `n` clubes de `lista` para as vagas a partir de `de`.
unsafe fn cqu6_bloco(this: usize, de: i32, lista: &[usize], n: i32) {
    for j in 0..n.max(0) {
        cqu6_vaga(this, de + j, lista[j as usize]);
    }
}

/// { 0x84d910 } (thiscall, ret 0, spa_cup.cpp; chamada pelo dia da Copa do Rei, 0x84d740, n = 1 até 2002-06-01):
/// `SpaCup::collect_entrants()`: os 83 inscritos da Copa do Rei. Pela ordem da tabela dos clubes ([0xae23b4], os
/// [0xae235c] − 2 × [0xae2350] primeiros, o limite relido a cada volta), os espanhóis (+0x53 = [0x9cf488]) que não são
/// time B ligado (cqu6_time_b) vão a três grupos: os de competição continental (+0x1db ≠ −1 e ≠ [0x9cf6f8]), até 7; senão
/// os da Primeira ([0x9cf624], até 20) e os da Segunda ([0x9cf628], até 22). Faltam = 83 − os três grupos; +0x3e = 83 e a
/// tabela (calloc 0x945501 de 83 × 6: push 6; push 0x53; add esp, 8 -> cdecl) em +0xa7 (sem memória: a caixa da linha
/// 0x1e0 e −1). As vagas de 0 a faltam − 1 são dos clubes de baixo (cqu6_de_baixo, sem time B, +0x1db = −1), da última
/// para a primeira: se a nação da copa ([+4] +0x5d) tem o bit 4 em +0x11c, até 120 deles numa lista (malloc 0x944e46 de
/// 0x2d0; sem memória: a caixa da linha 0x1ee e 0) com a categoria +0x5f, ordenada (qsort 0x945333 com 0x92f660: push
/// 0x92f660; push 6; push n; push lista; add esp, 0x10 -> cdecl), e deles os primeiros; senão na ordem da tabela. Se ainda
/// faltam, os espanhóis sem divisão ou da divisão genérica [0x9cf764] ou da [0x9cf6b0] (sem time B, +0x1db = −1). Depois
/// das vagas de baixo vêm a Segunda, a Primeira e os continentais, nessa ordem. Devolve 1.
unsafe fn cqu6_inscritos(this: usize) -> i32 {
    let base = cqu6_d(0xae23b4);
    let limite = || (cqu6_d(0xae235c) as i32).wrapping_sub((cqu6_d(0xae2350) as i32).wrapping_mul(2));
    let mut continentais = [0usize; 20];
    let mut primeira = [0usize; 20];
    let mut segunda = [0usize; 22];
    let (mut nc, mut np, mut ns) = (0i16, 0i16, 0i16);
    let mut i = 0i32;
    while i < limite() {
        let c = base + i as usize * 0x245;
        let nac = cqu6_d(c + 0x53);
        if nac != 0 && cqu6_d(nac) == cqu6_d(0x9cf488) && !cqu6_time_b(c) {
            let cont = cqu6_d(c + 0x1db) as i32;
            if cont != -1 && cont != cqu6_d(0x9cf6f8) as i32 && nc < 0x14 && nc < 7 {
                continentais[nc as usize] = c;
                nc += 1;
            } else if cqu6_d(c + 0x5b) != 0 {
                if cqu6_div(c, 0x9cf624) && np < 0x14 {
                    primeira[np as usize] = c;
                    np += 1;
                } else if cqu6_div(c, 0x9cf628) && ns < 0x16 {
                    crate::comparacao::ramo("cqu_6 segunda divisao");
                    segunda[ns as usize] = c;
                    ns += 1;
                }
            }
        }
        i += 1;
    }
    let faltam = 0x53i32 - ns as i32 - np as i32 - nc as i32;
    cqu6_p16(this + 0x3e, 0x53);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let t = zerada(0x53, 6);
    cqu6_p32(this + 0xa7, t as u32);
    if t == 0 {
        show_error_at(0xa82810, 0x1e0);
        return -1;
    }
    let faltam_w = faltam as i16 as i32;
    let mut k = 0i32;
    let nacao = cqu6_d(cqu6_d(this + 4) + 0x5d);
    if *((nacao + 0x11c) as *const u8) & 4 != 0 {
        let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
        let lista = alocar(0x2d0);
        if lista == 0 {
            show_error_at(0xa82810, 0x1ee);
            return 0;
        }
        let mut n = 0i32;
        let mut i = 0i32;
        while i < limite() {
            if n >= 0x78 {
                break;
            }
            let c = base + i as usize * 0x245;
            if cqu6_de_baixo(c) && !cqu6_time_b(c) && cqu6_d(c + 0x1db) as i32 == -1 {
                cqu6_p32(lista + n as usize * 6, c as u32);
                cqu6_p8(lista + n as usize * 6 + 4, *((c + 0x5f) as *const u8));
                n += 1;
            }
            i += 1;
        }
        let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
        ordenar(lista, n as u32, 6, 0x92f660);
        let mut j = 0i32;
        while j < faltam_w && j < n {
            k += 1;
            cqu6_vaga(this, faltam_w - 1 - j, cqu6_d(lista + j as usize * 6));
            j += 1;
        }
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lista);
    } else {
        let mut i = 0i32;
        while i < limite() {
            let c = base + i as usize * 0x245;
            if cqu6_de_baixo(c) && !cqu6_time_b(c) && cqu6_d(c + 0x1db) as i32 == -1 {
                k += 1;
                cqu6_vaga(this, faltam_w - k, c);
                if k >= faltam_w {
                    break;
                }
            }
            i += 1;
        }
    }
    if faltam_w - k > 0 && limite() > 0 {
        crate::comparacao::ramo("cqu_6 completar com os sem divisao");
        let mut i = 0i32;
        while i < limite() {
            let c = base + i as usize * 0x245;
            let nac = cqu6_d(c + 0x53);
            if nac != 0 && cqu6_d(nac) == cqu6_d(0x9cf488) && !cqu6_time_b(c) && cqu6_d(c + 0x1db) as i32 == -1
                && (cqu6_d(c + 0x5b) == 0 || cqu6_div(c, 0x9cf764) || cqu6_div(c, 0x9cf6b0)) {
                k += 1;
                cqu6_vaga(this, faltam_w - k, c);
                if k >= faltam_w {
                    break;
                }
            }
            i += 1;
        }
    }
    let (ns, np, nc) = (ns as i32, np as i32, nc as i32);
    cqu6_bloco(this, faltam_w, &segunda, ns); cqu6_bloco(this, faltam_w + ns.max(0), &primeira, np); // cqu6: a Segunda e depois a Primeira
    cqu6_bloco(this, faltam_w + ns.max(0) + np.max(0), &continentais, nc);
    1
}

unsafe extern "thiscall" fn cqu6_inscritos_da_copa_do_rei(this: usize) -> i32 {
    cqu6_inscritos(this)
}

/// Lado a lado a partir do mesmo estado (+0x3e; não sorteia): a original deixa a tabela dela em +0xa7, a cópia faz a sua,
/// comparada nas 83 vagas (6 bytes cada) e liberada; a devolução também.
unsafe extern "thiscall" fn comparar_cqu6_inscritos_da_copa_do_rei(this: usize) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> i32>(%%I2%%);
    let antes = cqu6_d(this + 0xa7);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (a, b, igual, _, _) = crate::comparacao::lado_a_lado(&[(this + 0x3e, 2usize)], || {
        let r = o(this);
        lo.set(cqu6_d(this + 0xa7));
        cqu6_p32(this + 0xa7, antes as u32);
        r
    }, || {
        let r = cqu6_inscritos(this);
        lr.set(cqu6_d(this + 0xa7));
        cqu6_p32(this + 0xa7, lo.get() as u32);
        r
    });
    let (lo, lr) = (lo.get(), lr.get());
    let tabela = lo == 0 || lr == 0 || core::slice::from_raw_parts(lo as *const u8, 0x53 * 6) == core::slice::from_raw_parts(lr as *const u8, 0x53 * 6);
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    PLACAR.registrar(igual && a == b && tabela, || format!("cqu6_inscritos_da_copa_do_rei({:#x}) devolveu {} x {} memória {} tabela {}", this, a, b, igual, tabela));
    a
}
