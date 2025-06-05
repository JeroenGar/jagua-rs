# Running WASM

> [!NOTE]
> 
> You can only do the above steps, if you have **COMPILED**
> using `wasm-pack` that is neatly wrapped in the `build.sh`
> file.
> 

Currently, this can support 2 environments for WASM+JS runtime:

1. Browser (v8 engine)
2. Node

## Runtime on Node 

```bash 
node index.js
```

That's it!

## Runtime on browser (v8)

Basically, you will need to open a live server in order for the browser engine to execute JavaScript, which will in turn,
call WASM `init()` and execute our logic.

Here is a minimal example for Linux (it is very similar for every OS though):

```bash 
python -m http.server 8080
# open index.html in the live server 
xdg-open http://localhost:8080/index.html # if you are on windows, just copy the URL and put it in your browser
```
