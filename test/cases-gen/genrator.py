import shutil
import sys
import time
from pathlib import Path
from typing import Optional, Dict, Sequence, Tuple

CASES_FILE = 'cases.txt'
TEMPLATE_DIR = 'template'

SUB_DIR_SYMBOL = 'SUB_DIR'

DEFINITIONS_FILE_NAME = 'defs.xly'

SUITES_FILE_NAME = 'exactly.suite'
COMMON_FILE_NAME = 'common.xly'


def exit_failure(msg: str):
    print(msg, file=sys.stderr)
    sys.exit(1)


class FileName:
    def __init__(self, original: str, renamed: Optional[str]):
        self.original = original
        self.renamed = renamed


class OriginalFileNamesFormatter:
    def __init__(self, file_names: Sequence[FileName]):
        self._file_names = file_names

    def __format__(self, format_spec):
        return '\n'.join(
            [fn.original for fn in self._file_names]
        )


class SymRefFormatter:
    def __init__(self, symbol: str):
        self._symbol = symbol

    def __format__(self, format_spec):
        return '@[{}]@'.format(self._symbol)


class FileNameRenamingsFormatter:
    def __init__(self, file_names: Sequence[FileName]):
        self._file_names = file_names

    @staticmethod
    def _format_file_name(file_name: FileName):
        if file_name.renamed is None:
            return file_name.original
        else:
            return '{} -> {}'.format(file_name.original, file_name.renamed)

    def __format__(self, format_spec):
        return '\n'.join(
            [self._format_file_name(fn) for fn in self._file_names]
        )

def str_lit(s: str) -> str:
    return '\'' + s + '\''

class FilesFormatter:
    def __init__(self, file_names: Sequence[FileName]):
        self._file_names = file_names

    @staticmethod
    def _format_file_name(file_name: FileName):
        return '  file {}'.format(str_lit(file_name.original))

    def __format__(self, format_spec):
        return '\n'.join(
            [self._format_file_name(fn) for fn in self._file_names]
        )


def is_empty_file_matcher(file_name: str) -> str:
    return '  {} : type file && contents is-empty'.format(str_lit(file_name))


class OriginalFilesFormatter:
    def __init__(self, file_names: Sequence[FileName]):
        self._file_names = file_names

    def __format__(self, format_spec):
        return '\n'.join(
            [is_empty_file_matcher(fn.original) for fn in self._file_names]
        )


class RenamedFilesFormatter:
    def __init__(self, file_names: Sequence[FileName]):
        self._file_names = file_names

    def _dst_file(self, fn: FileName) -> str:
        return fn.original if fn.renamed is None else fn.renamed

    def __format__(self, format_spec):
        return '\n'.join(
            [is_empty_file_matcher(self._dst_file(fn)) for fn in self._file_names]
        )


class SuiteListFormatter:
    def __init__(self, suites: Sequence[str]):
        self._suites = suites

    def __format__(self, format_spec):
        return '\n'.join(self._suites)


def in_subdir(subdir: str, fn: FileName) -> FileName:
    def sub(d: str, f: str) -> str:
        return str(Path(d) / f)

    original = sub(subdir, fn.original)
    renamed = None if fn.renamed is None else sub(subdir, fn.renamed)

    return FileName(original, renamed)


def definitions(file_names: Sequence[FileName]) -> str:
    subdir_ref = format(SymRefFormatter(SUB_DIR_SYMBOL))
    subdir_files = [in_subdir(subdir_ref, fn) for fn in file_names]

    return DEFINITIONS.format(
        GENERATION_TIME_STAMP=time.strftime("%a, %d %b %Y %H:%M:%S +0000", time.gmtime()),
        SUB_DIR_SYMBOL=SUB_DIR_SYMBOL,
        ORIGINAL_FILE_LIST=OriginalFileNamesFormatter(file_names),
        RENAMINGS_FILE_LIST=FileNameRenamingsFormatter(file_names),
        FILES=FilesFormatter(file_names),
        ORIGINAL_FILES_MATCHERS=OriginalFilesFormatter(file_names),
        RENAMED_FILES_MATCHERS=RenamedFilesFormatter(file_names),
        ORIGINAL_FILE_LIST__SUBDIR=OriginalFileNamesFormatter(subdir_files),
        RENAMINGS_FILE_LIST__SUBDIR=FileNameRenamingsFormatter(subdir_files),
    )


def read_cases(path: Path) -> Dict[str, FileName]:
    def parse_line(s: str) -> Tuple[str, FileName]:
        parts = s.split('/')
        if len(parts) == 2:
            return parts[0], FileName(parts[1], None)
        else:
            return parts[0], FileName(parts[1], parts[2])

    ret_val = dict()
    for line in path.read_text().splitlines(keepends=False):
        case_name, file_name = parse_line(line)
        ret_val[case_name] = file_name
    return ret_val


def main():
    if len(sys.argv) < 2:
        exit_failure(usage())
    cmd = sys.argv[1]
    if cmd in ['-h', '--help']:
        print(usage())
    elif cmd == 'defs':
        _cli_definitions(sys.argv[2:])
    elif cmd == 'all':
        _cli_all(sys.argv[2:])
    elif cmd == 'individual':
        _cli_individual(sys.argv[2:])
    else:
        exit_failure('Unknown command: ' + cmd)


def _cli_definitions(args: Sequence[str]):
    if len(args) != 1:
        exit_failure(usage())
    cases_file = Path(args[0])
    if not cases_file.is_file():
        exit_failure('Not a regular file: ' + str(cases_file))

    _do_definitions(cases_file)


def _cli_all(args: Sequence[str]):
    if len(args) != 2:
        exit_failure(usage())
    cases_file, template_dir = _parse_setup_files(args)

    dst_dir = Path(args[1])
    if dst_dir.exists():
        exit_failure('DST-DIR: Must not be an existing file: ' + str(template_dir))

    _do_all(cases_file, template_dir, dst_dir)


def _cli_individual(args: Sequence[str]):
    if len(args) != 2:
        exit_failure('individual: Invalid args')
    cases_file, template_dir = _parse_setup_files(args)

    dst_dir = Path(args[1])
    if dst_dir.exists():
        exit_failure('DST-DIR: Must not exist: ' + str(template_dir))

    _do_individual(cases_file, template_dir, dst_dir)


def _parse_setup_files(args: Sequence[str]) -> Tuple[Path, Path]:
    setup_dir = Path(args[0])
    cases_file = setup_dir / CASES_FILE
    template_dir = setup_dir / TEMPLATE_DIR
    if not cases_file.is_file():
        exit_failure('CASES-FILE: Not a regular file: ' + str(cases_file))
    if not template_dir.is_dir():
        exit_failure('TEMPLATE-DIR: Not a dir: ' + str(template_dir))

    return cases_file, template_dir


def _do_definitions(cases_file: Path):
    cases = read_cases(cases_file)
    definitions_text = definitions(list(cases.values()))
    print(definitions_text)


def _do_all(cases_file: Path, tmpl_dir: Path, dst_dir: Path):
    cases = read_cases(cases_file)
    _mk_xly_dir(dst_dir, tmpl_dir, list(cases.values()))


def _do_individual(cases_file: Path, tmpl_dir: Path, dst_dir: Path):
    cases = read_cases(cases_file)
    log('Creating DST-DIR: {}'.format(dst_dir))
    dst_dir.mkdir()
    for case_name, file_name in cases.items():
        log('Generating variant: ' + case_name)
        _mk_xly_dir(dst_dir / case_name, tmpl_dir, [file_name])

    suite_file = dst_dir / SUITES_FILE_NAME
    log('Creating suite: {}'.format(suite_file))
    suite_file.write_text(XLY_SUITE__INDIVIDUAL.format(
        SUITE_LIST=SuiteListFormatter(list(cases.keys()))
    ))

    common_file = dst_dir / COMMON_FILE_NAME
    log('Creating: {}'.format(common_file))
    common_file.write_text(COMMON_FILE__INDIVIDUAL)


def _mk_xly_dir(dst_dir: Path, tmpl_dir: Path, variants: Sequence[FileName]):
    definitions_text = definitions(variants)
    log('Copying template dir')
    shutil.copytree(tmpl_dir, dst_dir)
    definitions_path = dst_dir / DEFINITIONS_FILE_NAME
    log('Writing definitions file {}'.format(definitions_path))
    definitions_path.write_text(definitions_text)


def log(msg):
    print(msg, file=sys.stderr)


XLY_SUITE__INDIVIDUAL = """\
[suites]

{SUITE_LIST}
"""

COMMON_FILE__INDIVIDUAL = """\
including ../common.xly
"""

DEFINITIONS = """\
# Generated {GENERATION_TIME_STAMP}

def string {SUB_DIR_SYMBOL} = subdir.SUBEXT

def string ORIGINAL_FILE_LIST =
<<EOF
{ORIGINAL_FILE_LIST}
EOF

def string RENAMINGS_FILE_LIST =
<<EOF
{RENAMINGS_FILE_LIST}
EOF

def string ORIGINAL_FILE_LIST_subdir =
<<EOF
{ORIGINAL_FILE_LIST__SUBDIR}
EOF

def string RENAMINGS_FILE_LIST_subdir =
<<EOF
{RENAMINGS_FILE_LIST__SUBDIR}
EOF

def files-source FILES =
{{
{FILES}
}}

def files-matcher MATCHES_ORIGINAL_FILES = matches -full
{{
{ORIGINAL_FILES_MATCHERS}
}}

def files-matcher MATCHES_RENAMED_FILES = matches -full
{{
{RENAMED_FILES_MATCHERS}
}}
"""


def usage() -> str:
    return _USAGE.format(
        CASES_FILE=CASES_FILE,
        TEMPLATE_DIR=TEMPLATE_DIR,
    )


_USAGE = """\
COMMANDS
  defs CASES-FILE
  
    Print contents of defs.xly on stdout
  
  all  SETUP-DIR DST-DIR
  
    Creates an Exactly suite in DST-DIR representing cases for all renaming variants.
    
    DST-DIR must not exist.
  
  individual SETUP-DIR DST-DIR
  
    Creates directories in DST-DIR representing individual renaming variants.
    
    DST-DIR must exist as a directory.

SETUP-DIR
  An directory containing:
    {CASES_FILE}
      File with one case per line:
        <case name> <original file name> <replaced file name>
        
      where
        <case-name>          Name of test case usable as dir name (i.e. no strange chars) 
        <original file name> Name of file before processing.
        <replaced file name> Name of file after processing.
                             If the same as original file name, then this field should not be present.

  {TEMPLATE_DIR}
    A directory with Exactly files to be complemented with "defs.xly" generated by
    this program.
    These files are copied and supplied with "defs.xly" to serve as test cases."""

if __name__ == '__main__':
    main()
