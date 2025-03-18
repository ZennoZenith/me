default:
    echo 'Hello, world!'

dev:
    bacon run
# 
watch:
    cargo watch -x run

# watch-release:

fmt:
    dprint fmt *

tailwind:
    bunx @tailwindcss/cli -i ./input.css -o ./assets/css/tailwind.css 

tailwind-watch:
    watchexec -e html -- just tailwind

