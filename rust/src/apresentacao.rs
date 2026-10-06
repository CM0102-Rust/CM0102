//! Mostra o quadro de 16 bits do jogo na janela, sem trocar o modo de video do Windows.
//!
//! - Janela com area cliente do tamanho do quadro (800x600): copia 1:1, pixel a pixel.
//! - Area cliente maior (tela cheia sem bordas): escala proporcional, com faixas pretas; as
//!   mensagens de mouse sao convertidas de volta para coordenadas do quadro.
//!
//! RGB565 -> 8 bits por canal com replicacao de bits (31 -> 255, 63 -> 255).

use std::sync::Mutex;

use crate::registro::linha;
use crate::win::{self, HWND, RECT};

struct Tela {
    hwnd: usize,
    /// Quadro convertido para 32 bits, guardado entre apresentacoes (so a regiao suja e reconvertida).
    convertido: Vec<u32>,
    largura: i32,
    altura: i32,
    /// Onde o quadro foi desenhado na area cliente na ultima apresentacao.
    destino: RECT,
    proc_original: isize,
}

static TELA: Mutex<Tela> = Mutex::new(Tela {
    hwnd: 0,
    convertido: Vec::new(),
    largura: 0,
    altura: 0,
    destino: RECT { left: 0, top: 0, right: 0, bottom: 0 },
    proc_original: 0,
});

fn tela() -> std::sync::MutexGuard<'static, Tela> {
    TELA.lock().unwrap_or_else(|e| e.into_inner())
}

#[inline]
pub fn rgb565_para_32(p: u16) -> u32 {
    let r = ((p >> 11) & 31) as u32;
    let g = ((p >> 5) & 63) as u32;
    let b = (p & 31) as u32;
    ((r << 3 | r >> 2) << 16) | ((g << 2 | g >> 4) << 8) | (b << 3 | b >> 2)
}

/// Registra a janela do jogo e passa a filtrar as mensagens de mouse dela.
pub unsafe fn definir_janela(hwnd: HWND) {
    let mut t = tela();
    if t.hwnd == hwnd as usize {
        return;
    }
    t.hwnd = hwnd as usize;
    t.proc_original = win::SetWindowLongA(hwnd, win::GWL_WNDPROC, proc_janela as *const () as isize);
    linha(&format!("apresentacao: janela {:p}, procedimento original {:#x}", hwnd, t.proc_original));
}

/// Converte a regiao `suja` do quadro e mostra o quadro inteiro na janela.
pub unsafe fn apresentar(pixels: &[u16], largura: i32, altura: i32, suja: RECT) {
    let mut t = tela();
    if t.hwnd == 0 {
        return;
    }
    let n = (largura * altura) as usize;
    if t.largura != largura || t.altura != altura || t.convertido.len() != n {
        t.largura = largura;
        t.altura = altura;
        t.convertido = vec![0; n];
    }
    let x0 = suja.left.clamp(0, largura);
    let x1 = suja.right.clamp(0, largura);
    let y0 = suja.top.clamp(0, altura);
    let y1 = suja.bottom.clamp(0, altura);
    for y in y0..y1 {
        let ini = (y * largura) as usize;
        let (orig, dest) = (&pixels[ini..ini + largura as usize], &mut t.convertido[ini..ini + largura as usize]);
        for x in x0 as usize..x1 as usize {
            dest[x] = rgb565_para_32(orig[x]);
        }
    }

    let hwnd = t.hwnd as HWND;
    let mut cliente = RECT::default();
    win::GetClientRect(hwnd, &mut cliente);
    let (cw, ch) = (cliente.right, cliente.bottom);
    if cw <= 0 || ch <= 0 {
        return;
    }
    // Escala proporcional; 1:1 quando a area cliente tem o tamanho do quadro.
    let (dw, dh) = if cw * altura <= ch * largura {
        (cw, (cw * altura) / largura)
    } else {
        ((ch * largura) / altura, ch)
    };
    let (ox, oy) = ((cw - dw) / 2, (ch - dh) / 2);
    t.destino = RECT { left: ox, top: oy, right: ox + dw, bottom: oy + dh };

    let info = win::BITMAPINFOHEADER {
        biSize: core::mem::size_of::<win::BITMAPINFOHEADER>() as u32,
        biWidth: largura,
        biHeight: -altura, // de cima para baixo
        biPlanes: 1,
        biBitCount: 32,
        biCompression: 0,
        biSizeImage: 0,
        biXPelsPerMeter: 0,
        biYPelsPerMeter: 0,
        biClrUsed: 0,
        biClrImportant: 0,
    };
    let hdc = win::GetDC(hwnd);
    if hdc.is_null() {
        return;
    }
    if (dw, dh) != (cw, ch) {
        win::PatBlt(hdc, 0, 0, cw, oy, win::BLACKNESS);
        win::PatBlt(hdc, 0, oy + dh, cw, ch - oy - dh, win::BLACKNESS);
        win::PatBlt(hdc, 0, oy, ox, dh, win::BLACKNESS);
        win::PatBlt(hdc, ox + dw, oy, cw - ox - dw, dh, win::BLACKNESS);
    }
    win::SetStretchBltMode(hdc, win::COLORONCOLOR);
    win::StretchDIBits(
        hdc, ox, oy, dw, dh, 0, 0, largura, altura,
        t.convertido.as_ptr() as *const _, &info, win::DIB_RGB_COLORS, win::SRCCOPY,
    );
    win::ReleaseDC(hwnd, hdc);
}

/// Area cliente -> coordenadas do quadro, para o jogo continuar recebendo o mouse em 800x600.
fn converter_mouse(t: &Tela, lp: isize) -> isize {
    let d = t.destino;
    let (dw, dh) = (d.right - d.left, d.bottom - d.top);
    if dw <= 0 || dh <= 0 || (d.left, d.top, dw, dh) == (0, 0, t.largura, t.altura) {
        return lp;
    }
    let x = (lp & 0xffff) as i16 as i32;
    let y = ((lp >> 16) & 0xffff) as i16 as i32;
    let qx = ((x - d.left) * t.largura / dw).clamp(0, t.largura - 1);
    let qy = ((y - d.top) * t.altura / dh).clamp(0, t.altura - 1);
    ((qy as isize) << 16) | (qx as isize & 0xffff)
}

unsafe extern "system" fn proc_janela(hwnd: HWND, msg: u32, wp: usize, lp: isize) -> isize {
    let (original, lp) = {
        let t = tela();
        // WM_MOUSEMOVE..WM_MBUTTONDBLCLK e WM_XBUTTON*: coordenadas da area cliente.
        // WM_MOUSEWHEEL (0x20a) traz coordenadas de tela e nao e convertida.
        let mouse = (0x200..=0x209).contains(&msg) || (0x20b..=0x20d).contains(&msg);
        (t.proc_original, if mouse { converter_mouse(&t, lp) } else { lp })
    };
    win::CallWindowProcA(original, hwnd, msg, wp, lp)
}

#[cfg(test)]
mod testes {
    use super::*;

    #[test]
    fn conversao_565() {
        assert_eq!(rgb565_para_32(0x0000), 0x000000);
        assert_eq!(rgb565_para_32(0xffff), 0xffffff);
        assert_eq!(rgb565_para_32(0xf800), 0xff0000);
        assert_eq!(rgb565_para_32(0x07e0), 0x00ff00);
        assert_eq!(rgb565_para_32(0x001f), 0x0000ff);
        assert_eq!(rgb565_para_32(0x8410), 0x848284); // meio-cinza: 16/31, 32/63, 16/31
    }

    #[test]
    fn mouse_escalado() {
        let t = Tela {
            hwnd: 1,
            convertido: vec![],
            largura: 800,
            altura: 600,
            destino: RECT { left: 240, top: 0, right: 1680, bottom: 1080 },
            proc_original: 0,
        };
        assert_eq!(converter_mouse(&t, (540 << 16) | 960), (300 << 16) | 400); // centro
        assert_eq!(converter_mouse(&t, 100), 0); // faixa preta: fica na borda do quadro
        let um_para_um = Tela { destino: RECT { left: 0, top: 0, right: 800, bottom: 600 }, ..t };
        assert_eq!(converter_mouse(&um_para_um, (10 << 16) | 20), (10 << 16) | 20);
    }
}
