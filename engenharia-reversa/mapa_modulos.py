r"""Mapa de modulos: liga as funcoes do exe aos arquivos-fonte originais da SI.

Uso: python mapa_modulos.py <exe> <saida.csv> [<resumo.md>]

1. Inicios de funcao: alvos de `call rel32` no .text e ponteiros para o .text guardados em
   .rdata/.data (vtables, tabelas de funcoes), mais o ponto de entrada.
2. Rotulo direto: a funcao que empilha o endereco de um caminho "E:\dev\CM3\...\x.cpp" (usado nas
   mensagens de erro "x.cpp linha") pertence a x.cpp.
3. Rotulo por vizinhanca: o linker do Visual C++ 6 poe o codigo de cada .obj em sequencia, entao
   funcoes sem rotulo entre duas do mesmo arquivo pertencem a ele.
O CSV traz: endereco, tamanho (ate a proxima funcao), arquivo, como foi rotulada (direto/vizinhanca/-).
"""
import bisect
import collections
import re
import struct
import sys

import pefile

BASE = 0x400000


def main():
    if len(sys.argv) < 3:
        sys.exit(__doc__)
    exe, saida = sys.argv[1], sys.argv[2]
    resumo = sys.argv[3] if len(sys.argv) > 3 else None
    d = open(exe, 'rb').read()
    pe = pefile.PE(data=d, fast_load=True)
    secoes = {s.Name.rstrip(b'\0').decode(): s for s in pe.sections}
    texto = secoes['.text']
    t_ini, t_fim = BASE + texto.VirtualAddress, BASE + texto.VirtualAddress + texto.Misc_VirtualSize
    o_ini, o_fim = texto.PointerToRawData, texto.PointerToRawData + texto.Misc_VirtualSize

    # 1. inicios de funcao
    inicios = {BASE + pe.OPTIONAL_HEADER.AddressOfEntryPoint}
    i = d.find(b'\xe8', o_ini, o_fim)
    while i != -1:
        if i + 5 <= o_fim:
            alvo = struct.unpack_from('<i', d, i + 1)[0] + i + 5 + BASE
            if t_ini <= alvo < t_fim:
                inicios.add(alvo)
        i = d.find(b'\xe8', i + 1, o_fim)
    for nome in ('.rdata', '.data'):
        s = secoes[nome]
        bloco = d[s.PointerToRawData:s.PointerToRawData + s.SizeOfRawData]
        for k in range(0, len(bloco) - 3, 4):
            v = struct.unpack_from('<I', bloco, k)[0]
            if t_ini <= v < t_fim:
                inicios.add(v)
    # Descarta alvos que caem em preenchimento ou no meio de instrucoes obvias de dado.
    inicios = sorted(v for v in inicios if d[v - BASE] not in (0x00, 0xcc))

    # 2. rotulo direto pelos caminhos de arquivo-fonte
    fontes = {}
    for m in re.finditer(rb'E:\\dev\\CM3\\cm3 00-01\\([\x20-\x7e]+?\.(?:cpp|h))\x00', d):
        fontes[m.start() + BASE] = m.group(1).decode().replace('\\', '/')
    rotulo = collections.defaultdict(collections.Counter)
    for va, arq in fontes.items():
        agulha = b'\x68' + struct.pack('<I', va)  # push imm32
        k = d.find(agulha, o_ini, o_fim)
        while k != -1:
            f = bisect.bisect_right(inicios, k + BASE) - 1
            if f >= 0:
                rotulo[inicios[f]][arq] += 1
            k = d.find(agulha, k + 1, o_fim)
    arquivo = {f: c.most_common(1)[0][0] for f, c in rotulo.items()}
    como = {f: 'direto' for f in arquivo}

    # 3. vizinhanca: preenche lacunas entre duas funcoes do mesmo arquivo
    ultimo = None
    for idx, f in enumerate(inicios):
        if f in arquivo:
            if ultimo is not None and arquivo[inicios[ultimo]] == arquivo[f]:
                for j in range(ultimo + 1, idx):
                    if inicios[j] not in arquivo:
                        arquivo[inicios[j]] = arquivo[f]
                        como[inicios[j]] = 'vizinhanca'
            ultimo = idx

    with open(saida, 'w', encoding='utf-8', newline='\n') as out:
        out.write('endereco,tamanho,arquivo,rotulo\n')
        for idx, f in enumerate(inicios):
            tam = (inicios[idx + 1] if idx + 1 < len(inicios) else t_fim) - f
            out.write('%#x,%d,%s,%s\n' % (f, tam, arquivo.get(f, ''), como.get(f, '-')))

    n = len(inicios)
    n_dir = sum(1 for v in como.values() if v == 'direto')
    n_viz = sum(1 for v in como.values() if v == 'vizinhanca')
    bytes_rot = sum(((inicios[i + 1] if i + 1 < n else t_fim) - f) for i, f in enumerate(inicios) if f in arquivo)
    print('%d funcoes; %d com rotulo direto, %d por vizinhanca (%.0f%% do codigo); %d arquivos-fonte citados'
          % (n, n_dir, n_viz, 100.0 * bytes_rot / (t_fim - t_ini), len(set(fontes.values()))))

    if resumo:
        # Faixas contiguas por arquivo (para ler o mapa de cima).
        faixas = []
        for idx, f in enumerate(inicios):
            a = arquivo.get(f)
            fim_f = inicios[idx + 1] if idx + 1 < n else t_fim
            if faixas and faixas[-1][2] == a:
                faixas[-1][1] = fim_f
                faixas[-1][3] += 1
            else:
                faixas.append([f, fim_f, a, 1])
        por_arquivo = collections.defaultdict(lambda: [0, 0, []])
        for ini, fim_f, a, k in faixas:
            if a:
                por_arquivo[a][0] += fim_f - ini
                por_arquivo[a][1] += k
                por_arquivo[a][2].append('%#x-%#x' % (ini, fim_f))
        with open(resumo, 'w', encoding='utf-8', newline='\n') as out:
            out.write('# Mapa de módulos do cm0102.exe 3.9.68\n\n')
            out.write('Gerado por `engenharia-reversa/mapa_modulos.py` (ver o cabeçalho dele para o método). '
                      'Tabela completa por função: `docs/mapa_funcoes.csv`.\n\n')
            out.write('%d funções encontradas; %d rotuladas direto pelas mensagens de erro e %d por vizinhança, '
                      'cobrindo %.0f%% do código. Endereços em VA.\n\n' % (n, n_dir, n_viz, 100.0 * bytes_rot / (t_fim - t_ini)))
            out.write('| Arquivo | Funções | Bytes | Faixas |\n|---|---:|---:|---|\n')
            for a, (tam, k, fx) in sorted(por_arquivo.items(), key=lambda x: -x[1][0]):
                out.write('| `%s` | %d | %d | %s |\n' % (a, k, tam, ' '.join(fx[:4]) + (' …' if len(fx) > 4 else '')))


if __name__ == '__main__':
    main()
