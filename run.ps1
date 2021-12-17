cp ./secrets/.env.dev ./services/api_service/.env.dev
cp ./secrets/key.json ./services/api_service/key.json
mkdir -p ~/.config/gcloud/
cp ./secrets/key.json ~/.config/gcloud/application_default_credentials.json
docker-compose build
docker-compose up