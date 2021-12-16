from django.contrib import admin
from django.urls import path
import api_service.views as views

urlpatterns = [
    path('api/', views.index),
    path('api/get-all-users/', views.get_all_users)
]
