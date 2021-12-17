FROM python:3.8

WORKDIR /usr/src/api_service

ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1

RUN adduser --disabled-login user
USER user
RUN pip3 install --upgrade pip
COPY ./services/api_service/requirements.txt .
RUN pip3 install -r requirements.txt

COPY ./services/api_service/ .