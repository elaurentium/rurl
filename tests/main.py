import ctypes
import unittest
import os

lib_path = "./target/release/librurl.so"

if not os.path.exists(lib_path):
    raise FileNotFoundError(f"Library not found at {lib_path}. Please build the Rust library first.")

lib = ctypes.cdll.LoadLibrary(lib_path)

class HttpResponse(ctypes.Structure):
    _fields_ = [
        ("status", ctypes.c_char_p),
        ("body", ctypes.c_char_p),
    ]

lib.http_get.restype = ctypes.POINTER(HttpResponse)
lib.http_get.argtypes = [ctypes.c_char_p]

lib.http_post.restype = ctypes.POINTER(HttpResponse)
lib.http_post.argtypes = [ctypes.c_char_p, ctypes.c_char_p]

def to_c_string(py_str: str) -> bytes:
    return py_str.encode('utf-8')

def from_c_string(py_str: str) -> bytes:
    return py_str.encode('utf-8')

class TestClientUrl(unittest.TestCase):
    def test_http_get(self):
        url = to_c_string("http://httpbin.org/get")
        result = lib.http_get(url)
        self.assertIsNotNone(result, "Request GET failed")
        
        status = from_c_string(result.contents.status)
        body = from_c_string(result.contents.body)
        
        self.assertTrue(status.startswith(b"HTTP/1.1 200"), f"Status unexpected: {status}")
        self.assertTrue(len(body) > 0, "Body response is empty")
        

    def test_http_post(self):
        url = to_c_string("http://httpbin.org/post")
        body = to_c_string('{"test": "value"}')
        result = lib.http_post(url, body)
        self.assertIsNotNone(result, "Request POST failed")
        
        status = from_c_string(result.contents.status)
        body = from_c_string(result.contents.body)
        
        self.assertTrue(status.startswith(b"HTTP/1.1 200"), f"Status unexpected: {status}")
        self.assertTrue("test" in body.decode('utf-8'), "Body having unexpected content")
        
        # Liberar memória, se necessário
        # lib.free_response(result)

if __name__ == '__main__':
    unittest.main()
