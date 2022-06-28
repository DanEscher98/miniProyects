from argparse import ArgumentParser, Namespace

def get_args() -> Namespace:
	parser = ArgumentParser(prog='pyinterpreter')
	return parser.parse_args()
	
