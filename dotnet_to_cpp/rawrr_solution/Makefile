CPPFLAGS=-std=gnu++14 -g -O2 -fstack-protector-strong -I"/usr/local/lib/R/site-library/Rcpp/include" -I/usr/share/R/include/

cppbuild:
	g++ -Wall -o rawrrRcpp rawrrRcpp.cpp `pkg-config --cflags --libs mono-2` -lm -DMONO_EMBED_CPP_MAIN $(CPPFLAGS)

monobuild:
	xbuild && mono rawrrRcpp.exe 







Rcppbuild:
	R --no-save < Rawrr.R
run: cppbuild monobuild
	./rawrrRcpp	

clean:
	$(RM) -vfr *.json sourceCpp-x86_64-pc-linux-gnu-1.0.7 *.blob
