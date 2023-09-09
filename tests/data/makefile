.SUFFIXES:
# List of source files and their corresponding object files
SRCS := $(wildcard *.s)
OBJS := $(SRCS:.s=.o)

# Compiler and linker flags
CFLAGS := -lc -e __start

# Default target: build all executables
all: $(OBJS:.o=);

# Pattern rule to build an executable from an object file
%: %.o
	@echo "Building $@ from $<"
	clang $(CFLAGS) $^ -o $@

# Pattern rule to assemble .s files into .o files
%.o: %.s
	@echo "Assembling $< into $@"
	as $(DEBUGFLGS) $(LSTFLGS) $< -o $@