#ifndef PUBSUB_CONTROLLER_CONNECTION_FACTORY_H
#define PUBSUB_CONTROLLER_CONNECTION_FACTORY_H

#include <memory>

#include "google/cloud/pubsub/message.h"
#include "google/cloud/pubsub/subscriber_connection.h"
#include "google/cloud/pubsub/subscriber_options.h"
#include "google/cloud/pubsub/subscription.h"
#include "google/cloud/pubsub/publisher_connection.h"
#include "google/cloud/pubsub/publisher_options.h"

namespace eas::pubsub_controller {
class IConnFactory {
 public:
  virtual std::shared_ptr<google::cloud::pubsub::SubscriberConnection>
  MakeSubscriberConnection(google::cloud::pubsub::Subscription subscription,
                           google::cloud::Options opts) = 0;
  virtual std::shared_ptr<google::cloud::pubsub::PublisherConnection>
  MakePublisherConnection(google::cloud::pubsub::Topic topic,
                          google::cloud::Options opts) = 0;

  virtual ~IConnFactory(){};
};

class PubSubConnFactory final : public IConnFactory {
 public:
  std::shared_ptr<google::cloud::pubsub::SubscriberConnection>
  MakeSubscriberConnection(google::cloud::pubsub::Subscription subscription,
                           google::cloud::Options opts = {}) override;
  std::shared_ptr<google::cloud::pubsub::PublisherConnection>
  MakePublisherConnection(google::cloud::pubsub::Topic topic,
                          google::cloud::Options opts = {}) override;
};
}  // namespace eas::pubsub_controller

#endif  // PUBSUB_CONTROLLER_CONNECTION_FACTORY_H
