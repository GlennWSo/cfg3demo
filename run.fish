echo running debug mode
wasm-pack build --debug --target web --out-name web
echo redirecting server msg to server.log
python -m http.server #> server.log &
# firefox --private-window http://127.0.0.1:8000/
