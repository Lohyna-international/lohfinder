FROM python:3.8

WORKDIR /user_service

COPY ./services/user_service/requirements.txt .
COPY ./secrets/google_cloud.json .
COPY ./secrets/firebase.json .
COPY ./secrets/firebase_test.json .

RUN pip3 install -r requirements.txt

COPY ./services/user_service/ .

CMD ["python3", "main.py"]
