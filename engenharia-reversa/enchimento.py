r"""Reproduz o save acima de 2 GB sem precisar de uma carreira de anos (158a leva).

O save junta todos os *.tmp da pasta do jogo num contêiner (zipdir.cpp, pack 0x933610), na ordem
do nome. Um "0000_enchimento.tmp" grande entra primeiro e empurra os itens de verdade para além
de 2 GB: na original a gravação perde a posição (ftell = -1) e a leitura falha (comp_stats.cpp
6000/6007); com a 158a leva o save grava e carrega.

Uso:  python enchimento.py <pasta do jogo> [tamanho em MB, padrão 2200]   (cria o arquivo)
      python enchimento.py <pasta do jogo> --apagar                      (apaga)
Criar o arquivo com o jogo já aberto na carreira, logo antes de salvar (o jogo apaga os .tmp ao
começar e ao carregar). É E/S pesada: passa pela trava do laboratório.
"""
import os
import shutil
import sys

from laboratorio import Trava, pode_rodar

NOME = '0000_enchimento.tmp'


def main():
    pasta = sys.argv[1]
    alvo = os.path.join(pasta, NOME)
    if sys.argv[2:] == ['--apagar']:
        if os.path.exists(alvo):
            os.remove(alvo)
        print('apagado')
        return
    mb = int(sys.argv[2]) if len(sys.argv) > 2 else 2200
    if shutil.disk_usage(pasta).free < (2 * mb + 4096) << 20:
        sys.exit('espaço livre insuficiente (o save terá o mesmo tamanho)')
    ok, motivo = pode_rodar()
    if not ok:
        sys.exit(motivo)
    with Trava('enchimento.py'):
        criar(alvo, mb)
    print(f'{alvo}: {os.path.getsize(alvo):,} bytes')


def criar(alvo, mb):
    # ponytail: um bloco aleatório repetido; o RLE do tipo 4 só junta bytes iguais seguidos
    bloco = os.urandom(16 << 20)
    with open(alvo, 'wb') as f:
        for _ in range(mb // 16):
            f.write(bloco)


if __name__ == '__main__':
    main()
