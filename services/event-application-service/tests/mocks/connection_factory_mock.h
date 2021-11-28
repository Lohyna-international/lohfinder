#ifndef MOCKS_CONNECTION_FACTORY_MOCK_H
#define MOCKS_CONNECTION_FACTORY_MOCK_H

#include <gmock/gmock.h>

#include "google/cloud/pubsub/mocks/mock_publisher_connection.h"
#include "google/cloud/pubsub/mocks/mock_subscriber_connection.h"
#include "pubsub_controller/connection_factory.h"

class MockConnectionFactory final
    : public eas::pubsub_controller::IConnFactory {
 public:
  MockConnectionFactory()
      : sub_{std::make_shared<
            google::cloud::pubsub_mocks::MockSubscriberConnection>()},
        pub_{std::make_shared<
            google::cloud::pubsub_mocks::MockPublisherConnection>()} {}

  std::shared_ptr<google::cloud::pubsub::SubscriberConnection>
  MakeSubscriberConnection(google::cloud::pubsub::Subscription /*subscription*/,
                           google::cloud::Options /*opts*/) override {
    return sub_;
  }

  std::shared_ptr<google::cloud::pubsub::PublisherConnection>
  MakePublisherConnection(google::cloud::pubsub::Topic /*topic*/,
                          google::cloud::Options /*opts*/) override {
    return pub_;
  }

  std::shared_ptr<google::cloud::pubsub_mocks::MockSubscriberConnection> sub_;
  std::shared_ptr<google::cloud::pubsub_mocks::MockPublisherConnection> pub_;
};

#endif  // MOCKS_CONNECTION_FACTORY_MOCK_H
