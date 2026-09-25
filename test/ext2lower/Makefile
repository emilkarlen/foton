.PHONY: ind multi all clean

all: multi ind

ind: valid-args/single-path
multi: valid-args/multi

valid-args/single-path:
	python3 cases-gen/genrator.py individual cases-gen $@

valid-args/multi:
	python3 cases-gen/genrator.py all cases-gen $@

clean:
	rm -rf valid-args/multi
	rm -rf valid-args/single-path
