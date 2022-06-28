NAME = py_checker
FLAGS ?= --input-file tests/sample-urls.txt --asynchronous

init: .venv/bin/activate.fish
	source $(PWD)/.venv/bin/activate.fish

compile: $(NAME)/__main__.py
	pyinstaller $(NAME)/__main__.py --name=$(NAME) --onefile

run: $(NAME)/__main__.py
	@python -m $(NAME) $(FLAGS)

types: $(wildcard $(NAME)/*.py)
	@merge-pyi -i $< .pytype/pyi/$(NAME)/$(shell basename $< .py).pyi

clean:
	rm -rdf build dist $(NAME)/__pycache__/ $(NAME).spec
