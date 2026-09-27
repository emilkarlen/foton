import sys

from defs_file import *
from fs_elem import *


def generate(defs: Defs) -> str:
    # _r  : recursive
    # _nr : non-recursive
    extra_options = defs.get_list(DEFS_KEY_EXTRA_OPTIONS)
    src = defs.get_dir(DEFS_KEY_SRC)
    dst = defs.get_dir(DEFS_KEY_DST)
    deletions_r  = defs.get_dir(DEFS_KEY_DELETIONS_RECURSIVE)
    deletions_nr = defs.get_dir(DEFS_KEY_DELETIONS_NON_RECURSIVE)

    dst_with_removed_files_r  = dst.with_deletions_of(deletions_r)
    dst_with_removed_files_nr = dst.with_deletions_of(deletions_nr)

    deletion_files_r = deletions_r.deletion_elements()
    deletion_files_nr = deletions_nr.deletion_elements()

    tmpl_defs = {
        'EXTRA_OPTIONS': exactly.list_val(extra_options.elements),

        'SRC_DIR_FILES'          : src.to_exactly_files_source("", False),
        'SRC_DIR_FILES_CONDITION': src.to_exactly_files_condition("", False),

        'DST_DIR_FILES': dst.to_exactly_files_source("", False),
        'DST_DIR_FILES_CONDITION_BEFORE'  : dst.to_exactly_files_condition("", False),
        'DST_DIR_FILES_CONDITION_AFTER_r' : dst_with_removed_files_r.to_exactly_files_condition("", False),
        'DST_DIR_FILES_CONDITION_AFTER_nr': dst_with_removed_files_nr.to_exactly_files_condition("", False),

        'MOV_DIR_FILES_CONDITION_AFTER_r' : deletions_r.to_exactly_files_condition("", False),
        'MOV_DIR_FILES_CONDITION_AFTER_nr': deletions_nr.to_exactly_files_condition("", False),

        'REPORT_MATCHER_rm_r' : exactly.text_matcher_eq_lines(reporting.deletion_lines(DST_DIR_VAR_REF, deletion_files_r)),
        'REPORT_MATCHER_rm_nr': exactly.text_matcher_eq_lines(reporting.deletion_lines(DST_DIR_VAR_REF, deletion_files_nr)),
        'REPORT_MATCHER_mv_r' : exactly.text_matcher_eq_lines(reporting.move_lines(DST_DIR_VAR_REF, MOV_DIR_VAR_REF, deletion_files_r)),
        'REPORT_MATCHER_mv_nr': exactly.text_matcher_eq_lines(reporting.move_lines(DST_DIR_VAR_REF, MOV_DIR_VAR_REF, deletion_files_nr)),
        'DEBUG_DELETIONS': deletions_r.to_exactly_files_source("", False),
        }
    return XLY_DEFS_TMPL.format_map(tmpl_defs)



DEFS_KEY_SRC = 'SRC'
DEFS_KEY_DST = 'DST'
DEFS_KEY_EXTRA_OPTIONS = 'EXTRA_OPTIONS'
DEFS_KEY_DELETIONS_RECURSIVE     = 'DELETIONS_RECURSIVE'
DEFS_KEY_DELETIONS_NON_RECURSIVE = 'DELETIONS_NON_RECURSIVE'

DST_DIR_VAR_REF = exactly.ref_to('DST_DIR')
MOV_DIR_VAR_REF = exactly.ref_to('MOV_DIR')


XLY_DEFS_TMPL = """\
[cases]

*.case


[assert]

exit-code == 0
stderr is-empty


[setup]

including ../common.xly

def string SRC_DIR = src
def string DST_DIR = dst
def string MOV_DIR = move-to

def list EXTRA_OPTIONS = {EXTRA_OPTIONS}

dir @[SRC_DIR]@ =
{SRC_DIR_FILES}

dir @[DST_DIR]@ =
{DST_DIR_FILES}


[before-assert]

def text-matcher REPORT_MATCHER_rm_r  = {REPORT_MATCHER_rm_r}

def text-matcher REPORT_MATCHER_rm_nr = {REPORT_MATCHER_rm_nr}

def text-matcher REPORT_MATCHER_mv_r  = {REPORT_MATCHER_mv_r}

def text-matcher REPORT_MATCHER_mv_nr = {REPORT_MATCHER_mv_nr}

def files-condition SRC_DIR_CONTENTS =
{SRC_DIR_FILES_CONDITION}

def files-condition DST_DIR_CONTENTS_before =
{DST_DIR_FILES_CONDITION_BEFORE}

def files-condition DST_DIR_CONTENTS_after_r =
{DST_DIR_FILES_CONDITION_AFTER_r}

def files-condition DST_DIR_CONTENTS_after_nr =
{DST_DIR_FILES_CONDITION_AFTER_nr}

def files-condition MOV_DIR_CONTENTS_after_r =
{MOV_DIR_FILES_CONDITION_AFTER_r}

def files-condition MOV_DIR_CONTENTS_after_nr =
{MOV_DIR_FILES_CONDITION_AFTER_nr}
"""
