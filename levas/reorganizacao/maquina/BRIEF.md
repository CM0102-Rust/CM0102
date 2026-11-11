# Suspeitas no código de máquina: pedido aos revisores (11/11/2026)

As levas de legibilidade 34 a 58 só trocam nomes, mas duas mudaram o comportamento em rotinas que as carreiras não
alcançam (a tela de estatísticas e a notícia do olheiro). Para achar outras, a DLL foi compilada em cada commit e o
código de máquina comparado função a função (`engenharia-reversa/maquina_por_funcao.py`); cada arquivo lista, por
leva, as funções cujos acessos à memória mudaram (deslocamento num registro, largura ou extensão de sinal). O revisor
separa o ruído do compilador (inlining, convenção de chamada interna, `this` devolvido pela chamada) da mudança de
comportamento, e para esta dá o arquivo, a linha e a correção. O pedido completo está em [BRIEF.en.md](BRIEF.en.md).
