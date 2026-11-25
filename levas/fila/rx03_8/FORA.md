# rx03_8: glyph_hint_fit_outline_both_axes (0x93c680) — fora desta rodada

*Leitor rx03, rodada 6, 25/11/2026.*

- **O que é:** o ajuste do contorno do ajustador automático da T2K (autogrid): planeja e aplica a passada em x e depois a em y, movendo os pontos de borda marcados em ordem (um monte pelo `glyph_hint_heap_sift_down_by_coord` 0x93d780, já no rx03_7) e interpolando o resto. 4,3 KB, chama 15 rotinas do rx01 (0x933b20…0x934ae0) e o gancho vazio 0x5a8f60 (`ret`).
- **Por que ficou de fora:** é a maior rotina do ajustador e não roda no jogo (só com `T2K_GRID_FIT` numa fonte .t2k; o FONT usa cmd = 2). A prova precisa do mundo do `check_hint.py` do rx01 (ajustador, glifo e restrições sorteados), estendido para chamar o ajuste inteiro; não coube nesta rodada junto com as outras 70 rotinas da parte 3.
- **Quem chama:** o rx01_7 pelo endereço (`FN_GLYPH_HINT_FIT_OUTLINE_BOTH_AXES`); continua o exe até um item próprio.
- **Para a próxima rodada:** item sozinho (rotina grande, partida em pedaços: o plano de x, o de y, a interpolação), com o `check_hint.py` (0x93c680 hoje é um stub lá) e o `e2e.py` do rx03 com cmd 1/3 nas .t2k, que já passa por ela.
