"""Documentacao da 143a leva. Uso: python docs.py "<jogo real>" "<real game>" "<mutacoes>" "<mutations>"."""
import os, sys
sys.path.insert(0, os.path.join(os.path.dirname(os.path.abspath(__file__)), '..', '..', 'engenharia-reversa'))
from gen_docs import fazer

jogo = sys.argv[1] if len(sys.argv) > 1 else '0 divergências'
jogo_en = sys.argv[2] if len(sys.argv) > 2 else '0 divergences'
mut = sys.argv[3] if len(sys.argv) > 3 else ''
mut_en = sys.argv[4] if len(sys.argv) > 4 else ''
M = (mut + " Leva de uma rotina grande (9 KB). Só rodava como unidade de outras fitas: o comparador grava a unidade e, dentro dela, roda a própria comparação (como na 142ª). O evento 0x82 vai para a lista montado na pilha: a 0x68d0d0 confere o conteúdo, não o endereço. Na primeira cadeia a carreira Rust estourou o tempo: o despacho do modo Rust mandava a 0x714d40 e a 0x68d0d0 para a chamada padrão da dividida; o despacho próprio do movimento resolveu (armadilha nova). "
     f"Comparando até 2001-08-20: 0 divergências. Jogo real comparando até 2002-01-01: {jogo}. Carreiras Rust ×2 e original até 2002-06-01: 2.368.855.859 sorteios, impressão 459815f5e0dca92b, partidas 9ad4f994099c32e2, banco c8c977301cbf96ba")
ME = (mut_en + " A batch of one large routine (9 KB). It only ran as a unit of other tapes: the comparator records the unit and, inside it, runs its own comparison (as in batch 142). Event 0x82 goes to the list built on the stack: 0x68d0d0 checks the contents, not the address. In the first chain the Rust career timed out: the Rust-mode dispatch sent 0x714d40 and 0x68d0d0 to the dividida's default call; the movement's own dispatch fixed it (new pitfall). "
      f"Comparing to 2001-08-20: 0 divergences. Real game comparing to 2002-01-01: {jogo_en}. Rust ×2 and original careers to 2002-06-01: 2,368,855,859 draws, fingerprint 459815f5e0dca92b, matches 9ad4f994099c32e2, database c8c977301cbf96ba")
fazer(143, 319, 320, 'o movimento do jogador', 'the player move',
      ["| `mover_jogador()` | 0x6f09f0 | thiscall, ret 0x14 | match_pl.cpp (9 KB): o jogador anda uma casa (duas, se for rápido) para (x, y) com a ação dada. Sem a bola, a lesão na dividida e, sem estar virado para o alvo, só vira. Com a bola: o ataque pelo fôlego e pela ação, a narração da aproximação, o desafiante (falta com o evento 0x82, disputa limpa), a posse e as narrações da ação; depois a grade, o mais adiantado, os marcados que saem, o desgaste pelas marcas da posição e a bola (acompanha ou domina a livre). Comparada pela fita (narração, 0x714d40, domínio, bola parada, bola fora, o evento e as da lesão e da falta como unidades) |"],
      ["| `mover_jogador()` | 0x6f09f0 | thiscall, ret 0x14 | match_pl.cpp (9 KB): the player moves one cell (two, if fast) towards (x, y) with the given action. Without the ball, the tackle injury and, if not facing the target, only turns. With the ball: the attack from stamina and action, the approach commentary, the challenger (foul with event 0x82, clean contest), possession and the action commentary; then the grid, the furthest forward, the marked players released, wear from the position's marks and the ball (follows it or takes the loose one). Compared through the tape (commentary, 0x714d40, control, set piece, ball out, the event and the injury and foul calls as units) |"],
      M, ME,
      [('mover_jogador', 'player_move')],
      "'chute_a_gol': 'shot_on_goal',",
      armadilha=("**Despacho do modo Rust** (143ª leva): cada código que a cópia pede por `chamar` precisa de um ramo no despacho do modo Rust; o que cai no padrão da dividida (0x897760) roda a rotina errada e a carreira trava ou muda. Ao portar rotina com unidades novas, escrever o despacho dela (`despachar_movimento`) e conferir a carreira Rust antes das mutações.",
                 "**Rust-mode dispatch** (batch 143): every code the copy requests through `chamar` needs an arm in the Rust-mode dispatch; one that falls into the dividida's default (0x897760) runs the wrong routine and the career hangs or changes. When porting a routine with new units, write its dispatch (`despachar_movimento`) and check the Rust career before the mutations."))
