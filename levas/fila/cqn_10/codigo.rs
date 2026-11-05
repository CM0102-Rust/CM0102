// ---------- %%LEVA%%a leva: o dia das copas: a virada da Copa da Liga inglesa, as inscrições da Copa da Inglaterra e a convocação do Jogo das Estrelas (eng_cc_cup.cpp, eng_fa_cup.cpp, usa_mls_all_stars.cpp) ----------

unsafe fn cqn10_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const u32) as usize
}

unsafe fn cqn10_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

/// A primeira dword de Date(dia, mês, ano, dia da semana) (0x549f70: push semana; push ano; push mês; push dia; lea ecx,
/// local -> thiscall, ret 0x10): a palavra de baixo é o dia do ano, a de cima o ano.
unsafe fn cqn10_data(dia: u32, mes: u32, ano: u32, semana: u32) -> u32 {
    let construir: unsafe extern "thiscall" fn(*mut [u32; 2], u32, u32, u32, u32) -> usize = core::mem::transmute(0x549f70usize);
    let mut d = [0u32; 2];
    construir(&mut d, dia, mes, ano, semana);
    d[0]
}

/// A data `d` (dword de `cqn10_data`) é `hoje` (o Date de quem chama: +0 o dia do ano, +2 o ano)? A original compara o
/// ano e depois o dia.
unsafe fn cqn10_e_hoje(d: u32, hoje: usize) -> bool {
    (d >> 16) as u16 == cqn10_w(hoje + 2) as u16 && d as u16 == cqn10_w(hoje) as u16
}

/// Libera as tabelas da temporada antes de refazer: a lista dos inscritos (+0xa7), a das rodadas (+0xa3) e os confrontos
/// por rodada (+0xad, um vetor por rodada, +0x3c rodadas), cada um se existe (0x9452ca, cdecl). Os ponteiros ficam como
/// estão (as rotinas seguintes os refazem).
unsafe fn cqn10_liberar(this: usize) {
    let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x9452causize);
    if cqn10_d(this + 0xa7) != 0 {
        soltar(cqn10_d(this + 0xa7));
    }
    if cqn10_d(this + 0xa3) != 0 {
        soltar(cqn10_d(this + 0xa3));
    }
    if cqn10_d(this + 0xad) != 0 {
        let mut i = 0i32;
        while i < cqn10_w(this + 0x3c) as i32 {
            let p = cqn10_d(cqn10_d(this + 0xad) + i as usize * 4);
            if p != 0 {
                soltar(p);
            }
            i += 1;
        }
        soltar(cqn10_d(this + 0xad));
    }
}

/// A virtual `o` da copa (thiscall, sem argumentos; mov ecx, copa; call [vtable + o]).
unsafe fn cqn10_virtual(this: usize, o: usize) -> i32 {
    let f: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(cqn10_d(cqn10_d(this) + o));
    f(this)
}

/// A atualização da copa (0x51f890: push fase; push hoje; mov ecx, copa -> thiscall, ret 8), cuja devolução sai.
unsafe fn cqn10_base(this: usize, hoje: usize, fase: u32) -> u32 {
    let base: unsafe extern "thiscall" fn(usize, usize, u32) -> u32 = core::mem::transmute(0x51f890usize);
    base(this, hoje, fase)
}

/// { 0x56caf0 } (thiscall, ret 8, eng_cc_cup.cpp; a virtual +0x14 da Copa da Liga): `EngCcCup::update_for_date(hoje,
/// fase)`. Uma vez (+0x45 = 0), quando o ano do jogo ([0xde1f58] +0x439, palavra sem sinal) chega ao da copa (+0x40, com
/// sinal) e a fase não é 0: libera as tabelas (`cqn10_liberar`), +0x45 = 1, os inscritos (0x56ccc0, thiscall), as
/// rodadas (virtual +0x8c), a árvore dos confrontos (virtual +0x94) e a ordem prevista (virtual +0x5c); uma devolução
/// negativa dá a caixa (linhas 0x161, 0x168, 0x16f) e 0. Sempre, no fim, a atualização da copa.
unsafe extern "thiscall" fn cqn10_copa_da_liga_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqn10_d(this + 0x45) == 0 && cqn10_w(cqn10_d(0xde1f58) + 0x439) as u16 as i32 == cqn10_w(this + 0x40) as i32 && fase != 0 {
        cqn10_liberar(this);
        crate::comparacao::ramo("cqn_10 virada da copa da liga");
        core::ptr::write_unaligned((this + 0x45) as *mut u32, 1); // cqn10: a Copa da Liga refeita uma vez
        let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56ccc0usize);
        // ponytail: nas caixas a original zera só o al (o resto do eax vem da caixa); quem chama lê o byte
        if inscritos(this) < 0 {
            aviso_em(0x9c4224, 0x161);
            return 0;
        }
        if cqn10_virtual(this, 0x8c) < 0 {
            aviso_em(0x9c4224, 0x168);
            return 0;
        }
        if cqn10_virtual(this, 0x94) < 0 {
            aviso_em(0x9c4224, 0x16f);
            return 0;
        }
        cqn10_virtual(this, 0x5c);
    }
    cqn10_base(this, hoje, fase)
}

// ponytail: a virada libera e refaz as tabelas e a atualização da copa joga e sorteia: não rodam duas vezes. Só a
// original; o plano: +0x45 vai a 1 quando a condição vale, senão fica.
unsafe extern "thiscall" fn comparar_cqn10_copa_da_liga_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I0%%);
    let antes = cqn10_d(this + 0x45);
    let vira = antes == 0 && cqn10_w(cqn10_d(0xde1f58) + 0x439) as u16 as i32 == cqn10_w(this + 0x40) as i32 && fase != 0;
    let r = o(this, hoje, fase);
    let depois = cqn10_d(this + 0x45);
    PLACAR.registrar(depois == if vira { 1 } else { antes }, || format!("cqn10_copa_da_liga_dia({:#x}) vira {} +0x45 {} -> {}", this, vira, antes, depois));
    r
}

/// { 0x570550 } (thiscall, ret 8, eng_fa_cup.cpp; a virtual +0x14 da Copa da Inglaterra; quadro SEH pela notícia do
/// caminho de 1999, sem destrutor no caminho normal): `EngFaCup::update_for_date(hoje, fase)`. Com +0x45 = 0 e a fase
/// diferente de 0: em Date(1, 9, 1999) (o mês 9 é outubro) com +0xb2 = 0, a pergunta ao técnico humano sobre a entrada na
/// 5ª rodada (só em 1999: a original pelo trampolim); em Date(3, 9, ano de hoje) (3 de outubro), libera as tabelas, +0x45
/// = 1, os inscritos (0x56fab0, thiscall), as rodadas (virtual +0x8c), a árvore (virtual +0x94) — uma devolução negativa
/// dá a caixa (linhas 0x3d6, 0x3dd, 0x3e4) e 1 —, a ordem prevista (virtual +0x5c) e o sorteio da 1ª rodada (0x51c800:
/// push 0; mov ecx, copa -> thiscall, ret 4). Sempre, no fim, a atualização da copa.
unsafe extern "thiscall" fn cqn10_copa_da_inglaterra_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    if cqn10_d(this + 0x45) == 0 && fase != 0 {
        if cqn10_e_hoje(cqn10_data(1, 9, 0x7cf, 0xffff_ffff), hoje) && *((this + 0xb2) as *const u8) == 0 {
            // ponytail: a pergunta de 1999 monta e entrega uma notícia (só nesse dia, fora de qualquer carreira nova)
            return orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%)(this, hoje, fase);
        }
        // ponytail: o ano vai no ax com o resto do eax nos bytes altos; a 0x549f70 o lê como palavra
        if cqn10_e_hoje(cqn10_data(3, 9, cqn10_w(hoje + 2) as u16 as u32, 0xffff_ffff), hoje) { // cqn10: as inscrições da Copa da Inglaterra em 3/10
            cqn10_liberar(this);
            crate::comparacao::ramo("cqn_10 inscricoes da copa da inglaterra");
            core::ptr::write_unaligned((this + 0x45) as *mut u32, 1);
            let inscritos: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x56fab0usize);
            if inscritos(this) < 0 {
                aviso_em(0x9c42cc, 0x3d6);
                return 1;
            }
            if cqn10_virtual(this, 0x8c) < 0 {
                aviso_em(0x9c42cc, 0x3dd);
                return 1;
            }
            if cqn10_virtual(this, 0x94) < 0 {
                aviso_em(0x9c42cc, 0x3e4);
                return 1;
            }
            cqn10_virtual(this, 0x5c);
            let sortear: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(0x51c800usize);
            sortear(this, 0);
        }
    }
    cqn10_base(this, hoje, fase)
}

// ponytail: como a da Copa da Liga, só a original; o plano: +0x45 vai a 1 em 3 de outubro (com +0x45 = 0 e a fase), fora
// do caminho de 1999.
unsafe extern "thiscall" fn comparar_cqn10_copa_da_inglaterra_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I1%%);
    let antes = cqn10_d(this + 0x45);
    let vira = antes == 0 && fase != 0 && cqn10_e_hoje(cqn10_data(3, 9, cqn10_w(hoje + 2) as u16 as u32, 0xffff_ffff), hoje)
        && !(cqn10_e_hoje(cqn10_data(1, 9, 0x7cf, 0xffff_ffff), hoje) && *((this + 0xb2) as *const u8) == 0);
    let r = o(this, hoje, fase);
    let depois = cqn10_d(this + 0x45);
    PLACAR.registrar(!vira || depois == 1, || format!("cqn10_copa_da_inglaterra_dia({:#x}) vira {} +0x45 {} -> {}", this, vira, antes, depois));
    r
}

/// { 0x90a900 } (thiscall, ret 8, usa_mls_all_stars.cpp; a virtual +0x14 do Jogo das Estrelas; quadro SEH pela Date
/// local, sem destrutor): `UsaMlsAllStars::update_for_date(hoje, fase)`: com a fase diferente de 0, no dia de Date(26, 6,
/// ano de hoje, dia da semana 6) (o sábado de 26 de julho em diante [inferência]), a convocação das duas seleções
/// (0x90a980, mov ecx -> thiscall, ret 0); depois a atualização da copa.
unsafe extern "thiscall" fn cqn10_jogo_das_estrelas_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    // ponytail: o ano vai num registrador com lixo nos 16 bits altos; a 0x549f70 o lê como palavra
    let ano = cqn10_w(hoje + 2) as u16 as u32;
    let d = cqn10_data(0x1a, 6, ano, 6); // cqn10: a convocação do Jogo das Estrelas
    if fase != 0 && cqn10_e_hoje(d, hoje) {
        crate::comparacao::ramo("cqn_10 convocacao do jogo das estrelas");
        let convocar: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x90a980usize);
        convocar(this);
    }
    cqn10_base(this, hoje, fase)
}

// ponytail: a convocação mexe nos elencos e a atualização da copa joga: não rodam duas vezes. Só a original roda.
unsafe extern "thiscall" fn comparar_cqn10_jogo_das_estrelas_dia(this: usize, hoje: usize, fase: u32) -> u32 {
    orig::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%)(this, hoje, fase)
}
