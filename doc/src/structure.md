# API operation mapping structure

Python based OpenStack API binding tools are structured on a resource base,
where every API resource is a class/object having multiple methods for the
resource CRUD and other operations. Moreover microversion differences are also
being dealt inside this single object. This causes method typing being
problematic and not definite (i.e. when create and get operations return
different structures or microversions require modified types).

Since Rust is a strongly typed programming language the same approach is not
going to work (neither this approach proved to be a good one). Every unique API
call (url + method + payload type) is represented by a dedicated module (simple
Enum are still mapped into same module). All RPC-like actions of OpenStack
services are also represented by a dediated module. Also when the
operation supports different request body schemas in different microversions it
is also implemented by a dedicated module. This gives user a better control by
using an object with definite constraints explicitly declaring support of a
certain microversion.
