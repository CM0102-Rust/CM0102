# O veredito da diretoria sobre uma partida (MANAGER_MANAGER::build_board_match_reaction_text 0x6a69c0,
# manager_manager.cpp; leitor sx14, rodada 5, 23/11/2026). Roda depois de cada partida (5861 chamadas na carreira ENG
# ate 2002-06-01; chamada por update_board_confidence_after_match 0x695240, Rust, que so le o retorno, e pelo texto da
# noticia 0x6a6310). Convencoes conferidas na desmontagem:
# - a rotina: thiscall(this, clube, adversario, competicao, marcas, placar, _, _, ids, variante, derrota, sequencia,
#   meta, posicao, melhor, na_liga), ret 0x3c (15 argumentos na pilha; os dois agregados nunca sao lidos; dos cinco
#   ultimos so o byte baixo);
# - 0x53b900 cdecl(clube); 0x6181d0 cdecl(nacao, buf, 100, 9); 0x5c6710 thiscall([0xae2c74], competicao), ret 4;
#   0x6ab2d0 stdcall(ids, vitoria, buf, variante) (Rust, chamada direta); 0x66f4e0 (langlib) e 0x944c9f (sprintf) cdecl;
#   liga da divisao: [[0xadadfc] + [clube+0x57]*4], slot 0x9c (has_relegation) thiscall sem argumentos.
# - o switch das marcas (tabelas de salto 0x6a8098..0x6a8700 e cadeias de cmp) foi tirado por um emulador do despacho
#   (todas as marcas 0..0x1fff), com os argumentos de cada formato empilhados push a push; 0x660 e 0x688 entram por
#   sub/je, fora das tabelas.
# Prologo `sub esp, 0xce8` (6 bytes, fronteira de instrucao, sem relativo). Sem quadro SEH, sem x87, sem sorteio.
# Comparador: lado a lado no buffer de texto do jogo (0xde1f64, 2000 bytes) e no retorno; sem competicao so a original
# (ela le a string da diretoria antes de escreve-la: lixo de pilha, defeito).
SAVE = False
ROTINAS = [('build_board_match_reaction_text', 0x6a69c0, [0x81, 0xec, 0xe8, 0x0c, 0x00, 0x00])]
DOCS = ('o veredito da diretoria sobre uma partida',
        "the board's verdict on a match",
        [('build_board_match_reaction_text', 'MANAGER_MANAGER::build_board_match_reaction_text', '0x6a69c0', 'manager_manager.cpp', 'thiscall, ret 0x3c',
          'escreve no buffer de texto do jogo o veredito da diretoria (ou da federação, numa seleção: "A Federação <nacionalidade>") sobre a partida, pelas marcas: cerca de 45 casos com o seu modelo (com ou sem o placar, a competição antes ou depois do adversário; os que perguntam se é amistoso têm o modelo de amistoso); marcas sem caso escrevem direto o veredito padrão (0x800, 0x400 ou o simples) e devolvem 0. Depois, com a liga em dia, a frase da sequência: vitórias com o clube no máximo 3 posições acima da meta, "estamos muito impressionados"; derrotas com a meta não acima da melhor posição: mais de 4 abaixo da meta, "se a sequência não mudar, novo técnico"; mais de 4 acima numa divisão, perigo de rebaixamento (se a diretoria espera luta contra o rebaixamento e o clube está na zona) ou de perder a boa posição; mais de 2 abaixo, a preocupação pela expectativa da temporada. Sem frase e com as marcas 0x1800, a frase dos jogadores citados. Devolve 1 com "veredito frase" ou "veredito". Defeitos: sem competição cita a string da diretoria antes de montá-la; as frases "esta sequência de vitórias nos mantém no caminho..." e "Tomara que..." nunca são alcançadas',
          "writes into the game's text buffer the board's verdict (or the FA's, for a national team: \"The <nationality> Football Association\") on the match, by its marks: about 45 cases with their template (with or without the score, the competition before or after the opponent; those that ask whether it is a friendly have the friendly template); marks without a case write the default verdict straight away (0x800, 0x400 or the plain one) and return 0. Then, with the league up to date, the run sentence: a winning run with the club at most 3 places above the target, \"we are very impressed\"; a losing run with the target not above the best position: more than 4 below the target, \"if this run is not turned around, a new manager\"; more than 4 above it in a division, the danger of relegation (when the board expects a relegation battle and the club is in the zone) or of losing the good position; more than 2 below, the worry by the season expectation. Without a sentence and with marks 0x1800, the named players' sentence. Returns 1 with \"verdict sentence\" or \"verdict\". Defects: without a competition it quotes the board string before building it; the \"this run of victories is keeping us on track...\" and \"Hopefully...\" sentences are never reached")])
