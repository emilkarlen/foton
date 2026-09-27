import re
import sys

from fs_elem import *

class Elem:
    pass

class ElemDir(Elem, Dir):
    pass

class ElemList(Elem):
    def __init__(self, elements: list[str]):
        self.elements = tuple(elements)

class Defs:
    def __init__(self, defs:dict[str, Elem]):
        self._defs = defs

    def get_dir(self, key: str) -> Dir:
        val = self._defs.get(key)
        if val is None:
            exit_failure('Missing mandatory dir definition: {}'.format(key))
        if not isinstance(val, Dir):
            exit_failure('Definition is not a  dir: {}'.format(key))
        else:
            return val

    def get_list(self, key: str) -> ElemList:
        val = self._defs.get(key)
        if val is None:
            exit_failure('Missing mandatory list definition: {}'.format(key))
        if not isinstance(val, ElemList):
            exit_failure('Definition is not a  list: {}'.format(key))
        else:
            return val

def exit_failure(msg: str):
    print(msg, file=sys.stderr)
    sys.exit(1)


REF_CHAR = '&'

DIR_CONST_BEGIN = '{'
DIR_CONST_END = '}'

LIST_CONST_BEGIN = '['
LIST_CONST_END = ']'

class DefsParser:
    def __init__(self, p: Path, src: str):
        self._p = p
        self._tokens = _tokenize(src)
        self._sym_tbl = {}
        #print('DEBUG: INIT: ' + str(self._tokens))

    def parse(self) -> Defs:
        while len(self._tokens) != 0:
            (ident, dc) = self.consume_def()
            self._sym_tbl[ident] = dc
        return Defs(self._sym_tbl)

    def consume_def(self) -> tuple[str, ElemDir]:
        ident = self.consume_ident()
        self._consume_const(['='])
        value = self.consume_value()
        return (ident, value)

    def consume_value(self) -> ElemDir:
        next_token = self._next_non_empty('DIR-CONSTANT or REFERENCE')
        next_def = self.consume_ref__maybe(next_token)
        if next_def is None:
            next_def = self.consume_constant()
        return next_def

    def consume_ident(self) -> str:
        return self._consume_mandatory_token('IDENTIFIER')

    def consume_ref__maybe(self, next_token: str) -> ElemDir:
        if next_token[0] == REF_CHAR:
            self._consume_next__existing()
            target_def_name = next_token[1:]
            target_def = self._sym_tbl.get(target_def_name)
            if target_def is None:
                self._error('Reference to undefined value: {}'.format(target_def_name))
            else:
                return target_def
        else:
            return None

    def consume_constant(self) -> Elem:
        next_token = self._next_non_empty('DIR-CONSTANT or LIST-CONSTANT')
        if next_token == LIST_CONST_BEGIN:
            return self.consume_list_constant()
        else:
            return self.consume_dir_constant()

    def consume_list_constant(self) -> ElemList:
        ret_val = []
        self._consume_const([LIST_CONST_BEGIN])
        while self._next_is_not(LIST_CONST_END):
            e = self._consume_mandatory_token('LIST-ELEM')
            ret_val += [e]
        self._consume_const(LIST_CONST_END)
        return ElemList(ret_val)

    def consume_dir_constant(self) -> ElemDir:
        ret_val = []
        self._consume_const([DIR_CONST_BEGIN])
        while self._next_is_not(DIR_CONST_END):
            t = self._consume_const(['dir', 'file'])
            name = self._consume_mandatory_token('FS-ELEM-NAME')
            if t == 'file':
                ret_val += [(name, File())]
            elif t == 'dir':
                dc = self.consume_value()
                ret_val += [(name, dc)]
            else:
                self._error('Implementation error: Expecting dir|file, found {}'.format(t))
        self._consume_const(DIR_CONST_END)
        return ElemDir(ret_val)

    def _error(self, msg: str):
        raise Exception('In ' + str(self._p) + ": " + msg + '\n' + str(self._tokens))

    def _consume_const(self, cs: list[str]):
        x = self._consume_mandatory_token(str(cs))
        if x not in cs:
            self._error('Looking for {}, but found {}'.format(str(cs), x))
        return x

    def _next_non_empty(self, expected: str) -> str:
        if not self._tokens:
            exit_failure('Missing token, expected {}'.format(expected))
        else:
            if not self._tokens[0]:
                exit_failure('Next token is empty, expected {}'.format(expected))
            else:
                return self._tokens[0]

    def _next_is(self, x: str) -> bool:
        return self._tokens and self._tokens[0] == x

    def _next_is_not(self, x: str) -> bool:
        return self._tokens and self._tokens[0] != x

    def _consume_next__existing(self):
        if not self._tokens:
            return self._error('Implementation error: missing expected next token')
        else:
            del self._tokens[0]


    def _consume_mandatory_token(self, elem: str) -> str:
        if len(self._tokens) == 0:
            self._error('Expecting {}, but is at end of file'.format(elem))
        else:
            ret_val = self._tokens[0]
            del self._tokens[0]
            return ret_val


def _tokenize(s: str) -> list[Token]:
    return [
        x[1] if x[1] else x[0]
        for x in re.findall(r"'([^']*)'|(\S+)", s)
        ]
