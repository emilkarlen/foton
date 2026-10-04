import sys

from defs_file import *
from fs_elem import *


def generate(defs: Defs) -> str:
    # _r  : recursive
    # _nr : non-recursive
    # _es : extension short
    # _el : extension long
    extra_options = defs.get_list(DEFS_KEY_EXTRA_OPTIONS)
    src = defs.get_dir(DEFS_KEY_SRC)
    dst = defs.get_dir(DEFS_KEY_DST)
    deletions_r_es  = defs.get_dir(DEFS_KEY_DELETIONS_RECURSIVE)
    deletions_r_el = defs.get_dir(DEFS_KEY_DELETIONS_RECURSIVE_ExtLong)

    deletions_nr_es = deletions_r_es.with_only_root_elements()
    deletions_nr_el = deletions_r_el.with_only_root_elements()

    dst_with_removed_files_r_es  = dst.with_deletions_of(deletions_r_es)
    dst_with_removed_files_nr_es = dst.with_deletions_of(deletions_nr_es)
    dst_with_removed_files_r_el  = dst.with_deletions_of(deletions_r_el)
    dst_with_removed_files_nr_el = dst.with_deletions_of(deletions_nr_el)

    deletion_files_r_es = deletions_r_es.deletion_elements()
    deletion_files_nr_es = deletions_nr_es.deletion_elements()
    deletion_files_r_el = deletions_r_el.deletion_elements()
    deletion_files_nr_el = deletions_nr_el.deletion_elements()

    tmpl_defs = {
        'EXTRA_OPTIONS': exactly.list_val(extra_options.elements),

        'SRC_DIR_FILES'          : src.to_exactly_files_source("", False),
        'SRC_DIR_FILES_CONDITION': src.to_exactly_files_condition("", False),

        'DST_DIR_FILES': dst.to_exactly_files_source("", False),
        'DST_DIR_FILES_CONDITION_BEFORE'  : dst.to_exactly_files_condition("", False),
        'DST_DIR_FILES_CONDITION_AFTER_r_es' : dst_with_removed_files_r_es.to_exactly_files_condition("", False),
        'DST_DIR_FILES_CONDITION_AFTER_nr_es': dst_with_removed_files_nr_es.to_exactly_files_condition("", False),
        'DST_DIR_FILES_CONDITION_AFTER_r_el' : dst_with_removed_files_r_el.to_exactly_files_condition("", False),
        'DST_DIR_FILES_CONDITION_AFTER_nr_el': dst_with_removed_files_nr_el.to_exactly_files_condition("", False),

        'MOV_DIR_FILES_CONDITION_AFTER_r_es' : deletions_r_es.to_exactly_files_condition("", False),
        'MOV_DIR_FILES_CONDITION_AFTER_nr_es': deletions_nr_es.to_exactly_files_condition("", False),
        'MOV_DIR_FILES_CONDITION_AFTER_r_el' : deletions_r_el.to_exactly_files_condition("", False),
        'MOV_DIR_FILES_CONDITION_AFTER_nr_el': deletions_nr_el.to_exactly_files_condition("", False),

        'REPORT_MATCHER_rm_r_es' : exactly.text_matcher_eq_lines(reporting.deletion_lines(DST_DIR_VAR_REF, deletion_files_r_es)),
        'REPORT_MATCHER_rm_nr_es': exactly.text_matcher_eq_lines(reporting.deletion_lines(DST_DIR_VAR_REF, deletion_files_nr_es)),
        'REPORT_MATCHER_mv_r_es' : exactly.text_matcher_eq_lines(reporting.move_lines(DST_DIR_VAR_REF, MOV_DIR_VAR_REF, deletion_files_r_es)),
        'REPORT_MATCHER_mv_nr_es': exactly.text_matcher_eq_lines(reporting.move_lines(DST_DIR_VAR_REF, MOV_DIR_VAR_REF, deletion_files_nr_es)),

        'REPORT_MATCHER_rm_r_el' : exactly.text_matcher_eq_lines(reporting.deletion_lines(DST_DIR_VAR_REF, deletion_files_r_el)),
        'REPORT_MATCHER_rm_nr_el': exactly.text_matcher_eq_lines(reporting.deletion_lines(DST_DIR_VAR_REF, deletion_files_nr_el)),
        'REPORT_MATCHER_mv_r_el' : exactly.text_matcher_eq_lines(reporting.move_lines(DST_DIR_VAR_REF, MOV_DIR_VAR_REF, deletion_files_r_el)),
        'REPORT_MATCHER_mv_nr_el': exactly.text_matcher_eq_lines(reporting.move_lines(DST_DIR_VAR_REF, MOV_DIR_VAR_REF, deletion_files_nr_el)),
        'DEBUG_DELETIONS': deletions_r_es.to_exactly_files_source("", False),
        }
    return XLY_DEFS_TMPL.format_map(tmpl_defs)



DEFS_KEY_SRC = 'SRC'
DEFS_KEY_DST = 'DST'
DEFS_KEY_EXTRA_OPTIONS = 'EXTRA_OPTIONS'
DEFS_KEY_DELETIONS_RECURSIVE         = 'DELETIONS_RECURSIVE'
DEFS_KEY_DELETIONS_RECURSIVE_ExtLong = 'DELETIONS_RECURSIVE_ExtLong'

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

def text-matcher REPORT_MATCHER_rm_r_es  = {REPORT_MATCHER_rm_r_es}
def text-matcher REPORT_MATCHER_rm_nr_es = {REPORT_MATCHER_rm_nr_es}
def text-matcher REPORT_MATCHER_mv_r_es  = {REPORT_MATCHER_mv_r_es}
def text-matcher REPORT_MATCHER_mv_nr_es = {REPORT_MATCHER_mv_nr_es}
def text-matcher REPORT_MATCHER_rm_r_el  = {REPORT_MATCHER_rm_r_el}
def text-matcher REPORT_MATCHER_rm_nr_el = {REPORT_MATCHER_rm_nr_el}
def text-matcher REPORT_MATCHER_mv_r_el  = {REPORT_MATCHER_mv_r_el}
def text-matcher REPORT_MATCHER_mv_nr_el = {REPORT_MATCHER_mv_nr_el}

def files-condition SRC_DIR_CONTENTS =
{SRC_DIR_FILES_CONDITION}

def files-condition DST_DIR_CONTENTS_before =
{DST_DIR_FILES_CONDITION_BEFORE}


def files-condition DST_DIR_CONTENTS_after_r_es =
{DST_DIR_FILES_CONDITION_AFTER_r_es}

def files-condition DST_DIR_CONTENTS_after_nr_es =
{DST_DIR_FILES_CONDITION_AFTER_nr_es}

def files-condition MOV_DIR_CONTENTS_after_r_es =
{MOV_DIR_FILES_CONDITION_AFTER_r_es}

def files-condition MOV_DIR_CONTENTS_after_nr_es =
{MOV_DIR_FILES_CONDITION_AFTER_nr_es}

def files-condition DST_DIR_CONTENTS_after_r_el =
{DST_DIR_FILES_CONDITION_AFTER_r_el}

def files-condition DST_DIR_CONTENTS_after_nr_el =
{DST_DIR_FILES_CONDITION_AFTER_nr_el}

def files-condition MOV_DIR_CONTENTS_after_r_el =
{MOV_DIR_FILES_CONDITION_AFTER_r_el}

def files-condition MOV_DIR_CONTENTS_after_nr_el =
{MOV_DIR_FILES_CONDITION_AFTER_nr_el}
"""
