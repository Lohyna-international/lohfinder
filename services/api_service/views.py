#bussines logic will leave there
from rest_framework.decorators import api_view
from rest_framework.response import Response


@api_view(['GET'])
def index(response):
    print("asd  ")
    context = {
        "method name" : "method url"
    }
    return Response(context) 