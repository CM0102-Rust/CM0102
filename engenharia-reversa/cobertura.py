r"""Prepara a contagem de chamadas (cobertura) das funcoes de um ou mais modulos na simulacao.

Uso: python cobertura.py <trecho do arquivo-fonte> [...]     ex.: cobertura.py match_eng.cpp match_pl.cpp

Escolhe no mapa (docs/mapa_funcoes.csv) as funcoes dos modulos cujo inicio pode ser desviado com
seguranca (mesma analise do desvios.py) e grava jogo\cobertura.txt: "endereco bytes arquivo".
Com CM_COBERTURA=1 (simular.py --cobertura) a DLL poe em cada uma um contador
(inc [contador]; jmp original: nao toca pilha nem registradores, a carreira nao muda) e grava os
totais no ddraw.log quando a simulacao para.
"""
import csv
import os
import struct
import sys

import desvios

AQUI = os.path.dirname(os.path.abspath(__file__))
JOGO = os.path.join(AQUI, '..', 'jogo_testes' if os.path.isdir(os.path.join(AQUI, '..', 'jogo_testes')) else 'jogo')


def main():
    if len(sys.argv) < 2:
        sys.exit(__doc__)
    d, md, inicios, alvos = desvios.preparar(os.path.join(JOGO, 'cm0102.exe'))
    # So inicios comprovados: destino de call (jmp pode ser salto interno) ou ponteiro guardado
    # fora do .text (vtables). O mapa tem pedacos rotulados por vizinhanca que caem no meio de
    # instrucoes; um contador ali corrompe o codigo (o jogo caiu na primeira partida).
    # Varredura linear do .text inteiro: fronteiras reais de instrucao e destinos reais de call.
    # Procurar bytes E8 soltos, ou confiar nos inicios do mapa, deixou passar enderecos no meio de
    # instrucoes (0x6b6163, 0x6fe364): um contador ali corrompe o codigo e derruba o jogo.
    fronteiras, chamados = set(), set()
    md.skipdata = True
    ini, fim = desvios.TEXTO
    for ins in md.disasm_lite(d[ini:fim], ini + desvios.BASE):
        fronteiras.add(ins[0])
        if ins[2] == 'call' and ins[3].startswith('0x'):
            chamados.add(int(ins[3], 16))
    md.skipdata = False
    ponteiros = {struct.unpack_from('<I', d, o)[0] for o in range(desvios.TEXTO[1], len(d) - 3, 4)}
    # As rotinas ja desviadas (Prologo em motor.rs) nao levam contador: os dois desvios no mesmo
    # inicio se atropelam (o trampolim fica vazio e o Rust chama o endereco 0; 17/10/2026).
    import re
    feitas = {int(x, 16) for x in re.findall(r'endereco: (0x[0-9a-f]+)', open(os.path.join(AQUI, '..', 'rust', 'src', 'motor.rs'), encoding='utf-8').read())}
    linhas, recusadas = [], []
    for r in csv.DictReader(open(os.path.join(AQUI, '..', 'docs', 'mapa_funcoes.csv'))):
        # argumentos 0x... sao enderecos avulsos (rotinas sem modulo no mapa, como as do motor que o mapa de
        # maxevans60 atribui a match_man/match_events); os outros, trechos do nome do arquivo-fonte
        avulsos = {int(m, 16) for m in sys.argv[1:] if m.startswith('0x')}
        if int(r['endereco'], 16) not in avulsos and not any(m in r['arquivo'] for m in sys.argv[1:] if not m.startswith('0x')):
            continue
        va = int(r['endereco'], 16)
        if va in feitas:
            continue
        try:
            if va not in chamados and va not in ponteiros:
                raise ValueError('nao e inicio comprovado de rotina')
            if va not in fronteiras:
                raise ValueError('no meio de uma instrucao')
            bs, _ = desvios.analisar(d, va, md, inicios, alvos)
            if len(bs) > int(r['tamanho']):
                raise ValueError('prologo maior que a funcao')
            linhas.append('%x %s %s' % (va, bs.hex(), r['arquivo'].split('/')[-1]))
        except ValueError as e:
            recusadas.append('%#x (%s)' % (va, str(e).split(': ', 1)[-1][:40]))
    with open(os.path.join(JOGO, 'cobertura.txt'), 'w') as f:
        f.write('\n'.join(linhas) + '\n')
    print('%d funcoes com contador em jogo\\cobertura.txt; %d recusadas' % (len(linhas), len(recusadas)))
    for x in recusadas:
        print('  recusada', x)


if __name__ == '__main__':
    main()
