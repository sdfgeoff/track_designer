WASM_PACK_FLAGS = --target web --no-typescript

# If DEBUG=1, add --debug to the WASM_PACK flags
DEBUG ?= 0
ifeq ($(DEBUG), 1)
    WASM_PACK_FLAGS += --debug
else
    WASM_PACK_FLAGS += --release
endif

site:
	cd src; wasm-pack build $(WASM_PACK_FLAGS) site
