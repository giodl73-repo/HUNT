PAPER = main
DIST_DIR = ../docs

.PHONY: pdf clean dist

pdf:
	pdflatex $(PAPER).tex
	bibtex $(PAPER) || true
	pdflatex $(PAPER).tex
	pdflatex $(PAPER).tex

dist: pdf
	@mkdir -p $(DIST_DIR)
	cp $(PAPER).pdf $(DIST_DIR)/games-same-input-divergence.pdf
	@echo "Copied to $(DIST_DIR)/"

clean:
	rm -f *.aux *.bbl *.blg *.log *.out *.toc *.pdf
