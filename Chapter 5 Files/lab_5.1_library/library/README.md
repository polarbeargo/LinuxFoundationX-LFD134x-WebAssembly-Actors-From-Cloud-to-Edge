# Lab 5.1

To run this:

```
wasmcloud -m ./manifest.yaml
```

Then when it's running (give the HTTP server a second to start), you can issue commands to interact with the service:

```
curl -v -X localhost:6147/books/1234
curl -v -X POST localhost:6147/books -d '{"isbn": "1234", "title": "A new book", "description": "The newest of books", "price": 30}'
curl -V -X localhost:6147/books/1234
```

There are other scenarios you can test by sending `PUT` and `DELETE` methods