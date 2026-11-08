// ---------- %%LEVA%%a leva: o contêiner do save: ler, gravar e fechar um item (o RLE do tipo 4) (zipdir.cpp) ----------
//
// O ZFILE (0xa3191 bytes, do zopen 0x9326e0): +0 o FILE da CRT, +4 modo (1 gravando), +8 posição
// lógica (bytes descomprimidos), +0xc o item, +0x10/+0x14 repetição do RLE (entregues/contagem),
// +0x18 o byte da repetição, +0x19 cabeçalho (tipo +0x19, a4 +0x1d, quantos +0x21) e índice
// (+0x25, 0x10c por item: posição, tamanho descomprimido, nome), buffer de leitura +0x41705 e de
// escrita +0x72445 (0x30d40 cada), cursores +0xa3185 (leitura), +0xa3189 (escrita), +0xa318d (lidos).
// RLE: byte <= 0x80 sai como está; > 0x80 = (byte − 0x80) cópias do byte seguinte (1 a 0x7f).

/// Lê um dword sem alinhamento no ZFILE.
unsafe fn zf_read_i32(z: usize, o: usize) -> i32 {
    core::ptr::read_unaligned((z + o) as *const i32)
}

/// Grava um dword sem alinhamento no ZFILE.
unsafe fn zf_write_i32(z: usize, o: usize, v: i32) {
    core::ptr::write_unaligned((z + o) as *mut i32, v)
}

/// O próximo byte comprimido do item: do buffer de leitura (+0x41705, cursor +0xa3185 até +0xa318d)
/// ou, acabado, de um `fread` de 0x30d40 bytes (o primeiro byte do bloco novo, mesmo se veio vazio).
unsafe fn zf_next_byte(z: usize) -> u8 {
    let ler: unsafe extern "C" fn(usize, u32, u32, usize) -> i32 = core::mem::transmute(0x945bfeusize);
    let c = zf_read_i32(z, 0xa3185);
    if c < zf_read_i32(z, 0xa318d) {
        let b = *((z + 0x41705 + c as usize) as *const u8);
        zf_write_i32(z, 0xa3185, c + 1);
        return b;
    }
    // ponytail: o ramo marca o bloco, nao cada chamada (a zread roda milhoes de vezes na carga)
    crate::comparison::branch("save zread bloco");
    zf_write_i32(z, 0xa318d, ler(z + 0x41705, 1, 0x30d40, zf_read_i32(z, 0) as usize));
    let b = *((z + 0x41705) as *const u8);
    zf_write_i32(z, 0xa3185, 1);
    b
}

/// { 0x932c30 } (cdecl, zipdir.cpp): `zread(buf, tam, quantos, z)` como o `fread`, dentro do item
/// aberto para leitura (senão 0): o pedido (`tam × quantos`, int) é cortado no fim do item e soma à
/// posição lógica. Tipo 3: `fread` direto de `pedido / tam` itens. Tipo 4: decodifica o RLE; uma
/// repetição pode atravessar chamadas (+0x10 de +0x14). Devolve `pedido / tam` (divisão com sinal,
/// como a original: `tam` 0 derruba as duas).
unsafe extern "C" fn zipdir_zread(buf: *mut u8, tam: i32, quantos: i32, z: usize) -> i32 {
    let ler: unsafe extern "C" fn(*mut u8, i32, i32, usize) -> i32 = core::mem::transmute(0x945bfeusize);
    if z == 0 || zf_read_i32(z, 4) != 0 {
        return 0;
    }
    let tamanho_do_item = zf_read_i32(z, 0x29 + 0x10c * zf_read_i32(z, 0xc) as usize);
    let mut pedido = tam.wrapping_mul(quantos);
    let pos = zf_read_i32(z, 8);
    if pos > tamanho_do_item.wrapping_sub(pedido) {
        pedido = tamanho_do_item.wrapping_sub(pos);
        if pedido <= 0 {
            return 0;
        }
    }
    let fim = zf_read_i32(z, 8).wrapping_add(pedido);
    zf_write_i32(z, 8, fim);
    if zf_read_i32(z, 0x19) != 4 {
        crate::comparison::branch("save zread tipo 3");
        let q = pedido / tam;
        zf_write_i32(z, 8, tam.wrapping_mul(q).wrapping_sub(pedido).wrapping_add(fim));
        return ler(buf, tam, q, zf_read_i32(z, 0) as usize);
    }
    let mut k = 0i32;
    if pedido > 0 {
        'fora: loop {
            // o resto da repetição pendente
            let n = zf_read_i32(z, 0x14);
            if n != 0 && zf_read_i32(z, 0x10) < n {
                loop {
                    if k >= pedido {
                        break 'fora;
                    }
                    *buf.add(k as usize) = *((z + 0x18) as *const u8);
                    k += 1;
                    let e = zf_read_i32(z, 0x10) + 1;
                    zf_write_i32(z, 0x10, e);
                    if e >= zf_read_i32(z, 0x14) {
                        break;
                    }
                }
            }
            if k >= pedido {
                break;
            }
            zf_write_i32(z, 0x14, 0);
            zf_write_i32(z, 0x10, 0);
            loop {
                let b = zf_next_byte(z);
                if b <= 0x80 {
                    *buf.add(k as usize) = b;
                    k += 1;
                } else {
                    zf_write_i32(z, 0x14, b as i32 - 0x80);
                    *((z + 0x18) as *mut u8) = zf_next_byte(z);
                }
                if zf_read_i32(z, 0x14) == 0 {
                    if k >= pedido {
                        break 'fora;
                    }
                    continue;
                }
                if k < pedido {
                    continue 'fora;
                }
                break 'fora;
            }
        }
    }
    pedido / tam
}

// ponytail: lê do arquivo e anda o cursor do FILE e do item; não roda duas vezes. A prova do save
// (carga, --sav) e a gravação (o zseek relê pela zread) passam por aqui.
unsafe extern "C" fn compare_zipdir_zread(buf: *mut u8, tam: i32, quantos: i32, z: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(*mut u8, i32, i32, usize) -> i32>(%%I0%%)(buf, tam, quantos, z)
}

/// Um byte comprimido para o buffer de escrita (+0x72445, cursor +0xa3189); cheio, grava o buffer
/// inteiro (`fwrite`) e o byte vai para o começo.
unsafe fn zf_put_byte(z: usize, b: u8) {
    let escrever: unsafe extern "C" fn(usize, u32, i32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    let p = zf_read_i32(z, 0xa3189);
    if p < 0x30d40 {
        *((z + 0x72445 + p as usize) as *mut u8) = b;
        zf_write_i32(z, 0xa3189, p + 1);
        return;
    }
    // ponytail: o ramo marca o bloco, nao cada chamada (a zwrite roda milhoes de vezes no save)
    crate::comparison::branch("save zwrite bloco");
    escrever(z + 0x72445, 1, p, zf_read_i32(z, 0) as usize);
    zf_write_i32(z, 0xa3189, 1);
    *((z + 0x72445) as *mut u8) = b;
}

/// { 0x932e10 } (cdecl, zipdir.cpp): `zwrite(buf, tam, quantos, z)` como o `fwrite`, no item aberto
/// para gravar (senão 0): soma `tam × quantos` ao tamanho do item no índice e à posição lógica.
/// Tipo 3: `fwrite` direto (devolve o que ele devolver). Tipo 4: codifica o RLE byte a byte,
/// guardando a repetição em +0x18/+0x14 entre as chamadas; byte diferente fecha a repetição (1 com
/// byte <= 0x80 sai literal, senão o par 0x80 + n, byte); a 0x7f sai o par e recomeça. Devolve `quantos`.
unsafe extern "C" fn zipdir_zwrite(buf: *const u8, tam: i32, quantos: i32, z: usize) -> i32 {
    let escrever: unsafe extern "C" fn(*const u8, i32, i32, usize) -> i32 = core::mem::transmute(0x945ce6usize);
    if z == 0 || zf_read_i32(z, 4) != 1 {
        return 0;
    }
    let item = 0x29 + 0x10c * zf_read_i32(z, 0xc) as usize;
    let total = tam.wrapping_mul(quantos);
    zf_write_i32(z, item, zf_read_i32(z, item).wrapping_add(total));
    zf_write_i32(z, 8, zf_read_i32(z, 8).wrapping_add(total));
    if zf_read_i32(z, 0x19) != 4 {
        crate::comparison::branch("save zwrite tipo 3");
        return escrever(buf, tam, quantos, zf_read_i32(z, 0) as usize);
    }
    let repetido = (z + 0x18) as *mut u8;
    let mut k = 0i32;
    while k < total {
        let n = zf_read_i32(z, 0x14);
        if n != 0 {
            let c = *repetido;
            if *buf.add(k as usize) == c {
                if n.wrapping_add(0x80) < 0xff {
                    zf_write_i32(z, 0x14, n + 1);
                    k += 1;
                    continue;
                }
                // 0x7f cópias: sai o par e recomeça
                crate::comparison::branch("save zwrite par 0x7f");
                zf_put_byte(z, (n as u8).wrapping_add(0x80));
                zf_put_byte(z, *repetido);
            } else if n <= 1 && c <= 0x80 {
                zf_put_byte(z, c);
            } else {
                let contagem = (n as u8).wrapping_add(0x80);
                zf_put_byte(z, contagem);
                zf_put_byte(z, *repetido);
            }
        }
        zf_write_i32(z, 0x14, 1);
        *repetido = *buf.add(k as usize);
        k += 1;
    }
    quantos
}

// ponytail: grava no arquivo (buffer e fwrite) e muda o índice; não roda duas vezes. A prova do save
// compara o .qst tipo 4 gravado com e sem a leva.
unsafe extern "C" fn compare_zipdir_zwrite(buf: *const u8, tam: i32, quantos: i32, z: usize) -> i32 {
    original_fn::<unsafe extern "C" fn(*const u8, i32, i32, usize) -> i32>(%%I1%%)(buf, tam, quantos, z)
}

/// { 0x932b00 } (cdecl, zipdir.cpp): `zclose(&z)`. Gravando: esvazia o buffer de escrita; se o item
/// tem tamanho, no tipo 4 fecha a repetição pendente direto no arquivo (literal ou par, como no
/// zwrite) e regrava cabeçalho + índice inteiros (0x416ec bytes) no começo (`fseek` 0). Lendo: zera
/// os cursores de leitura. Fecha o FILE, libera o ZFILE e zera o ponteiro.
unsafe extern "C" fn zclose(zz: *mut usize) {
    let escrever: unsafe extern "C" fn(usize, u32, i32, usize) -> u32 = core::mem::transmute(0x945ce6usize);
    let buscar: unsafe extern "C" fn(usize, i32, i32) -> i32 = core::mem::transmute(0x946a22usize);
    let fechar: unsafe extern "C" fn(usize) -> i32 = core::mem::transmute(0x945a09usize);
    let liberar: unsafe extern "C" fn(usize) = core::mem::transmute(0x944c94usize);
    let z = *zz;
    if z == 0 {
        return;
    }
    crate::comparison::branch("save zclose");
    let f = || zf_read_i32(z, 0) as usize;
    if zf_read_i32(z, 4) == 1 {
        let n = zf_read_i32(z, 0xa3189);
        if n > 0 {
            escrever(z + 0x72445, 1, n, f());
            zf_write_i32(z, 0xa3189, 0);
        }
    } else {
        zf_write_i32(z, 0xa318d, 0);
        zf_write_i32(z, 0xa3185, 0);
    }
    if zf_read_i32(z, 4) == 1 && zf_read_i32(z, 0x29 + 0x10c * zf_read_i32(z, 0xc) as usize) != 0 {
        let n = zf_read_i32(z, 0x14);
        if zf_read_i32(z, 0x19) == 4 && n != 0 {
            if n <= 1 && *((z + 0x18) as *const u8) <= 0x80 {
                escrever(z + 0x18, 1, 1, f());
            } else {
                let marca = (n as u8).wrapping_add(0x80);
                escrever(&marca as *const u8 as usize, 1, 1, f());
                escrever(z + 0x18, 1, 1, f());
            }
        }
        buscar(f(), 0, 0);
        escrever(z + 0x19, 0x416ec, 1, f());
    }
    fechar(f());
    zf_write_i32(z, 0, 0);
    liberar(z);
    *zz = 0;
}

// ponytail: grava, fecha o arquivo e libera; efeito único. A prova do save compara o .qst gravado.
unsafe extern "C" fn compare_zclose(zz: *mut usize) {
    original_fn::<unsafe extern "C" fn(*mut usize)>(%%I2%%)(zz)
}
