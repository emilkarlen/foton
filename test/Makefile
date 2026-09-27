#  Generates test cases, needed for running the tests

.PHONY: all clean

all:
	make -C cmd/ext2lower all
	make -C cmd/sync-deletions/valid-args all

clean:
	make -C cmd/ext2lower clean
	make -C cmd/sync-deletions/valid-args clean
