PAPERS = games-ai-expert-panel-creative \
         games-creative-quality-tiers \
         games-generative-hunt-pipeline \
         games-human-ai-calibration \
         games-persona-authoring-traits \
         games-profile-taxonomy-creative \
         games-same-input-divergence

.PHONY: all pdf clean

all: pdf

pdf:
	@for paper in $(PAPERS); do \
		echo "==> Building $$paper..."; \
		$(MAKE) -C publications/$$paper pdf || echo "WARN: $$paper failed"; \
	done

clean:
	@for paper in $(PAPERS); do \
		$(MAKE) -C publications/$$paper clean; \
	done
