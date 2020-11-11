CC=clang
CFLAGS=--target=$(TARGET) -emit-llvm -O3 -ffreestanding -fno-builtin -Wall -Wno-unused-function

%.bc: %.c
	$(CC) -c $(CFLAGS) $< -o $@

CHARJ=charj.bc
BPF_DEMO=lib.bc

all: $(CHARJ) ${BPF_DEMO}

# target: https://clang.llvm.org/docs/CrossCompilation.html
# don't include swap
$(CHARJ): TARGET=x86_64-apple-darwin-macho # macos

$(BPF_DEMO): TARGET=bpf
