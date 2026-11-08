use super::*;

// ---------- 208a leva: as áreas da interface (1): organizar a grade, iniciar e desenhar uma área (Area.cpp) ----------

// A área (0xbf1 bytes, 249 por Display a partir de Display+4; criada pela lista 0x55fba0): +0 x1,
// +4 y1, +8 x2, +0xc y2; +0x10 a primeira e +0x14 a última linha dos itens; +0x18 as marcas; as
// grades de no máximo 30 colunas e 30 linhas: +0x1c início e +0x10c fim das colunas, +0x94 início e
// +0x184 fim das linhas (i32, relativos a x1/y1); +0x1fc a linha pedida para a rolagem; +0x200 a
// imagem guardada; +0x204 as áreas; +0x208 os itens (0x18c bytes cada); +0x20c a cor; +0x20e o item
// dono (áreas que abrem por cima, -1 nas outras); +0x212 a lista dos itens (1200 i16); +0xb72 o
// índice da área; +0xb74 quantos itens; +0xb76 o primeiro item à mostra; +0xb7a..0xbab a barra de
// rolagem; +0xbab colunas e +0xbac linhas (u8); +0xbad os pesos das colunas e +0xbcb os das linhas
// (30 u8 cada); +0xbe9 aberta; +0xbed tem barra de rolagem.

/// O item `k` (i16) da tabela de itens `base` (0x18c bytes cada).
pub(crate) fn area_item(base: usize, k: i16) -> usize {
    (base as isize).wrapping_add(k as isize * 0x18c) as usize
}

/// O item da posição `i` (i16) da lista da área (+0x212).
pub(crate) unsafe fn area_item_da_lista(area: usize, i: i16) -> usize {
    let k = core::ptr::read_unaligned((area as isize + i as isize * 2 + 0x212) as *const i16);
    area_item(*((area + 0x208) as *const usize), k)
}

pub(crate) unsafe fn area_i32(area: usize, desl: isize) -> i32 {
    core::ptr::read_unaligned((area as isize).wrapping_add(desl) as *const i32)
}

pub(crate) unsafe fn area_gravar_i32(area: usize, desl: isize, v: i32) {
    core::ptr::write_unaligned((area as isize).wrapping_add(desl) as *mut i32, v)
}

/// splits the area width and height by the column and row weights (up to 30 each; more raises the Area.cpp 497/507 box)
/// and sets up the scrollbar
/// Original: `Area::layout` (0x403350, Area.cpp, thiscall).
///
/// [pt] { 0x403350 } (thiscall, Area.cpp): organiza a grade da área. Grava o índice da área no item dono
/// (+0x7c, se houver; e zera +0xbe9) e zera o primeiro item à mostra; fora das áreas só de grade
/// (marca 0x80000), com colunas e linhas, calcula as margens (marca 1: 0; senão 2 nas laterais e, em
/// cima e embaixo, 2 com a marca 0x10 ou uma linha só, 8 nas outras), liga a barra de rolagem (0x403600)
/// quando os itens passam das linhas e reparte a largura e a altura pelos pesos. Mais de 30 colunas
/// ou linhas: a original (caixa de erro, Area.cpp 497 e 507).
pub(crate) unsafe extern "thiscall" fn area_layout(area: usize) {
    let marcas = *((area + 0x18) as *const u32);
    let colunas = *((area + 0xbab) as *const u8);
    let linhas = *((area + 0xbac) as *const u8);
    if marcas & 0x80000 == 0 && colunas != 0 && (colunas > 30 || (linhas != 0 && linhas > 30)) {
        // ponytail: a original grava o dono e o +0xb76 antes do erro; repetir é inofensivo (mesmos valores)
        return orig::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_LAYOUT)(area);
    }
    crate::comparacao::ramo("telas organizar a area");
    let dono = core::ptr::read_unaligned((area + 0x20e) as *const i16);
    if dono != -1 {
        *((area + 0xbe9) as *mut u32) = 0;
        let item = area_item(*((area + 0x208) as *const usize), dono);
        core::ptr::write_unaligned((item + 0x7c) as *mut u16, core::ptr::read_unaligned((area + 0xb72) as *const u16));
    }
    core::ptr::write_unaligned((area + 0xb76) as *mut i16, 0);
    if marcas & 0x80000 != 0 || colunas == 0 || linhas == 0 {
        return;
    }
    let (lado, cima): (i32, i32) = if marcas & 1 != 0 {
        (0, 0)
    } else if marcas & 0x10 != 0 || linhas == 1 {
        (2, 2)
    } else {
        (2, 8)
    };
    let mut largura = area_i32(area, 8).wrapping_sub(2 * lado).wrapping_sub(area_i32(area, 0)).wrapping_add(1);
    let altura = (-2 * cima).wrapping_sub(area_i32(area, 4)).wrapping_add(area_i32(area, 0xc)).wrapping_add(1);
    if area_i32(area, 0x14).wrapping_add(1) > linhas as i32 {
        let barra: unsafe extern "thiscall" fn(usize, i32, i32) = core::mem::transmute(FN_AREA_SETUP_SCROLLBAR);
        barra(area, lado, cima);
        largura = largura.wrapping_sub(0x16);
    }
    // colunas: +0x1c início e +0x10c fim, pelos pesos +0xbad
    let pesos = |desl: usize, n: usize| (0..n).map(|k| *((area + desl + k) as *const u8) as i32).fold(0i32, |a, b| a.wrapping_add(b));
    let n = *((area + 0xbab) as *const u8) as i32 - 1;
    let soma = pesos(0xbad, (n + 1) as usize);
    let mut acumulado = 0i32;
    let mut c = 0i32;
    while c <= n {
        acumulado = acumulado.wrapping_add(*((area as isize + c as isize + 0xbad) as *const u8) as i32);
        if c == 0 {
            area_gravar_i32(area, 0x1c, lado);
        } else {
            area_gravar_i32(area, c as isize * 4 + 0x1c, area_i32(area, c as isize * 4 + 0x108).wrapping_add(2));
        }
        if c < *((area + 0xbab) as *const u8) as i32 - 1 {
            area_gravar_i32(area, c as isize * 4 + 0x10c, acumulado.wrapping_mul(largura).wrapping_div(soma).wrapping_add(lado - 1));
        } else {
            area_gravar_i32(area, c as isize * 4 + 0x10c, largura.wrapping_add(lado - 1));
        }
        c += 1;
        if c > *((area + 0xbab) as *const u8) as i32 - 1 {
            break;
        }
    }
    // linhas: +0x94 início e +0x184 fim, pelos pesos +0xbcb
    let n = *((area + 0xbac) as *const u8) as i32 - 1;
    let soma = pesos(0xbcb, (n + 1) as usize);
    let mut acumulado = 0i32;
    let mut l = 0i32;
    while l <= n {
        acumulado = acumulado.wrapping_add(*((area as isize + l as isize + 0xbcb) as *const u8) as i32);
        if l == 0 {
            area_gravar_i32(area, 0x94, cima);
        } else {
            area_gravar_i32(area, l as isize * 4 + 0x94, area_i32(area, l as isize * 4 + 0x180).wrapping_add(2));
        }
        let fim = if l < *((area + 0xbac) as *const u8) as i32 - 1 {
            acumulado.wrapping_mul(altura).wrapping_div(soma).wrapping_add(cima - 1)
        } else {
            altura.wrapping_add(cima - 1)
        };
        area_gravar_i32(area, l as isize * 4 + 0x184, fim);
        l += 1;
        if l > *((area + 0xbac) as *const u8) as i32 - 1 {
            break;
        }
    }
}

/// A área inteira, o +0x7c do item dono (se houver) e o ponteiro da barra de rolagem do teclado
/// (0xad9c58, que a 0x403600 grava) têm de sair iguais.
pub(crate) unsafe extern "thiscall" fn comparar_area_layout(area: usize) {
    let o = orig::<unsafe extern "thiscall" fn(usize)>(IDX_AREA_LAYOUT);
    let marcas = *((area + 0x18) as *const u32);
    let colunas = *((area + 0xbab) as *const u8);
    let linhas = *((area + 0xbac) as *const u8);
    if marcas & 0x80000 == 0 && colunas != 0 && (colunas > 30 || (linhas != 0 && linhas > 30)) {
        return o(area);
    }
    let mut reg = vec![(area, 0xbf1), (0xad9c58, 4)];
    let dono = core::ptr::read_unaligned((area + 0x20e) as *const i16);
    if dono != -1 {
        reg.push((area_item(*((area + 0x208) as *const usize), dono) + 0x7c, 2));
    }
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(&reg, || o(area), || area_layout(area));
    PLACAR.registrar(igual, || format!("area_layout {:#x}: diferencas em {:?}", area, diferencas_da_tela(&mo, &mr)));
}

/// initialises the area: rectangle clamped to the screen, fields, cleared scrollbar and the column and row weights; the
/// visible rows clamped at 30 (the grid size), the rest goes to the scroll bar
/// Original: `Area::init` (0x402ac0, Area.cpp, thiscall).
///
/// [pt] { 0x402ac0 } (thiscall, Area.cpp): inicia a área. Área que abre por cima de um item (`dono` ≠ -1):
/// pesos 1 nas 30 colunas e linhas, retângulo zero, 1 coluna e 0 linhas. Nas outras: ordena x1/x2 e
/// y1/y2 (troca 0x90ba90), x1/y1 no mínimo 0 e x2/y2 no máximo a tela (0x31f/0x257 no exe de 800x600; o
/// exe de 1920x1080 troca esses imediatos, lidos aqui do próprio exe). Grava os campos, zera a barra
/// de rolagem e, fora das áreas só de grade (0x80000), copia os pesos (com 1..29 colunas/linhas e
/// ponteiro) ou enche de 1 (inclusive com 30 ou mais: aí a original passa do vetor de 30, como aqui).
#[allow(clippy::too_many_arguments)]
pub(crate) unsafe extern "thiscall" fn init(
    area: usize, areas: usize, itens: usize, indice: u32, x1: i32, y1: i32, x2: i32, y2: i32, colunas: u32,
    pesos_colunas: usize, linhas: u32, pesos_linhas: usize, marcas: u32, cor: u32, dono: u32,
) {
    crate::comparacao::ramo("telas iniciar a area");
    let linhas = travar_linhas(linhas);
    let (mut x1, mut y1, mut x2, mut y2) = (x1, y1, x2, y2);
    let (mut colunas, mut pesos_colunas, mut linhas, mut pesos_linhas) = (colunas as u8, pesos_colunas, linhas as u8, pesos_linhas);
    if dono as u16 as i16 != -1 {
        core::ptr::write_bytes((area + 0xbad) as *mut u8, 1, 30);
        core::ptr::write_bytes((area + 0xbcb) as *mut u8, 1, 30);
        x1 = 0;
        y1 = 0;
        x2 = 0;
        y2 = 0;
        colunas = 1;
        linhas = 0;
        pesos_colunas = 0;
        pesos_linhas = 0;
    } else {
        let trocar: unsafe extern "C" fn(*mut i32, *mut i32, u32) = core::mem::transmute(FN_UTILS_SWAP_BYTES);
        if x1 > x2 {
            trocar(&mut x1, &mut x2, 4);
        }
        if y1 > y2 {
            trocar(&mut y1, &mut y2, 4);
        }
    }
    let x1 = if x1 > 0 { x1 } else { 0 };
    let y1 = if y1 > 0 { y1 } else { 0 };
    // os limites da tela: imediatos de `cmp edx, imm` e `mov edx, imm` (0x402b7c) e `mov esi, imm` (0x402b8d)
    let x2 = if x2 >= core::ptr::read_unaligned(0x402b7e as *const i32) { core::ptr::read_unaligned(0x402b85 as *const i32) } else { x2 };
    let teto_y = core::ptr::read_unaligned(0x402b8e as *const i32);
    let y2 = if y2 >= teto_y { teto_y } else { y2 };
    area_gravar_i32(area, 0, x1);
    area_gravar_i32(area, 4, y1);
    area_gravar_i32(area, 8, x2);
    area_gravar_i32(area, 0xc, y2);
    *((area + 0xbac) as *mut u8) = linhas;
    *((area + 0xbab) as *mut u8) = colunas;
    area_gravar_i32(area, 0x10, 0x7fffffff);
    area_gravar_i32(area, 0x14, 0);
    core::ptr::write_bytes((area + 0xb7a) as *mut u8, 0, 0x31);
    core::ptr::write_unaligned((area + 0x20e) as *mut u16, dono as u16);
    core::ptr::write_unaligned((area + 0x204) as *mut usize, areas);
    core::ptr::write_unaligned((area + 0x208) as *mut usize, itens);
    core::ptr::write_unaligned((area + 0xb72) as *mut u16, indice as u16);
    core::ptr::write_unaligned((area + 0x20c) as *mut u16, cor as u16);
    area_gravar_i32(area, 0xbed, 0);
    area_gravar_i32(area, 0x200, 0);
    area_gravar_i32(area, 0xba6, 0);
    core::ptr::write_unaligned((area + 0x210) as *mut i16, -1);
    area_gravar_i32(area, 0xbe9, 1);
    core::ptr::write_unaligned((area + 0xb76) as *mut i16, -1);
    core::ptr::write_unaligned((area + 0x18) as *mut u32, marcas);
    area_gravar_i32(area, 0x1fc, -1);
    if marcas & 0x80000 != 0 {
        return;
    }
    for (n, pesos, desl) in [(colunas, pesos_colunas, 0xbad), (linhas, pesos_linhas, 0xbcb)] {
        if pesos != 0 && n != 0 && n < 30 {
            core::ptr::copy(pesos as *const u8, (area + desl) as *mut u8, n as usize);
        } else {
            core::ptr::write_bytes((area + desl) as *mut u8, 1, n as usize);
        }
    }
}

/// A área e os 0xd9 bytes depois dela (os pesos de 30 ou mais linhas passam do fim) têm de sair iguais.
#[allow(clippy::too_many_arguments)]
/// A trava das 30 linhas: a grade da área tem no máximo 30 linhas (vetor fixo em +0xbcb; o Area::layout
/// mostra a caixa "Area.cpp 507" acima disso, e o Area::init já escreve os pesos fora do vetor). Em
/// 1920 × 1080 o relatório de uma partida antiga pede mais; travadas em 30, o resto vai para a barra de
/// rolagem, como em 800 × 600 (onde nunca passa de 30 e a trava não muda nada). Fica no Area::init porque
/// o começo do Display::add_area (0x55fba0) já é desvio do exe de 1080 (escala das coordenadas).
// ponytail: as 30 linhas se espalham pela altura da área (linhas mais altas em 1080); layout pela
// resolução de verdade fica para a modernização (PLANO_LIMITE.md); a original e a cópia recebem a trava
// igual no comparador, então a comparação segue valendo
pub(crate) fn travar_linhas(linhas: u32) -> u32 {
    if linhas & 0xff > 30 {
        crate::comparacao::ramo("telas linhas da area travadas em 30");
        (linhas & !0xff) | 30
    } else {
        linhas
    }
}

pub(crate) unsafe extern "thiscall" fn comparar_init(
    area: usize, areas: usize, itens: usize, indice: u32, x1: i32, y1: i32, x2: i32, y2: i32, colunas: u32,
    pesos_colunas: usize, linhas: u32, pesos_linhas: usize, marcas: u32, cor: u32, dono: u32,
) {
    let o = orig::<unsafe extern "thiscall" fn(usize, usize, usize, u32, i32, i32, i32, i32, u32, usize, u32, usize, u32, u32, u32)>(IDX_INIT);
    let reg = [(area, 0xbcb + 0xff)];
    let (_, _, igual, mo, mr) = crate::comparacao::lado_a_lado(
        &reg,
        || o(area, areas, itens, indice, x1, y1, x2, y2, colunas, pesos_colunas, travar_linhas(linhas), pesos_linhas, marcas, cor, dono),
        || init(area, areas, itens, indice, x1, y1, x2, y2, colunas, pesos_colunas, linhas, pesos_linhas, marcas, cor, dono),
    );
    PLACAR.registrar(igual, || format!("init {:#x}: diferencas em {:?}", area, diferencas_da_tela(&mo, &mr)));
}

/// draws the area: the panel, the grid lines or each visible widget in its cell, and the scrollbar
/// Original: `Area::draw` (0x402e70, Area.cpp, thiscall).
///
/// [pt] { 0x402e70 } (thiscall, Area.cpp): desenha a área. Com `redesenhar`, libera a imagem guardada
/// (0x5e42b0); área que abre por cima guarda o fundo (0x5e3dd0); desenha o painel (0x5e5df0). Área só
/// de grade (0x80000): as linhas da grade (0x5e3730, cor [0xae317e]) e fim. Nas outras: guarda o fundo
/// da barra de rolagem, põe cada item à mostra na sua célula (as grades +0x1c/+0x10c e +0x94/+0x184,
/// quando o item não tem retângulo próprio em +0x28/+0x30) e o desenha (0x5ee590) até pass_ball da última
/// linha; depois a barra (0x403c80). Sem o primeiro item à mostra (+0xb76 = -1): a original (caixa de
/// erro Area.cpp 245 e a organização 0x403350).
pub(crate) unsafe extern "thiscall" fn draw(area: usize, redesenhar: u32) {
    if core::ptr::read_unaligned((area + 0xb76) as *const i16) == -1 {
        return orig::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DRAW)(area, redesenhar);
    }
    crate::comparacao::ramo("telas desenhar a area");
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(FN_GUI_FREE_IMAGE);
    let guardar: unsafe extern "C" fn(i32, i32, i32, i32, usize) -> usize = core::mem::transmute(FN_GUI_SAVE_SCREEN_IMAGE);
    let imagem = (area + 0x200) as *mut usize;
    if redesenhar != 0 && *imagem != 0 {
        liberar(*imagem);
        *imagem = 0;
    }
    if core::ptr::read_unaligned((area + 0x20e) as *const i16) != -1 && *imagem == 0 {
        *imagem = guardar(area_i32(area, 0), area_i32(area, 4), area_i32(area, 8), area_i32(area, 0xc), 0);
    }
    // ponytail: a original empurra edx com a cor em dx e, em cima, sobra de chamadas anteriores
    // (imprevisível); a cor é u16 para o painel, aqui estendida com zero
    let painel: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32, u32) = core::mem::transmute(FN_GUI_DRAW_PANEL);
    let cor = core::ptr::read_unaligned((area + 0x20c) as *const u16) as u32;
    painel(area_i32(area, 0), area_i32(area, 4), area_i32(area, 8), area_i32(area, 0xc), *((area + 0x18) as *const u32), cor, 0);
    if *((area + 0x18) as *const u32) & 0x80000 != 0 {
        let linha: unsafe extern "C" fn(i32, i32, i32, i32, u32, u32) = core::mem::transmute(FN_GRAPHICS_DRAW_LINE);
        let mut k: i16 = 1;
        while *((area + 0xbac) as *const u8) > 1 && k < *((area + 0xbac) as *const u8) as i16 {
            let y1 = area_i32(area, 4);
            let y = area_i32(area, 0xc).wrapping_sub(y1).wrapping_add(1).wrapping_mul(k as i32) / *((area + 0xbac) as *const u8) as i32;
            let y = y.wrapping_add(y1);
            linha(area_i32(area, 0), y, area_i32(area, 8), y, 2, *(G_GUI_COLOUR_GREY as *const u16) as u32);
            k = k.wrapping_add(1);
        }
        let mut k: i16 = 1;
        while *((area + 0xbab) as *const u8) > 1 && k < *((area + 0xbab) as *const u8) as i16 {
            let x1 = area_i32(area, 0);
            let x = area_i32(area, 8).wrapping_sub(x1).wrapping_add(1).wrapping_mul(k as i32) / *((area + 0xbab) as *const u8) as i32;
            let x = x.wrapping_add(x1);
            linha(x, area_i32(area, 4), x, area_i32(area, 0xc), 2, *(G_GUI_COLOUR_GREY as *const u16) as u32);
            k = k.wrapping_add(1);
        }
        return;
    }
    if area_i32(area, 0xbed) != 0 && *imagem == 0 {
        *imagem = guardar(area_i32(area, 0), area_i32(area, 4), area_i32(area, 8).wrapping_sub(0x16), area_i32(area, 0xc), 0);
    }
    let desenhar_item: unsafe extern "thiscall" fn(usize, u32) = core::mem::transmute(FN_GUIO_DRAW);
    let total = || core::ptr::read_unaligned((area + 0xb74) as *const i16);
    let mut i = core::ptr::read_unaligned((area + 0xb76) as *const i16);
    let mut parar = false;
    while i < total() && !parar {
        let item = area_item_da_lista(area, i);
        let primeiro = area_item_da_lista(area, core::ptr::read_unaligned((area + 0xb76) as *const i16));
        let d = area_i32(item, 0x24).wrapping_sub(area_i32(primeiro, 0x24));
        if d > *((area + 0xbac) as *const u8) as i32 - 1 {
            parar = true;
        } else {
            area_posicionar_item(area, item, d);
            desenhar_item(area_item_da_lista(area, i), redesenhar);
        }
        i = i.wrapping_add(1);
    }
    if area_i32(area, 0xbed) != 0 {
        let barra: unsafe extern "thiscall" fn(usize) = core::mem::transmute(FN_AREA_DRAW_SCROLLBAR);
        barra(area);
    }
}

/// O retângulo do item na tela (+0x10 x1, +0x14 y1, +0x18 x2, +0x1c y2): o próprio (+0x28..+0x34)
/// ou, onde for 0, a célula da grade (coluna +0x20; linha = primeira linha da área + `d`).
pub(crate) unsafe fn area_posicionar_item(area: usize, item: usize, d: i32) {
    let (mut x1, mut x2, mut y1, mut y2) = (area_i32(item, 0x28), area_i32(item, 0x2c), area_i32(item, 0x30), area_i32(item, 0x34));
    if x1 == 0 {
        let c = area_i32(item, 0x20) as isize;
        x1 = area_i32(area, c * 4 + 0x1c).wrapping_add(area_i32(area, 0));
        x2 = area_i32(area, c * 4 + 0x10c).wrapping_add(area_i32(area, 0));
    }
    if y1 == 0 {
        let l = area_i32(area, 0x10).wrapping_add(d) as isize;
        y1 = area_i32(area, l * 4 + 0x94).wrapping_add(area_i32(area, 4));
        y2 = area_i32(area, l * 4 + 0x184).wrapping_add(area_i32(area, 4));
    }
    area_gravar_i32(item, 0x10, x1);
    area_gravar_i32(item, 0x18, x2);
    area_gravar_i32(item, 0x14, y1);
    area_gravar_i32(item, 0x1c, y2);
}

// ponytail: desenhar guarda e libera imagens (aloca) e escurece o fundo (desenhar duas vezes escurece
// duas vezes): no modo comparação só a original roda; a cópia se prova no jogo do Lucas em modo Rust
// (contador "telas desenhar a area" no ddraw.log)
pub(crate) unsafe extern "thiscall" fn comparar_draw(area: usize, redesenhar: u32) {
    orig::<unsafe extern "thiscall" fn(usize, u32)>(IDX_DRAW)(area, redesenhar)
}

