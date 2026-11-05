// ---------- %%LEVA%%a leva: a divisão de baixo espanhola (o construtor e a tabela com os times B e os melhores da Segunda B) e a tabela da Segunda espanhola (os times B que não sobem) (spa_lower.cpp, spa_second.cpp) ----------

unsafe fn cqm6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqm6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

unsafe fn cqm6_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes).
unsafe fn cqm6_registro(g: usize) -> usize {
    cqm6_d(0xae23d0).wrapping_add(cqm6_d(g).wrapping_mul(0x6b))
}

/// A competição do clube (+0x57, o id no +0) é a de id [g]?
unsafe fn cqm6_na(c: usize, g: usize) -> bool {
    let comp = cqm6_d(c + 0x57);
    comp != 0 && cqm6_d(comp) == cqm6_d(g)
}

/// { 0x851020 } (thiscall, ret 4, spa_lower.cpp, quadro SEH só no caminho de exceção): `SpaLower::SpaLower(ano)`, a
/// divisão de baixo espanhola (criada pela Primera quando a nação não tem o bit 4 de +0x11c): a base (League::League
/// 0x682200: mov ecx -> thiscall, ret 0), a vtable 0x96fb1c, o ano (+0x40), a regra de disciplina 0xff (nenhuma) e o
/// registro da competição de id [0x9cf6b0] (+4). Tenta carregar o estado (0x687b10: push 1 -> thiscall, ret 4);
/// senão, só a liga (+0x44 = 0xff, +0x30 = −1, +0x2c = 0), as regras (0x853790), a tabela (0x8533d0) e os jogos
/// (League::create_fixtures 0x6835c0), todos mov ecx -> thiscall, ret 0; cada falha, a caixa da sua linha (0x3f,
/// 0x46, 0x4d do fonte 0xa82880) e fim. Devolve this.
unsafe extern "thiscall" fn construir_divisao_de_baixo_espanhola(this: usize, ano: u32) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x853790usize);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x8533d0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    base(this);
    cqm6_pd(this, 0x96fb1c);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    *((this + 0x50) as *mut u8) = 0xff;
    cqm6_pd(this + 4, cqm6_registro(0x9cf6b0));
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparacao::ramo("cqm_6 divisao de baixo nova");
    *((this + 0x44) as *mut u8) = 0xff;
    cqm6_pd(this + 0x30, 0xffff_ffff);
    cqm6_pd(this + 0x2c, 0);
    if regras(this) == 0 {
        aviso_em(0xa82880, 0x3f);
        return this;
    }
    if tabela(this) == 0 {
        aviso_em(0xa82880, 0x46);
        return this;
    }
    if jogos(this) == 0 {
        aviso_em(0xa82880, 0x4d);
    }
    this
}

/// Aloca e sorteia (pela tabela): só a original, conferindo a devolução, a vtable, o ano, a disciplina e o registro.
unsafe extern "thiscall" fn comparar_construir_divisao_de_baixo_espanhola(this: usize, ano: u32) -> usize {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I0%%);
    let r = o(this, ano);
    let ok = r == this && cqm6_d(this) == 0x96fb1c && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && *((this + 0x50) as *const u8) == 0xff && cqm6_d(this + 4) == cqm6_registro(0x9cf6b0);
    PLACAR.registrar(ok, || format!("construir_divisao_de_baixo_espanhola({:#x}): devolveu {:#x}, vtable {:#x}, +4 = {:#x}", this, r, cqm6_d(this), cqm6_d(this + 4)));
    r
}

/// O clube de cima do clube `c` (club_get_linked_b_team 0x540a50: push 1; push &eh_a; push c; add esp, 0xc -> cdecl),
/// se `c` é o time B dele (eh_a = 0) e ele está na competição de id [g].
unsafe fn cqm6_time_b_de(c: usize, g: usize) -> bool {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut eh_a: u32 = 0;
    let p = parceiro(c, &mut eh_a, 1);
    p != 0 && eh_a == 0 && cqm6_na(p, g)
}

/// { 0x855b40 } (thiscall, ret 0, spa_second.cpp): `SpaSecond::create_table()`: a Segunda tem 22 clubes (+0x3e),
/// a tabela (malloc 0x944e46 de 0x512: push; add esp, 4 -> cdecl; sem memória, a caixa da linha 0x298 e 0) recebe, na
/// ordem do banco ([0xae23b4], [0xae235c] clubes de 0x245 bytes), os clubes da competição de id [0x9cf628] até encher
/// (League::init_table_row 0x687430: push 0; push 0; push clube; push linha (a palavra empurrada tem o byte da linha e o
/// lixo da pilha em cima; a 0x687430 só lê o byte); mov ecx, this -> thiscall, ret 0x10); o time B de um clube da
/// Primera ([0x9cf624]) leva o destino −4 (+0x37 = 0xfc: não pode subir). Cheia: o fundo de prêmios (+0x28 =
/// League::calc_prize_fund 0x682f70: push 50000; mov ecx -> thiscall, ret 4), +0xe1 = 1 e 1; senão 0.
unsafe extern "thiscall" fn tabela_da_segunda(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let premios: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x682f70usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x16);
    let t = alocar(0x512);
    cqm6_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xa828bc, 0x298);
        return 0;
    }
    let mut k: u8 = 0;
    let n = cqm6_d(0xae235c) as i32;
    let base = cqm6_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245; // cqm6: o clube i do banco
        if cqm6_na(c, 0x9cf628) {
            iniciar(this, k as u32, c, 0, 0);
            if cqm6_time_b_de(c, 0x9cf624) {
                crate::comparacao::ramo("cqm_6 time b na segunda");
                *((cqm6_d(this + 0xb1) + (k as i8 as isize as usize).wrapping_mul(0x3b) + 0x37) as *mut u8) = 0xfc;
            }
            k = k.wrapping_add(1);
            if k as i8 as i16 == cqm6_w(this + 0x3e) {
                break;
            }
        }
        i = i.wrapping_add(1);
    }
    crate::comparacao::ramo("cqm_6 tabela da segunda");
    if k as i8 as i16 != cqm6_w(this + 0x3e) {
        return 0;
    }
    let f = premios(this, 0xc350);
    cqm6_pd(this + 0x28, f as usize);
    *((this + 0xe1) as *mut u8) = 1;
    1
}

/// Aloca: só a original. O plano, feito antes: os clubes da Segunda na ordem do banco (até 22) e a marca dos times B.
/// Depois: a devolução, +0x3e e as linhas (o clube e o destino −4 dos marcados).
unsafe extern "thiscall" fn comparar_tabela_da_segunda(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I1%%);
    let n = cqm6_d(0xae235c) as i32;
    let base = cqm6_d(0xae23b4);
    let mut plano: Vec<(usize, bool)> = Vec::new();
    for i in 0..n.max(0) as usize {
        let c = base + i * 0x245;
        if cqm6_na(c, 0x9cf628) {
            plano.push((c, cqm6_time_b_de(c, 0x9cf624)));
            if plano.len() == 0x16 {
                break;
            }
        }
    }
    let r = o(this);
    let t = cqm6_d(this + 0xb1);
    let mut ok = r == (plano.len() == 0x16) as u32 && cqm6_w(this + 0x3e) == 0x16 && t != 0;
    if ok {
        for (k, &(c, b)) in plano.iter().enumerate() {
            let l = t + k * 0x3b;
            ok = ok && cqm6_d(l) == c && (!b || *((l + 0x37) as *const u8) == 0xfc);
        }
    }
    PLACAR.registrar(ok, || format!("tabela_da_segunda({:#x}): devolveu {}, plano {:x?}", this, r, plano));
    r
}

/// Os candidatos da divisão de baixo espanhola, do registro local de 100 da original: os clubes (todos, na ordem do
/// banco, até 100) das competições de id [0x9cf62c], [0x9cf630], [0x9cf634], [0x9cf638] [inferência: os 4 grupos da
/// Segunda B] ou, se espanhóis, da de id [0x9cf764] [inferência: a divisão genérica de baixo], que não são time B nem
/// time A de ninguém (club_get_linked_b_team devolve 0).
unsafe fn cqm6_candidatos() -> Vec<usize> {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let mut v = Vec::new();
    let n = cqm6_d(0xae235c) as i32;
    let base = cqm6_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n && v.len() < 100 {
        let c = base + (i as u16 as usize) * 0x245;
        let comp = cqm6_d(c + 0x57);
        let nacao = cqm6_d(c + 0x53);
        if comp != 0 && nacao != 0 {
            let id = cqm6_d(comp);
            let grupo = [0x9cf62c, 0x9cf630, 0x9cf634, 0x9cf638].iter().any(|&g| id == cqm6_d(g));
            if grupo || (cqm6_d(nacao) == cqm6_d(0x9cf488) && id == cqm6_d(0x9cf764)) {
                let mut eh_a: u32 = 0;
                if parceiro(c, &mut eh_a, 1) == 0 {
                    v.push(c);
                }
            }
        }
        i = i.wrapping_add(1);
    }
    v
}

/// Os times B espanhóis (fora da Primera e da Segunda) cujo clube de cima está na competição de id [g], na ordem do
/// banco, até 22 linhas (`k` já usadas). Os últimos `2 × [0xae2350]` clubes do banco ficam de fora [inferência: os
/// clubes criados depois, sem divisão].
unsafe fn cqm6_times_b(g: usize, k: &mut u8, linhas: i16, mut por: impl FnMut(usize, bool)) {
    let n = (cqm6_d(0xae235c) as i32).wrapping_sub((cqm6_d(0xae2350) as i32).wrapping_mul(2));
    let base = cqm6_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245;
        let comp = cqm6_d(c + 0x57);
        let nacao = cqm6_d(c + 0x53);
        if comp != 0 && nacao != 0 && cqm6_d(nacao) == cqm6_d(0x9cf488) && cqm6_d(comp) != cqm6_d(0x9cf624) && cqm6_d(comp) != cqm6_d(0x9cf628) {
            let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
            let mut eh_a: u32 = 0;
            let p = parceiro(c, &mut eh_a, 1);
            let entra = p != 0 && eh_a == 0 && cqm6_d(p + 0x57) != 0 && (*k as i8 as i16) < linhas && cqm6_d(cqm6_d(p + 0x57)) == cqm6_d(g);
            if entra {
                *k = k.wrapping_add(1);
            }
            por(c, entra);
        }
        i = i.wrapping_add(1);
    }
}

/// { 0x8533d0 } (thiscall, ret 0, spa_lower.cpp, 947 bytes): `SpaLower::create_table()`: a divisão de baixo tem 22
/// clubes (+0x3e), a tabela (malloc de 0x512; sem memória, a caixa da linha 0x2ac e 0). Primeiro, os clubes que estão
/// nela (a competição de id [0x9cf6b0]) passam à de id [0x9cf764] [inferência: a divisão genérica]. Depois enche, na
/// ordem do banco: os times B espanhóis cujo clube de cima está na Primera, depois os da Segunda (cada um com +0x60 =
/// o registro desta competição; os outros times espanhóis de fora das duas, com +0x60 = 0); o resto com os
/// candidatos (cqm6_candidatos), ordenados pela reputação e com os 3 × faltam primeiros embaralhados
/// (comp_util_sort_and_shuffle_top_clubs 0x4afbd0: push faltam; push quantos; push lista; add esp, 0xc -> cdecl; sorteia),
/// cada um com +0x60 = o registro. As linhas: League::init_table_row 0x687430 (push 0; push 0; push clube; push eax
/// (al = a linha, os bits de cima são lixo de outra conta; a 0x687430 só lê o byte); mov ecx, this -> thiscall, ret
/// 0x10). Devolve se encheu.
unsafe extern "thiscall" fn tabela_da_divisao_de_baixo_espanhola(this: usize) -> u32 {
    let alocar: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944e46usize);
    let iniciar: unsafe extern "thiscall" fn(usize, u32, usize, u32, u32) = core::mem::transmute(0x687430usize);
    let embaralhar: unsafe extern "C" fn(usize, u32, u32) = core::mem::transmute(0x4afbd0usize);
    core::ptr::write_unaligned((this + 0x3e) as *mut i16, 0x16);
    let t = alocar(0x512);
    cqm6_pd(this + 0xb1, t);
    if t == 0 {
        aviso_em(0xa82880, 0x2ac);
        return 0;
    }
    // os clubes desta divisão passam à genérica
    let n = (cqm6_d(0xae235c) as i32).wrapping_sub((cqm6_d(0xae2350) as i32).wrapping_mul(2));
    let base = cqm6_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < n {
        let c = base + (i as u16 as usize) * 0x245;
        if cqm6_na(c, 0x9cf6b0) {
            cqm6_pd(c + 0x57, cqm6_registro(0x9cf764));
        }
        i = i.wrapping_add(1);
    }
    let mut k: u8 = 0;
    for g in [0x9cf624usize, 0x9cf628] { // cqm6: os times B da Primera e da Segunda
        let linhas = cqm6_w(this + 0x3e);
        // a conta das linhas para a condição anda junto com `k`
        let mut kk = k;
        cqm6_times_b(g, &mut kk, linhas, |c, entra| {
            if entra {
                iniciar(this, k as u32, c, 0, 0);
                k = k.wrapping_add(1);
                cqm6_pd(c + 0x60, cqm6_d(this + 4));
            } else {
                cqm6_pd(c + 0x60, 0);
            }
        });
    }
    crate::comparacao::ramo("cqm_6 tabela da divisao de baixo");
    let mut lista = cqm6_candidatos();
    let quantos = lista.len() as u16 as u32;
    let faltam = (cqm6_w(this + 0x3e) as u16).wrapping_sub(k as i8 as i16 as u16) as u32; // cqm6: quantos faltam
    lista.resize(100, 0);
    embaralhar(lista.as_mut_ptr() as usize, quantos, faltam);
    let mut j: i16 = 0;
    while j < quantos as u16 as i16 && (k as i8 as i16) < cqm6_w(this + 0x3e) {
        let c = lista[j as usize];
        iniciar(this, k as u32, c, 0, 0);
        k = k.wrapping_add(1);
        cqm6_pd(c + 0x60, cqm6_d(this + 4));
        j += 1;
    }
    (k as i8 as i16 == cqm6_w(this + 0x3e)) as u32
}

/// Aloca e sorteia: só a original. O plano, feito antes: os times B (da Primera e da Segunda) das primeiras linhas.
/// Depois: +0x3e e o clube das primeiras linhas. O +0x60 não entra: a segunda passada (a dos times B da Segunda) zera o
/// +0x60 de todo time espanhol que não entra nela, inclusive os da Primera que a primeira acabou de pôr na tabela
/// (comportamento da original, que a cópia repete; achado pela comparação de 8 ligas, 05/11/2026).
unsafe extern "thiscall" fn comparar_tabela_da_divisao_de_baixo_espanhola(this: usize) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize) -> u32>(%%I2%%);
    let mut plano: Vec<usize> = Vec::new();
    let mut k: u8 = 0;
    for g in [0x9cf624usize, 0x9cf628] {
        cqm6_times_b(g, &mut k, 0x16, |c, entra| {
            if entra {
                plano.push(c);
            }
        });
    }
    let r = o(this);
    let t = cqm6_d(this + 0xb1);
    let mut ok = cqm6_w(this + 0x3e) == 0x16 && t != 0;
    if ok {
        for (i, &c) in plano.iter().enumerate() {
            ok = ok && cqm6_d(t + i * 0x3b) == c;
        }
    }
    PLACAR.registrar(ok, || format!("tabela_da_divisao_de_baixo_espanhola({:#x}): devolveu {}, plano {:x?}, tabela {:x?}", this, r, plano,
        (0..plano.len()).map(|i| if t != 0 { cqm6_d(t + i * 0x3b) } else { 0 }).collect::<Vec<_>>()));
    r
}
