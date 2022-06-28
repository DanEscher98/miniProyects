SHELL = /usr/bin/fish
NAME = py_checker
FLAGS ?= --input-file tests/sample-urls.txt --asynchronous

.venv/bin/activate.fish: requirements.txt
	virtualenv .venv --prompt $(NAME)

init: .venv/bin/activate.fish
	source $(PWD)/.venv/bin/activate.fish
	# @$(shell python3 -m pip install -r requirements.txt)

compile: $(NAME)/__main__.py
	pyinstaller $(NAME)/__main__.py --name=$(NAME) --onefile

run: $(NAME)/__main__.py
	@python3 -m $(NAME) $(FLAGS)

types: $(wildcard $(NAME)/*.py)
	@merge-pyi -i $< .pytype/pyi/$(NAME)/$(shell basename $< .py).pyi

clean:
	rm -rdf build dist $(NAME)/__pycache__/ $(NAME).spec
