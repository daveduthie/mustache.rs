default: repl devServer

repl: clean cljsdeps
	clj -M:dev:fig --build dev --repl # fighweel broken when bundle + modules
	# clj -M -m cljs.main --compile-opts build.edn -w src/cljs -c

cljsdeps:
	./bin/mkcljsdeps

devServer:
	npm start

clean:
	rm -rf target/public
