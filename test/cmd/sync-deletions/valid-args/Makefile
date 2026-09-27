.PHONY: all clean

DEFS_PATH = '*.def'

PGM       = python3 cases-gen/genrator.py
PGM_GEN   = $(PGM) mksuite cases-gen/cases '*.def'
PGM_CLEAN = $(PGM) clean '*.def'

all:
	$(PGM_GEN) multiple-files-in-dst--sub-dirs
	$(PGM_GEN) multiple-files-in-dst--no-sub-dirs
	$(PGM_GEN) single-file-in-dst-no-sub-dirs/any-ext
	$(PGM_GEN) single-file-in-dst-no-sub-dirs/given-ext


clean:
	$(PGM_CLEAN) multiple-files-in-dst--sub-dirs
	$(PGM_CLEAN) multiple-files-in-dst--no-sub-dirs
	$(PGM_CLEAN) single-file-in-dst-no-sub-dirs/any-ext
	$(PGM_CLEAN) single-file-in-dst-no-sub-dirs/given-ext

err:
	$(PGM) INVALID-CMD '*.def' multiple-files-in-dst--sub-dirs
