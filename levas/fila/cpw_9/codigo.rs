// ---------- %%LEVA%%a leva: o dia das lesões: as lesões que envelhecem e as lesões no treino, as notícias da volta aos treinos e o jogador a um cartão da suspensão (injury.cpp, discipline.cpp) ----------

unsafe fn cpw9_d(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}

unsafe fn cpw9_i(a: usize) -> i32 {
    core::ptr::read_unaligned(a as *const i32)
}

unsafe fn cpw9_w(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}

unsafe fn cpw9_uw(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}

unsafe fn cpw9_b(a: usize) -> i8 {
    *(a as *const i8)
}

unsafe fn cpw9_sortear(n: i32) -> i32 {
    let f: unsafe extern "C" fn(i32) -> i32 = core::mem::transmute(0x90cfc0usize);
    f(n)
}

/// Soma `delta` ao byte `a` (o `add`/`dec` de 8 bits da original).
unsafe fn cpw9_somar_byte(a: usize, delta: u8) {
    *(a as *mut u8) = (*(a as *const u8)).wrapping_add(delta);
}

/// O registro de 31 bytes da pessoa de índice `idx` em [this].
unsafe fn cpw9_registro(this: usize, idx: i32) -> usize {
    cpw9_d(this).wrapping_add((idx as u32).wrapping_mul(31) as usize)
}

/// A categoria da recorrência (+0x12 do registro, com sinal) é a da tabela de tipos (0x9e7ad9, 13 bytes por tipo)?
unsafe fn cpw9_recorrente(this: usize, idx: i32, categoria: u8) -> bool {
    let r = cpw9_b(cpw9_registro(this, idx) + 0x12);
    r > -1 && *((0x9e7ad9 + (r as isize * 13) as usize) as *const u8) == categoria
}

/// O ensaio do comparador: a cópia roda só a fase do treino (sem soltar a lista das batidas, sem o físico do dia 180,
/// sem a lesão nova nem as rotinas do fim); `CPW9_PAROU` marca que ela chegou a uma lesão nova (0x62d250, que aloca e
/// manda notícia) ou ao físico de todos.
static mut CPW9_ENSAIO: bool = false;
static mut CPW9_PAROU: bool = false;
/// O que a cópia deixou nas regiões da fase do treino, à espera de a original chegar às notícias da volta (0x62e010).
static mut CPW9_ESPERA: Option<(Vec<(usize, usize)>, crate::comparacao::Memoria)> = None;

/// O switch dos tipos de lesão (tipo − 0xc, byte da tabela 0x62bcd8): 0 os do +0x1b/+0x36, 1 nenhuma lesão (0x14), 2 o
/// 0x1c, 3 o 0x2d, 4 os 0x3b e 0x3c, 5 só a lesão.
const CPW9_CASOS: [u8; 0x31] = [
    0, 5, 5, 5, 5, 5, 5, 5, 1, 5, 5, 0, 5, 5, 0, 0, 2, 5, 5, 0, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 5, 3, 5, 5, 5, 5, 5, 5,
    5, 5, 5, 5, 5, 5, 5, 4, 4,
];

/// { 0x62b7b0 } (thiscall, ret 0, injury.cpp): `Injury::daily_update()`, pelo laço do dia (0x5cccd0) e pela ida até a
/// data do começo (0x5cc850), todo dia: solta a lista das batidas da partida (+0x28, 0x68d040 e free); os dois contadores
/// (+0x2c, +0x2e) de cada lesão da tabela (0x30 bytes em [+4], conta +8) descem um; no dia 180 da primeira temporada
/// (0x456770 com a data do começo), o físico de todos (0x62c680). Fora do sábado e do domingo (0x54a3f0: 5 e 6), as lesões
/// no treino: [0xae2364] / 150 sorteios (pelo menos 10) de uma pessoa: jogador (+0x61) — senão o sorteio não conta —
/// nos cargos 11, 13 e 15, presente (0x418f60), sem lesão (+0xe = −1), passando pelos sorteios de risco (o +0x2d da ficha,
/// a propensão +0x1e e a recorrência +0x12/+0x13 do registro de 31 bytes; nota 20: sorteio(5) contra sorteio(2)), um em
/// três e propensão de 20: a categoria (a da recorrência ou sorteada pelas faixas 0x9e7f05/0x9e7f06), o tipo (0x62d140,
/// grave nos primeiros 1/14 dos sorteios), os atributos que a lesão gasta (+0x1b, +0x36, +0x1d da ficha) e a lesão nova
/// (0x62d250); fora da temporada (0x62ce30) nada; categoria 7 com nota abaixo de 15 não conta. No fim as notícias da volta
/// (0x62e010) e os surtos de doença (0x62e9e0).
unsafe extern "thiscall" fn injury_daily_update(this: usize) {
    let lista = cpw9_d(this + 0x28);
    if lista != 0 {
        if !CPW9_ENSAIO {
            let destruir: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x68d040usize);
            let soltar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
            destruir(lista);
            soltar(lista);
        }
        core::ptr::write_unaligned((this + 0x28) as *mut u32, 0);
    }
    let base = cpw9_d(this + 4);
    let mut k = 0i32;
    while k < cpw9_i(this + 8) {
        let e = base.wrapping_add((k as u32).wrapping_mul(0x30) as usize) + 0x2e;
        core::ptr::write_unaligned((e - 2) as *mut u16, cpw9_uw(e - 2).wrapping_sub(1));
        core::ptr::write_unaligned(e as *mut u16, cpw9_uw(e).wrapping_sub(1));
        k += 1;
    }
    let anterior: unsafe extern "thiscall" fn(usize, u32, u32) -> u32 = core::mem::transmute(0x456770usize);
    if anterior(0xae2c90, cpw9_d(0xdd6ec8) as u32, cpw9_d(0xdd6ecc) as u32) != 0 && cpw9_w(0xae2c90) == 0xb4 {
        if CPW9_ENSAIO {
            CPW9_PAROU = true;
            return;
        }
        let fisico: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
        let mut p = cpw9_d(0xae23bc);
        let mut i = 0i32;
        while i < cpw9_i(0xae2364) {
            fisico(this, p);
            i += 1;
            p = p.wrapping_add(0x6e);
        }
    }
    let semana: unsafe extern "thiscall" fn(usize) -> i32 = core::mem::transmute(0x54a3f0usize);
    if semana(0xae2c90) as i16 != 6 && semana(0xae2c90) as i16 != 5 { // cpw9: sábado e domingo sem treino
        crate::comparacao::ramo("cpw_9 dia de treino");
        cpw9_treino(this);
    }
    if CPW9_ENSAIO {
        return;
    }
    let noticias: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x62e010usize);
    let surtos: unsafe extern "thiscall" fn(usize) = core::mem::transmute(0x62e9e0usize);
    noticias(this);
    surtos(this);
}

/// As lesões no treino do dia (o meio da 0x62b7b0).
unsafe fn cpw9_treino(this: usize) {
    let ausente: unsafe extern "thiscall" fn(usize, usize, u32) -> u8 = core::mem::transmute(0x418f60usize);
    let fora_da_temporada: unsafe extern "stdcall" fn(usize) -> u32 = core::mem::transmute(0x62ce30usize);
    let escolher: unsafe extern "thiscall" fn(usize, i8, i8, usize) -> u8 = core::mem::transmute(0x62d140usize);
    let mut quantos = (cpw9_i(0xae2364) / 150) as i16; // cpw9: os sorteios do treino
    if quantos < 10 {
        quantos = 10;
    }
    let total = quantos as i32;
    let graves = (total / 14) as i16 as i32;
    let mut i = 0i32;
    while i < total {
        let idx = cpw9_sortear(cpw9_i(0xae2364));
        let p = cpw9_d(0xae23bc).wrapping_add((idx as u32).wrapping_mul(0x6e) as usize);
        'proximo: {
            if cpw9_d(p + 0x61) == 0 {
                i -= 1; // não jogador: o sorteio não conta
                break 'proximo;
            }
            let cargo = *((p + 0x3d) as *const u8);
            if cargo != 0xb && cargo != 0xf && cargo != 0xd {
                break 'proximo;
            }
            if ausente(cpw9_d(0xad9c6c), p, 0) != 0 {
                break 'proximo;
            }
            if cpw9_b(cpw9_d(p + 0x61) + 0xf) == 0x14 {
                let r5 = cpw9_sortear(5);
                let r2 = cpw9_sortear(2);
                if r5 > r2 {
                    break 'proximo;
                }
            }
            let rec = cpw9_registro(this, idx);
            if cpw9_i(rec + 0xe) != -1 {
                break 'proximo;
            }
            if cpw9_b(rec + 0x12) > -1 {
                let rec = cpw9_registro(this, idx);
                let r = cpw9_sortear(100);
                let lim = ((cpw9_b(cpw9_d(p + 0x61) + 0x2d) as i32 - 3) * 5 + cpw9_b(rec + 0x1e) as i32 + cpw9_b(rec + 0x13) as i32) * 2;
                if r > lim {
                    break 'proximo;
                }
            } else {
                let r = cpw9_sortear(0xa0);
                let lim = ((cpw9_b(cpw9_d(p + 0x61) + 0x2d) as i32 - 3) * 5 + cpw9_b(cpw9_registro(this, idx) + 0x1e) as i32) * 2; // cpw9: o risco sem recorrência
                if r > lim {
                    break 'proximo;
                }
                crate::comparacao::ramo("cpw_9 risco sem recorrencia");
            }
            if cpw9_sortear(3) != 0 {
                break 'proximo;
            }
            let rec = cpw9_registro(this, idx);
            if cpw9_b(rec + 0x1e) < 0x14 {
                break 'proximo;
            }
            let mut categoria: u8 = 0;
            let mut sortear_categoria = true;
            if cpw9_b(rec + 0x12) > -1 {
                let r1 = cpw9_sortear(cpw9_b(rec + 0x13) as i32 + 2);
                let r2 = cpw9_sortear(3);
                if r1 > r2 {
                    let r = cpw9_b(cpw9_registro(this, idx) + 0x12) as isize;
                    categoria = *((0x9e7ad9 + (r * 13) as usize) as *const u8);
                    sortear_categoria = false;
                }
            }
            if sortear_categoria {
                loop {
                    categoria = cpw9_sortear(0xc) as u8;
                    if categoria == 0 && cpw9_sortear(100) != 0 {
                        continue;
                    }
                    let c = categoria as i8 as isize * 3;
                    let n = *((0x9e7f06 + c as usize) as *const i8) as i32 - *((0x9e7f05 + c as usize) as *const i8) as i32 + 2;
                    if cpw9_sortear(n) != 0 {
                        break;
                    }
                }
            }
            if cpw9_b(cpw9_d(p + 0x61) + 0xf) < 0xf && categoria == 7 {
                i -= 1;
                break 'proximo;
            }
            if fora_da_temporada(p) != 0 {
                break 'proximo;
            }
            let grave: i8 = if i < graves { -1 } else { 0 };
            let tipo = escolher(this, categoria as i8, grave, p);
            let k = (tipo as i8 as i32).wrapping_sub(0xc);
            let caso = if (k as u32) > 0x30 { 5 } else { CPW9_CASOS[k as usize] };
            if caso == 1 {
                break 'proximo; // o tipo 0x14: nenhuma lesão
            }
            if CPW9_ENSAIO {
                CPW9_PAROU = true;
                return;
            }
            cpw9_gasto_da_lesao(this, idx, p, categoria, caso);
            let nova: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32, u32, u32, u32, u32) -> i32 = core::mem::transmute(0x62d250usize);
            // ponytail: a original empurra o tipo num dword cujos bytes de cima não têm valor; a 0x62d250 lê um char
            nova(this, tipo as u32, 0, p, 0, 1, 0xffff_ffff, 0, 0);
        }
        i += 1;
    }
}

/// Os atributos que a lesão gasta (os casos 0, 2, 3 e 4 do switch da 0x62b7b0): a ficha (+0x61) é relida a cada vez.
unsafe fn cpw9_gasto_da_lesao(this: usize, idx: i32, p: usize, categoria: u8, caso: u8) {
    let ficha = || cpw9_d(p + 0x61);
    match caso {
        0 => {
            let a = cpw9_b(ficha() + 0x1b);
            if a == 0x14 {
                if cpw9_recorrente(this, idx, categoria) {
                    let f = ficha();
                    let r = cpw9_sortear(3) as u8;
                    cpw9_somar_byte(f + 0x1b, 0xffu8.wrapping_sub(r));
                } else if cpw9_sortear(3) != 0 {
                    let f = ficha();
                    let r = cpw9_sortear(2) as u8;
                    cpw9_somar_byte(f + 0x1b, 0xffu8.wrapping_sub(r));
                }
            } else if a > 0xf && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1b, 0xff);
            }
            if cpw9_b(ficha() + 0x36) == 0x14 {
                if cpw9_recorrente(this, idx, categoria) {
                    let f = ficha();
                    let r = cpw9_sortear(3) as u8;
                    cpw9_somar_byte(f + 0x36, 0xffu8.wrapping_sub(r));
                } else if cpw9_sortear(3) != 0 {
                    cpw9_somar_byte(ficha() + 0x36, 0xff);
                }
            } else if cpw9_b(ficha() + 0x1b) > 0x11 && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1b, 0xff);
            }
        }
        2 => {
            if cpw9_recorrente(this, idx, categoria) && cpw9_b(ficha() + 0x1b) == 0x14 && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1b, 0xff);
            }
        }
        3 => {
            let c = cpw9_b(ficha() + 0x1d);
            if c == 0x14 {
                *((ficha() + 0x1d) as *mut u8) = 0x13;
            } else if c > 0xf && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1d, 0xff);
            }
        }
        4 => {
            let c = cpw9_b(ficha() + 0x1d);
            if c == 0x14 {
                if cpw9_sortear(3) != 0 {
                    cpw9_somar_byte(ficha() + 0x1d, 0xff);
                }
            } else if c > 0xf && cpw9_sortear(3) == 0 {
                cpw9_somar_byte(ficha() + 0x1d, 0xff);
            }
        }
        _ => {}
    }
}

/// Comparador lado a lado da fase do treino: a cópia em ensaio (sem soltar a lista, sem a lesão nova e sem as rotinas
/// do fim) e a original a partir do mesmo estado (o cabeçalho do Injury, a tabela de lesões e o sorteio); a original é
/// conferida quando chega às notícias da volta (0x62e010, desviada no mesmo item), antes das rotinas do fim. Com lesão
/// nova (aloca e manda notícia) ou com o físico de todos (dia 180 da primeira temporada): só a original.
unsafe extern "thiscall" fn comparar_injury_daily_update(this: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I0%%);
    let mut reg = vec![(this, 0x2c)];
    let n = cpw9_i(this + 8);
    if n > 0 && cpw9_d(this + 4) != 0 {
        reg.push((cpw9_d(this + 4), n as usize * 0x30));
    }
    reg.extend(crate::aleatorio::REGIOES_DO_ESTADO);
    let antes = crate::comparacao::Memoria::ler(&reg);
    CPW9_ENSAIO = true;
    CPW9_PAROU = false;
    crate::aleatorio::pausar_rastro(|| crate::aleatorio::sem_contar(|| injury_daily_update(this)));
    CPW9_ENSAIO = false;
    let mr = crate::comparacao::Memoria::ler(&reg);
    antes.escrever();
    if CPW9_PAROU {
        crate::comparacao::ramo("cpw_9 lesao nova so na original");
        return o(this);
    }
    *core::ptr::addr_of_mut!(CPW9_ESPERA) = Some((reg, mr));
    o(this);
    // sem a 0x62e010 desviada (desligada), a original não passou pelo ponto de conferência: fica sem comparação
    *core::ptr::addr_of_mut!(CPW9_ESPERA) = None;
}

/// { 0x62e010 } (thiscall, ret 0, injury.cpp, quadro SEH só das datas locais): `Injury::post_due_recovery_news()`, pelo
/// dia das lesões (0x62b7b0), todo dia. Nos clubes dos 16 técnicos humanos (as últimas 16 pessoas de [0xae23bc]), cada
/// jogador do elenco (50 vagas +0xd7) lesionado (+0xe): no dia em que começo (+0x20) + dias parados (+0x2a, mais de 5)
/// chega, a notícia da volta ao treino leve (0x630ee0, uma vez: +0x1c do registro de 31 bytes) e o físico do dia
/// (0x62c680). Depois a fila das voltas ([+0x10], 22 bytes: +0 data A, +8 data B, +0x10 id, +0x14 e +0x15 o que a
/// notícia leva, conta +0x18): as de data A hoje (bsearch 0x946464 com 0x778e90, voltando enquanto não for antes de hoje,
/// 0x456740) dão a notícia com os dias entre B e A (0x54cc30); as de data B hoje (bsearch com 0x632040) a notícia da volta
/// completa; as de data B já passada saem do começo da fila.
unsafe extern "thiscall" fn post_due_recovery_news(this: usize) {
    crate::comparacao::ramo("cpw_9 noticias de volta");
    let volta: unsafe extern "thiscall" fn(usize, usize, u32, u32, u32, u32) = core::mem::transmute(0x630ee0usize);
    let fisico: unsafe extern "thiscall" fn(usize, usize) = core::mem::transmute(0x62c680usize);
    let mais: unsafe extern "thiscall" fn(usize, *mut u8, u32) -> usize = core::mem::transmute(0x54c5b0usize);
    for k in 0..0x10i32 {
        let n = cpw9_i(0xae2364);
        let s = cpw9_d(0xae23bc).wrapping_add(((n - k - 1) as u32).wrapping_mul(0x6e) as usize);
        let clube = cpw9_d(s + 0x39);
        if clube == 0 {
            continue;
        }
        for j in 0..50usize {
            let p = cpw9_d(clube + 0xd7 + 4 * j);
            if p == 0 || cpw9_i(p) >= cpw9_i(0xae2364) {
                continue;
            }
            let i = cpw9_i(cpw9_registro(this, cpw9_i(p)) + 0xe);
            if i == -1 {
                continue;
            }
            let inj = cpw9_d(this + 4).wrapping_add((i as u32).wrapping_mul(0x30) as usize);
            let (dia, ano) = (cpw9_w(0xae2c90), cpw9_uw(0xae2c92));
            let mut t = [0u8; 8];
            let r = mais(inj + 0x20, t.as_mut_ptr(), cpw9_uw(inj + 0x2a) as u32);
            if ano == cpw9_uw(r + 2) && dia == cpw9_w(r) && cpw9_w(inj + 0x2a) > 5 {
                let marca = cpw9_registro(this, cpw9_i(p)) + 0x1c;
                if *(marca as *const u8) == 0 {
                    *(marca as *mut u8) = 1;
                    let dias = cpw9_uw(inj + 0x28).wrapping_sub(cpw9_uw(inj + 0x2a));
                    // ponytail: os bytes de cima dos argumentos de 8 e 16 bits não têm valor na original; a 0x630ee0 lê char/short
                    volta(this, p, 0, *(inj as *const u8) as u32, 0, dias as u32);
                }
            }
            fisico(this, p);
        }
    }
    if cpw9_d(this + 0x10) == 0 {
        return;
    }
    let padrao: unsafe extern "thiscall" fn(*mut u8) -> usize = core::mem::transmute(0x54a110usize);
    let copiar: unsafe extern "thiscall" fn(*mut u8, *mut u8, usize) -> usize = core::mem::transmute(0x417c10usize);
    let bsearch: unsafe extern "C" fn(*const u8, usize, u32, u32, usize) -> usize = core::mem::transmute(0x946464usize);
    let depois: unsafe extern "thiscall" fn(*const u8, u32, u32) -> u32 = core::mem::transmute(0x456740usize);
    let diferenca: unsafe extern "thiscall" fn(usize, *mut u8, u32, u32) -> usize = core::mem::transmute(0x54cc30usize);
    let (mut d1, mut d2, mut t) = ([0u8; 8], [0u8; 8], [0u8; 8]);
    padrao(d1.as_mut_ptr());
    padrao(d2.as_mut_ptr());
    copiar(d1.as_mut_ptr(), t.as_mut_ptr(), 0xae2c90);
    copiar(d2.as_mut_ptr(), t.as_mut_ptr(), 0xae2c90);
    let pd1 = d1.as_ptr() as usize;
    let (d1dia, d1ano) = (cpw9_w(pd1), cpw9_uw(pd1 + 2));
    let pd2 = d2.as_ptr() as usize;
    let (d2dia, d2ano) = (cpw9_w(pd2), cpw9_uw(pd2 + 2));
    let ultimo = || cpw9_d(this + 0x10).wrapping_add((cpw9_i(this + 0x18) as u32).wrapping_mul(0x16) as usize).wrapping_sub(0x16);
    let pessoa = |e: usize| cpw9_d(0xae23bc).wrapping_add((cpw9_d(e) as u32).wrapping_mul(0x6e) as usize);
    // as de data A hoje
    let mut e = bsearch(d1.as_ptr(), cpw9_d(this + 0x10), cpw9_i(this + 0x18) as u32, 0x16, 0x778e90);
    if e != 0 {
        if e > cpw9_d(this + 0x10) {
            loop {
                e -= 0x16;
                let (dia, ano) = (cpw9_w(e), cpw9_uw(e + 2));
                if ano > d1ano || (ano == d1ano && dia > d1dia) || depois(d1.as_ptr(), cpw9_d(e) as u32, cpw9_d(e + 4) as u32) != 0 {
                    e += 0x16;
                    break;
                }
                if e <= cpw9_d(this + 0x10) {
                    break;
                }
            }
        }
        while e <= ultimo() {
            let (dia, ano) = (cpw9_w(e), cpw9_uw(e + 2));
            if ano > d1ano || (ano == d1ano && dia > d1dia) {
                break;
            }
            if depois(d1.as_ptr(), cpw9_d(e) as u32, cpw9_d(e + 4) as u32) != 0 {
                break;
            }
            let p = pessoa(e + 0x10);
            let mut dif = [0u8; 8];
            diferenca(e + 8, dif.as_mut_ptr(), cpw9_d(e) as u32, cpw9_d(e + 4) as u32);
            let pd = dif.as_ptr() as usize;
            let dias = (cpw9_uw(pd + 2) as u32).wrapping_mul(365).wrapping_add(cpw9_uw(pd) as u32) as u16;
            volta(this, p, 0, *((e + 0x15) as *const u8) as u32, *((e + 0x14) as *const u8) as u32, dias as u32);
            e += 0x16;
        }
    }
    // as de data B hoje: a volta completa
    let mut e = bsearch(d1.as_ptr(), cpw9_d(this + 0x10), cpw9_i(this + 0x18) as u32, 0x16, 0x632040);
    if e == 0 {
        return;
    }
    let diferente = |b: usize| {
        let (dia, ano) = (cpw9_w(b), cpw9_uw(b + 2));
        ano != d2ano || dia != d2dia
    };
    if e > cpw9_d(this + 0x10) {
        loop {
            e -= 0x16;
            if diferente(e + 8) {
                e += 0x16;
                break;
            }
            if e <= cpw9_d(this + 0x10) {
                break;
            }
        }
    }
    while e <= ultimo() {
        if diferente(e + 8) {
            break;
        }
        let p = pessoa(e + 0x10);
        volta(this, p, 1, *((e + 0x15) as *const u8) as u32, *((e + 0x14) as *const u8) as u32, 0);
        e += 0x16;
    }
    // as de data B já passada saem do começo da fila
    let n = cpw9_i(this + 0x18);
    let mut tirar: i16 = 0;
    if n > 0 {
        let (hd, ha) = (cpw9_w(0xae2c90), cpw9_uw(0xae2c92));
        let mut q = cpw9_d(this + 0x10) + 0xa;
        let mut i = 0i32;
        while i < n {
            let ano = cpw9_uw(q);
            if ha < ano || (ha == ano && !(hd > cpw9_w(q - 2))) {
                break;
            }
            tirar = tirar.wrapping_add(1);
            i += 1;
            q += 0x16;
        }
        if tirar != 0 && tirar as i32 != n {
            let base = cpw9_d(this + 0x10);
            let t = tirar as i32 as usize;
            core::ptr::copy((base + t * 0x16) as *const u8, base as *mut u8, (n as usize - t) * 0x16);
        }
    }
    core::ptr::write_unaligned((this + 0x18) as *mut i32, cpw9_i(this + 0x18).wrapping_sub(tirar as i32));
}

// ponytail: as notícias da volta (0x630ee0) alocam e o físico do dia mexe no sorteio: só a original; a cópia leva o
// ramo. É também o ponto de conferência da fase do treino do dia das lesões (comparar_injury_daily_update).
unsafe extern "thiscall" fn comparar_post_due_recovery_news(this: usize) {
    if let Some((reg, mr)) = (*core::ptr::addr_of_mut!(CPW9_ESPERA)).take() {
        let mo = crate::comparacao::Memoria::ler(&reg);
        PLACAR.registrar(mo == mr, || format!("injury_daily_update({:x}) {:x?} x {:x?}", this, mo, mr));
    }
    orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%)(this)
}

/// { 0x55d360 } (thiscall, ret 8, discipline.cpp): `Discipline::is_one_booking_from_ban(pessoa, competicao)`: com id
/// válido, a competição ([[0xadadfc] + 4 × id]) e o órgão dela (+0x50, 0xff sem; [this] + 4 × órgão), a pergunta ao órgão
/// (0x7e8d60 com o id, 0, 0 e o +4 da competição); senão 0. Pela tela do clube (0x4849f0).
unsafe extern "thiscall" fn is_one_booking_from_ban(this: usize, pessoa: usize, competicao: usize) -> u32 {
    if pessoa != 0 && cpw9_i(pessoa) >= cpw9_i(0xae2364) {
        return 0;
    }
    let comp = cpw9_d(cpw9_d(0xadadfc).wrapping_add((cpw9_i(competicao) as u32).wrapping_mul(4) as usize));
    if comp == 0 || pessoa == 0 {
        return 0;
    }
    let orgao = *((comp + 0x50) as *const u8);
    if orgao == 0xff {
        return 0;
    }
    let o = cpw9_d(cpw9_d(this).wrapping_add((orgao as i8 as i32 as u32).wrapping_mul(4) as usize));
    if o == 0 {
        return 0;
    }
    let perguntar: unsafe extern "thiscall" fn(usize, u32, u32, u32, u32) -> u32 = core::mem::transmute(0x7e8d60usize);
    perguntar(o, cpw9_d(pessoa) as u32, 0, 0, cpw9_d(comp + 4) as u32)
}

unsafe extern "thiscall" fn comparar_is_one_booking_from_ban(this: usize, pessoa: usize, competicao: usize) -> u32 {
    let r = orig::<unsafe extern "thiscall" fn(usize, usize, usize) -> u32>(%%I2%%)(this, pessoa, competicao);
    let n = is_one_booking_from_ban(this, pessoa, competicao);
    PLACAR.registrar(r == n, || format!("is_one_booking_from_ban(pessoa {:x}, competicao {:x}) original={:x} rust={:x}", pessoa, competicao, r, n));
    r
}
