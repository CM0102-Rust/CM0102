// ---------- %%LEVA%%a leva: a copa: os registros das pessoas pelo desfecho, as honras da temporada e os semifinalistas para a UEFA (Cup.cpp) ----------

// A escuta das chamadas de efeito (usada também pelos itens cpb_2 e cpb_3, que vêm depois deste na fila).
// As rotinas da copa quase só despacham: o efeito está nas rotinas que chamam (registros das pessoas, histórico,
// notícia, finanças, o progresso do clube), originais que alocam e não podem rodar duas vezes. O comparador roda a
// original com os `call` dela trocados (só durante a chamada) por porteiros que gravam cada chamada (rotina, this,
// argumentos, resposta) e a deixam seguir; as chamadas virtuais com efeito vão por uma cópia da vtable do objeto com
// o porteiro na posição. Desfeitas as trocas, a cópia Rust roda conferindo cada chamada de efeito contra a fita (sem
// executá-la; devolve a resposta gravada). As consultas puras (0x669470, 0x5c6710, 0x4a4850, 0x5fa730, 0x53b3d0,
// 0x5a0590, 0x522360, a posição 0x88) rodam de verdade nos dois lados.

/// Uma chamada de efeito: a rotina chamada, o this (ecx; 0 nas cdecl), os argumentos (até 7) e a resposta (eax).
#[derive(Clone, Copy, PartialEq, Debug)]
struct Cpb1Chamada {
    alvo: usize,
    this: usize,
    args: [u32; 7],
    n: usize,
    resp: u32,
}

static mut CPB1_FITA: Vec<Cpb1Chamada> = Vec::new();
/// Profundidade dentro de uma chamada gravada: só as chamadas de fora (0) entram na fita.
static mut CPB1_DENTRO: u32 = 0;
/// Conferindo (a cópia no comparador): a posição na fita; None: as chamadas vão ao jogo.
static mut CPB1_LENDO: Option<usize> = None;
/// A primeira chamada da cópia que não bateu com a fita (texto para a mensagem).
static mut CPB1_ERRO: Option<String> = None;
/// As vtables trocadas: (objeto, a rotina verdadeira da posição escutada).
static mut CPB1_TROCAS: Vec<(usize, usize)> = Vec::new();

unsafe fn cpb1_u32(a: usize) -> u32 {
    core::ptr::read_unaligned(a as *const u32)
}
unsafe fn cpb1_us(a: usize) -> usize {
    core::ptr::read_unaligned(a as *const usize)
}
unsafe fn cpb1_i16(a: usize) -> i16 {
    core::ptr::read_unaligned(a as *const i16)
}
unsafe fn cpb1_u16(a: usize) -> u16 {
    core::ptr::read_unaligned(a as *const u16)
}
unsafe fn cpb1_u8(a: usize) -> u8 {
    *(a as *const u8)
}
/// `base + k*tam` com a aritmética de 32 bits da original (índices com sinal).
fn cpb1_em(base: usize, k: i32, tam: i32) -> usize {
    base.wrapping_add(k.wrapping_mul(tam) as usize)
}

/// Grava uma chamada de fora (profundidade 0); devolve a posição na fita para a resposta.
unsafe fn cpb1_anotar(alvo: usize, this: usize, a: &[u32]) -> Option<usize> {
    if CPB1_DENTRO != 0 {
        return None;
    }
    let mut args = [0u32; 7];
    args[..a.len()].copy_from_slice(a);
    let fita = &mut *core::ptr::addr_of_mut!(CPB1_FITA);
    fita.push(Cpb1Chamada { alvo, this, args, n: a.len(), resp: 0 });
    Some(fita.len() - 1)
}

/// Roda a chamada verdadeira um nível abaixo e guarda a resposta na fita.
unsafe fn cpb1_seguir(k: Option<usize>, f: impl FnOnce() -> u32) -> u32 {
    CPB1_DENTRO += 1;
    let r = f();
    CPB1_DENTRO -= 1;
    if let Some(k) = k {
        (&mut *core::ptr::addr_of_mut!(CPB1_FITA))[k].resp = r;
    }
    r
}

// Os porteiros dos `call` diretos (A = a rotina chamada), um por convenção e número de argumentos.
unsafe extern "thiscall" fn cpb1_t0<const A: usize>(this: usize) -> u32 {
    let k = cpb1_anotar(A, this, &[]);
    cpb1_seguir(k, || core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(A)(this))
}
unsafe extern "thiscall" fn cpb1_t1<const A: usize>(this: usize, a: u32) -> u32 {
    let k = cpb1_anotar(A, this, &[a]);
    cpb1_seguir(k, || core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> u32>(A)(this, a))
}
unsafe extern "thiscall" fn cpb1_t3<const A: usize>(this: usize, a: u32, b: u32, c: u32) -> u32 {
    let k = cpb1_anotar(A, this, &[a, b, c]);
    cpb1_seguir(k, || core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(A)(this, a, b, c))
}
unsafe extern "thiscall" fn cpb1_t5<const A: usize>(this: usize, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let k = cpb1_anotar(A, this, &[a, b, c, d, e]);
    cpb1_seguir(k, || core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(A)(this, a, b, c, d, e))
}
unsafe extern "thiscall" fn cpb1_t7<const A: usize>(this: usize, a: u32, b: u32, c: u32, d: u32, e: u32, f: u32, g: u32) -> u32 {
    let k = cpb1_anotar(A, this, &[a, b, c, d, e, f, g]);
    cpb1_seguir(k, || {
        core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32, u32, u32) -> u32>(A)(this, a, b, c, d, e, f, g)
    })
}
unsafe extern "C" fn cpb1_c5<const A: usize>(a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let k = cpb1_anotar(A, 0, &[a, b, c, d, e]);
    cpb1_seguir(k, || core::mem::transmute::<usize, unsafe extern "C" fn(u32, u32, u32, u32, u32) -> u32>(A)(a, b, c, d, e))
}

/// A rotina verdadeira da vtable trocada do objeto (a primeira troca dele).
unsafe fn cpb1_verdadeira(this: usize) -> usize {
    (*core::ptr::addr_of!(CPB1_TROCAS)).iter().find(|t| t.0 == this).map(|t| t.1).unwrap_or(0)
}
// Os porteiros das posições de vtable (a rotina vem da troca do objeto).
unsafe extern "thiscall" fn cpb1_v2(this: usize, a: u32, b: u32) -> u32 {
    let alvo = cpb1_verdadeira(this);
    let k = cpb1_anotar(alvo, this, &[a, b]);
    cpb1_seguir(k, || core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32) -> u32>(alvo)(this, a, b))
}
unsafe extern "thiscall" fn cpb1_v5(this: usize, a: u32, b: u32, c: u32, d: u32, e: u32) -> u32 {
    let alvo = cpb1_verdadeira(this);
    let k = cpb1_anotar(alvo, this, &[a, b, c, d, e]);
    cpb1_seguir(k, || core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(alvo)(this, a, b, c, d, e))
}

/// A chamada de efeito da cópia Rust. No jogo (`CPB1_LENDO` vazio) roda `f`, a rotina pelo endereço (ou pelo ponteiro
/// da vtable, como a original); conferindo, compara (rotina, this, argumentos sob `mascara`) com a próxima da fita e
/// devolve a resposta gravada, sem rodar.
unsafe fn cpb1_chamar(alvo: usize, this: usize, args: &[u32], mascara: &[u32], f: impl FnOnce() -> u32) -> u32 {
    let Some(k) = CPB1_LENDO else {
        return f();
    };
    CPB1_LENDO = Some(k + 1);
    let fita = &*core::ptr::addr_of!(CPB1_FITA);
    let bate = |g: &Cpb1Chamada| {
        g.alvo == alvo && g.this == this && g.n == args.len() && args.iter().zip(&g.args).zip(mascara).all(|((a, b), m)| a & m == b & m)
    };
    match fita.get(k) {
        Some(g) if bate(g) => g.resp,
        g => {
            if (*core::ptr::addr_of!(CPB1_ERRO)).is_none() {
                CPB1_ERRO = Some(format!("chamada {}: rust {:#x}(this {:x}, {:x?}) x original {:x?}", k, alvo, this, args, g));
            }
            0
        }
    }
}
const CPB1_TUDO: [u32; 7] = [!0; 7];

/// Troca o rel32 do `call` em `sitio` para saltar a `para`; devolve o rel32 antigo.
unsafe fn cpb1_trocar_call(sitio: usize, rel: Option<i32>, para: usize) -> i32 {
    let p = (sitio + 1) as *mut i32;
    let mut antiga = 0u32;
    crate::win::VirtualProtect(p as *mut _, 4, crate::win::PAGE_EXECUTE_READWRITE, &mut antiga);
    let velho = core::ptr::read_unaligned(p);
    core::ptr::write_unaligned(p, rel.unwrap_or((para as isize - (sitio as isize + 5)) as i32));
    crate::win::VirtualProtect(p as *mut _, 4, antiga, &mut antiga);
    crate::win::FlushInstructionCache(crate::win::GetCurrentProcess(), p as *const _, 4);
    velho
}

/// O comparador com escuta: roda a original com os `call` de `sitios` (endereço do e8, porteiro) e as posições de
/// `vtabs` (objeto, deslocamento na vtable, porteiro) escutados, desfaz as trocas e roda a cópia conferindo contra a
/// fita; `regioes` são as escritas diretas da rotina (lado_a_lado). Devolve (resposta da original, igual, mensagem).
unsafe fn cpb1_comparar<R: Copy + PartialEq + core::fmt::Debug>(sitios: &[(usize, usize)], vtabs: &[(usize, usize, usize)],
    regioes: &[(usize, usize)], original: impl FnOnce() -> R, rust: impl FnOnce() -> R) -> (R, bool, String) {
    // a comparação de dentro (uma original escutada que chama outra rotina comparada) guarda a de fora
    // (as trocas de fora ficam visíveis: um porteiro de fora acionado aqui dentro ainda acha a rotina verdadeira)
    let fora = (core::mem::take(&mut *core::ptr::addr_of_mut!(CPB1_FITA)), CPB1_DENTRO, (*core::ptr::addr_of_mut!(CPB1_LENDO)).take(),
        (*core::ptr::addr_of_mut!(CPB1_ERRO)).take(), (*core::ptr::addr_of!(CPB1_TROCAS)).len());
    CPB1_DENTRO = 0;
    let (r, n, igual, mo, mr) = crate::comparacao::lado_a_lado_leve(regioes, || {
        let velhos: Vec<i32> = sitios.iter().filter(|s| *(s.0 as *const u8) == 0xe8).map(|&(s, p)| cpb1_trocar_call(s, None, p)).collect();
        let mut copias: Vec<Box<[usize; 128]>> = Vec::new();
        let mut vts = Vec::new();
        for &(obj, pos, porteiro) in vtabs {
            let vt = cpb1_us(obj);
            let mut c = Box::new([0usize; 128]); // da RTTI (vt-4) a 127 posições
            core::ptr::copy_nonoverlapping((vt - 4) as *const usize, c.as_mut_ptr(), 128);
            (*core::ptr::addr_of_mut!(CPB1_TROCAS)).push((obj, c[1 + pos / 4]));
            c[1 + pos / 4] = porteiro;
            *(obj as *mut usize) = c.as_ptr() as usize + 4;
            vts.push((obj, vt));
            copias.push(c);
        }
        let r = original();
        for &(obj, vt) in vts.iter().rev() {
            *(obj as *mut usize) = vt;
        }
        for (&(s, _), v) in sitios.iter().filter(|s| *(s.0 as *const u8) == 0xe8).zip(velhos) {
            cpb1_trocar_call(s, Some(v), 0);
        }
        drop(copias);
        r
    }, || {
        CPB1_LENDO = Some(0);
        rust()
    });
    let lidas = (*core::ptr::addr_of_mut!(CPB1_LENDO)).take().unwrap_or(0);
    let fita = core::mem::take(&mut *core::ptr::addr_of_mut!(CPB1_FITA));
    let erro = (*core::ptr::addr_of_mut!(CPB1_ERRO)).take();
    let ok = igual && r == n && erro.is_none() && lidas == fita.len();
    let msg = if ok {
        String::new()
    } else {
        format!("original={:?} rust={:?} chamadas original {} rust {} {} fita {:x?} memoria {:x?} x {:x?}",
            r, n, fita.len(), lidas, erro.unwrap_or_default(), fita, mo, mr)
    };
    let (f, d, l, e, t) = fora;
    CPB1_FITA = f;
    CPB1_DENTRO = d;
    CPB1_LENDO = l;
    CPB1_ERRO = e;
    (*core::ptr::addr_of_mut!(CPB1_TROCAS)).truncate(t);
    (r, ok, msg)
}

// As consultas puras que as rotinas da copa chamam (pelo endereço, rodam nos dois lados).
/// 0x669470 key_nation_index_of_comp (cdecl): o índice da nação-chave da competição (char).
unsafe fn cpb1_indice_da_nacao(comp: u32) -> i8 {
    core::mem::transmute::<usize, unsafe extern "C" fn(u32) -> u32>(0x669470)(comp) as u8 as i8
}
/// 0x5c6710 Friendly::is_friendly_competition([0xae2c74], comp).
unsafe fn cpb1_e_amistoso(comp: u32) -> u32 {
    core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> u32>(0x5c6710)(cpb1_us(0xae2c74), comp)
}

/// { 0x519890 } (thiscall ret 0x18, Cup.cpp): `Cup::update_staff_records_for_status(clube, desfecho, _, _, partida, a6)`:
/// partida com +0x3a = 1, copa com +0x2c ≠ 0 ou etapa (+0x44) ≠ 0xff: nada. Desfecho 1 (campeão): registro de campeão
/// (0x86a800) e, se a copa for a da Liga inglesa (chave 0x9cf750), ou europeia (nação +0x5d → continente +0x71 com a
/// chave 0x9cfa10) que não é amistoso, com o estado do clube (0xae2d14, id*6) ainda -1 e sendo a copa principal da
/// nação-chave (0xb63d7e + k*0x48), o estado do clube vira a chave da Copa da UEFA (0x9cf6f0). 2: vice (0x86b0b0);
/// 3: terceiro (0x86b710); 0: classificado para a fase seguinte (0x86bdd0; nada se a seguinte é a última, 0x8c); 0xff:
/// eliminado (0x86c000). A importância é (fase*99)/(fases−1 ou fases)+1 no byte baixo; os outros argumentos levam os
/// bits altos que a original deixa nos registradores. Devolve 0.
unsafe extern "thiscall" fn copa_registros_pelo_desfecho(copa: usize, clube: u32, desfecho: u32, _a3: u32, _a4: u32, partida: usize, a6: u32) -> u32 {
    if cpb1_u8(partida + 0x3a) == 1 || cpb1_u32(copa + 0x2c) != 0 || cpb1_u8(copa + 0x44) != 0xff {
        return 0;
    }
    let registros = cpb1_us(0xdd7f00);
    let comp = cpb1_u32(copa + 4);
    let registro3 = |alvo: usize| {
        cpb1_chamar(alvo, registros, &[clube, partida as u32, a6], &CPB1_TUDO, || {
            core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(alvo)(registros, clube, partida as u32, a6)
        });
    };
    let registro5 = |alvo: usize, a: [u32; 5]| {
        cpb1_chamar(alvo, registros, &a, &CPB1_TUDO, || {
            core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, u32) -> u32>(alvo)(registros, a[0], a[1], a[2], a[3], a[4])
        });
    };
    let importancia = |q: i32| (q as u32 & 0xffff_ff00) | (q as u8).wrapping_add(1) as u32; // `inc al`
    match desfecho as u8 {
        1 => {
            registro3(0x86a800); // campeão
            let estado = || cpb1_em(cpb1_us(0xae2d14), cpb1_u32(clube as usize) as i32, 6);
            if cpb1_u32(comp as usize) != cpb1_u32(0x9cf750) {
                let nacao = cpb1_us(comp as usize + 0x5d);
                if nacao == 0 {
                    return 0;
                }
                let continente = cpb1_us(nacao + 0x71);
                if continente == 0 || cpb1_u32(continente) != cpb1_u32(0x9cfa10) || cpb1_e_amistoso(comp) != 0 || cpb1_u32(estado()) != u32::MAX {
                    return 0;
                }
                if cpb1_indice_da_nacao(comp) == -1 {
                    return 0;
                }
                let k = cpb1_indice_da_nacao(comp) as i32; // a original pergunta duas vezes
                if cpb1_u32(cpb1_em(0xb63d7e, k, 0x48)) != comp {
                    return 0;
                }
            }
            core::ptr::write_unaligned(estado() as *mut u32, cpb1_u32(0x9cf6f0)); // a Copa da UEFA
        }
        2 => registro3(0x86b0b0), // vice
        3 => registro3(0x86b710), // terceiro
        0 => {
            let fase = cpb1_i16(partida + 0x34);
            let fases = cpb1_i16(copa + 0x3c);
            let seguinte = fase.wrapping_add(1);
            if seguinte >= fases {
                return 0;
            }
            let rodadas = cpb1_us(copa + 0xa3);
            let tipo = cpb1_u16(cpb1_em(rodadas, seguinte as i32, 0x68) + 7);
            if tipo == 0x8c {
                return 0;
            }
            let q = (fase as i32 * 99) / (fases as i32 - 1); // classificado
            registro5(0x86bdd0, [clube, comp, (partida as u32 & 0xffff_0000) | cpb1_u16(partida + 0x32) as u32,
                (rodadas as u32 & 0xffff_0000) | tipo as u32, importancia(q)]);
        }
        0xff => {
            let t = cpb1_i16(partida + 0x34) as i32 * 99;
            let fases = cpb1_i16(copa + 0x3c) as i32;
            let (q, resto) = (t / fases, t % fases); // eliminado
            let imp = importancia(q);
            registro5(0x86c000, [clube, comp, (imp & 0xffff_0000) | cpb1_u16(partida + 0x32) as u32,
                (resto as u32 & 0xffff_0000) | cpb1_u16(partida + 0x30) as u32, imp]);
        }
        _ => {}
    }
    0
}

unsafe extern "thiscall" fn comparar_copa_registros_pelo_desfecho(copa: usize, clube: u32, desfecho: u32, a3: u32, a4: u32, partida: usize, a6: u32) -> u32 {
    let o = orig::<unsafe extern "thiscall" fn(usize, u32, u32, u32, u32, usize, u32) -> u32>(%%I0%%);
    // a escrita direta: o estado do clube (só no campeão)
    let mut reg = Vec::new();
    if desfecho as u8 == 1 && clube != 0 {
        reg.push((cpb1_em(cpb1_us(0xae2d14), cpb1_u32(clube as usize) as i32, 6), 4));
    }
    let sitios = [(0x5198d5, cpb1_t3::<0x86a800> as *const () as usize), (0x5199a5, cpb1_t3::<0x86b0b0> as *const () as usize), (0x5199c7, cpb1_t3::<0x86b710> as *const () as usize),
        (0x519a33, cpb1_t5::<0x86bdd0> as *const () as usize), (0x519a75, cpb1_t5::<0x86c000> as *const () as usize)];
    let (r, ok, msg) = cpb1_comparar(&sitios, &[], &reg, || o(copa, clube, desfecho, a3, a4, partida, a6),
        || copa_registros_pelo_desfecho(copa, clube, desfecho, a3, a4, partida, a6));
    PLACAR.registrar(ok, || format!("copa_registros_pelo_desfecho(desfecho {}) {}", desfecho as u8, msg));
    r
}

/// { 0x519a90 } (thiscall, Cup.cpp): `Cup::record_season_honours()`: o campeão, o vice e o terceiro (times com +5 = 1,
/// 2 e 3; para no primeiro trio completo); na copa galesa (chave 0x9cf958) o terceiro é o primeiro time com progresso
/// (+0x73 da estatística 0x4a4850) 3. A seleção anfitriã: se a posição 0x88 da competição diz que tem anfitrião, as
/// nações de 0x5fa730 (duas: a de maior +0x8e, empate a primeira; uma: a primeira) dão a seleção (0x53b3d0). Grava o
/// histórico (0x4afce0) e, fora de amistoso, sendo a copa principal da nação-chave, os semifinalistas para a UEFA
/// (0x519fc0).
unsafe extern "thiscall" fn copa_honras_da_temporada(copa: usize) {
    let n = cpb1_i16(copa + 0x3e);
    let (mut campeao, mut vice, mut terceiro) = (0u32, 0u32, 0u32);
    if n > 0 {
        let times = cpb1_us(copa + 0xa7);
        let mut k: i16 = 0;
        loop {
            let e = cpb1_em(times, k as i32, 6);
            match cpb1_u8(e + 5) {
                1 => campeao = cpb1_u32(e),
                2 => vice = cpb1_u32(e),
                3 => terceiro = cpb1_u32(e),
                _ => {}
            }
            if campeao != 0 && vice != 0 && terceiro != 0 {
                break;
            }
            k = k.wrapping_add(1);
            if k >= n {
                break;
            }
        }
    }
    let comp = cpb1_us(copa + 4);
    if comp != 0 && cpb1_u32(comp) == cpb1_u32(0x9cf958) && n > 0 {
        let mut k: i16 = 0;
        loop {
            let clube = cpb1_u32(cpb1_em(cpb1_us(copa + 0xa7), k as i32, 6));
            let objeto = cpb1_us(cpb1_em(cpb1_us(0xadadfc), cpb1_u32(cpb1_us(copa + 4)) as i32, 4));
            let entrada = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32) -> usize>(0x4a4850)(cpb1_us(objeto + 8), clube);
            if cpb1_u8(entrada + 0x73) == 3 {
                terceiro = cpb1_u32(cpb1_em(cpb1_us(copa + 0xa7), k as i32, 6)); // galesa
                break;
            }
            k = k.wrapping_add(1);
            if k >= cpb1_i16(copa + 0x3e) {
                break;
            }
        }
    }
    let comp = cpb1_u32(copa + 4);
    let objeto = cpb1_us(cpb1_em(cpb1_us(0xadadfc), cpb1_u32(comp as usize) as i32, 4));
    let tem_anfitriao = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(cpb1_us(cpb1_us(objeto) + 0x88))(objeto);
    let mut selecao = 0u32;
    if tem_anfitriao != 0 {
        let (mut primeira, mut segunda) = (0i32, 0i32);
        let r = core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, *mut i32, *mut i32, u32) -> u32>(0x5fa730)(
            cpb1_us(0xb5e134), cpb1_u32(comp as usize), cpb1_u16(copa + 0x40) as u32, &mut primeira, &mut segunda, 0) as u8;
        let nacoes = cpb1_us(0xae23a8);
        let principal = |nacao: usize| core::mem::transmute::<usize, unsafe extern "C" fn(usize) -> u32>(0x53b3d0)(nacao);
        if r == 2 {
            let (a, b) = (cpb1_em(nacoes, segunda, 290), cpb1_em(nacoes, primeira, 290));
            selecao = if cpb1_i16(b + 0x8e) >= cpb1_i16(a + 0x8e) { principal(b) } else { principal(a) }; // anfitriãs
        } else if r != 0 {
            selecao = principal(cpb1_em(nacoes, primeira, 290));
        }
    }
    cpb1_chamar(0x4afce0, 0, &[copa as u32, campeao, vice, terceiro, selecao], &CPB1_TUDO, || {
        core::mem::transmute::<usize, unsafe extern "C" fn(usize, u32, u32, u32, u32) -> u32>(0x4afce0)(copa, campeao, vice, terceiro, selecao)
    });
    let comp = cpb1_u32(copa + 4);
    if cpb1_e_amistoso(comp) == 0 {
        let k = cpb1_indice_da_nacao(comp) as i32;
        if cpb1_u32(cpb1_em(0xb63d7e, k, 0x48)) == comp {
            cpb1_chamar(0x519fc0, copa, &[], &CPB1_TUDO, || {
                core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize) -> u32>(0x519fc0)(copa)
            });
        }
    }
}

unsafe extern "thiscall" fn comparar_copa_honras_da_temporada(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I1%%);
    let sitios = [(0x519c40, cpb1_c5::<0x4afce0> as *const () as usize), (0x519c7d, cpb1_t0::<0x519fc0> as *const () as usize)];
    let (_, ok, msg) = cpb1_comparar(&sitios, &[], &[], || o(copa), || copa_honras_da_temporada(copa));
    PLACAR.registrar(ok, || format!("copa_honras_da_temporada {}", msg));
}

/// { 0x519fc0 } (thiscall, Cup.cpp): `Cup::register_semi_final_losers_for_uefa()`: acha a rodada das semifinais (+7 =
/// 0x82) e a final (0x96); os dois times da semifinal que não estão na final (o primeiro e o segundo achados, pelos
/// ids) vão para a semeadura da UEFA (0x905790 em [0xde1f58]: competição, perdedor 1, perdedor 2; −1 se faltar). Sem
/// semifinal ou final, ou um time −1 numa semifinal antes de achar os dois: nada.
unsafe extern "thiscall" fn copa_semifinalistas_para_a_uefa(copa: usize) {
    if cpb1_i16(copa + 0x3c) <= 0 {
        return;
    }
    let rodadas = cpb1_us(copa + 0xa3);
    let (mut semi, mut fin) = (-1i16, -1i16);
    let mut k: i16 = 0;
    loop {
        match cpb1_u16(cpb1_em(rodadas, k as i32, 0x68) + 7) {
            0x82 => semi = k,
            0x96 => fin = k,
            _ => {}
        }
        if semi != -1 && fin != -1 {
            break;
        }
        k = k.wrapping_add(1);
        if k >= cpb1_i16(copa + 0x3c) {
            break;
        }
    }
    if semi == -1 || fin == -1 {
        return;
    }
    let lacos = cpb1_us(copa + 0xad);
    let final_ = cpb1_us(cpb1_em(lacos, fin as i32, 4));
    let (f1, f2) = (cpb1_u16(final_), cpb1_u16(final_ + 2));
    let n = cpb1_i16(cpb1_em(rodadas, semi as i32, 0x68) + 0x1a);
    let id = |t: u16| cpb1_u32(cpb1_us(cpb1_em(cpb1_us(copa + 0xa7), t as i16 as i32, 6)));
    let mut primeiro = u32::MAX;
    let mut segundo = u32::MAX;
    if n > 0 {
        let semis = cpb1_us(cpb1_em(lacos, semi as i32, 4));
        let mut i: i16 = 0;
        'lacos: loop {
            let t = cpb1_em(semis, i as i32, 0x35);
            for lado in [0, 2] {
                let time = cpb1_u16(t + lado);
                if time == f1 || time == f2 {
                    continue; // finalista
                }
                if time == 0xffff {
                    return;
                }
                if primeiro != u32::MAX {
                    segundo = id(time);
                    break 'lacos;
                }
                primeiro = id(time);
            }
            i = i.wrapping_add(1);
            if i >= n {
                break;
            }
        }
    }
    let semeadura = cpb1_us(0xde1f58);
    let comp = cpb1_u32(cpb1_us(copa + 4));
    cpb1_chamar(0x905790, semeadura, &[comp, primeiro, segundo], &CPB1_TUDO, || {
        core::mem::transmute::<usize, unsafe extern "thiscall" fn(usize, u32, u32, u32) -> u32>(0x905790)(semeadura, comp, primeiro, segundo)
    });
}

unsafe extern "thiscall" fn comparar_copa_semifinalistas_para_a_uefa(copa: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(%%I2%%);
    let (_, ok, msg) = cpb1_comparar(&[(0x51a137, cpb1_t3::<0x905790> as *const () as usize)], &[], &[], || o(copa), || copa_semifinalistas_para_a_uefa(copa));
    PLACAR.registrar(ok, || format!("copa_semifinalistas_para_a_uefa {}", msg));
}
