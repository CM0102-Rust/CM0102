# Os textos das notícias do destino dos clubes nos estaduais gaúcho e carioca e na MLS (bra_reg_gaucho.cpp,
# bra_reg_rio.cpp, usa_mls.cpp; leitor rx13, rodada 6, 25/11/2026). Simulação com as 8 ligas (Brasil e EUA); o texto só
# sai com o modo de textos das notícias (CM_TEXTO_NOTICIAS, --textos) e um técnico humano como destinatário (0 chamadas
# na cobertura do mundo até 2002-08-01). Usa o fate_base_text, o fate_headline e o compare_fate_text do rx13_8:
# integrar depois dele (INTEGRAR_ANTES.txt).
# Todas thiscall(clube, destino, fase, a4, a5, a6, a7, a8, noticia, texto) ret 0x28 (cada push conferido).
# - 0x432770 BraRegGaucho e 0x43ab20 BraRegRio: o mesmo corpo (diff das desmontagens: só a fase da final, 0 e 2). Fase
#   0xff: destino 6, "have qualified for the {liga} semi finals." (com notícia) ou "qualify for semi finals", com o nome
#   completo da competição (+4) e o gênero do nome curto (+0x52), os dois vínculos (a competição pelo nome +4); outro
#   destino, base(-1). Fase da final: com notícia base(fase); sem, destino 1 "have won the {liga} championship." (só o
#   vínculo do clube), outro base(fase, a8, 0). Outras: nada. Prólogo mov al, [esp+0xc]; push esi (5).
# - 0x908360 UsaMls: fase (com sinal) < 2 e destino 6: com notícia comp_util_write_group_qualify_news 0x4b4590 cdecl
#   (clube, a4, competição, 6, notícia, texto); sem, "qualify for quarter finals" com o vínculo do clube cujo alvo é o
#   endereço do nome (DEFEITO: push eax duas vezes, o nome +0x38 no lugar do clube). Fases 2 e 3: com notícia
#   0x4b0b80 cdecl(clube, a5, a8, destino, a6, texto); sem, destino 1 "qualify for {liga} final" / "win {liga}" (os dois
#   vínculos), outro 0x4b4420 cdecl(clube, a5, destino, texto). Outras: nada. Prólogo mov eax, [esp+8]; push ebx (5).
# Iguais no cm0102-1080.exe. Comparadores: as manchetes lado a lado (compare_text_writer); o resto só a original.
SAVE = False
LIGAS = 'ENG ESP ITA FRA BRA JPN AUS USA'
ROTINAS = [('bra_reg_gaucho_build_fate_news_text', 0x432770, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('bra_reg_rio_build_fate_news_text', 0x43ab20, [0x8a, 0x44, 0x24, 0x0c, 0x56]),
           ('usa_mls_build_fate_news_text', 0x908360, [0x8b, 0x44, 0x24, 0x08, 0x53])]
DOCS = ('os textos das notícias do destino dos clubes nos estaduais gaúcho e carioca e na MLS',
        'the fate news texts of the Gaucho and Rio state leagues and of the MLS',
        [('bra_reg_gaucho_build_fate_news_text', 'BraRegGaucho::build_fate_news_text', '0x432770', 'bra_reg_gaucho.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino no Gauchão: na temporada, quem passa às semifinais (com a notícia completa ou a manchete curta); na final (fase 0), o campeão sem a notícia completa; os outros destinos, o texto base da competição',
          'the fate news text in the Gaucho state league: over the season, a club through to the semi finals (with the full news or the short headline); in the final (stage 0), the champion without the full news; every other fate, the base competition text'),
         ('bra_reg_rio_build_fate_news_text', 'BraRegRio::build_fate_news_text', '0x43ab20', 'bra_reg_rio.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino no Carioca: como o do Gauchão, com a final na fase 2',
          "the fate news text in the Rio state league: as the Gaucho one, with the final at stage 2"),
         ('usa_mls_build_fate_news_text', 'UsaMls::build_fate_news_text', '0x908360', 'usa_mls.cpp', 'thiscall, ret 0x28',
          'o texto da notícia do destino na MLS: na temporada, quem passa às quartas de final; nas semifinais e na final, o resultado do confronto com a notícia completa, senão "classifica-se para a final" e "vence a liga" para o vencedor e a frase genérica de vitória ou derrota para os outros',
          'the fate news text in the MLS: over the season, a club through to the quarter finals; in the semi finals and the final, the tie result with the full news, otherwise "qualify for the final" and "win the league" for the winner and the generic win or defeat sentence for the others')])
CAMPOS = []
INTERESSA = []
