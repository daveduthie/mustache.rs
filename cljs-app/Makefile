default: repl devServer

repl: clean cljsdeps
	clj -M:dev:fig --build dev --repl

cljsdeps:
	./bin/mkcljsdeps

devServer:
	npm start

clean:
	rm -rf target/public
