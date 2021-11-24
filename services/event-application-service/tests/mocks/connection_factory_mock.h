#ifndef MOCKS_CONNECTION_FACTORY_MOCK_H
#define MOCKS_CONNECTION_FACTORY_MOCK_H

#include <gmock/gmock.h>

#include "pubsub_controller/connection_factory.h"

class MockConnectionFactory final
    : public eas::pubsub_controller::IConnFactory {
 public:
  MOCK_METHOD(std::shared_ptr<google::cloud::pubsub::SubscriberConnection>,
              MakeSubscriberConnection,
              (google::cloud::pubsub::Subscription subscription,
               google::cloud::Options opts),
              (override));
  MOCK_METHOD(std::shared_ptr<google::cloud::pubsub::PublisherConnection>,
              MakePublisherConnection,
              (google::cloud::pubsub::Topic topic, google::cloud::Options opts),
              (override));
};

#endif // MOCKS_CONNECTION_FACTORY_MOCK_H