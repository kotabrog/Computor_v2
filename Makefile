NAME := computorv2

release:
	@cd computor_v2 && cargo build --release
	@cp computor_v2/target/release/computor_v2 ./$(NAME)

debug:
	@cd computor_v2 && cargo build
	@cp computor_v2/target/debug/computor_v2 ./$(NAME)

$(NAME): release

all: release

clean:
	@cd computor_v2 && cargo clean

fclean: clean
	@rm -f ./$(NAME)

re: fclean all

.PHONY: all clean fclean re release debug
