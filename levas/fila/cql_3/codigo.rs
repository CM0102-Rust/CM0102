// ---------- %%LEVA%%a leva: os números de camisa: numerar as ligas no começo do jogo e a inscrição nas copas europeias (squad_manager.cpp) ----------

unsafe fn cql3_d(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}

/// As ligas que a 0x859dc0 da original mandou numerar (gravadas pelo comparador da 0x859e20 enquanto o da 0x859dc0
/// roda a original), para conferir o plano da cópia.
static mut SQUAD_NUMBERS_PLAN: Option<Vec<usize>> = None;

/// As ligas que a 0x859dc0 numera: as competições da tabela [0xadadfc] (conta [0xadadf8], relida a cada volta) que
/// são liga de divisão (0x4b0700 com o id [[+4]], cdecl de 1) e usam números (bit 2 do +0x52).
unsafe fn for_each_numbered_league(mut cada: impl FnMut(usize)) {
    let divisao: unsafe extern "C" fn(i32) -> u32 = core::mem::transmute(0x4b0700usize);
    let mut i = 0i32;
    while i < cql3_d(0xadadf8) as i32 {
        let c = cql3_d(cql3_d(0xadadfc) as usize + 4 * i as usize) as usize;
        if c != 0 && divisao(cql3_d(cql3_d(c + 4) as usize) as i32) != 0 && *((c + 0x52) as *const u8) & 2 != 0 {
            crate::comparison::branch("cql_3 numera a liga");
            cada(c); // cql3: numera a liga
        }
        i += 1;
    }
}

/// { 0x859dc0 } (thiscall, ret 0, squad_manager.cpp, `SQUAD_MANAGER::init_new_game`): zera o ano das inscrições (+0,
/// word) e numera (0x859e20(this, liga, 1, 0)) cada liga de divisão que usa números (`for_each_numbered_league`).
unsafe extern "thiscall" fn init_new_game(this: usize) {
    let numerar: unsafe extern "thiscall" fn(usize, usize, u32, u32) = core::mem::transmute(0x859e20usize);
    *(this as *mut u16) = 0;
    for_each_numbered_league(|c| numerar(this, c, 1, 0));
}

/// Só a original (ela numera as ligas pela 0x859e20, que manda notícia ao técnico humano: não roda duas vezes); o plano
/// da cópia (as ligas, achadas antes, sem numerar) tem de ser o que a original fez (gravado pelo comparador da
/// 0x859e20) e o +0 zerado.
unsafe extern "thiscall" fn compare_init_new_game(this: usize) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let mut plano = Vec::new();
    for_each_numbered_league(|c| plano.push(c));
    SQUAD_NUMBERS_PLAN = Some(Vec::new());
    o(this);
    let feito = SQUAD_NUMBERS_PLAN.take().unwrap_or_default();
    let zerado = *(this as *const u16) == 0;
    SCOREBOARD.record(feito == plano && zerado, || format!("init_new_game: plano {:x?} original {:x?} ano zerado {}", plano, feito, zerado));
}

/// { 0x859e20 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::assign_league_squad_numbers`): numera os clubes
/// da liga `liga` (a lista do método +0x58 da competição, thiscall(liga, &lista, 0), devolve a conta em ax; a lista é
/// solta no fim pela 0x9452ca). Em 1998 vale sempre `inicial`. Pula o time B (0x540a50(clube, &principal, 1) com o
/// clube ligado e principal = 0). Clube de técnico humano (0x7875e0, com ecx = [0xb7431c]) sem `automatico`: os números
/// negativos do elenco (0x541f90(clube, lista, 0)) viram positivos e o técnico recebe o pedido da lista de números
/// (0x85aec0, original). Senão: com `inicial`, cada jogador do elenco (da tabela das pessoas com contrato e com ficha)
/// recebe o número do banco (o +4 da ficha, entre 0 e 50); e o elenco é numerado (0x859fe0(clube, 1)).
unsafe extern "thiscall" fn assign_league_squad_numbers(this: usize, liga: usize, mut inicial: u32, automatico: u32) {
    let parceiro: unsafe extern "C" fn(usize, *mut u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let pedido: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x85aec0usize);
    let numero: unsafe extern "thiscall" fn(usize, usize, usize) -> i8 = core::mem::transmute(0x85a760usize);
    let gravar: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x85a850usize);
    let numerar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x859fe0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if *(0xae2c92 as *const u16) == 0x7ce {
        inicial = 1;
    }
    let clubes: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(cql3_d(cql3_d(liga) as usize + 0x58) as usize);
    let mut lista = 0usize;
    let n = clubes(liga, &mut lista, 0) as i16 as i32;
    for i in 0..n.max(0) as usize {
        let c = cql3_d(lista + 4 * i) as usize;
        let mut principal = 0u32;
        if parceiro(c, &mut principal, 1) != 0 && principal == 0 {
            continue;
        }
        let mut sq = [0usize; 50];
        elenco(c, sq.as_mut_ptr(), 0);
        if humano(cql3_d(0xb7431c) as usize, c) != 0 && automatico == 0 {
            for &p in sq.iter() {
                if p != 0 && numero(this, p, c) < 0 {
                    gravar(this, p, c, (numero(this, p, c) as u8).wrapping_neg() as u32);
                }
            }
            pedido(this, c);
            continue;
        }
        if inicial != 0 {
            for &p in sq.iter() {
                if p == 0 || cql3_d(p) as i32 >= cql3_d(0xae2364) as i32 {
                    continue;
                }
                let f = cql3_d(p + 0x61) as usize;
                if f == 0 {
                    continue;
                }
                let b = *((f + 4) as *const i8);
                let k = if b < 0 { 0 } else if b > 0x32 { 0x32 } else { b as i32 };
                crate::comparison::branch("cql_3 numero do banco");
                gravar(this, p, c, k as u32); // cql3: o número do banco
            }
        }
        numerar(this, c, 1);
    }
    if lista != 0 {
        soltar(lista);
    }
}

/// Lado a lado: o +0x3a dos contratos (o do clube e o de empréstimo) de quem está no elenco de cada clube da liga e
/// no do clube ligado, e a marca da caixa. Com algum clube de técnico humano sem `automatico` (a notícia do pedido não
/// roda duas vezes), só a original. A lista dos clubes vem do mesmo método +0x58 (solta depois).
unsafe extern "thiscall" fn compare_assign_league_squad_numbers(this: usize, liga: usize, inicial: u32, automatico: u32) {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32)>(%%I1%%);
    if let Some(v) = (*core::ptr::addr_of_mut!(SQUAD_NUMBERS_PLAN)).as_mut() {
        v.push(liga);
    }
    let parceiro: unsafe extern "C" fn(usize, u32, u32) -> usize = core::mem::transmute(0x540a50usize);
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    let clubes: unsafe extern "thiscall" fn(usize, *mut usize, u32) -> u16 = core::mem::transmute(cql3_d(cql3_d(liga) as usize + 0x58) as usize);
    let mut lista = 0usize;
    let n = clubes(liga, &mut lista, 0) as i16 as i32;
    let cs: Vec<usize> = (0..n.max(0) as usize).map(|i| cql3_d(lista + 4 * i) as usize).collect();
    if lista != 0 {
        soltar(lista);
    }
    if automatico == 0 && cs.iter().any(|&c| humano(cql3_d(0xb7431c) as usize, c) != 0) {
        return o(this, liga, inicial, automatico);
    }
    let mut reg = vec![(0xb67a34usize, 4usize)];
    for &c in &cs {
        let l = parceiro(c, 0, 1);
        for q in [c, l] {
            if q == 0 {
                continue;
            }
            for k in 0..50 {
                let p = cql3_d(q + 0xd7 + 4 * k) as usize;
                if p == 0 {
                    continue;
                }
                for ct in [contrato(cql3_d(0xae1050) as usize, p), emprestimo(cql3_d(0xae1050) as usize, p)] {
                    if ct != 0 && !reg.contains(&(ct + 0x3a, 1)) {
                        reg.push((ct + 0x3a, 1));
                    }
                }
            }
        }
    }
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, liga, inicial, automatico), || assign_league_squad_numbers(this, liga, inicial, automatico));
    SCOREBOARD.record(igual, || {
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).take(12).map(|(a, b)| format!("{:x}: {:?}/{:?}", a.0, a.1, b.1)).collect();
        format!("assign_league_squad_numbers(liga {:x}, {} {}): {}", liga, inicial, automatico, d.join(", "))
    });
}

/// As vagas da escalação de referência da 0x85aae0: os 11 titulares e até 14 reservas (as da 0x859fe0, `SQUAD_MANAGER_REFERENCE_LINEUP_SLOTS`,
/// até a 19ª; a 20ª aqui é de goleiro).
const EURO_REFERENCE_LINEUP_SLOTS: [u16; 25] = [
    0x1, 0x804, 0x84, 0x210, 0x204, 0x204, 0x810, 0x210, 0x240, 0x240, 0x90, // os titulares
    0x10, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, 0x40, 0x1, 0x4, 0x40, 0x40, 0x10, 0x4, // os reservas
];

/// { 0x85aae0 } (thiscall, ret 0xc, squad_manager.cpp, `SQUAD_MANAGER::select_euro_registered_players`; o this não é
/// lido): a inscrição do clube `c` nas copas europeias na etapa `etapa` (1 a primeira; `marca` [inferência: a lista A
/// ou B]), pela marca de inscrição da pessoa (+0x6d: 1 ou 2 inscrito, 4 e 8 outras marcas). Do elenco (0x541f90(c,
/// lista, 0)), os candidatos são os da tabela das pessoas com contrato, sem contrato do tipo 5 (+0x35 & 0x3f; o
/// contrato achado fica em [0xae1080]), sem contrato de empréstimo e sem as marcas 8 e 0x10 (+0x6d & 0x18); fora da
/// etapa 1, a marca 1 cai para 0 e a 4 vira 2, e os 2 são contados. A escalação de referência (0x6e2000 com
/// `EURO_REFERENCE_LINEUP_SLOTS`, os 11 titulares e 14 reservas, ou 9 na etapa 1 sem `marca`) escolhe os melhores. Na etapa 3, até 2
/// inscritos (2) que ficaram fora da escalação perdem a inscrição (vão a 4 com `marca`, senão 0) e a conta baixa (no
/// mínimo 21). Depois cada escalado: na etapa 1, é inscrito (1 com `marca`, senão 2) e contado; nas outras, quem ainda
/// não é 2 nem tem a marca 8 é inscrito do mesmo jeito enquanto a conta for menor que as vagas da escalação. Devolve a
/// conta dos inscritos (char).
unsafe extern "thiscall" fn select_euro_registered_players(_this: usize, c: usize, etapa: u32, marca: u32) -> u8 {
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let contrato: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let emprestimo: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf6d0usize);
    let pick: AssignPositionsFn = core::mem::transmute(0x6e2000usize);
    let etapa = etapa as u8;
    let mut sq = [0usize; 50];
    elenco(c, sq.as_mut_ptr(), 0);
    let reservas: u8 = if etapa == 1 { if marca != 0 { 0xe } else { 9 } } else { 0xe };
    let mut cand = [0usize; 50];
    let mut n = 0u8;
    let mut conta = 0i8;
    let mut tirados = 0i8;
    let m6d = |p: usize| (p + 0x6d) as *mut u8;
    for &p in sq.iter() {
        if p == 0 || cql3_d(p) as i32 >= cql3_d(0xae2364) as i32 {
            continue;
        }
        let k = contrato(cql3_d(0xae1050) as usize, p);
        *(0xae1080 as *mut u32) = k as u32;
        if k != 0 && *((k + 0x35) as *const u8) & 0x3f == 5 {
            continue;
        }
        if emprestimo(cql3_d(0xae1050) as usize, p) != 0 || *m6d(p) & 0x18 != 0 {
            continue;
        }
        cand[n as i8 as usize] = p;
        n = n.wrapping_add(1);
        let a = *m6d(p);
        if a == 0 || etapa == 1 {
            continue;
        }
        if a == 1 {
            *m6d(p) = 0;
        } else if a == 4 {
            *m6d(p) = 2;
        }
        if *m6d(p) == 2 {
            conta = conta.wrapping_add(1);
        }
    }
    let vagas = EURO_REFERENCE_LINEUP_SLOTS;
    let mut esc = [0usize; 50];
    pick(cand.as_ptr() as usize, n as u32, vagas.as_ptr() as usize, 0xb, reservas as u32, esc.as_mut_ptr() as usize, 0x5840, c, 0, 0, 0, 0, 0, 0, 0);
    if etapa == 3 {
        let mut d = 0i8;
        while d < n as i8 && tirados < 2 {
            let p = cand[d as usize];
            if p != 0 && *m6d(p) == 2 && !esc[..0x19].contains(&p) {
                *m6d(p) = if marca != 0 { 4 } else { 0 };
                tirados += 1;
                conta = conta.wrapping_sub(1);
                if conta <= 0x15 {
                    conta = 0x15;
                }
            }
            d += 1;
        }
    }
    let total = reservas as i8 as i32 + 0xb;
    let mut b = 0i32;
    while b < total {
        let p = esc[b as usize];
        if p != 0 {
            let v = if marca != 0 { 1 } else { 2 };
            if etapa == 1 {
                crate::comparison::branch("cql_3 inscreve na etapa 1");
                *m6d(p) = v; // cql3: inscreve
                conta = conta.wrapping_add(1);
            } else {
                let a = *m6d(p);
                if a != 2 && a & 8 == 0 && (conta as i32) < total {
                    crate::comparison::branch("cql_3 inscreve depois");
                    *m6d(p) = v; // cql3: inscreve depois
                    conta = conta.wrapping_add(1);
                }
            }
        }
        b = (b as i8).wrapping_add(1) as i32;
    }
    conta as u8
}

/// Lado a lado: a marca de inscrição (+0x6d) de quem está no elenco (a mesma 0x541f90), o contrato achado [0xae1080]
/// e a devolução.
unsafe extern "thiscall" fn compare_select_euro_registered_players(this: usize, c: usize, etapa: u32, marca: u32) -> u8 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32, u32) -> u8>(%%I2%%);
    let elenco: unsafe extern "C" fn(usize, *mut usize, u32) -> u32 = core::mem::transmute(0x541f90usize);
    let mut sq = [0usize; 50];
    elenco(c, sq.as_mut_ptr(), 0);
    let mut reg = vec![(0xae1080usize, 4usize)];
    for &p in sq.iter() {
        if p != 0 && !reg.contains(&(p + 0x6d, 1)) {
            reg.push((p + 0x6d, 1));
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(this, c, etapa, marca), || select_euro_registered_players(this, c, etapa, marca));
    SCOREBOARD.record(igual && r == n, || {
        let d: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter(|(a, b)| a.1 != b.1).take(12).map(|(a, b)| format!("{:x}: {:?}/{:?}", a.0, a.1, b.1)).collect();
        format!("select_euro_registered_players(clube {:x}, etapa {}, marca {}): original {} rust {} {}", c, etapa as u8, marca, r, n, d.join(", "))
    });
    r
}
