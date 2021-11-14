default: dev repl

repl: cljsdeps
	clj -M:dev:fig --build dev --repl

cljsdeps:
	./bin/mkcljsdeps

dev:
	npm start

clean:
	rm -rf target/public
