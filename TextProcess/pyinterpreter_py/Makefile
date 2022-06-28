NAME = pyinterpreter
FLAGS ?= --help

init: .venv/bin/activate.fish
	source $(PWD)/.venv/bin/activate.fish

compile: $(NAME)/__main__.py
	pyinstaller $(NAME)/__main__.py --name=$(NAME) --onefile

run: $(NAME)/__main__.py
	@python -m $(NAME) $(FLAGS)

clean:
	rm -rdf build dist $(NAME)/__pycache__/ $(NAME).spec
