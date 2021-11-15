FROM python:3.8

WORKDIR /user_service

COPY requirements.txt .

RUN pip3 install -r requirements.txt

COPY . .

CMD ["python3", "main.py"]
