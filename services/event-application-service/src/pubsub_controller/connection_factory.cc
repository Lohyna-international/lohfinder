#include "connection_factory.h"

namespace pubsub = google::cloud::pubsub;

namespace eas::pubsub_controller {
std::shared_ptr<google::cloud::pubsub::SubscriberConnection>
PubSubConnFactory::MakeSubscriberConnection(pubsub::Subscription subscription,
                                            google::cloud::Options opts) {
  return pubsub::MakeSubscriberConnection(std::move(subscription),
                                          std::move(opts));
}

std::shared_ptr<pubsub::PublisherConnection>
PubSubConnFactory::MakePublisherConnection(google::cloud::pubsub::Topic topic,
                                           google::cloud::Options opts) {
  return pubsub::MakePublisherConnection(std::move(topic), std::move(opts));
}
}  // namespace eas::pubsub_controller