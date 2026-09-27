FM_TYPE_FILE = 'type file'
FM_TYPE_DIR  = 'type dir'

FC_ELEM_FILE = 'file'
FC_ELEM_DIR = 'dir'

SUITE_FILE_NAME = 'exactly.suite'

FC_IS_DIR_AND_MATCHES_FULL = 'type dir && dir-contents matches -full'

def list_val(elements) -> str:
    return ' '.join([
        str_const(e)
        for e in elements
        ])

def text_matcher_eq_lines(ls: list[str]) -> str:
    if not ls:
        return 'is-empty'
    else:
        return '\n'.join(['equals', '<<EOF'] + ls + ['EOF'])

def ref_to(s: str) -> str:
    return '@[' + s + ']@'

def str_const(s: str) -> str:
    for ch in s:
        if ch.isspace():
            return "'{}'".format(s)
    return s
