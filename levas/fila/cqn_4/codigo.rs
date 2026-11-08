// ---------- %%LEVA%%a leva: as ligas de fundo francesas: a CFA (4 grupos de 18) e a divisão de baixo (2 grupos de 20): os construtores, as regras e os destrutores (fra_cfa.cpp, fra_lower.cpp) ----------

unsafe fn cqn4_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn4_pd(a: usize, v: usize) {
    core::ptr::write_unaligned(a as *mut u32, v as u32)
}

/// O registro de competição de id [g] (vetor [0xae23d0], 0x6b bytes).
unsafe fn cqn4_registro(g: usize) -> usize {
    cqn4_d(0xae23d0).wrapping_add(cqn4_d(g).wrapping_mul(0x6b))
}

/// { 0x5ba0a0 } (thiscall, ret 0, fra_cfa.cpp; usada também pela divisão de baixo): `FraCfa::set_up_rules()`: turno e
/// returno (+0x3c = 2), 3 pontos por vitória (+0xc2) e 1 por empate (+0xc3), a categoria 2 das partidas (+0xc4), os
/// desempates 4, 1, 2 (+0xc5..+0xc7: [inferência] a posição, o saldo, os gols), o tipo 1 (+0x42), nenhuma vaga de
/// acesso, de play-off nem de rebaixamento (+0xbe..+0xc1 = 0), a liga de cima de id [0x9cf624] (+0x1c) e nenhuma de
/// baixo (+0x20 = −1), os bits de regra 0x43 (+0xd9); por fim o calendário (a virtual +0x3c: push 0; push &+0x3a;
/// push &+0xa9; push −1; mov ecx, esi -> thiscall, ret 0x10) em +0xba. Devolve 1.
unsafe extern "thiscall" fn fra_cfa_set_up_rules(this: usize) -> u32 {
    core::ptr::write_unaligned((this + 0x3c) as *mut u16, 2);
    *((this + 0xc4) as *mut u8) = 2;
    *((this + 0xc7) as *mut u8) = 2;
    *((this + 0xc2) as *mut u8) = 3; // cqn4: os pontos por vitória nas ligas de fundo francesas
    *((this + 0xc3) as *mut u8) = 1;
    *((this + 0x42) as *mut u8) = 1;
    *((this + 0xc5) as *mut u8) = 4;
    *((this + 0xc6) as *mut u8) = 1;
    *((this + 0xbe) as *mut u8) = 0;
    *((this + 0xbf) as *mut u8) = 0;
    *((this + 0xc0) as *mut u8) = 0;
    *((this + 0xc1) as *mut u8) = 0;
    // ponytail: a original grava em +0x1c o id de [0x9cf624] (o mesmo global que a Segunda espanhola usa para a
    // Primera); fiel.
    cqn4_pd(this + 0x1c, cqn4_d(0x9cf624));
    cqn4_pd(this + 0x20, 0xffff_ffff);
    core::ptr::write_unaligned((this + 0xd9) as *mut u16, 0x43); // cqn4: os bits de regra das ligas de fundo
    let calendario: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32) -> usize = core::mem::transmute(cqn4_d(cqn4_d(this) + 0x3c));
    let g = calendario(this, 0xffff_ffff, this + 0xa9, this + 0x3a, 0);
    cqn4_pd(this + 0xba, g);
    1
}

/// O calendário (a virtual +0x3c) aloca: só a original, conferindo os campos e a devolução 1.
unsafe extern "thiscall" fn comparar_fra_cfa_set_up_rules(this: usize) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize) -> u32>(%%I0%%);
    let r = o(this);
    let b = |k: usize| *((this + k) as *const u8);
    let ok = r == 1 && core::ptr::read_unaligned((this + 0x3c) as *const u16) == 2 && b(0xc2) == 3 && b(0xc3) == 1 && b(0xc4) == 2
        && b(0xc5) == 4 && b(0xc6) == 1 && b(0xc7) == 2 && b(0x42) == 1 && b(0xbe) == 0 && b(0xbf) == 0 && b(0xc0) == 0 && b(0xc1) == 0
        && cqn4_d(this + 0x1c) == cqn4_d(0x9cf624) && cqn4_d(this + 0x20) == 0xffff_ffff
        && core::ptr::read_unaligned((this + 0xd9) as *const u16) == 0x43;
    PLACAR.registrar(ok, || format!("fra_cfa_set_up_rules({:#x}): devolveu {}, campos fora do esperado", this, r));
    r
}

/// O que muda entre as duas ligas de fundo.
struct BackgroundLeague {
    vtabela: usize,
    /// o id do registro da competição (+4)
    competicao: usize,
    /// os ids das competições cujos clubes entram todos; os outros vêm da divisão genérica ([0x9cf764])
    todas: &'static [usize],
    /// só os da nação de id [g] (a divisão de baixo: a França) entre os da genérica, e sem competição de reserva
    /// (+0x60 = 0); a CFA pede, ao contrário, a competição de reserva = a própria ([[+0x60]] = [competicao])
    nacao: Option<usize>,
    /// o máximo da lista (72 a CFA, 40 a de baixo) e o máximo dos da genérica (34 e 20)
    maximo: i16,
    genericos: i16,
    /// os grupos e a tabela de cada um (thiscall(liga, lista), ret 4: 0x5b9fa0 e 0x5be230)
    grupos: usize,
    por_grupo: usize,
    tabela: usize,
    arquivo: usize,
    /// as caixas: regras, tabela do grupo, jogos
    linhas: [u32; 3],
}

/// O corpo dos dois construtores (thiscall(this, ano), ret 4, quadro SEH só no caminho de exceção; sem estado salvo):
/// a base (League::League 0x682200: mov ecx -> thiscall, ret 0), a vtable, o ano (+0x40), +0x2c = 0, sem disciplina
/// (+0x50 = 0xff), o registro (+4), a fase 0xff (+0x44) e −1 (+0x30); as regras (0x5ba0a0: mov ecx -> thiscall, ret 0;
/// falhando, a caixa e fim). Depois a lista dos clubes, na ordem do banco (os [0xae235c] − 2 × [0xae2350] primeiros),
/// até o máximo: os das competições `todas`; e, até o máximo dos genéricos, os da divisão genérica com a condição da
/// liga. A lista é embaralhada (comp_util_shuffle_pointer_array 0x4afb40: push quantos; push lista; add esp, 8 ->
/// cdecl; sorteia) e cada grupo, com os seus clubes em sequência na lista, ganha a tabela (falhando, a caixa), os jogos
/// (League::create_fixtures 0x6835c0: mov ecx -> thiscall, ret 0; falhando, a caixa) e a tabela solta
/// (League::free_table 0x687970: push 0; mov ecx -> thiscall, ret 4): os jogos do grupo ficam no calendário do jogo e a
/// liga segue sem tabela. A lista é um registro local de 72 ou 40 ponteiros; a cópia usa um vetor zerado (a carreira
/// enche a lista; a original leria lixo da pilha sem clubes bastantes).
unsafe fn construct_group_league(this: usize, ano: u32, p: &BackgroundLeague) -> usize {
    let base: unsafe extern "thiscall" fn(usize) -> usize = core::mem::transmute(0x682200usize);
    let regras: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x5ba0a0usize);
    let embaralhar: unsafe extern "C" fn(usize, u32) = core::mem::transmute(0x4afb40usize);
    let tabela: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(p.tabela);
    let jogos: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x6835c0usize);
    let soltar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x687970usize);
    base(this);
    cqn4_pd(this, p.vtabela);
    core::ptr::write_unaligned((this + 0x40) as *mut u16, ano as u16);
    cqn4_pd(this + 0x2c, 0);
    *((this + 0x50) as *mut u8) = 0xff;
    cqn4_pd(this + 4, cqn4_registro(p.competicao));
    *((this + 0x44) as *mut u8) = 0xff;
    cqn4_pd(this + 0x30, 0xffff_ffff);
    if regras(this) == 0 {
        show_error_at(p.arquivo, p.linhas[0]);
        return this;
    }
    crate::comparacao::ramo("cqn_4 liga de fundo francesa");
    let mut lista = vec![0usize; p.maximo as usize];
    let mut k: i16 = 0;
    let mut genericos: i16 = 0;
    let n = (cqn4_d(0xae235c) as i32).wrapping_sub((cqn4_d(0xae2350) as i32).wrapping_mul(2));
    let base_clubes = cqn4_d(0xae23b4);
    let mut i: i32 = 0;
    while i < n {
        let c = base_clubes + (i as usize) * 0x245;
        let comp = cqn4_d(c + 0x57);
        let id = if comp != 0 { cqn4_d(comp) } else { 0 };
        if comp != 0 && p.todas.iter().any(|&g| id == cqn4_d(g)) {
            lista[k as usize] = c;
            k += 1;
            if k == p.maximo {
                break;
            }
        } else {
            let entra = match p.nacao {
                Some(g) => {
                    let nacao = cqn4_d(c + 0x53);
                    nacao != 0 && cqn4_d(nacao) == cqn4_d(g) && cqn4_d(c + 0x60) == 0 && comp != 0 && id == cqn4_d(0x9cf764)
                }
                None => {
                    let reserva = cqn4_d(c + 0x60);
                    comp != 0 && id == cqn4_d(0x9cf764) && reserva != 0 && cqn4_d(reserva) == cqn4_d(p.competicao)
                }
            };
            if entra && genericos < p.genericos { // cqn4: o máximo dos clubes da divisão genérica
                lista[k as usize] = c;
                k += 1;
                genericos += 1;
                if k == p.maximo {
                    break;
                }
            }
        }
        i += 1;
    }
    embaralhar(lista.as_mut_ptr() as usize, k as u16 as u32); // cqn4: o sorteio da lista
    for g in 0..p.grupos {
        if tabela(this, lista.as_ptr() as usize + g * p.por_grupo * 4) == 0 {
            show_error_at(p.arquivo, p.linhas[1]);
            return this;
        }
        if jogos(this) == 0 {
            show_error_at(p.arquivo, p.linhas[2]);
            return this;
        }
        soltar(this, 0);
    }
    this
}

const FRA_CFA_CFA_LEAGUE: BackgroundLeague = BackgroundLeague {
    vtabela: 0x96a7b0,
    competicao: 0x9cf5d8,
    todas: &[0x9cf5cc, 0x9cf5d0], // cqn4: os clubes da primeira e da segunda divisões na CFA
    nacao: None,
    maximo: 0x48,
    genericos: 0x22,
    grupos: 4,
    por_grupo: 0x12,
    tabela: 0x5b9fa0,
    arquivo: 0x9ccaf0,
    linhas: [0x40, 0x64, 0x6b],
};
const FRENCH_LOWER_DIVISION: BackgroundLeague = BackgroundLeague {
    vtabela: 0x96ab78,
    competicao: 0x9cf548,
    todas: &[0x9cf5d4],
    nacao: Some(0x9cf300),
    maximo: 0x28,
    genericos: 0x14,
    grupos: 2,
    por_grupo: 0x14,
    tabela: 0x5be230,
    arquivo: 0x9ccbd0,
    linhas: [0x40, 0x62, 0x69],
};

/// { 0x5b91c0 } (thiscall, ret 4, fra_cfa.cpp, 596 bytes): `FraCfa::FraCfa(ano)`, a CFA (a quarta divisão francesa, de
/// fundo; competição de id [0x9cf5d8]): até 72 clubes, os da primeira e da segunda divisões ([0x9cf5cc], [0x9cf5d0];
/// [inferência: os times reserva]) e até 34 da divisão genérica cuja competição de reserva é a CFA, embaralhados em 4
/// grupos de 18. Fonte 0x9ccaf0, linhas 0x40, 0x64, 0x6b.
unsafe extern "thiscall" fn construct_fra_cfa(this: usize, ano: u32) -> usize {
    construct_group_league(this, ano, &FRA_CFA_CFA_LEAGUE)
}

/// { 0x5bd600 } (thiscall, ret 4, fra_lower.cpp, 578 bytes): `FraLower::FraLower(ano)`, a divisão de baixo francesa
/// (competição de id [0x9cf548]): até 40 clubes, os do National ([0x9cf5d4]) e até 20 franceses ([0x9cf300]) da divisão
/// genérica sem competição de reserva, embaralhados em 2 grupos de 20. Fonte 0x9ccbd0, linhas 0x40, 0x62, 0x69.
unsafe extern "thiscall" fn construct_fra_lower(this: usize, ano: u32) -> usize {
    construct_group_league(this, ano, &FRENCH_LOWER_DIVISION)
}

/// Sorteiam e alocam: só a original, conferindo a devolução, a vtable, o ano, a disciplina, o registro e a liga sem
/// tabela no fim (+0x3e = 0, +0xb1 = 0: a última tabela foi solta).
unsafe fn compare_construct_group_league(nome: &str, this: usize, ano: u32, p: &BackgroundLeague, original: impl FnOnce() -> usize) -> usize {
    let r = original();
    let ok = r == this && cqn4_d(this) == p.vtabela && core::ptr::read_unaligned((this + 0x40) as *const u16) == ano as u16
        && *((this + 0x50) as *const u8) == 0xff && cqn4_d(this + 4) == cqn4_registro(p.competicao)
        && cqn4_d(this + 0xb1) == 0;
    PLACAR.registrar(ok, || format!("{}({:#x}): devolveu {:#x}, vtable {:#x}, +4 = {:#x}, +0xb1 = {:#x}", nome, this, r, cqn4_d(this), cqn4_d(this + 4), cqn4_d(this + 0xb1)));
    r
}

unsafe extern "thiscall" fn comparar_construct_fra_cfa(this: usize, ano: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I1%%);
    compare_construct_group_league("construct_fra_cfa", this, ano, &FRA_CFA_CFA_LEAGUE, || o(this, ano))
}

unsafe extern "thiscall" fn comparar_construct_fra_lower(this: usize, ano: u32) -> usize {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32) -> usize>(%%I2%%);
    compare_construct_group_league("construct_fra_lower", this, ano, &FRENCH_LOWER_DIVISION, || o(this, ano))
}

/// O corpo dos dois destrutores (thiscall, ret 0, quadro SEH só no caminho de exceção; iguais fora a vtable): a vtable
/// da classe, a tabela solta (League::free_table 0x687970: push 0 -> thiscall, ret 4), o calendário [+0xba] solto
/// (0x9452ca: push; add esp, 4 -> cdecl) e zerado, cada fase de 0 a +0x30 (com +0x30 ≥ 0) destruída pela sua virtual
/// 0 com 1 (o destrutor que solta: push 1; mov ecx, fase -> thiscall, ret 4), o vetor [+0xc] solto e zerado e, por fim,
/// League::~League (0x682300: mov ecx -> thiscall, ret 0).
unsafe fn destroy_group_league(this: usize, vtabela: usize) {
    let soltar_tabela: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x687970usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let base: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x682300usize);
    cqn4_pd(this, vtabela);
    soltar_tabela(this, 0);
    let g = cqn4_d(this + 0xba);
    if g != 0 {
        soltar(g);
        cqn4_pd(this + 0xba, 0);
    }
    if (cqn4_d(this + 0x30) as i32) >= 0 {
        let mut i: i32 = 0;
        loop {
            let f = cqn4_d(cqn4_d(this + 0xc) + i as usize * 4);
            if f != 0 {
                let destruir: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(cqn4_d(cqn4_d(f)));
                destruir(f, 1);
            }
            i += 1;
            if i > cqn4_d(this + 0x30) as i32 {
                break;
            }
        }
    }
    let v = cqn4_d(this + 0xc);
    if v != 0 {
        soltar(v);
        cqn4_pd(this + 0xc, 0);
    }
    base(this);
}

/// { 0x5b9460 } (thiscall, ret 0, fra_cfa.cpp): `FraCfa::~FraCfa()`.
unsafe extern "thiscall" fn destroy_fra_cfa(this: usize) {
    crate::comparacao::ramo("cqn_4 destrutor da cfa");
    destroy_group_league(this, 0x96a7b0)
}

/// { 0x5bd870 } (thiscall, ret 0, fra_lower.cpp): `FraLower::~FraLower()`.
unsafe extern "thiscall" fn destroy_fra_lower(this: usize) {
    crate::comparacao::ramo("cqn_4 destrutor da divisao de baixo");
    destroy_group_league(this, 0x96ab78)
}

/// Soltam memória: só a original.
unsafe extern "thiscall" fn comparar_destroy_fra_cfa(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I3%%)(this)
}

unsafe extern "thiscall" fn comparar_destroy_fra_lower(this: usize) {
    original_fn::<unsafe extern "thiscall" fn(usize)>(%%I4%%)(this)
}
