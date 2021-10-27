FILE 	= cli
MODULE 	= newtonraphson
FlAGS	= -Wall -Wextra -pedantic -std=c++17
DEPEND	= problem.cpp $(MODULE).cpp
PORT	= 8000

$(FILE).out : $(DEPEND)
	clang -lstdc++ -o $(FILE).out $(FILE).cpp $(DEPEND) $(FALGS)

run : $(FILE).out
	./$(FILE).out

$(MODULE).js : $(DEPEND)
	emcc -I. -o $(MODULE).js -Oz -s MODULARIZE=1 \
		-s EXPORT_NAME=createModule --bind bindings.cpp $(DEPEND)

web : $(MODULE).js
	python3 -m http.server $(PORT) & \
		brave-browser http://localhost:$(PORT)

clear :
	bash killport.sh
	rm *.out *.wasm $(MODULE).js
