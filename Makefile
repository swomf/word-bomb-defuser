word-lists/combined-list.txt: $(wildcard word-lists/component-lists/*.txt)
	awk 1 $^ | tr '[:upper:]' '[:lower:]' | sort -u | awk 'NF' > $@
