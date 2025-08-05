default:
    echo 'Hello, world!'

dev:
    bacon run
 
watch:
    # watchexec -q -c -w src/ "cargo run"
    watchexec -q -c -w templates/ -w src/  -r --stop-signal SIGKILL "cargo run"
        
watch-test:
    watchexec -q -c -w tests/ "cargo test -q quick_dev -- --nocapture"
# watch-release:

fmt:
    dprint fmt *

tailwind:
    # --optimize
    # -m --minify
    bunx @tailwindcss/cli -i ./input.css -o ./assets/css/tailwind.css

tailwind-watch:
    watchexec -e html -- just tailwind

