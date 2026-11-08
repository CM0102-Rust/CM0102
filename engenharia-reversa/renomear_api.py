r"""A API pública da infraestrutura em inglês (etapa 4 da reorganização, 08/11/2026; o leitor de fora, rodada 8:
`detours::desviar`, `PASSAGEM`, `VENENO`, `rng::instalar`...). Para cada módulo, um mapa nome velho -> novo: a forma
qualificada (`detours::desviar`) troca em todo o código; dentro do arquivo que define o módulo, a forma simples troca
também (só no código, nunca em comentário nem em texto entre aspas: as linhas do registro ficam).

Uso: python renomear_api.py [--gravar]
"""
import glob
import os
import re
import sys

R = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
SRC = os.path.join(R, 'rust', 'src')
sys.path.insert(0, os.path.join(R, 'engenharia-reversa'))
import traduzir_locais as tl  # noqa: E402

MAPA = {
    'detours': {'desviar_sonda': 'hook_probe', 'desviar': 'hook', 'isolar_importacao': 'isolate_import', 'isolar': 'isolate',
                'RETORNO': 'RETURN_TO', 'PASSAGEM': 'PASSTHROUGH', 'VENENO': 'POISON', 'ISOLAR': 'ISOLATE',
                'resumo_trechos': 'stub_summary', 'reconferir_no_dia': 'recheck_on_day', 'instalar_adiado': 'install_deferred',
                'instalar': 'install', 'contar': 'count_calls', 'preservar_ecx_cdecl': 'preserve_ecx_cdecl',
                'preservar_ecx': 'preserve_ecx', 'aplicar_resolucao': 'apply_resolution', 'exe_com_patch': 'exe_is_patched',
                'conferir_assinaturas': 'check_signatures', 'resumo_assinaturas': 'signature_summary', 'autotestes': 'self_tests'},
    'rng': {'instalar': 'install', 'autoteste': 'self_test', 'Modo': 'Mode', 'Comparar': 'Compare', 'Observar': 'Observe'},
    'simulation': {'instalar': 'install', 'registrar_excecao': 'record_exception', 'partida_terminou': 'match_finished',
                   'interpretar': 'interpret', 'blocos_do_jogo': 'game_blocks'},
    'iat_hooks': {'trocar_importacao': 'replace_import', 'para_cada_importacao': 'for_each_import',
                  'instalar_cedo': 'install_early', 'instalar': 'install'},
    'stack_probe': {'instalar_lista': 'install_list', 'montar': 'build_stub'},
    'spy': {'embrulhar': 'wrap', 'Embrulho': 'Wrapper', 'Tipo': 'Kind', 'Superficie': 'Surface'},
    'extension': {'VERSAO': 'VERSION', 'instante': 'instant'},
    'crt': {'erro': 'error_box'},
    'allocator': {'regiao_do_jogo': 'game_region', 'Alocador': 'Allocator', 'entrar_visivel': 'enter_visible',
                  'sair_visivel': 'leave_visible', 'blocos_do_banco': 'database_blocks', 'pico_dll': 'dll_peak'},
    'date': {'bissexto': 'is_leap_year', 'data_padrao': 'default_date', 'data_nova': 'new_date',
             'dia_da_semana': 'weekday', 'mais_semanas': 'add_weeks', 'ajustar_semana': 'adjust_week',
             'data_dma': 'from_dmy', 'dias_ate': 'days_until', 'instalar_passagem': 'install_passthrough',
             'autoteste': 'self_test', 'instalar': 'install'},
}


def main():
    n = 0
    for p in glob.glob(os.path.join(SRC, '**', '*.rs'), recursive=True):
        t = open(p, encoding='utf-8', newline='').read()
        mod_proprio = os.path.splitext(os.path.basename(p))[0] if os.path.dirname(p) == SRC else None

        def f(s):
            for mod, m in MAPA.items():
                for a, b in m.items():
                    s = re.sub(r'\b%s::%s\b' % (mod, a), '%s::%s' % (mod, b), s)
                    s = re.sub(r'\b(%s::\w+)::%s\b' % (mod, a), r'\1::%s' % b, s)  # rng::Modo::Comparar
                if mod == mod_proprio:
                    for a, b in m.items():
                        s = re.sub(r'(?<![\w"])%s\b' % a, b, s)
            return s
        novo = tl.so_codigo(t, f)
        if novo != t:
            n += 1
            if '--gravar' in sys.argv:
                open(p, 'w', encoding='utf-8', newline='').write(novo)
    print('%d arquivos' % n)


if __name__ == '__main__':
    main()
