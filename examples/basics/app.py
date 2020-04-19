from oxidize import App

app = App()

def view_home(request):
    template = "index.html"
    context = {"message": "Hello World!"}
    return template, context
