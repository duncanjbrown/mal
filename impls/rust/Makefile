
UPPER_STEPS = step4_if_fn_do step5_tco step6_file step7_quote step8_macros step9_try stepA_mal
STEPS = step0_repl step1_read_print step2_eval step3_env $(UPPER_STEPS)

all: $(STEPS)

dist: mal

mal: stepA_mal
	cp $< $@

%: src/bin/%.rs
	cargo build --bin $*
	cp target/debug/$* $@

STEP0_DEPS = Cargo.toml
STEP1_DEPS = $(STEP0_DEPS) src/reader.rs src/types.rs src/printer.rs src/lib.rs
STEP3_DEPS = $(STEP1_DEPS)
STEP4_DEPS = $(STEP3_DEPS) core.rs

step0_repl: $(STEP0_DEPS)
step1_read_print step2_eval: $(STEP1_DEPS)
step3_env: $(STEP3_DEPS)
$(UPPER_STEPS): $(STEP4_DEPS)

.PHONY: clean

clean:
	cargo clean
	rm -f $(STEPS)
	rm -f mal
