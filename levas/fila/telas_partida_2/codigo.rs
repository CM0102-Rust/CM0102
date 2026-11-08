// ---------- %%LEVA%%a leva: as telas da partida (2): a tela dos últimos placares (match_screens.cpp) ----------

/// O gerente de telas (o objeto [0xb74450], que tem a vaga da tela atual em +0x382a e um `Display` por
/// vaga em +0x7fc + 0x300 x vaga).
const SCREEN_MANAGER: usize = 0xb74450;

/// As regiões que a montagem de uma tela mexe: o gerente de telas (0x3840 bytes) e o `Display` da tela
/// atual (a lista de exibição: itens de 0x18c bytes em +0xba95e, quantos em +0x12e9a0; 0x12f550 bytes).
unsafe fn screen_build_regions() -> Vec<(usize, usize)> {
    let vaga = *((SCREEN_MANAGER + 0x382a) as *const i16);
    let mut r = vec![(SCREEN_MANAGER, 0x3840)];
    if (0..64).contains(&vaga) {
        let display = core::ptr::read_unaligned((SCREEN_MANAGER + 0x7fc + vaga as usize * 0x300) as *const usize);
        if display != 0 {
            r.push((display, 0x12f550));
        }
    }
    r
}

/// { 0x71b1b0 } (cdecl, match_screens.cpp): abre a tela dos últimos placares da competição `entrada` na
/// vaga `vaga` (0x7fe4a0 com a montagem 0x71b300 e a resposta 0x71b3d0) e grava os parâmetros dela
/// (0x7ff130): 0 e 0xe (a entrada) e 0xf..0x14 (0, −1, 1, 0, 1, 0). Vaga ou entrada nula: a original
/// (caixa de erro).
unsafe extern "C" fn open_latest_scores(vaga: u32, entrada: usize) {
    if vaga == 0 || entrada == 0 {
        return original_fn::<unsafe extern "C" fn(u32, usize)>(%%I0%%)(vaga, entrada);
    }
    crate::comparison::branch("screens open the latest scores");
    let abrir: unsafe extern "thiscall" fn(usize, u32, usize, usize, u32, u32, u32) -> u32 = core::mem::transmute(0x7fe4a0usize);
    if abrir(SCREEN_MANAGER, vaga, 0x71b300, 0x71b3d0, 0, 0, 0) == 0 {
        return;
    }
    let gravar: unsafe extern "thiscall" fn(usize, u32, u32, usize, u32) = core::mem::transmute(0x7ff130usize);
    for (k, v) in [(0u32, 0usize), (0xe, entrada), (0xf, 0), (0x10, usize::MAX), (0x11, 1), (0x12, 0), (0x13, 1), (0x14, 0)] {
        gravar(SCREEN_MANAGER, vaga, k, v, 0);
    }
}

/// { 0x71b300 } (cdecl, match_screens.cpp): monta a tela dos últimos placares: o fundo da competição
/// (registro de 107 bytes em [0xae23d0] pelo id do parâmetro 0xe, 0x41c410), o título "Latest Scores"
/// (0x66f4e0 em 0xde1f64, cores 0x839590, 0x803f30), a barra de menu (0x75db00 com 4), o painel dos
/// placares (0x723d20, de 0x50 a 0x217) e os botões de voltar e avançar (0x5edcc0, desligados quando o
/// gerente não tem para onde ir).
unsafe extern "C" fn build_latest_scores() {
    crate::comparison::branch("screens build the latest scores");
    let ler: unsafe extern "thiscall" fn(usize, u32) -> usize = core::mem::transmute(0x7ff010usize);
    let entrada = ler(SCREEN_MANAGER, 0xe);
    let id = *(entrada as *const i32);
    let fundo: unsafe extern "C" fn(usize, u32, u32) -> i32 = core::mem::transmute(0x41c410usize);
    fundo((*(0xae23d0 as *const usize) as isize + id as isize * 107) as usize, 0, 0);
    let texto: unsafe extern "C" fn(usize, usize, ...) -> i32 = core::mem::transmute(0x66f4e0usize);
    texto(0xde1f64, 0xa16328, 0xad9c64);
    let cor: unsafe extern "C" fn(u32) -> u32 = core::mem::transmute(0x839590usize);
    let (c1, c0) = (cor(1), cor(0));
    let titulo: unsafe extern "thiscall" fn(usize, u32, u32, usize) = core::mem::transmute(0x803f30usize);
    titulo(SCREEN_MANAGER, c0, c1, 0xde1f64);
    let menu: unsafe extern "C" fn(u32, u32) = core::mem::transmute(0x75db00usize);
    menu(4, 0);
    let (mut topo, mut base) = (0x50i32, 0x217i32);
    let painel: unsafe extern "C" fn(*mut i32, *mut i32) = core::mem::transmute(0x723d20usize);
    painel(&mut topo, &mut base);
    let sem_avancar: unsafe extern "thiscall" fn(usize) -> u32 = core::mem::transmute(0x7febb0usize);
    let sem_voltar: unsafe extern "thiscall" fn(usize, u32) -> u32 = core::mem::transmute(0x7fec60usize);
    let avancar = (sem_avancar(SCREEN_MANAGER) == 0) as u32;
    let voltar = (sem_voltar(SCREEN_MANAGER, 0) == 0) as u32;
    let botoes: unsafe extern "C" fn(u32, u32) -> u32 = core::mem::transmute(0x5edcc0usize);
    botoes(voltar, avancar);
}

/// { 0x71b3d0 } (cdecl, match_screens.cpp): a resposta da tela dos últimos placares ao item `item`.
/// Sem [0xb77c84]: zera 0xb6b7a8/ac/b0; fora do modo 2 da partida ([[0xb67a50]+0x34]+0x67) e com o
/// técnico humano da vez conectado (0x5fd210), marca a hora (0x5fdd30 com 0x90c510); depois a troca de
/// competição (0x725860) decide; sem decisão, −11 com [0xb6b7b0] ou redesenha com [0xb6b7a8]
/// (0x802d00). Por fim a barra de menu (0x7615d0, senão 0x7643d0).
unsafe extern "C" fn latest_scores_input(item: u32) -> i32 {
    crate::comparison::branch("screens respond latest scores");
    let g = |a: usize| *(a as *const u32);
    if g(0xb77c84) == 0 {
        let m = g(0xb67a50) as usize;
        *(0xb6b7ac as *mut u32) = 0;
        *(0xb6b7b0 as *mut u32) = 0;
        *(0xb6b7a8 as *mut u32) = 0;
        if *((*((m + 0x34) as *const usize) + 0x67) as *const u8) != 2 {
            let humano = *(0xb77c7a as *const i16) as isize;
            if *((0xb74c26 + humano * 0x300) as *const u32) != 0 {
                let tecnicos = g(0xb63c98) as usize;
                let conectado: unsafe extern "thiscall" fn(usize, usize) -> u32 = core::mem::transmute(0x5fd210usize);
                if conectado(tecnicos, 0) != 0 {
                    let agora: unsafe extern "C" fn() -> u32 = core::mem::transmute(0x90c510usize);
                    let marcar: unsafe extern "thiscall" fn(usize, u32, usize) = core::mem::transmute(0x5fdd30usize);
                    marcar(tecnicos, agora(), 0);
                }
            }
        }
        let trocar: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x725860usize);
        let r = trocar(item);
        if r != 0 {
            return r;
        }
        if g(0xb6b7b0) != 0 {
            return -11;
        }
        if g(0xb6b7a8) != 0 {
            let redesenhar: unsafe extern "thiscall" fn(usize, u32, u32) = core::mem::transmute(0x802d00usize);
            redesenhar(SCREEN_MANAGER, 0, 0);
        }
    }
    let comando: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7615d0usize);
    let r = comando(item);
    if r != 0 {
        return r;
    }
    let ligacao: unsafe extern "C" fn(u32) -> i32 = core::mem::transmute(0x7643d0usize);
    ligacao(item)
}

// ponytail: abrir e responder mexem em telas, menu e hora (não rodam duas vezes): no modo comparação só a
// original roda; as cópias se provam no jogo do Lucas em modo Rust (contadores "telas ..." no ddraw.log)
unsafe extern "C" fn compare_open_latest_scores(vaga: u32, entrada: usize) {
    original_fn::<unsafe extern "C" fn(u32, usize)>(%%I0%%)(vaga, entrada)
}

/// A montagem é comparada de verdade: as duas montam a tela a partir do mesmo estado e o gerente de
/// telas, o `Display` da tela atual (a lista de exibição) e o título (0xde1f64) têm de sair iguais.
unsafe extern "C" fn compare_build_latest_scores() {
    let o = original_fn::<unsafe extern "C" fn()>(%%I1%%);
    let mut reg = screen_build_regions();
    reg.push((0xde1f64, 0x100));
    // a foto de fundo (0x41c410) sorteia: o estado do gerador volta para a segunda rodada (sem ele o
    // gerador andava duas vezes no modo comparação e a carreira se deslocava; 204ª)
    reg.extend_from_slice(&crate::rng::STATE_REGIONS);
    let (_, _, igual, mo, mr) = crate::comparison::side_by_side(&reg, || o(), || crate::rng::without_counting(|| build_latest_scores()));
    SCOREBOARD.record(igual, || {
        // so as diferencas (a lista e grande): onde comecam e quantos bytes
        let dif: Vec<String> = mo.0.iter().zip(mr.0.iter()).filter_map(|((a, x), (_, y))| {
            let k = x.iter().zip(y.iter()).position(|(p, q)| p != q)?;
            Some(format!("{:#x}+{:#x}", a, k))
        }).collect();
        format!("build_latest_scores: diferencas em {:?}", dif)
    });
}

unsafe extern "C" fn compare_latest_scores_input(item: u32) -> i32 {
    original_fn::<unsafe extern "C" fn(u32) -> i32>(%%I2%%)(item)
}
