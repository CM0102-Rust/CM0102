// ---------- %%LEVA%%a leva: a lesão nova (o registro, a duração pelo fisioterapeuta, a recaída e a aposentadoria), a notícia da lesão e a ausência sem licença (injury.cpp, awol.cpp) ----------

unsafe fn cqb5_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cqb5_b(a: usize) -> u8 {
    *(a as *const u8)
}

unsafe fn cqb5_c(a: usize) -> i32 {
    *(a as *const i8) as i32
}

unsafe fn cqb5_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cqb5_por_w(a: usize, v: i16) {
    core::ptr::write_unaligned(a as *mut i16, v);
}

unsafe fn cqb5_sorteio(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// O registro físico de 31 bytes da pessoa `id` em [this].
unsafe fn physical_record(this: usize, id: usize) -> usize {
    cqb5_d(this).wrapping_add(id.wrapping_mul(0x1f))
}

/// A lesão `i` (0x30 bytes) do vetor [this+4].
unsafe fn injury_entry(this: usize, i: usize) -> usize {
    cqb5_d(this + 4).wrapping_add(i.wrapping_mul(0x30))
}

/// O contrato da pessoa (0x4cf5a0: thiscall([0xae1050], pessoa), ret 4), guardado em [0xae1080].
unsafe fn injury_main_contract_stored(p: usize) -> usize {
    let f: unsafe extern "thiscall" fn(usize, usize) -> usize = core::mem::transmute(0x4cf5a0usize);
    let c = f(cqb5_d(0xae1050), p);
    *(0xae1080 as *mut usize) = c;
    c
}

/// O clube do contrato `c` (+4 = id no registro de 0x245 bytes em [0xae23b4]; 0 sem contrato).
unsafe fn injury_club_of_contract(c: usize) -> usize {
    if c == 0 { 0 } else { cqb5_d(0xae23b4).wrapping_add(cqb5_d(c + 4).wrapping_mul(0x245)) }
}

/// A tabela dos tipos de lesão (13 bytes por tipo): +0 (0x9e7adb) dias de base (i16), +2 (0x9e7add) dias variáveis
/// (i16), +4 (0x9e7adf) percentual da volta ao treino (i8), +5 (0x9e7ae0) pode voltar (i32).
fn injury_type_offset(tipo: i8) -> usize {
    (tipo as i32 * 13) as usize
}

/// Os dias da volta ao treino (+0x2a) pela duração `dias`: ftol(percentual × 0,01 × dias) (fild, fmul qword
/// [0x967898], fimul, _ftol 0x945640; x87 em dupla precisão), no máximo 60 dias antes do fim.
unsafe fn return_to_training_days(tipo: i8, dias: i16) -> i16 {
    let pct = cqb5_c(0x9e7adf + injury_type_offset(tipo));
    let x = match_pl_ftol(pct as f64 * read_f64_at(0x967898) * dias as f64) as i16;
    if (x as i32) + 0x3c < dias as i32 { dias.wrapping_sub(0x3c) } else { x }
}

/// { 0x62d250 } (thiscall, ret 0x20, injury.cpp, quadro de 0x220 bytes sem SEH): `Injury::add_injury(tipo, retroativa,
/// pessoa, partida, surto, arg da notícia, recorrência, só_recorrência)`, pelo dia das lesões (0x62b7b0), pelos efeitos
/// físicos da partida (0x62c080), pelos registros iniciais (0x62cf70), pela recaída (0x62e500), pelos surtos de doença
/// (0x62e9e0) e por duas originais. Pessoa nula, sem ficha (+0x61) ou tipo fora de 0..0x51: a original (caixa e −1); id
/// além de [0xae2364]: −1. O físico do dia (0x62c680) e o treino ([0xde1f44], 0x8b1750 com 0). Só recorrência: com
/// recorrência e o tipo que pode voltar, a marca de recaída do registro (+0x12 tipo, +0x13: 1 se sorteio(rec / 10) e
/// sorteio(rec) derem zero com rec negativo; rec se positivo); −1. Senão, com lesão (+0xe ≠ −1): só os tipos 0x4d, 0x4e,
/// 0x4f e 0x51 a trocam (os dias restantes +0x2c ficam, a velha vai ao histórico 0x58e340 e a vaga fica livre). +0x1d
/// = tipo; a primeira vaga livre (byte 0xff) do vetor [this+4] (conta [this+8]) ou mais 100 vagas (realloc; sem
/// memória, a caixa da linha 0x909 e −1). O tipo na vaga; a partida (pelo clube da casa +0x1c ou de fora +0x20 que tem
/// a pessoa nos 50 do elenco +0xd7) grava o registro da partida em +1 (0x7e4750), senão +0x1a = 0xff. O melhor
/// fisioterapeuta do clube (0x544410 dos três +0x1cf com +0x69). Dias = base + variáveis × (35 − melhor) / 20 (menos
/// sorteio deles se sorteio(melhor + 5) > sorteio(20); 0 no tipo 0x51) + os restantes da velha (+ sorteios pela idade
/// e pela propensão +0x2d da ficha, com base acima de 10). O +0xa do registro de 79 bytes sobe 1 a cada sorteio(5 ×
/// natural + 12) zero, por dia, até 100. Lesão de agora (retroativa 0): de 100 dias, fora do clube humano e fora da
/// lista, a marca 100 do clube na lista de observação; dias ≥ sorteio(50) + 30, os clubes que miram o jogador o
/// suspendem (0x842b00). +0x28 = +0x2c = dias, +0x2a = +0x2e = a volta ao treino; +0x20 = a data (hoje, ou hoje −
/// sorteio(dias) − 1 na retroativa). O físico de novo, a notícia (0x62dca0), +0xe = a vaga, a recorrência, a seleção
/// (0x775cd0), +0x1c = 0, a recaída (0x62e500), a contagem das recaídas (de duas: aposentadoria 0x7a7f20 por sorteio),
/// a aposentadoria pela idade e pela volta ao treino longa e o surto (0x62e820) nos tipos 0, 1 e 2. Devolve a vaga.
unsafe extern "thiscall" fn add_injury(this: usize, tipo_arg: u32, retroativa: u32, p: usize, jogo: usize, surto: u32, noticia: u32, rec_arg: u32, so_recorrencia: u32) -> i32 {
    let tipo = tipo_arg as u8 as i8;
    let original = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, u32, u32, u32) -> i32>(%%I0%%);
    if p == 0 {
        return original(this, tipo_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia); // a caixa: só a original
    }
    if cqb5_d(p) as i32 >= cqb5_d(0xae2364) as i32 {
        return -1;
    }
    if cqb5_d(p + 0x61) == 0 || !(0..0x52).contains(&tipo) {
        return original(this, tipo_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia); // as caixas: só a original
    }
    let fisico: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
    fisico(this, p);
    if cqb5_d(0xde1f44) != 0 {
        let treino: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x8b1750usize);
        treino(cqb5_d(0xde1f44), p, 0);
    }
    let t = injury_type_offset(tipo);
    let rec = rec_arg as u8 as i8;
    if so_recorrencia != 0 {
        if rec == 0 || cqb5_d(0x9e7ae0 + t) == 0 {
            return -1;
        }
        if rec < 0 {
            if cqb5_sorteio(rec as i32 / 10) == 0 {
                let r = physical_record(this, cqb5_d(p));
                *((r + 0x12) as *mut u8) = tipo as u8;
                *((r + 0x13) as *mut u8) = 1;
            }
            return -1;
        }
        let r = physical_record(this, cqb5_d(p));
        *((r + 0x12) as *mut u8) = tipo as u8;
        *((r + 0x13) as *mut u8) = rec as u8;
        return -1;
    }
    let antiga = cqb5_d(physical_record(this, cqb5_d(p)) + 0xe) as i32;
    let mut restantes: u16 = 0; // a palavra baixa do local (a alta é 0)
    if antiga != -1 {
        if !(tipo == 0x4d || tipo == 0x4e || tipo == 0x4f || tipo == 0x51) {
            return -1;
        }
        crate::comparison::branch("cqb_5 troca a lesao");
        let r = physical_record(this, cqb5_d(p));
        let l = injury_entry(this, antiga as usize);
        restantes = cqb5_w(l + 0x2c) as u16;
        let historico: unsafe extern "thiscall" fn(usize, usize, usize, usize) -> usize = core::mem::transmute(0x58e340usize);
        historico(cqb5_d(this + 0xc), r + 0x14, r + 0x18, l);
        *(l as *mut u8) = 0xff;
        core::ptr::write_unaligned((physical_record(this, cqb5_d(p)) + 0xe) as *mut i32, -1);
    }
    *((physical_record(this, cqb5_d(p)) + 0x1d) as *mut u8) = tipo as u8;
    // a vaga
    let mut vaga: i32 = -1;
    let vetor = cqb5_d(this + 4);
    if vetor != 0 {
        let n = cqb5_d(this + 8) as i32;
        let mut i = 0;
        while i < n {
            if cqb5_b(vetor + i as usize * 0x30) == 0xff {
                vaga = i;
                break;
            }
            i += 1;
        }
    }
    if vaga == -1 {
        let novo = (cqb5_d(this + 8) as i32).wrapping_add(100);
        let tam = (novo as usize).wrapping_mul(0x30);
        let v = if vetor != 0 {
            let realocar: unsafe extern "C" fn(usize, usize) -> usize = core::mem::transmute(0x945667usize);
            realocar(vetor, tam)
        } else {
            let alocar: unsafe extern "C" fn(usize) -> usize = core::mem::transmute(0x944e46usize);
            alocar(tam)
        };
        *((this + 4) as *mut usize) = v;
        if v == 0 {
            show_error_at(0x9e7f28, 0x909); // injury.cpp
            return -1;
        }
        let velho = cqb5_d(this + 8) as i32;
        let mut i = velho;
        while i < novo {
            *((cqb5_d(this + 4) + i as usize * 0x30) as *mut u8) = 0xff;
            i += 1;
        }
        *((this + 8) as *mut i32) = novo;
        vaga = velho;
    }
    let off = vaga as usize * 0x30;
    *((cqb5_d(this + 4) + off) as *mut u8) = tipo as u8;
    let mut da_partida = false;
    if jogo != 0 {
        for k in 0..50usize {
            let mut clube = cqb5_d(jogo + 0x1c);
            let achou = if cqb5_d(clube + 0xd7 + 4 * k) == p {
                true
            } else if cqb5_d(cqb5_d(jogo + 0x20) + 0xd7 + 4 * k) == p {
                clube = cqb5_d(jogo + 0x20);
                true
            } else {
                false
            };
            if achou {
                if clube != 0 {
                    let registrar: unsafe extern "C" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7e4750usize);
                    registrar(jogo, clube, cqb5_d(this + 4) + off + 1);
                    da_partida = true;
                }
                break;
            }
        }
    }
    if !da_partida {
        *((cqb5_d(this + 4) + off + 0x1a) as *mut u8) = 0xff;
    }
    // o melhor fisioterapeuta
    let mut melhor: i8 = 0;
    if cqb5_d(p + 0x39) != 0 {
        let conhecer: unsafe extern "C" fn(usize, i8) -> i32 = core::mem::transmute(0x544410usize);
        for k in 0..3usize {
            let f = cqb5_d(cqb5_d(p + 0x39) + 0x1cf + 4 * k);
            if f == 0 {
                continue;
            }
            let np = cqb5_d(f + 0x69);
            if np == 0 {
                continue;
            }
            if conhecer(np, cqb5_b(np + 0x1e) as i8) as i8 > melhor {
                let np = cqb5_d(cqb5_d(cqb5_d(p + 0x39) + 0x1cf + 4 * k) + 0x69);
                melhor = conhecer(np, cqb5_b(np + 0x1e) as i8) as i8;
            }
        }
    }
    // os dias
    crate::comparison::branch("cqb_5 duracao");
    let mut extra = (cqb5_w(0x9e7add + t) as i32).wrapping_mul(0x23 - melhor as i32) / 20; // cqb5: o fisioterapeuta encurta
    let s1 = cqb5_sorteio(melhor as i32 + 5);
    let s2 = cqb5_sorteio(0x14);
    if s1 > s2 {
        extra = extra.wrapping_sub(cqb5_sorteio(extra as i16 as i32));
    }
    if tipo == 0x51 {
        extra = 0;
    }
    extra = extra.wrapping_add(restantes as i32);
    let idade = cqb5_b(p + 0x18) as i8;
    if tipo != 0x51 && cqb5_w(0x9e7adb + t) > 10 {
        if idade > 0x1e {
            let ax = (cqb5_c(cqb5_d(p + 0x61) + 0x2d) as i16).wrapping_add(idade as i16).wrapping_sub(0x23);
            if ax > 0 {
                extra = extra.wrapping_add(cqb5_sorteio(ax as i32));
            }
        }
        let al = cqb5_b(cqb5_d(p + 0x61) + 0x2d) as i8;
        if al > 10 {
            extra = extra.wrapping_add(cqb5_sorteio((al as i16 - 10) as i32));
        }
    }
    let dias = cqb5_w(0x9e7adb + t).wrapping_add(extra as i16);
    let n = dias as i32;
    let mut i = 0;
    while i < n {
        let g = cqb5_d(0xae2d0c).wrapping_add(cqb5_d(p).wrapping_mul(0x4f)) + 0xa;
        if cqb5_b(g) >= 100 {
            break;
        }
        if cqb5_sorteio(cqb5_c(cqb5_d(p + 0x61) + 0x34) * 5 + 0xc) == 0 {
            let g = cqb5_d(0xae2d0c).wrapping_add(cqb5_d(p).wrapping_mul(0x4f)) + 0xa;
            *(g as *mut u8) = cqb5_b(g).wrapping_add(1);
        }
        i += 1;
    }
    if retroativa == 0 {
        let clube = cqb5_d(p + 0x39);
        if clube != 0 {
            let humano: unsafe extern "C" fn(usize, u32, u8, usize, u32) -> u32 = core::mem::transmute(0x600a50usize);
            if humano(clube, 0, 1, 0, 0) == 0 && dias >= 100 {
                let listada: unsafe extern "C" fn(usize, usize) -> u32 = core::mem::transmute(0x5444a0usize);
                if listada(p, 0) == 0 && cqb5_d(0xde1f54) != 0 {
                    let tm = cqb5_d(0xde1f54);
                    let m = cqb5_d(cqb5_d(tm + 0x856) + 0xdc).wrapping_add(cqb5_d(cqb5_d(p + 0x39)).wrapping_mul(9)) + 8;
                    *(m as *mut u8) = 100;
                }
            }
        }
        let s = cqb5_sorteio(0x32);
        if n >= s + 0x1e && cqb5_d(0xde1f54) != 0 && cqb5_d(cqb5_d(0xde1f54) + 0x856) != 0 {
            crate::comparison::branch("cqb_5 suspende as miras");
            let suspender: unsafe extern "thiscall" fn(usize, usize, u8) = core::mem::transmute(0x842b00usize);
            suspender(cqb5_d(cqb5_d(0xde1f54) + 0x856), p, 0); // cqb5: os clubes que o miram suspendem
        }
    }
    cqb5_por_w(cqb5_d(this + 4) + off + 0x28, dias);
    cqb5_por_w(cqb5_d(this + 4) + off + 0x2c, dias);
    crate::comparison::branch("cqb_5 volta ao treino");
    let volta = return_to_training_days(tipo, dias); // cqb5: a volta ao treino pelo percentual
    cqb5_por_w(cqb5_d(this + 4) + off + 0x2a, volta);
    cqb5_por_w(cqb5_d(this + 4) + off + 0x2e, cqb5_w(cqb5_d(this + 4) + off + 0x2a));
    let copiar: unsafe extern "thiscall" fn(usize, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let mut saida = [0u8; 8];
    if retroativa != 0 {
        let s = cqb5_sorteio(n);
        let menos: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c8e0usize);
        let mut d = [0u8; 8];
        let r = menos(0xae2c90, d.as_mut_ptr(), (s + 1) as u32);
        copiar(cqb5_d(this + 4) + off + 0x20, saida.as_mut_ptr(), r);
    } else {
        copiar(cqb5_d(this + 4) + off + 0x20, saida.as_mut_ptr(), 0xae2c90);
    }
    fisico(this, p);
    let noticia_fn: unsafe extern "thiscall" fn(usize, usize, usize, u32) = core::mem::transmute(0x62dca0usize);
    noticia_fn(this, p, cqb5_d(this + 4) + off, noticia);
    core::ptr::write_unaligned((physical_record(this, cqb5_d(p)) + 0xe) as *mut i32, vaga);
    if rec != 0 && cqb5_d(0x9e7ae0 + t) != 0 {
        if rec < 0 {
            if cqb5_sorteio(rec as i32 / 10) == 0 {
                *((physical_record(this, cqb5_d(p)) + 0x12) as *mut u8) = tipo as u8;
                if cqb5_sorteio(rec as i32) == 0 {
                    *((physical_record(this, cqb5_d(p)) + 0x13) as *mut u8) = 1;
                }
            }
        } else {
            *((physical_record(this, cqb5_d(p)) + 0x12) as *mut u8) = tipo as u8;
            *((physical_record(this, cqb5_d(p)) + 0x13) as *mut u8) = rec as u8;
        }
    }
    if cqb5_d(p + 0x24) != 0 && cqb5_d(0xb74318) != 0 {
        let tirar: unsafe extern "thiscall" fn(usize, usize, u32) = core::mem::transmute(0x775cd0usize);
        tirar(cqb5_d(0xb74318), p, cqb5_w(cqb5_d(this + 4) + off + 0x28) as u16 as u32);
    }
    *((physical_record(this, cqb5_d(p)) + 0x1c) as *mut u8) = 0;
    let recaida: unsafe extern "thiscall" fn(usize, usize, usize) = core::mem::transmute(0x62e500usize);
    recaida(this, physical_record(this, cqb5_d(p)), cqb5_d(this + 4) + off);
    let aposentar: unsafe extern "thiscall" fn(usize, usize, i16) = core::mem::transmute(0x7a7f20usize);
    let r = physical_record(this, cqb5_d(p));
    if cqb5_b(r + 0x12) as i8 == tipo && rec >= 0 {
        *((r + 0x13) as *mut u8) = cqb5_b(r + 0x13).wrapping_add(1);
        let c = cqb5_b(physical_record(this, cqb5_d(p)) + 0x13) as i8;
        if c > 1 {
            let mut e = 200 - cqb5_w(0x9e7adb + t) as i32;
            if e <= 5 {
                e = 5;
            }
            if rec > 0 {
                e /= rec as i32;
            }
            let s1 = cqb5_sorteio(c as i32);
            let s2 = cqb5_sorteio(e);
            if s1 > s2 && cqb5_d(0xb7434c) != 0 {
                crate::comparison::branch("cqb_5 aposenta pelas recaidas");
                aposentar(cqb5_d(0xb7434c), p, cqb5_w(cqb5_d(this + 4) + off + 0x2a));
            }
        }
    }
    // a aposentadoria pela idade e pela volta ao treino longa
    let volta = || cqb5_w(cqb5_d(this + 4) + off + 0x2a) as i32;
    let propensao = || cqb5_c(cqb5_d(p + 0x61) + 0x2d);
    let anos = || cqb5_c(p + 0x18);
    let encerra = (idade > 0x24 && volta() > 0x78 - propensao() - anos() && cqb5_sorteio(volta()) > 0x5a - anos())
        || (idade > 0x21 && volta() > 0x96 - propensao() - anos() && cqb5_sorteio(volta()) > 0x5a - anos())
        || (idade > 0x1f && volta() > 0x12c - anos() && cqb5_sorteio(volta()) > 0xc8 - anos())
        || (volta() > 0x1f4 && cqb5_sorteio(cqb5_c(cqb5_d(p + 0x61) + 0x34) + 5) == 0);
    if encerra && cqb5_d(0xb7434c) != 0 {
        crate::comparison::branch("cqb_5 aposenta pela lesao");
        aposentar(cqb5_d(0xb7434c), p, cqb5_w(cqb5_d(this + 4) + off + 0x2a));
    }
    if surto != 0 && (tipo == 1 || tipo == 0 || tipo == 2) {
        let clube = cqb5_d(p + 0x39);
        if clube != 0 {
            let surto_fn: unsafe extern "thiscall" fn(usize, usize, u32, i32, usize) = core::mem::transmute(0x62e820usize);
            surto_fn(this, 0xae2c90, tipo_arg, cqb5_d(clube) as i32, 0);
        }
    }
    vaga
}

/// Comparador da 0x62d250: só a original (aloca, entrega a notícia, agenda aposentadoria e chama originais que sorteiam
/// no meio, como o treino 0x8b1750 e a recaída 0x62e500, que pode lesionar de novo: não pode rodar duas vezes). Confere
/// o que a lesão garante depois: argumentos inválidos ou só recorrência dão −1; −1 de resto só com a pessoa ainda
/// lesionada (+0xe ≠ −1) ou sem vetor; vaga válida com a pessoa ainda nela (+0xe = vaga, sem recaída que a tenha
/// trocado): o tipo na vaga e em +0x1d, +0x1c = 0, +0x2c = +0x28, +0x2e = +0x2a e +0x2a = a volta ao treino pela
/// duração +0x28 (o x87 da cópia).
unsafe extern "thiscall" fn compare_add_injury(this: usize, tipo_arg: u32, retroativa: u32, p: usize, jogo: usize, surto: u32, noticia: u32, rec_arg: u32, so_recorrencia: u32) -> i32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, u32, u32, usize, usize, u32, u32, u32, u32) -> i32>(%%I0%%);
    let r = o(this, tipo_arg, retroativa, p, jogo, surto, noticia, rec_arg, so_recorrencia);
    let tipo = tipo_arg as u8 as i8;
    let valida = p != 0 && (cqb5_d(p) as i32) < cqb5_d(0xae2364) as i32 && cqb5_d(p + 0x61) != 0 && (0..0x52).contains(&tipo);
    let ok = if !valida || so_recorrencia != 0 {
        r == -1
    } else {
        let reg = physical_record(this, cqb5_d(p));
        let atual = cqb5_d(reg + 0xe) as i32;
        if r == -1 {
            atual != -1 || cqb5_d(this + 4) == 0
        } else if r < 0 || atual != r {
            r >= 0 // trocada depois (a recaída lesionou de novo): nada a conferir
        } else {
            let l = injury_entry(this, r as usize);
            // a recaída (0x62e500) pode lesionar de novo a própria pessoa com 0x4d/0x4e/0x4f: a troca libera esta vaga e
            // a nova lesão a reocupa (a primeira livre), com o tipo dela; confere a nova
            let t = cqb5_b(l) as i8;
            let tipo = if t != tipo && (0x4d..=0x4f).contains(&t) { t } else { tipo };
            cqb5_b(l) as i8 == tipo
                && cqb5_b(reg + 0x1d) as i8 == tipo
                && cqb5_b(reg + 0x1c) == 0
                && cqb5_w(l + 0x2c) == cqb5_w(l + 0x28)
                && cqb5_w(l + 0x2e) == cqb5_w(l + 0x2a)
                && cqb5_w(l + 0x2a) == return_to_training_days(tipo, cqb5_w(l + 0x28))
        }
    };
    SCOREBOARD.record(ok, || format!("add_injury(tipo {:#x}, retroativa {}, pessoa {:#x}, recorrencia {}, so {}) = {}", tipo_arg, retroativa, p, rec_arg as u8 as i8, so_recorrencia, r));
    r
}

/// { 0x62dca0 } (o mapa a dá como stdcall: a original troca o ecx por [0xde1f54] logo na entrada; o único ponto de
/// chamada, na 0x62d250, põe a Injury em ecx: thiscall com o this sem uso; ret 0xc; quadro SEH sem destrutor):
/// `injury_post_injury_news(pessoa, lesão, arg)`. Os clubes interessados (0x8d1da0 em [0xde1f54] com 0x5a, até 30) e a
/// seleção (+0x24) se couber; a notícia 0x7d0 (local): 0..3 a pessoa (id e os +0x33 de +4/+8/+0xc, 0 se faltam), 4 =
/// o tipo, 5/6 = a data da lesão (+0x20 com sinal, +0x22 sem), 7 = os dias (+0x28), 8/9 = o nascimento (+0x10, +0x12),
/// 0xa..0xc = o registro da partida (+0x12, +0x16, +9) ou −1 sem partida (+0x1a = 0xff), 0xd = arg, 0xe = id do clube
/// ou −1, 0xf = 0x420e9; a partida na notícia (0x7e4de0 com o byte 0x10). Mais de 100 dias: com potencial (+0xd da
/// ficha) acima de 8000, a todos (0x786780); senão aos interessados (0x7861e0). Até 100: clube humano (0x7875e0), ao
/// clube; o clube do contrato (0x4cf5a0) se for outro; a seleção.
unsafe extern "thiscall" fn injury_post_injury_news(_this: usize, p: usize, l: usize, arg: u32) {
    crate::comparison::branch("cqb_5 noticia da lesao");
    let tm = cqb5_d(0xde1f54);
    let mut clubes = [0usize; 30];
    let mut n: i8 = 0;
    if tm != 0 {
        let interessados: unsafe extern "thiscall" fn(usize, usize, u32, *mut usize, u32) -> u8 = core::mem::transmute(0x8d1da0usize);
        n = interessados(tm, p, 0x5a, clubes.as_mut_ptr(), 0x1e) as i8;
    }
    let sel = cqb5_d(p + 0x24);
    if sel != 0 && (0..0x1e).contains(&n) {
        clubes[n as usize] = sel;
        n += 1;
    }
    let construir: unsafe extern "thiscall" fn(usize, u32, u32) -> usize = core::mem::transmute(0x77bbe0usize);
    let gravar: unsafe extern "thiscall" fn(usize, i8, u32) -> u32 = core::mem::transmute(0x785790usize);
    let entregar: unsafe extern "thiscall" fn(usize, usize, usize) -> u32 = core::mem::transmute(0x7861e0usize);
    let mut local = [0u32; 0x38]; // a notícia local (0xe0 bytes)
    let nw = local.as_mut_ptr() as usize;
    construir(nw, 0x7d0, 0);
    gravar(nw, 0, cqb5_d(p) as u32);
    for k in 1..4usize {
        let q = cqb5_d(p + 4 * k);
        gravar(nw, k as i8, if q != 0 { cqb5_d(q + 0x33) as u32 } else { 0 });
    }
    gravar(nw, 4, cqb5_c(l) as u32);
    gravar(nw, 5, cqb5_w(l + 0x20) as i32 as u32);
    gravar(nw, 6, cqb5_w(l + 0x22) as u16 as u32);
    gravar(nw, 7, cqb5_w(l + 0x28) as i32 as u32);
    gravar(nw, 8, cqb5_w(p + 0x10) as i32 as u32);
    gravar(nw, 9, cqb5_w(p + 0x12) as u16 as u32);
    if cqb5_b(l + 0x1a) != 0xff {
        gravar(nw, 0xa, cqb5_d(l + 0x12) as u32);
        gravar(nw, 0xb, cqb5_d(l + 0x16) as u32);
        gravar(nw, 0xc, cqb5_d(l + 9) as u32);
    } else {
        gravar(nw, 0xa, u32::MAX);
        gravar(nw, 0xb, u32::MAX);
        gravar(nw, 0xc, u32::MAX);
    }
    gravar(nw, 0xd, arg);
    let clube = cqb5_d(p + 0x39);
    gravar(nw, 0xe, if clube != 0 { cqb5_d(clube) as u32 } else { u32::MAX });
    gravar(nw, 0xf, 0x420e9);
    let mut marca: u8 = 0x10;
    let partida_na_noticia: unsafe extern "C" fn(usize, usize, *mut u8) = core::mem::transmute(0x7e4de0usize);
    partida_na_noticia(l + 1, nw, &mut marca);
    let news = cqb5_d(0xb7431c);
    if cqb5_w(l + 0x28) > 100 {
        let f = cqb5_d(p + 0x61);
        if f != 0 && cqb5_w(f + 0xd) > 0x1f40 {
            let todos: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x786780usize);
            todos(cqb5_d(0xb7431c), nw);
        } else {
            let mut i: i8 = 0;
            while i < n {
                entregar(cqb5_d(0xb7431c), nw, clubes[i as usize]);
                i += 1;
            }
        }
        return;
    }
    if cqb5_d(p + 0x39) == 0 {
        return;
    }
    let humano: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x7875e0usize);
    if humano(news, cqb5_d(p + 0x39)) == 0 {
        return;
    }
    entregar(cqb5_d(0xb7431c), nw, cqb5_d(p + 0x39));
    let c = injury_main_contract_stored(p);
    if c != 0 && injury_club_of_contract(c) != 0 {
        let x = injury_club_of_contract(injury_main_contract_stored(p));
        if x != cqb5_d(p + 0x39) {
            let y = injury_club_of_contract(injury_main_contract_stored(p));
            entregar(cqb5_d(0xb7431c), nw, y);
        }
    }
    let sel = cqb5_d(p + 0x24);
    if sel != 0 {
        entregar(cqb5_d(0xb7431c), nw, sel);
    }
}

/// Comparador da 0x62dca0: só a original (entrega a notícia, que não pode sair duas vezes; a cópia leva o ramo no modo
/// Rust). ponytail: sem plano a conferir além do que a original entrega.
unsafe extern "thiscall" fn compare_injury_post_injury_news(this: usize, p: usize, l: usize, arg: u32) {
    original_fn::<unsafe extern "thiscall" fn(usize, usize, usize, u32)>(%%I1%%)(this, p, l, arg)
}

/// { 0x418f60 } (thiscall, ret 8, awol.cpp): `AWOL::is_player_awol(pessoa, só_ler)`, pela seleção, pelo treino, pelo
/// preço pedido, pelo dia da comissão e pelas transferências. Pessoa nula ou id além de [0xae2364]: 0 (só o al). Com
/// `só_ler`, o bit 0 da marca da pessoa ([this+0xc] + id). Senão, com o bit e sem contrato (0x4cf5a0, guardado em
/// [0xae1080]) ou sem clube do contrato, a ausência acaba (0x4196a0); devolve o bit relido.
unsafe extern "thiscall" fn is_player_awol(this: usize, p: usize, so_ler: u32) -> u32 {
    if p == 0 || cqb5_d(p) as i32 >= cqb5_d(0xae2364) as i32 {
        return 0;
    }
    let marca = || cqb5_b(cqb5_d(this + 0xc) + cqb5_d(p)) as u32 & 1;
    if so_ler == 0 && marca() != 0 {
        let c = injury_main_contract_stored(p);
        if c == 0 || injury_club_of_contract(c) == 0 {
            crate::comparison::branch("cqb_5 ausencia acaba sem clube");
            let acabar: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x4196a0usize);
            acabar(this, p);
        }
    }
    marca()
}

/// Comparador da 0x418f60 (quente: sem alocar): lado a lado em [0xae1080] e no al devolvido; com o bit, sem `só_ler` e
/// sem contrato ou clube (a 0x4196a0 tira e libera o nó da lista: não pode rodar duas vezes), só a original.
unsafe extern "thiscall" fn compare_is_player_awol(this: usize, p: usize, so_ler: u32) -> u32 {
    let o = original_fn::<unsafe extern "thiscall" fn(usize, usize, u32) -> u32>(%%I2%%);
    if p != 0 && (cqb5_d(p) as i32) < cqb5_d(0xae2364) as i32 && so_ler == 0 && cqb5_b(cqb5_d(this + 0xc) + cqb5_d(p)) & 1 != 0 {
        let c = injury_main_contract_stored(p); // a original o refaz igual (e regrava [0xae1080])
        if c == 0 || injury_club_of_contract(c) == 0 {
            return o(this, p, so_ler);
        }
    }
    let (r, n, igual, mo, mr) = crate::comparison::side_by_side_light(&[(0xae1080, 4)], || o(this, p, so_ler), || is_player_awol(this, p, so_ler));
    SCOREBOARD.record(igual && r as u8 == n as u8, || format!("is_player_awol(pessoa {:#x}, so_ler {}) original={} {:x?} rust={} {:x?}", p, so_ler, r as u8, mo, n as u8, mr));
    r
}
