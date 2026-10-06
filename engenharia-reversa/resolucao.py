r"""Muda a resolucao nativa do CM 01/02 (cria um exe novo e converte as imagens de fundo).

Uso:
  python resolucao.py <exe portatil 800x600> <exe de saida> <largura> <altura> <pasta do jogo>
  ex.: python resolucao.py ..\jogo\cm0102-portatil.exe ..\jogo\cm0102-1080.exe 1920 1080 ..\jogo

O layout das telas e fixo em 800x600 dentro do exe. A comunidade (CM0102Patcher, nckstwrt) mapeou
como leva-lo a outras resolucoes; os dados vem de `referencias/CM0102Patcher` (lidos dali, nao copiados):
  1. patch "to1280x800" (Patcher.cs): prepara o exe para telas maiores, com nomes de arquivo novos
     para os fundos (Data\bkg1280_800.rgn, Data\m800.mbr, Data\g800.mbr);
  2. patch "tapanispacemaker" (Patcher.cs);
  3. constantes de largura/altura e 11 valores de layout (ResolutionChanger.cs, valores calibrados
     para algumas resolucoes; 1920x1080 e uma delas).
Antes de escrever, confere que cada trecho do patch ainda tem os bytes originais do 3.9.68 (assim
nenhuma das correcoes do exe portatil e atropelada) e falha se algo nao bater.

Imagens (formato preservado: RGB565 nos .rgn, RGB555 nos .mbr):
  - Data\default_pic.rgn -> Data\bkg1280_800.rgn na resolucao nova;
  - Data\game.mbr / match.mbr -> Data\g800.mbr / m800.mbr (barra lateral: 126 x altura);
  - Pictures\*.rgn de 800x600 -> resolucao nova, com o original guardado em Pictures-800x600\.
  As fotos sao recortadas no centro para a proporcao nova (sem distorcer) e reescaladas.
"""
import hashlib
import os
import re
import shutil
import struct
import sys

import numpy as np
from PIL import Image

AQUI = os.path.dirname(os.path.abspath(__file__))
PATCHER = os.path.join(AQUI, '..', 'referencias', 'CM0102Patcher')
MD5_PORTATIL = 'eac1b8cc332553ef2a051b1a46fd935a'  # cm0102-portatil.exe gerado por patch_portatil.py


def falha(msg):
    sys.exit('ERRO: ' + msg)


# ---------- dados do CM0102Patcher ----------

def ler_patches():
    """Devolve ({nome: [(offset, bytes)]}, {nome: [(offset, bytes originais)]}) do Patcher.cs."""
    linhas = open(os.path.join(PATCHER, 'Patcher.cs'), encoding='utf-8').read().splitlines()
    ini_p = next(i for i, l in enumerate(linhas) if 'List<HexPatch>> patches =' in l)
    ini_r = next(i for i, l in enumerate(linhas) if 'List<HexPatch>> ReversePatches =' in l)
    hp = re.compile(r'new HexPatch\((0x[0-9A-Fa-f]+|\d+),\s*"([0-9A-Fa-f]+)"\)')

    def dicionario(ini):
        d = {}
        for l in linhas[ini + 1:]:
            if l.strip() == '};':
                break
            m = re.match(r'\s*(?:/\*.*?\*/)?\s*\{\s*"(\w+)"', l)
            if m and m.group(1) not in d:
                d[m.group(1)] = [(int(o, 0), bytes.fromhex(h)) for o, h in hp.findall(l)]
        return d

    return dicionario(ini_p), dicionario(ini_r)


def ler_resolucao():
    """Listas de offsets e os valores de layout por resolucao, do ResolutionChanger.cs."""
    texto = open(os.path.join(PATCHER, 'ResolutionChanger.cs'), encoding='utf-8').read()

    def lista(nome):
        m = re.search(r'List<int> ' + nome + r' = new List<int> \{(.*?)\};', texto, re.S)
        return [int(x, 16) for x in re.findall(r'0x[0-9A-Fa-f]+', m.group(1))]

    listas = {n: lista(n) for n in ('widthOffsets', 'heightOffsets', 'widthMinus1Offsets', 'heightMinus1Offsets',
                                    'unknownsPart1', 'unknownsPart2')}
    calibrados = {}
    for m in re.finditer(r'if \(width == (\d+) && height == (\d+)\)\s*\{(.*?)\n\s*\}', texto, re.S):
        valores = dict((k, int(v)) for k, v in re.findall(r'(\w+) = (\d+);', m.group(3)))
        extras = [(int(o, 16), int(b, 0)) for o, b in re.findall(r'bw\.Seek\((0x[0-9A-Fa-f]+), SeekOrigin\.Begin\);\s*bw\.Write\(\(byte\)(0x[0-9A-Fa-f]+|\d+)\);', m.group(3))]
        calibrados[(int(m.group(1)), int(m.group(2)))] = (valores, extras)
    return listas, calibrados


# ---------- exe ----------

def aplicar_exe(dados, largura, altura):
    patches, originais = ler_patches()
    listas, calibrados = ler_resolucao()
    if (largura, altura) not in calibrados:
        falha('%dx%d nao tem valores de layout calibrados no ResolutionChanger.cs (%s)' % (
            largura, altura, ', '.join('%dx%d' % k for k in sorted(calibrados))))
    valores, extras = calibrados[(largura, altura)]

    escritas = {}  # offset -> byte, para detectar sobreposicao

    def escrever(off, bs, origem):
        for i, b in enumerate(bs):
            escritas[off + i] = (b, origem)
        dados[off:off + len(bs)] = bs

    for nome in ('to1280x800', 'tapanispacemaker'):
        novo, velho = patches[nome], originais[nome]
        if [o for o, _ in novo] != [o for o, _ in velho]:
            falha('patch %s: listas de offsets diferentes no Patcher.cs' % nome)
        for (off, bs), (_, orig) in zip(novo, velho):
            atual = bytes(dados[off:off + len(orig)])
            if atual != orig and atual != bs:
                falha('patch %s em %#x: esperava os bytes originais %s, achei %s (conflito com outra correcao?)'
                      % (nome, off, orig.hex(), atual.hex()))
            escrever(off, bs, nome)

    def palavra(off, v, origem):
        escrever(off, struct.pack('<h', v), origem)

    for off in listas['widthOffsets']:
        palavra(off, largura, 'largura')
    for off in listas['heightOffsets']:
        palavra(off, altura, 'altura')
    for off in listas['widthMinus1Offsets']:
        palavra(off, largura - 1, 'largura-1')
    for off in listas['heightMinus1Offsets']:
        palavra(off, altura - 1, 'altura-1')
    for off, b in extras:
        escrever(off, bytes([b]), 'extra')
    u1 = valores['unknown1']
    for off in listas['unknownsPart1']:
        escrever(off, bytes([u1]), 'layout')
    for off in listas['unknownsPart2']:
        palavra(off, u1 * 10, 'layout')
    palavra(0x201E62, valores['widthMod'], 'layout')
    palavra(0x201E7E, valores['heightMod'], 'layout')
    escrever(0x201E8D, bytes([valores['unknownNegative']]), 'layout')
    escrever(0x201F2B, bytes([u1]), 'layout')
    escrever(0x201F4F, bytes([valores['unknown2']]), 'layout')
    palavra(0x38900F, valores['unknown3'], 'layout')
    palavra(0x470CCE, valores['centering1'], 'layout')
    palavra(0x470F57, valores['centering1'], 'layout')
    palavra(0x4750E2, valores['centering2'], 'layout')
    return len(escritas)


# ---------- imagens ----------

def ler_rgn(caminho):
    """(largura, altura, cabecalho de 36 bytes, pixels uint16). Arquivo: w, h, tamanho, cabecalho, pixels."""
    d = open(caminho, 'rb').read()
    w, h, _ = struct.unpack_from('<3I', d)
    return w, h, d[12:48], np.frombuffer(d, '<u2', w * h, 48).reshape(h, w)


def mascaras(cab):
    """Mascaras R, G, B do cabecalho (bytes 32..44 do arquivo): RGB565 nos .rgn, RGB555 nos .mbr."""
    return struct.unpack_from('<3I', cab, 20)


def _deslocamento(m):
    return (m & -m).bit_length() - 1


def para_imagem(cab, px):
    canais = []
    for m in mascaras(cab):
        sh, mx = _deslocamento(m), m >> _deslocamento(m)
        canais.append((((px.astype(np.uint32) & m) >> sh) * 255 + mx // 2) // mx)
    return Image.fromarray(np.dstack(canais).astype(np.uint8), 'RGB')


def de_imagem(im, cab):
    a = np.asarray(im, dtype=np.uint32)
    px = np.zeros(a.shape[:2], np.uint32)
    for i, m in enumerate(mascaras(cab)):
        sh, mx = _deslocamento(m), m >> _deslocamento(m)
        px |= ((a[:, :, i] * mx + 127) // 255) << sh
    h, w = px.shape
    return struct.pack('<3I', w, h, w * h * 2) + cab + px.astype('<u2').tobytes()


def reescalar(origem, destino, largura, altura, recortar=True):
    w, h, cab, px = ler_rgn(origem)
    im = para_imagem(cab, px)
    if recortar:  # recorta no centro para a proporcao nova, sem distorcer
        alvo = largura / altura
        if w / h > alvo:
            nw = round(h * alvo)
            im = im.crop(((w - nw) // 2, 0, (w - nw) // 2 + nw, h))
        else:
            nh = round(w / alvo)
            im = im.crop((0, (h - nh) // 2, w, (h - nh) // 2 + nh))
    im = im.resize((largura, altura), Image.LANCZOS)
    open(destino, 'wb').write(de_imagem(im, cab))


def converter_imagens(pasta, largura, altura):
    data = os.path.join(pasta, 'Data')
    reescalar(os.path.join(data, 'default_pic.rgn'), os.path.join(data, 'bkg1280_800.rgn'), largura, altura)
    menu = 126 if largura > 800 else 90
    for orig, novo in (('game.mbr', 'g800.mbr'), ('match.mbr', 'm800.mbr')):
        reescalar(os.path.join(data, orig), os.path.join(data, novo), menu, altura, recortar=False)
    fotos = os.path.join(pasta, 'Pictures')
    guarda = os.path.join(pasta, 'Pictures-800x600')
    os.makedirs(guarda, exist_ok=True)
    n = 0
    for nome in sorted(os.listdir(fotos)):
        if not nome.lower().endswith('.rgn'):
            continue
        orig = os.path.join(guarda, nome)
        if not os.path.exists(orig):
            shutil.copy2(os.path.join(fotos, nome), orig)
        w, h = struct.unpack_from('<2I', open(orig, 'rb').read(8))
        if (w, h) != (800, 600):
            continue
        reescalar(orig, os.path.join(fotos, nome), largura, altura)
        n += 1
    return n


def main():
    if len(sys.argv) != 6:
        sys.exit(__doc__)
    origem, destino, largura, altura, pasta = sys.argv[1], sys.argv[2], int(sys.argv[3]), int(sys.argv[4]), sys.argv[5]
    dados = bytearray(open(origem, 'rb').read())
    if hashlib.md5(dados).hexdigest() != MD5_PORTATIL:
        falha('%s nao e o cm0102-portatil.exe gerado por patch_portatil.py' % origem)
    n = aplicar_exe(dados, largura, altura)
    open(destino, 'wb').write(dados)
    print('exe: %d bytes alterados -> %s (md5 %s)' % (n, destino, hashlib.md5(dados).hexdigest()))
    fotos = converter_imagens(pasta, largura, altura)
    print('imagens: fundo padrao, 2 barras laterais e %d fotos em %dx%d' % (fotos, largura, altura))


if __name__ == '__main__':
    main()
