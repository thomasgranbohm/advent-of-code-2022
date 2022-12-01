COOKIE := $(shell cat ./.cookie)

DAY_OF_THE_MONTH := $(wordlist 2,$(words $(MAKECMDGOALS)),$(MAKECMDGOALS))
$(eval $(DAY_OF_THE_MONTH):;@:)

ifndef DAY_OF_THE_MONTH
DAY_OF_THE_MONTH := $(shell date +%-d)	
endif

DAY_DIR := day-$(shell printf %02d $(DAY_OF_THE_MONTH))

day: download-input
	@echo "Day $(DAY_OF_THE_MONTH) created!"

download-input: create-day
ifeq ($(COOKIE),)
	@echo "Cookie file is empty."
	@exit 1;
endif
	@echo "Downloading new input..."
	@curl -s -b "session=$(COOKIE)" -o "$(DAY_DIR)/src/input.txt" https://adventofcode.com/2022/day/$(DAY_OF_THE_MONTH)/input

create-day:
	@echo "Creating new day..."
	@cargo new -q $(DAY_DIR)
	@echo "Copying template..."
	@cp template.rs $(DAY_DIR)/src/main.rs
