# Oxidize

`oxidize` provides an interface in python for building highly performant
web applications using python for business logic and rust for everything 
that needs to be fast. Compiling SQL Queries, Rendering Templates, and 
routing with complex regular expressions are all done in rust for maximum
performance and memory efficiency. Then when its time to inject business logic,
oxidize binds back into python to get logic for rendering pages, retrieving objects
from the database, and others. 

## Installation
TODO

## Getting Started

Take a look at this basic example to see get a feel for the basics. This source is 
loaded in the examples directory.

```python
from oxidize import App

app = App()

def index_view(request):
    template = "index.html"
    context = {"message": "Hello World!"}
    return template, context
```

In the directory, you will see a sample contains a little more than this. We define a template
and a `config/defaults.yaml`. This contains the routing and template configuration.
