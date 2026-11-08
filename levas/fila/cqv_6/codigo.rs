// ---------- %%LEVA%%a leva: a Copa do Brasil: o construtor e os 71 inscritos (bra_cup.cpp) ----------

unsafe fn cqv6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqv6_p8(a: usize, v: u8) {
    *(a as *mut u8) = v;
}

unsafe fn cqv6_p16(a: usize, v: u16) {
    core::ptr::write_unaligned(a as *mut u16, v)
}

unsafe fn cqv6_p32(a: usize, v: u32) {
    core::ptr::write_unaligned(a as *mut u32, v)
}

/// As rodadas da copa pelo método +0x3c da vtable (push 0; push &+0x3a; push &+0x3c; push −1; mov ecx, this -> thiscall,
/// ret 0x10): a tabela das rodadas, gravada em +0xa3.
unsafe fn cqv6_rodadas(this: usize) {
    let f: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqv6_d(cqv6_d(this) + 0x3c));
    let t = f(this, 0xffff_ffff, this + 0x3c, this + 0x3a, 0);
    cqv6_p32(this + 0xa3, t as u32);
}

/// As estatísticas (operator new 0x944cf1 de 0x5ce: push; add esp, 4 -> cdecl; Comp_Stats::Comp_Stats 0x49ee70: push
/// this; mov ecx, novo -> thiscall, ret 4) em +8; o quadro SEH da original cobre só esse new. Devolve se criou.
unsafe fn cqv6_estatisticas(this: usize) -> bool {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let n = novo(0x5ce);
    let s = if n != 0 { estatisticas(n, this) } else { 0 };
    cqv6_p32(this + 8, s as u32);
    s != 0
}

/// O plano que o comparador confere depois da original: a devolução (this), a vtable, o registro (+4) e o ano (+0x40);
/// na copa nova (+0x44 = 0xff), cada campo fixo que o construtor grava e as rodadas e as estatísticas criadas.
unsafe fn cqv6_plano(this: usize, r: usize, ano: u32, dono: usize, vt: usize, campos: &[(usize, u8, u32)], rodadas: bool) -> Option<String> {
    if r != this || cqv6_d(this) != vt || cqv6_d(this + 4) != dono || core::ptr::read_unaligned((this + 0x40) as *const u16) != ano as u16 {
        return Some(format!("devolveu {:#x}, vtable {:#x}, +4 {:#x}, ano {}", r, cqv6_d(this), cqv6_d(this + 4), core::ptr::read_unaligned((this + 0x40) as *const u16)));
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
    if (rodadas && cqv6_d(this + 0xa3) == 0) || cqv6_d(this + 8) == 0 {
        return Some(format!("rodadas {:#x}, estatísticas {:#x}", cqv6_d(this + 0xa3), cqv6_d(this + 8)));
    }
    None
}

/// { 0x427430 } (thiscall, ret 8, bra_cup.cpp; da criação da carreira, n = 1): `BraCup::BraCup(ano, registro)`: o construtor de
/// a Copa do Brasil
/// sobre a base Cup::Cup (0x518640, thiscall, ret 0); os campos fixos (a vtable 0x967cf8, o registro em +4, o ano em +0x40 e
/// os da tabela CQV6_CAMPOS_COPA_DO_BRASIL); se o estado salvo carrega (Cup::load_from_disk 0x51fc00: push 1; mov ecx, this ->
/// thiscall, ret 4; devolve ≠ 0), devolve this. Na copa nova, em ordem: os 71 inscritos (BraCup::select_entrants) (0x427f00, thiscall, ret 0, sem conferir); a tabela dos 71 times (BraCup::build_team_table) (0x427c20, thiscall, ret 0; a caixa da linha 0x4b se devolver 0); as rodadas (o método +0x3c da vtable, em +0xa3); a árvore dos confrontos (Cup::map_fixture_tree) (0x518790, thiscall, ret 0; a caixa da linha 0x59 se devolver 0); as estatísticas em +8 (a caixa da linha 0x60 sem elas); a classificação final (Cup::rank_teams) (0x5223a0, thiscall, ret 0, sem conferir); as rodadas (+0x3c da vtable, em +0xa3);
/// as estatísticas em +8; cada passo com a sua caixa (show_error_at) e a devolução de this se falhar. Devolve this.
/// Regra: tipo 1 (clubes de uma nação); fundo de prêmios de 500.000; 7 reservas no banco e 3 substituições; regra de disciplina 6 [+0x49/+0x4a/+0x50/+0x42 pela leitura da referência das competições, §2.1].
unsafe fn cqv6_copa_do_brasil_corpo(this: usize, ano: u32, dono: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x518640usize);
    base(this);
    cqv6_p32(this + 4, dono as u32);
    cqv6_p32(this, 0x967cf8);
    cqv6_p16(this + 0x40, ano as u16);
    cqv6_p8(this + 0xab, 0x0);
    cqv6_p8(this + 0x44, 0xff);
    cqv6_p32(this + 0x30, 0xffffffff);
    cqv6_p32(this + 0x2c, 0x0);
    cqv6_p8(this + 0x42, 0x1);
    cqv6_p8(this + 0x50, 0x6);
    cqv6_p8(this + 0xb1, 0x0);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x51fc00usize);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqv_6 copa_do_brasil nova");
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x427f00usize);
    f(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x427c20usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x98a3bc, 0x4b);
        return this;
    }
    cqv6_p8(this + 0x49, 0x7);
    cqv6_p8(this + 0x4a, 0x3);
    cqv6_p32(this + 0x28, 0x7a120);
    cqv6_rodadas(this);
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x518790usize);
    let r = f(this);
    if r == 0 {
        show_error_at(0x98a3bc, 0x59);
        return this;
    }
    if !cqv6_estatisticas(this) {
        show_error_at(0x98a3bc, 0x60);
        return this;
    }
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x5223a0usize);
    f(this);
    this
}

const CQV6_CAMPOS_COPA_DO_BRASIL: [(usize, u8, u32); 10] = [(0x28, 4, 0x7a120), (0x2c, 4, 0x0), (0x30, 4, 0xffffffff), (0x42, 1, 0x1), (0x44, 1, 0xff), (0x49, 1, 0x7), (0x4a, 1, 0x3), (0x50, 1, 0x6), (0xab, 1, 0x0), (0xb1, 1, 0x0)];

unsafe extern "thiscall" fn cqv6_copa_do_brasil(this: usize, ano: u32, dono: usize) -> usize {
    cqv6_copa_do_brasil_corpo(this, ano, dono) // cqv6: copa_do_brasil
}

/// Só a original (aloca, sorteia e lê o arquivo do save), conferindo o plano (cqv6_plano).
unsafe extern "thiscall" fn compare_cqv6_copa_do_brasil(this: usize, ano: u32, dono: usize) -> usize {
    let r = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%)(this, ano, dono);
    let erro = cqv6_plano(this, r, ano, dono, 0x967cf8, &CQV6_CAMPOS_COPA_DO_BRASIL, true);
    SCOREBOARD.record(erro.is_none(), || format!("cqv6_copa_do_brasil({:#x}, {}, {:#x}): {}", this, ano as u16, dono, erro.clone().unwrap_or_default()));
    r
}

/// Os 71 nomes fixos da Copa do Brasil, na ordem das vagas (textos do exe; lidos de lá, como a original): Vasco (0),
/// Remo, Paysandu, River, Sampaio Corrêa, Nacional (AM), ABC, América (RN), Ceará, Confiança, CRB, Sergipe, Comercial
/// Bandeirantes, Comercial, Juventus, Serra, Tocantinópolis, União, Camaçari, Fluminense de Feira, Juazeiro, Vitória,
/// Caxias, Internacional, Juventude, Pelotas, Anapolina, Anápolis, Atlético Goianiense, Goiânia, Atlético Mineiro,
/// Caldense, Uberlândia, Villa Nova, Paraná, Londrina, Malutrom, Coritiba, Central, Ferroviário, Porto, Santa Cruz,
/// América (RJ), Americano, Bangu, Botafogo, Avaí, Criciúma, Figueirense, Tubarão, Botafogo (SP), Palmeiras, Rio
/// Branco, Santos, Sport, Ponte Preta, São Paulo, Gama, São Raimundo, Fluminense, Goiás, Vila Nova, Náutico, Bahia,
/// Fortaleza, Joinville, América (MG), Grêmio, Flamengo, Atlético Paranaense e Corinthians (70).
const CQV6_NOMES: [usize; 71] = [0x98a398, 0x98aa20, 0x98aa0c, 0x98a9f4, 0x98a9d4, 0x98a9b8, 0x98a9a4, 0x98a988, 0x98a974, 0x98a954, 0x98a93c, 0x98a924, 0x98a8f8, 0x98a8e0, 0x98a8b4, 0x98a8a0, 0x98a880, 0x98a86c, 0x98a854, 0x98a830, 0x98a818, 0x98a800, 0x98a7dc, 0x98a7c0, 0x98a7a8, 0x98a790, 0x98a770, 0x98a758, 0x98a73c, 0x98a724, 0x98a70c, 0x98a6ec, 0x98a6d0, 0x98a6b4, 0x98a6a4, 0x98a68c, 0x98a67c, 0x98a664, 0x98a650, 0x98a634, 0x98a61c, 0x98a600, 0x98a5e4, 0x98a5c4, 0x98a5ac, 0x98a58c, 0x98a578, 0x98a560, 0x98a544, 0x98a52c, 0x98a510, 0x98a4f0, 0x98a4d4, 0x98a4bc, 0x98a4a4, 0x98a484, 0x98a46c, 0x98a450, 0x98a344, 0x98a434, 0x98a420, 0x98a380, 0x98a364, 0x98a294, 0x98a408, 0x98a3f0, 0x98a328, 0x98a304, 0x98a2e4, 0x98a2c8, 0x98a2a8];

/// O nome do clube (+4) é igual ao texto `t` (a comparação inline da original, byte a byte até o zero)?
unsafe fn cqv6_mesmo_nome(c: usize, t: usize) -> bool {
    let mut i = 0usize;
    loop {
        let a = *((c + 4 + i) as *const u8);
        if a != *((t + i) as *const u8) {
            return false;
        }
        if a == 0 {
            return true;
        }
        i += 1;
    }
}

/// Os doze estaduais da Copa do Brasil: o id ([g]), o byte de controle (a marca do campeão no passo 2, depois a conta
/// dos outros), o grupo (0: vagas 0x36..; 1: 0x3d..; 2: 0x42..) e o teto dos outros (5 ou 7). A ordem é a dos testes da
/// original; os índices são os bytes da pilha dela menos 0x13. [inferência: os três de teto 7 (0x9cf998, 0x9cf994,
/// 0x9cf9a4) são o Paulista, o Carioca e o Mineiro]
const CQV6_ESTADUAIS: [(usize, usize, usize, i8); 12] = [
    (0x9cf9c0, 0xc, 1, 5), (0x9cf998, 0xb, 0, 7), (0x9cf9c8, 0x7, 2, 5), (0x9cf9b4, 0xd, 1, 5), (0x9cf9d4, 0x9, 2, 5),
    (0x9cf994, 0xf, 0, 7), (0x9cf9a4, 0xe, 1, 7), (0x9cf9d0, 0x4, 2, 5), (0x9cf9bc, 0xa, 1, 5), (0x9cf9dc, 0x5, 2, 5),
    (0x9cf9b0, 0x6, 1, 5), (0x9cf9e4, 0x8, 2, 5),
];

/// O estadual do clube (+0x60; +0 o id do registro), pela ordem dos testes da original.
unsafe fn cqv6_qual(c: usize) -> Option<usize> {
    let id = cqv6_d(cqv6_d(c + 0x60));
    CQV6_ESTADUAIS.iter().position(|e| id == cqv6_d(e.0))
}

/// { 0x427f00 } (thiscall, ret 0, bra_cup.cpp; do construtor da Copa do Brasil, 0x427430, na criação da carreira; n = 1
/// até 2002-06-01): `BraCup::select_entrants()`: os 71 inscritos (o livro +0x14, calloc 0x945501(6, 0x47): push 0x47;
/// push 6; add esp, 8 -> cdecl, 0x47 registros de 6 bytes; +0x36 = +0x38 = 0x47; +0x4b = 0xff; sem memória: a caixa da
/// linha 0x1e0).
/// 1) Pela ordem da tabela dos clubes, cada brasileiro (+0x53 = [0x9cf260]) cujo nome (+4) é um dos 71 fixos
///    (CQV6_NOMES) vai à vaga daquele nome. Com as 71 vagas cheias, acabou.
/// 2) Senão (banco editado): até 200 clubes dos doze estaduais (+0x60) numa lista, ordenada pela reputação (qsort 0x945333
///    com 0x4b07d0: push 0x4b07d0; push 4; push n; push lista; add esp, 0x10 -> cdecl); o melhor de cada estadual vai ao
///    grupo dele (as vagas 0x36.., 0x3d.. ou 0x42..), e até 5 da divisão [0x9cf9ec] (a Série B [inferência]) ao grupo
///    0x36; depois os outros, de cada estadual até 5 (7 nos três maiores), nas vagas de 0 em diante. Sem 54 + 7 + 5 + 5,
///    a caixa da linha 0x3ce.
unsafe fn cqv6_selecao(this: usize) {
    *((this + 0x4b) as *mut u8) = 0xff;
    core::ptr::write_unaligned((this + 0x38) as *mut u16, 0x47);
    core::ptr::write_unaligned((this + 0x36) as *mut u16, 0x47);
    let zerada: unsafe extern "C" fn(u32, u32) -> usize = core::mem::transmute(0x945501usize);
    let livro = zerada(6, 0x47);
    cqv6_p32(this + 0x14, livro as u32);
    if livro == 0 {
        show_error_at(0x98a3bc, 0x1e0);
        return;
    }
    let base = cqv6_d(0xae23b4);
    let mut i: i16 = 0;
    while (i as i32) < cqv6_d(0xae235c) as i32 {
        let c = base + i as u16 as usize * 0x245;
        let nac = cqv6_d(c + 0x53);
        if nac != 0 && cqv6_d(nac) == cqv6_d(0x9cf260) {
            for (k, &t) in CQV6_NOMES.iter().enumerate() { // cqv6: os 71 nomes
                if cqv6_mesmo_nome(c, t) {
                    cqv6_p32(cqv6_d(this + 0x14) + k * 6, c as u32); // cqv6: a vaga do nome
                }
            }
        }
        i = i.wrapping_add(1);
    }
    let mut k: i16 = 0;
    while k < 0x47 {
        if cqv6_d(cqv6_d(this + 0x14) + k as usize * 6) == 0 {
            break;
        }
        k += 1;
    }
    if k == 0x47 {
        crate::comparison::branch("cqv_6 os 71 nomes");
        return;
    }
    crate::comparison::branch("cqv_6 completar pelos estaduais");
    // os contadores da original (bytes da pilha, zerados no começo): [0] a próxima vaga dos outros, [1..3] a conta de
    // cada grupo (0x3d, 0x42, 0x36), [4..0xf] a marca e a conta de cada estadual, [0x10] os da divisão [0x9cf9ec]
    let mut b = [0i8; 0x11];
    let conta_do_grupo = |g: usize| [3usize, 1, 2][g];
    let primeira_do_grupo = |g: usize| [0x36i32, 0x3d, 0x42][g];
    let mut lista = [0usize; 200];
    let mut n: i16 = 0;
    let mut i: i16 = 0;
    while (i as i32) < cqv6_d(0xae235c) as i32 {
        if n >= 0xc8 {
            break;
        }
        let c = base + i as u16 as usize * 0x245;
        if cqv6_d(c + 0x60) != 0 && cqv6_qual(c).is_some() {
            lista[n as usize] = c;
            n += 1;
        }
        i = i.wrapping_add(1);
    }
    let ordenar: unsafe extern "C" fn(usize, u32, u32, usize) = core::mem::transmute(0x945333usize);
    ordenar(lista.as_mut_ptr() as usize, n as i32 as u32, 4, 0x4b07d0);
    for j in 0..n.max(0) as usize {
        let c = lista[j];
        if let Some(e) = cqv6_qual(c) {
            let (_, marca, g, _) = CQV6_ESTADUAIS[e];
            if b[marca] == 0 {
                let cg = conta_do_grupo(g);
                cqv6_p32(cqv6_d(this + 0x14) + (primeira_do_grupo(g) + b[cg] as i32) as usize * 6, c as u32);
                b[marca] = 1;
                b[cg] = b[cg].wrapping_add(1);
                lista[j] = 0;
            }
        }
        let c = lista[j];
        let d = if c != 0 { cqv6_d(c + 0x57) } else { 0 };
        if d != 0 && cqv6_d(d) == cqv6_d(0x9cf9ec) && b[0x10] < 5 {
            cqv6_p32(cqv6_d(this + 0x14) + (0x36 + b[3] as i32) as usize * 6, c as u32);
            b[3] = b[3].wrapping_add(1);
            b[0x10] = b[0x10].wrapping_add(1);
            lista[j] = 0;
        }
    }
    for j in 0..n.max(0) as usize {
        let c = lista[j];
        if c == 0 {
            continue;
        }
        if let Some(e) = cqv6_qual(c) {
            let (_, marca, _, teto) = CQV6_ESTADUAIS[e];
            if b[marca] < teto {
                cqv6_p32(cqv6_d(this + 0x14) + b[0] as i32 as usize * 6, c as u32);
                b[0] = b[0].wrapping_add(1);
                b[marca] = b[marca].wrapping_add(1);
            }
        }
    }
    if !(b[0] == 0x36 && b[3] == 7 && b[1] == 5 && b[2] == 5) {
        show_error_at(0x98a3bc, 0x3ce);
    }
}

unsafe extern "thiscall" fn cqv6_inscritos_da_copa_do_brasil(this: usize) {
    cqv6_selecao(this)
}

/// Lado a lado a partir do mesmo estado (+0x36/+0x38, +0x4b, [0xb67a34]; não sorteia): a original deixa o livro dela em
/// +0x14, a cópia faz o seu, comparado nos 0x47 registros (6 bytes) e liberado.
unsafe extern "thiscall" fn compare_cqv6_inscritos_da_copa_do_brasil(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let antes = cqv6_d(this + 0x14);
    let lo = core::cell::Cell::new(0usize);
    let lr = core::cell::Cell::new(0usize);
    let (_, _, igual, _, _) = crate::comparison::side_by_side(&[(this + 0x36, 4usize), (this + 0x4b, 1), (0xb67a34, 4)], || {
        o(this);
        lo.set(cqv6_d(this + 0x14));
        cqv6_p32(this + 0x14, antes as u32);
    }, || {
        cqv6_selecao(this);
        lr.set(cqv6_d(this + 0x14));
        cqv6_p32(this + 0x14, lo.get() as u32);
    });
    let (lo, lr) = (lo.get(), lr.get());
    let livro = lo == 0 || lr == 0 || core::slice::from_raw_parts(lo as *const u8, 0x47 * 6) == core::slice::from_raw_parts(lr as *const u8, 0x47 * 6);
    if lr != 0 && lr != lo {
        let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
        soltar(lr);
    }
    SCOREBOARD.record(igual && livro, || format!("cqv6_inscritos_da_copa_do_brasil({:#x}) memória {} livro {}", this, igual, livro));
}
