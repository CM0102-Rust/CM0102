// ---------- %%LEVA%%a leva: o construtor da primeira divisão francesa: as duas competições-marcador, os clubes da CFA e da divisão de baixo levados à divisão genérica e as duas ligas de fundo montadas e soltas (fra_first.cpp) ----------

unsafe fn cqq6_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqq6_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes; a original faz id × 3 × 9 × 4 − id).
unsafe fn cqq6_registro(g: usize) -> usize {
    cqq6_d(0xae23d0).wrapping_add(cqq6_d(g).wrapping_mul(0x6b))
}

/// Uma competição-marcador (operator new 0x944cf1 de 0xa3: push; add esp, 4 -> cdecl; sem memória: 0): Comp::Comp
/// (0x48c580: mov ecx -> thiscall, ret 0), a vtable `vtabela`, +0x42 = 1, o registro de id [g] em +4 e sem disciplina
/// (+0x50 = 0xff).
unsafe fn new_marker_comp(vtabela: usize, g: usize) -> usize {
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let comp: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x48c580usize);
    let c = novo(0xa3);
    if c == 0 {
        return 0;
    }
    comp(c);
    cqq6_pd(c, vtabela);
    *((c + 0x42) as *mut u8) = 1;
    cqq6_pd(c + 4, cqq6_registro(g));
    *((c + 0x50) as *mut u8) = 0xff;
    c
}

/// { 0x5bb160 } (thiscall, ret 8, fra_first.cpp, 988 bytes, quadro SEH só para os operator new): `FraFirst::FraFirst(ano,
/// registro)`: a base (League::League 0x682200: mov ecx -> thiscall, ret 0), a vtable 0x96aa24, o ano (+0x40, palavra),
/// o registro (+4) e a disciplina 0xc (+0x50); em +0xee e +0xf2 as competições-marcador da CFA (vtable 0x96a994, id
/// [0x9cf5d8]) e da divisão de baixo (0x96a904, id [0x9cf548]). Tenta carregar do arquivo temporário
/// (League::load_from_temp_file 0x687b10: push 1; mov ecx -> thiscall, ret 4): carregou, pronto. Senão, a liga nova:
/// cada clube (os [0xae235c] − 2 × [0xae2350] primeiros de [0xae23b4], 0x245 bytes; a conta refeita a cada volta) cuja
/// competição (+0x57) é a CFA passa à divisão genérica (registro de id [0x9cf764]) com a CFA como competição de reserva
/// (+0x60); o da divisão de baixo, à genérica sem reserva (+0x60 = 0). Depois fase 0xff (+0x44), fase atual −1 (+0x30),
/// sem fases (+0x2c, +0xc); as regras (0x5bc410), a tabela (0x5bc2b0), os jogos (League::create_fixtures 0x6835c0), a
/// ordem (League::sort_table 0x6827d0: push 0 -> thiscall, ret 4), as estatísticas (operator new de 0x5ce e
/// Comp_Stats::Comp_Stats 0x49ee70: push liga; mov ecx, novo -> thiscall, ret 4) em +8 e as posições previstas
/// (0x68a850: mov ecx -> thiscall, ret 0); cada passo que falha mostra a caixa da sua linha (0x65, 0x6c, 0x73, 0x7c,
/// 0x83) e encerra. Por fim, as duas ligas de fundo: a CFA (operator new de 0xee; FraCfa::FraCfa 0x5b91c0: push ano;
/// mov ecx -> thiscall, ret 4) e a divisão de baixo (0x5bd600, o mesmo), que deixam os seus jogos no calendário do jogo,
/// e são destruídas (a virtual 0 com 1: push 1; mov ecx -> thiscall, ret 4). Devolve this.
unsafe extern "thiscall" fn construct_fra_first(this: usize, ano: u32, registro: usize) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let carregar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x687b10usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5bc410usize);
    let tabela: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5bc2b0usize);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let ordenar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x6827d0usize);
    let novo: unsafe extern "C" fn(u32) -> usize = core::mem::transmute(0x944cf1usize);
    let estatisticas: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x49ee70usize);
    let previstas: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68a850usize);
    let cfa: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x5b91c0usize);
    let baixo: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x5bd600usize);
    base(this);
    cqq6_pd(this, 0x96aa24);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    cqq6_pd(this + 4, registro);
    *((this + 0x50) as *mut u8) = 0xc; // cqq6: a disciplina da primeira francesa
    let m = new_marker_comp(0x96a994, 0x9cf5d8);
    cqq6_pd(this + 0xee, m);
    let m = new_marker_comp(0x96a904, 0x9cf548);
    cqq6_pd(this + 0xf2, m);
    if carregar(this, 1) != 0 {
        return this;
    }
    crate::comparison::branch("cqq_6 new french first division");
    let n = || (cqq6_d(0xae235c) as i32).wrapping_sub((cqq6_d(0xae2350) as i32).wrapping_mul(2));
    if n() > 0 {
        let mut p = cqq6_d(0xae23b4) + 0x57;
        let mut i: i32 = 0;
        loop {
            let comp = cqq6_d(p);
            if comp != 0 {
                if cqq6_d(comp) == cqq6_d(0x9cf5d8) {
                    cqq6_pd(p + 9, comp); // cqq6: a CFA vira a competição de reserva
                    cqq6_pd(p, cqq6_registro(0x9cf764));
                } else if cqq6_d(comp) == cqq6_d(0x9cf548) {
                    cqq6_pd(p + 9, 0); // cqq6: a divisão de baixo fica sem reserva
                    cqq6_pd(p, cqq6_registro(0x9cf764));
                }
            }
            i += 1;
            p += 0x245;
            if i >= n() {
                break;
            }
        }
    }
    *((this + 0x44) as *mut u8) = 0xff;
    cqq6_pd(this + 0x30, 0xffff_ffff);
    cqq6_pd(this + 0x2c, 0);
    cqq6_pd(this + 0xc, 0);
    if regras(this) == 0 {
        show_error_at(0x9ccb5c, 0x65);
        return this;
    }
    if tabela(this) == 0 {
        show_error_at(0x9ccb5c, 0x6c);
        return this;
    }
    if jogos(this) == 0 {
        show_error_at(0x9ccb5c, 0x73);
        return this;
    }
    if ordenar(this, 0) == 0 {
        show_error_at(0x9ccb5c, 0x7c);
        return this;
    }
    let e = novo(0x5ce);
    let s = if e != 0 { estatisticas(e, this) } else { 0 };
    cqq6_pd(this + 8, s);
    if s == 0 {
        show_error_at(0x9ccb5c, 0x83);
        return this;
    }
    previstas(this);
    let ano = core::ptr::read_unaligned((this + 0x40) as *const u16) as u32;
    let a = novo(0xee);
    let ligas_cfa = if a != 0 { cfa(a, ano) } else { 0 };
    let b = novo(0xee);
    let ligas_baixo = if b != 0 { baixo(b, ano) } else { 0 };
    for l in [ligas_cfa, ligas_baixo] {
        if l != 0 {
            let destruir: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(cqq6_d(cqq6_d(l)));
            destruir(l, 1);
        }
    }
    this
}

/// Sorteia (as ligas de fundo), aloca e mexe nos clubes: só a original, conferindo a devolução, a vtable, o ano, o
/// registro, a disciplina e os dois marcadores (vtable e registro) e, na liga nova, os campos do começo e as
/// estatísticas.
unsafe extern "thiscall" fn compare_construct_fra_first(this: usize, ano: u32, registro: usize) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, usize) -> usize>(%%I0%%);
    let r = o(this, ano, registro);
    let marcador = |m: usize, vt: usize, g: usize| m == 0 || (cqq6_d(m) == vt && cqq6_d(m + 4) == cqq6_registro(g) && *((m + 0x50) as *const u8) == 0xff);
    let mut ok = r == this && cqq6_d(this) == 0x96aa24 && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && cqq6_d(this + 4) == registro && *((this + 0x50) as *const u8) == 0xc
        && marcador(cqq6_d(this + 0xee), 0x96a994, 0x9cf5d8) && marcador(cqq6_d(this + 0xf2), 0x96a904, 0x9cf548);
    if *((this + 0x44) as *const u8) == 0xff && cqq6_d(this + 8) != 0 {
        ok = ok && cqq6_d(this + 0x2c) == 0 && cqq6_d(this + 0xc) == 0;
    }
    SCOREBOARD.record(ok, || format!("construct_fra_first({:#x}): devolveu {:#x}, vtable {:#x}, +0xee = {:#x}, +0xf2 = {:#x}, +8 = {:#x}", this, r, cqq6_d(this), cqq6_d(this + 0xee), cqq6_d(this + 0xf2), cqq6_d(this + 8)));
    r
}
