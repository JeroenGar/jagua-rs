import http.server
import socketserver

PORT = 8081

class CORSHTTPRequestHandler(http.server.SimpleHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Cross-Origin-Opener-Policy", "same-origin")
        self.send_header("Cross-Origin-Embedder-Policy", "require-corp")
        super().end_headers()

# Serve current directory
with socketserver.TCPServer(("", PORT), CORSHTTPRequestHandler) as httpd:
    print(f"-- Serving on http://localhost:{PORT}")
    print(f"Open 'http://localhost:{PORT}/index.html' in your browser!!")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\n> Ctrl+C detected, shutting down server.")
        httpd.shutdown()
        httpd.server_close()
        print("> Server stopped cleanly.")
