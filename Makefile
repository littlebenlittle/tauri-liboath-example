build=$(CURDIR)/dist

build:
	@wasm-pack build --target web --out-name app

package:
	@echo 'syncing...'
	@rsync -r pkg/snippets $(build)/
	@rsync pkg/app.js $(build)/
	@rsync pkg/app_bg.wasm $(build)/
	@echo 'done'

rollup:
	@([ -z "$(x)" ] && echo "not yet working as expected") \
	    || npx rollup --plugin wasm --format=iife --input pkg/app.js --file $(build)/app.js --output.name app

watch:
	cargo watch -w src -s 'make build && make package'

serve:
	@cargo tauri dev
