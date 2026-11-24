# O aviso de chave fixa faltando (index.cpp; leitor sx12b, rodada 5, 23/11/2026).
# report_missing_key_id 0x60d780: cdecl(nome); sprintf 0x944c9f(0xde1f64, "Unable to find the %s index", nome), depois a
# caixa "Error" do index.cpp linha 0x16b1 (o mesmo caminho do show_error_at: _splitpath, versao, sprintf, a caixa e a
# marca do mouse 0xb67a34 zerada). Chamada por set_key_ids 0x601a40 (1235 lugares) so quando a chave nao e achada.
# Caminho no jogo para o Lucas: so com uma base editada sem um registro que o exe procura pelo nome (ver
# docs/TESTE_CHAVES.md); na base normal nao roda.
# Comparador: so a original (a caixa).
# Fora (RELATIVAS): get_region_article_form 0x617780 comeca com call 0x66f4d0: entraria com (0x617780, 0).
TELAS = True
SAVE = False
ROTINAS = [('report_missing_key_id', 0x60d780, [0x8b, 0x44, 0x24, 0x04, 0x81, 0xec, 0x00, 0x02, 0x00, 0x00])]
DOCS = ('o aviso de chave fixa faltando',
        'the missing fixed key report',
        [('report_missing_key_id', 'report_missing_key_id', '0x60d780', 'index.cpp', 'cdecl',
          'avisa que falta uma chave fixa no banco de dados: escreve "Unable to find the %s index" com o nome dela no buffer de texto comum e mostra a caixa "Error"',
          'reports a fixed key missing from the database: writes "Unable to find the %s index" with its name into the shared text buffer and shows the "Error" box')])
