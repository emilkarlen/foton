.PHONY: all clean

all: multi ind

ind:
	python3 cases-gen/genrator.py individual cases-gen single-path

multi:
	python3 cases-gen/genrator.py all cases-gen multi

clean:
	rm -rf multi
	rm -rf single-path
